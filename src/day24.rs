use std::cmp::{max, min};
use std::convert;
use std::fmt;
use std::ops;
use std::rc::Rc;

use crate::day24_alu::{InputName, Instruction, OpName, RegisterName, RegisterOrConstant};
use crate::types::{AdventResult, Answer, Day, DayPart};
use crate::value_range::ValueRange;

use Instruction::*;
use OpName::*;
use RegisterOrConstant::*;

// A linear polynomial of input values
#[derive(Clone, Copy, Eq, PartialEq)]
struct Polynomial {
    // coefficients for each input, plus one more for a constant
    coefficients: [i64; 15],
}

impl Polynomial {
    fn constant(n: i64) -> Polynomial {
        let mut coefficients = [0; 15];
        coefficients[14] = n;
        Polynomial { coefficients }
    }

    fn input(input_name: InputName) -> Polynomial {
        // TODO: get_range only works if coefficients are positive
        let mut coefficients = [0; 15];
        coefficients[input_name.index()] = 1;
        Polynomial { coefficients }
    }

    fn times(&self, scalar: i64) -> Polynomial {
        let mut coefficients = [0; 15];
        for i in 0..15 {
            coefficients[i] = self.coefficients[i] * scalar;
        }
        Polynomial { coefficients }
    }

    fn modulo(&self, scalar: i64) -> Polynomial {
        let mut coefficients = [0; 15];
        for i in 0..15 {
            coefficients[i] = Mod.perform(self.coefficients[i], scalar);
        }
        Polynomial { coefficients }
    }

    fn get_constant(&self) -> Option<i64> {
        if (0..14).all(|i| self.coefficients[i] == 0) {
            Some(self.coefficients[14])
        } else {
            None
        }
    }

    /// Dividing through by a scalar works if you know the sum of remainders
    /// is less than the scalar, so they can be divide independently.
    fn div(&self, scalar: i64) -> Option<Polynomial> {
        let mut max_remainder = self.coefficients[14] % scalar;
        for i in 0..14 {
            max_remainder += (self.coefficients[i] % scalar) * 9;
        }
        if max_remainder < scalar {
            let mut coefficients = [0; 15];
            for i in 0..15 {
                coefficients[i] = Div.perform(self.coefficients[i], scalar);
            }
            Some(Polynomial { coefficients })
        } else {
            None
        }
    }
}

impl ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coefficients = self.coefficients.clone();
        for i in 0..15 {
            coefficients[i] += other.coefficients[i];
        }
        Polynomial { coefficients }
    }
}

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.coefficients.iter().filter(|c| **c != 0).count();
        if 1 < count {
            write!(f, "[")?;
        }
        let mut first = true;
        for (coefficient, input_name) in self.coefficients.iter().zip(InputName::all().iter()) {
            if *coefficient != 0 {
                if first {
                    first = false;
                } else {
                    write!(f, " + ")?;
                }
                if *coefficient == 1 {
                    write!(f, "{:?}", input_name)?;
                } else {
                    write!(f, "{:?}{:?}", coefficient, input_name)?;
                }
            }
        }
        let constant = self.coefficients[14];
        if constant != 0 || first {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "{:?}", constant)?;
        }
        if 1 < count {
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[test]
fn test_polynomial() {
    let two = Polynomial::constant(2);
    let a = Polynomial::input(InputName::first());
    assert_eq!(Some(2), two.get_constant());
    assert_eq!(None, a.get_constant());
    assert_eq!((a + two).times(5), a.times(5) + two.times(5))
}

#[derive(Clone, Eq, PartialEq)]
enum Expr {
    Poly(Polynomial),
    Op(OpName, NewExpr, NewExpr),
}

impl Expr {
    fn constant(scalar: i64) -> Expr {
        Expr::Poly(Polynomial::constant(scalar))
    }

    fn input(input_name: InputName) -> Expr {
        Expr::Poly(Polynomial::input(input_name))
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Poly(polynomial) => write!(f, "{:?}", polynomial),
            Expr::Op(op_name, a, b) => write!(f, "({:?} {:?} {:?})", a, op_name, b),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct NewExpr {
    details: Rc<Expr>,
}

impl NewExpr {
    /// Matchable details of this expression.
    fn details<'a>(&'a self) -> &'a Expr {
        &*self.details
    }

    /// Returns an expression holding a constant value
    fn constant(n: i64) -> NewExpr {
        NewExpr::poly(Polynomial::constant(n))
    }

    /// Returns an expression holding a polynomial
    fn poly(polynomial: Polynomial) -> NewExpr {
        let details = Rc::new(Expr::Poly(polynomial));
        NewExpr { details }
    }

    /// Returns an expression holding an operation
    fn op(op_name: OpName, lhs: NewExpr, rhs: NewExpr) -> NewExpr {
        let details = Rc::new(Expr::Op(op_name, lhs, rhs));
        NewExpr { details }
    }

    /// Returns the constant value of an expression if it's a polynomial
    /// with only a constant part.
    fn get_constant(&self) -> Option<i64> {
        match self.details() {
            Expr::Poly(polynomial) => polynomial.get_constant(),
            _ => None,
        }
    }

    fn evaluate(&self, inputs: &[i64; 14]) -> i64 {
        match self.details() {
            Expr::Poly(polynomial) => {
                let mut result = polynomial.coefficients[14]; // constant part
                for (input, coefficient) in inputs.iter().zip(polynomial.coefficients) {
                    result += input * coefficient;
                }
                result
            }
            Expr::Op(op_name, lhs, rhs) => {
                op_name.perform(lhs.evaluate(inputs), rhs.evaluate(inputs))
            }
        }
    }
}

impl fmt::Debug for NewExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self.details)
    }
}

impl convert::From<&Rc<Expr>> for NewExpr {
    fn from(expr: &Rc<Expr>) -> NewExpr {
        NewExpr {
            details: expr.clone(),
        }
    }
}

impl convert::From<&Expr> for NewExpr {
    fn from(expr: &Expr) -> NewExpr {
        NewExpr {
            details: Rc::new(expr.clone()),
        }
    }
}

/// Returns the constant value of an expression if it's a polynomial
/// with only a constant part.
fn get_constant(expr: &Expr) -> Option<i64> {
    if let Expr::Poly(polynomial) = expr {
        polynomial.get_constant()
    } else {
        None
    }
}

/// Calculates the range of possible values of an expression
fn get_range(expr: &NewExpr) -> ValueRange {
    match expr.details() {
        Expr::Poly(polynomial) => {
            // Start with the constant part
            let mut start = polynomial.coefficients[14];
            let mut end = polynomial.coefficients[14];

            // Update based on min/max values for each input times that input's coefficient
            for i in 0..14 {
                let coefficient = polynomial.coefficients[i];
                start += coefficient * 1;
                end += coefficient * 9;
            }
            ValueRange::new(start, end)
        }
        Expr::Op(op_name, lhs, rhs) => {
            let lhs_range = get_range(lhs);
            let rhs_range = get_range(rhs);
            match op_name {
                Add => ValueRange::add_forward(lhs_range, rhs_range),
                Mul => ValueRange::mul_forward(lhs_range, rhs_range),
                Div => ValueRange::div_forward(lhs_range, rhs_range),
                Mod => ValueRange::mod_forward(lhs_range, rhs_range),
                Eql => ValueRange::eql_forward(lhs_range, rhs_range),
            }
        }
    }
}

fn both_ways<T: Copy>(a: T, b: T) -> [(T, T); 2] {
    [(a, b), (b, a)]
}

fn simplify_in_mod_helper(expr: &NewExpr, modulus: i64) -> Option<NewExpr> {
    match expr.details() {
        Expr::Poly(polynomial) => {
            if let Some(n) = polynomial.get_constant() {
                if n % modulus != n {
                    println!("    => {:?}", n % modulus);
                    Some(NewExpr::constant(n % modulus))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Expr::Op(op_name, lhs, rhs) => {
            match op_name {
                Add => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(NewExpr::op(*op_name, simplified_lhs, rhs.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(rhs, modulus) {
                        Some(NewExpr::op(*op_name, lhs.clone(), simplified_rhs))
                    } else {
                        None
                    }
                }
                Mul => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(NewExpr::op(*op_name, simplified_lhs, rhs.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(&rhs, modulus) {
                        Some(NewExpr::op(*op_name, lhs.clone(), simplified_rhs))
                    } else {
                        None
                    }
                }
                // Div => {
                //     // In the context of a mod operation, we can recursively look at addends and multiplicands.
                //     if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                //         Some(Expr::Op(*op_name, Rc::new(simplified_lhs), rhs_rc.clone()))
                //     } else {
                //         None
                //     }
                // }
                _ => None,
            }
        }
    }
}

fn simplify_in_mod(expr: &NewExpr, modulus: i64) -> Option<NewExpr> {
    if let Some(simpler) = simplify_in_mod_helper(expr, modulus) {
        if let Some(even_simpler) = simplify(simpler.details()) {
            Some(NewExpr::from(even_simpler))
        } else {
            Some(NewExpr::from(simpler))
        }
    } else {
        None
    }
}

fn simplify(expr: &Expr) -> Option<NewExpr> {
    // TODO: NewExpr
    if let Expr::Op(op_name, lhs, rhs) = expr {
        // operating on two constants can be done now
        if let Some(lhs_value) = lhs.get_constant() {
            if let Some(rhs_value) = rhs.get_constant() {
                return Some(NewExpr::poly(Polynomial::constant(
                    op_name.perform(lhs_value, rhs_value),
                )));
            }
        }
        match op_name {
            Add => {
                for (side_a, side_b) in both_ways(lhs, rhs) {
                    if let Some(n) = side_b.get_constant() {
                        if n == 0 {
                            return Some(side_a.clone());
                        }
                    }
                    if let Expr::Poly(poly_a) = side_a.details() {
                        if let Expr::Poly(poly_b) = side_b.details() {
                            return Some(NewExpr::poly(*poly_a + *poly_b));
                        }
                    }
                }
                None
            }
            Mul => {
                for (side_a, side_b) in both_ways(lhs, rhs) {
                    if let Some(n) = side_a.get_constant() {
                        if n == 0 {
                            return Some(NewExpr::constant(0));
                        }
                        if n == 1 {
                            return Some(side_b.clone());
                        }
                    }
                    if let Expr::Poly(side_a_poly) = side_a.details() {
                        if let Some(n) = side_b.get_constant() {
                            return Some(NewExpr::poly(side_a_poly.times(n)));
                        }
                    }
                    // if let Expr::Op(Add, addend_1_rc, addend_2_rc) = side_a {
                    //     let addend_1 = &**addend_1_rc;
                    //     let addend_2 = &**addend_2_rc;
                    //     return Some(Expr::Op(
                    //         Add,
                    //         Rc::new(Expr::Op(
                    //             Mul,
                    //             Rc::new(addend_1.clone()),
                    //             Rc::new(side_b.clone()),
                    //         )),
                    //         Rc::new(Expr::Op(
                    //             Mul,
                    //             Rc::new(addend_2.clone()),
                    //             Rc::new(side_b.clone()),
                    //         )),
                    //     ));
                    // }
                }
                // Put constants on the left if they can't be folded in
                if let Some(_) = rhs.get_constant() {
                    return Some(NewExpr::op(Mul, rhs.clone(), lhs.clone()));
                }
                None
            }
            Div => {
                if let Some(n) = lhs.get_constant() {
                    if n == 0 {
                        return Some(NewExpr::constant(0));
                    }
                }
                if let Some(n) = rhs.get_constant() {
                    if n == 1 {
                        return Some(lhs.clone());
                    }
                    if let Expr::Poly(polynomial) = lhs.details() {
                        if let Some(simpler_polynomial) = polynomial.div(n) {
                            return Some(NewExpr::poly(simpler_polynomial));
                        }
                    }
                }
                if get_range(&NewExpr::from(expr)) == ValueRange::new(0, 0) {
                    return Some(NewExpr::constant(0));
                }
                None
            }
            Mod => {
                if let Some(modulus) = rhs.get_constant() {
                    if let Expr::Poly(lhs_poly) = lhs.details() {
                        return Some(NewExpr::poly(lhs_poly.modulo(modulus)));
                    }
                    if let Some(simplified) = simplify_in_mod(lhs, modulus) {
                        return Some(NewExpr::op(Mod, simplified, rhs.clone()));
                    }
                }
                {
                    let lhs_range = get_range(lhs);
                    let rhs_range = get_range(rhs);
                    if 0 <= lhs_range.start() && lhs_range.end() < rhs_range.start() {
                        println!("YYY from {:?} {:?}", expr, get_range(&NewExpr::from(expr)));
                        println!("YYY to   {:?} {:?}", lhs, get_range(lhs));
                        return Some(lhs.clone());
                    }
                }

                None
            }
            Eql => {
                let lhs_range = get_range(lhs);
                let rhs_range = get_range(rhs);
                let ranges_overlap = max(lhs_range.start(), rhs_range.start())
                    <= min(lhs_range.end(), rhs_range.end());
                if !ranges_overlap {
                    return Some(NewExpr::constant(0));
                }
                None
            }
        }
    } else {
        None
    }
}

#[test]
fn test_simplify() {
    fn get_w_expression(lines: &[&str]) -> Expr {
        // TODO
        let mut state = State::start();
        for line in lines {
            let instruction = line.parse().unwrap();
            state = state.after(&instruction);
        }
        state.registers[0].details().clone()
    }

    // Register starts at 0
    assert_eq!(Expr::constant(0), get_w_expression(&[]));

    // Math with constants evaluates the expression
    assert_eq!(
        get_w_expression(&["add w 10"]),
        get_w_expression(&["add w 5", "add x 5", "add w x"])
    );

    // Adding 0 is identity in both directions
    assert_eq!(
        get_w_expression(&["inp w"]),
        get_w_expression(&["inp x", "add w x"])
    );
    assert_eq!(
        get_w_expression(&["inp w"]),
        get_w_expression(&["inp w", "add w x"])
    );

    // Multiplying by 1 is identity in both directions
    assert_eq!(
        get_w_expression(&["inp w"]),
        get_w_expression(&["inp x", "add w 1", "mul w x"])
    );
    assert_eq!(
        get_w_expression(&["inp w"]),
        get_w_expression(&["inp w", "add x 1", "mul w x"])
    );

    // Dividing by 1 is identity
    assert_eq!(
        get_w_expression(&["inp w"]),
        get_w_expression(&["inp w", "add x 1", "div w x"])
    );

    // Comparing an input with something out of range is always 0
    assert_eq!(
        get_w_expression(&[]),
        get_w_expression(&["inp w", "add x 14", "eql w x"])
    );

    // When the ranges of two expressions don't overlap, they cannot be equal
    assert_eq!(
        get_w_expression(&[]),
        get_w_expression(&["inp w", "mul w 100", "mod w 20", "add x 25", "eql w x"])
    );

    // Multiplying a polynomial times a constant
    assert_eq!(
        // a * 5 + 5
        get_w_expression(&["inp w", "mul w 5", "add w 5"]),
        // (a + 1) * 5
        get_w_expression(&["inp w", "add w 1", "mul w 5"])
    );

    // Modding a polynomial with a constant
    assert_eq!(
        // 8
        get_w_expression(&["add w 3"]),
        // (a * 5 + 8) % 5
        get_w_expression(&["inp w", "mul w 5", "add w 8", "mod w 5"]),
    );

    // The mod of a sum of terms can drop terms that multiply by the modulus.
    assert_eq!(
        // 8
        get_w_expression(&["add w 3"]),
        // ((a = 2) * 5 + 8) % 5
        get_w_expression(&["inp w", "eql w 2", "mul w 5", "add w 8", "mod w 5"]),
    );

    // Constants go on the left
    assert_eq!(
        // 25 * (a / 26)
        get_w_expression(&["add w 25", "inp x", "div x 26", "mul w x"]),
        // (a / 26) * 25
        get_w_expression(&["inp w", "div w 26", "add x 25", "mul w x"])
    );

    // When the numerator of a mod is within the range of the mod, you can drop the mod
    assert_eq!(
        // (a = b)
        get_w_expression(&["inp x", "inp y", "add w x", "eql w y"]),
        // (a = b) mod 5
        get_w_expression(&["inp x", "inp y", "add w x", "eql w y", "mod w 5"])
    );

    // Dividing a polynomial by a scalar can divide through if we know the remainders
    // can't add up to more than the scalar.
    assert_eq!(
        // [a + 7]
        get_w_expression(&["inp w", "add x 7", "add w x"]),
        // [26a + b + 185] / 26
        get_w_expression(&[
            "inp w",
            "mul w 26",
            "inp x",
            "add w x",
            "add w 185",
            "div w 26"
        ]),
    );

    // Distributive multiplication
    // assert_eq!(
    //     // a * b + a * c
    //     get_w_expression(&["inp w", "add x w", "inp y", "mul w y", "inp y", "mul x y", "add w x"]),
    //     // a * (b + c)
    //     get_w_expression(&["inp w", "inp x", "inp y", "add x y", "mul w x"])
    // );
}

struct State {
    next_input: Option<InputName>,
    registers: [NewExpr; 4],
}

fn set_register(
    register_name: RegisterName,
    value: Expr,
    old_registers: &[NewExpr; 4],
) -> [NewExpr; 4] {
    let mut result = old_registers.clone();
    result[register_name.index()] = NewExpr::from(&value);
    result
}

impl State {
    fn start() -> State {
        State {
            next_input: Some(InputName::first()),
            registers: [
                NewExpr::constant(0),
                NewExpr::constant(0),
                NewExpr::constant(0),
                NewExpr::constant(0),
            ],
        }
    }

    fn after(self: &State, instruction: &Instruction) -> State {
        match instruction {
            Inp(register_name) => {
                if let Some(input_name) = self.next_input {
                    State {
                        next_input: input_name.next(),
                        registers: set_register(
                            *register_name,
                            Expr::input(input_name),
                            &self.registers,
                        ),
                    }
                } else {
                    panic!("no more input names");
                }
            }
            Op(op_name, lhs_register_name, register_or_constant) => {
                let lhs = self.registers[lhs_register_name.index()].clone();
                let rhs = match register_or_constant {
                    Register(rhs_register_name) => {
                        self.registers[rhs_register_name.index()].clone()
                    }
                    Constant(n) => NewExpr::constant(*n),
                };
                let mut expr = NewExpr::op(*op_name, lhs, rhs);
                while let Some(simplified) = simplify(expr.details()) {
                    println!("SIMPLIFY {:?} => {:?}", expr, simplified);
                    expr = simplified;
                }
                State {
                    next_input: self.next_input,
                    registers: set_register(
                        *lhs_register_name,
                        expr.details().clone(),
                        &self.registers,
                    ),
                }
            }
        }
    }
}

fn print_state(state: &State) {
    // println!("next input: {:?}", state.next_input);
    for (r, expr) in RegisterName::all().into_iter().zip(state.registers.iter()) {
        println!("{:?} = {:?}   {:?}", r, get_range(expr), *expr);
    }
    println!("");
}

fn indent(indentation: usize) {
    for _ in 0..indentation {
        print!("  ");
    }
}

fn print_tree(expr: &NewExpr, indentation: usize) {
    let range = get_range(expr);
    match expr.details() {
        Expr::Poly(polynomial) => println!(
            "{:?} {{{:?} .. {:?}}}",
            polynomial,
            range.start(),
            range.end()
        ),
        Expr::Op(op_name, lhs, rhs) => {
            println!("{:?} {{{:?} .. {:?}}}", op_name, range.start(), range.end());
            indent(indentation + 1);
            print_tree(lhs, indentation + 1);
            indent(indentation + 1);
            print_tree(rhs, indentation + 1);
        }
    }
}

fn evaluate_instructions(instructions: &[Instruction], inputs: &[i64; 14]) -> i64 {
    let mut next_input_index = 0;
    let mut registers: [i64; 4] = [0; 4];
    for instruction in instructions {
        match instruction {
            Inp(register_name) => {
                registers[register_name.index()] = inputs[next_input_index];
                next_input_index += 1;
            }
            Op(op_name, lhs_register_name, rhs) => {
                let rhs_value = match rhs {
                    Constant(n) => *n,
                    Register(rhs_register_name) => registers[rhs_register_name.index()],
                };
                registers[lhs_register_name.index()] =
                    op_name.perform(registers[lhs_register_name.index()], rhs_value);
            }
        }
    }
    registers[3]
}

fn evaluate_one(instructions: &[Instruction], z_expr: &NewExpr, inputs: &[i64; 14]) {
    let from_instructions = evaluate_instructions(instructions, inputs);
    let from_simplified = z_expr.evaluate(inputs);
    println!(
        "EVAL: {:?} => {:?} {:?}",
        inputs, from_instructions, from_simplified
    );
    if from_instructions != from_simplified {
        panic!("ERROR: Simplified expression did not match");
    }
}

fn day_24_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut state = State::start();
    let mut instructions = Vec::new();
    for line in lines {
        println!("INSTRUCTION: {:?}\n", line);
        let instruction = line.parse().unwrap();
        state = state.after(&instruction);
        instructions.push(instruction);
        print_state(&state);
    }
    println!("\n\n\n\n\n\n");
    let z_expr = &state.registers[3];
    print_tree(z_expr, 0);
    for n in 0..=8 {
        let mut inputs = [n; 14];
        for i in 0..13 {
            evaluate_one(&instructions[..], z_expr, &inputs);
            inputs[i] += 1;
        }
    }
    evaluate_one(&instructions[..], z_expr, &[9; 14]);
    Ok(0)
}

fn day_24_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_24() -> Day {
    Day::new(
        24,
        DayPart::new(day_24_a, 0, 0),
        DayPart::new(day_24_b, 0, 0),
    )
}
