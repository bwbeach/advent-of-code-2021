use std::cmp::{max, min};
use std::fmt;
use std::ops;
use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::day24_alu::{InputName, OpName, RegisterName, RegisterOrConstant};
use crate::types::{AdventResult, Answer, Day, DayPart};

use OpName::*;
use RegisterOrConstant::*;

#[derive(Debug)]
enum Instruction {
    Inp(RegisterName),
    Op(OpName, RegisterName, RegisterOrConstant),
}

use Instruction::*;

impl Instruction {
    // TODO: convert to FromStr
    fn parse(s: &str) -> Instruction {
        let words: Vec<_> = s.split_whitespace().collect();
        if words[0] == "inp" {
            if words.len() != 2 {
                panic!("wrong length of inp instruction");
            }
            Inp(words[1].parse().unwrap())
        } else {
            Op(
                words[0].parse().unwrap(),
                words[1].parse().unwrap(),
                words[2].parse().unwrap(),
            )
        }
    }
}

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
    Op(OpName, Rc<Expr>, Rc<Expr>),
}

impl Expr {
    fn constant(scalar: i64) -> Expr {
        Expr::Poly(Polynomial::constant(scalar))
    }

    fn input(input_name: InputName) -> Expr {
        Expr::Poly(Polynomial::input(input_name))
    }

    fn evaluate(self: &Expr, inputs: &[i64; 14]) -> i64 {
        match self {
            Expr::Poly(polynomial) => {
                let mut result = polynomial.coefficients[14]; // constant part
                for (input, coefficient) in inputs.iter().zip(polynomial.coefficients) {
                    result += input * coefficient;
                }
                result
            }
            Expr::Op(op_name, lhs_rc, rhs_rc) => {
                let lhs = &**lhs_rc;
                let rhs = &**rhs_rc;
                op_name.perform(lhs.evaluate(inputs), rhs.evaluate(inputs))
            }
        }
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

/// Returns the constant value of an expression if it's a polynomial
/// with only a constant part.
fn get_constant(expr: &Expr) -> Option<i64> {
    if let Expr::Poly(polynomial) = expr {
        polynomial.get_constant()
    } else {
        None
    }
}

/// What the possible range of values when multiplying values
/// from two ranges?
fn mul_range(lhs: &RangeInclusive<i64>, rhs: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let mut start = lhs.start() * rhs.start();
    let mut end = lhs.end() * rhs.end();
    for left in [lhs.start(), lhs.end()] {
        for right in [rhs.start(), rhs.end()] {
            start = min(start, left * right);
            end = max(end, left * right);
        }
    }
    start..=end
}

#[test]
fn test_mul_range() {
    assert_eq!(25..=100, mul_range(&(5..=10), &(5..=10)));
    assert_eq!(-50..=100, mul_range(&(-5..=10), &(5..=10)));
    assert_eq!(-100..=-25, mul_range(&(-10..=-5), &(5..=10)));
    assert_eq!(-100..=50, mul_range(&(-10..=5), &(5..=10)));
}

/// What the possible range of values when multiplying values
/// from two ranges?
fn div_range(lhs: &RangeInclusive<i64>, rhs: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    if *lhs.start() < 0 || *rhs.start() <= 0 {
        panic!("div range includes 0 or negative: {:?}  {:?}", lhs, rhs);
    }
    RangeInclusive::new(lhs.start() / rhs.end(), lhs.end() / rhs.start())
}

/// Calculates the range of possible values of an expression
fn get_range(expr: &Expr) -> RangeInclusive<i64> {
    match expr {
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
            start..=end
        }
        Expr::Op(op_name, lhs_rc, rhs_rc) => {
            let lhs_range = get_range(&**lhs_rc);
            let rhs_range = get_range(&**rhs_rc);
            match op_name {
                Add => RangeInclusive::new(
                    lhs_range.start() + rhs_range.start(),
                    lhs_range.end() + rhs_range.end(),
                ),
                Mul => mul_range(&lhs_range, &rhs_range),
                Div => div_range(&lhs_range, &rhs_range),
                Mod => RangeInclusive::new(0, *rhs_range.end()),
                Eql => 0..=1,
            }
        }
    }
}

fn both_ways<T: Copy>(a: T, b: T) -> [(T, T); 2] {
    [(a, b), (b, a)]
}

fn simplify_in_mod_helper(expr: &Expr, modulus: i64) -> Option<Expr> {
    match expr {
        Expr::Poly(polynomial) => {
            if let Some(n) = polynomial.get_constant() {
                if n % modulus != n {
                    println!("    => {:?}", n % modulus);
                    Some(Expr::constant(n % modulus))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Expr::Op(op_name, lhs_rc, rhs_rc) => {
            let lhs = &**lhs_rc;
            let rhs = &**rhs_rc;
            match op_name {
                Add => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(Expr::Op(*op_name, Rc::new(simplified_lhs), rhs_rc.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(rhs, modulus) {
                        Some(Expr::Op(*op_name, lhs_rc.clone(), Rc::new(simplified_rhs)))
                    } else {
                        None
                    }
                }
                Mul => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(Expr::Op(*op_name, Rc::new(simplified_lhs), rhs_rc.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(rhs, modulus) {
                        Some(Expr::Op(*op_name, lhs_rc.clone(), Rc::new(simplified_rhs)))
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

fn simplify_in_mod(expr: &Expr, modulus: i64) -> Option<Expr> {
    if let Some(simpler) = simplify_in_mod_helper(expr, modulus) {
        if let Some(even_simpler) = simplify(&simpler) {
            Some(even_simpler)
        } else {
            Some(simpler)
        }
    } else {
        None
    }
}

fn simplify(expr: &Expr) -> Option<Expr> {
    if let Expr::Op(op_name, lhs_rc, rhs_rc) = expr {
        let lhs = &**lhs_rc;
        let rhs = &**rhs_rc;
        // operating on two constants can be done now
        if let Some(lhs_value) = get_constant(lhs) {
            if let Some(rhs_value) = get_constant(rhs) {
                return Some(Expr::Poly(Polynomial::constant(
                    op_name.perform(lhs_value, rhs_value),
                )));
            }
        }
        match op_name {
            Add => {
                for (side_a, side_b) in both_ways(lhs, rhs) {
                    if let Some(n) = get_constant(side_b) {
                        if n == 0 {
                            return Some(side_a.clone());
                        }
                    }
                    if let Expr::Poly(poly_a) = side_a {
                        if let Expr::Poly(poly_b) = side_b {
                            return Some(Expr::Poly(*poly_a + *poly_b));
                        }
                    }
                }
                None
            }
            Mul => {
                for (side_a, side_b) in both_ways(lhs, rhs) {
                    if let Some(n) = get_constant(side_a) {
                        if n == 0 {
                            return Some(Expr::constant(0));
                        }
                        if n == 1 {
                            return Some(side_b.clone());
                        }
                    }
                    if let Expr::Poly(side_a_poly) = side_a {
                        if let Some(n) = get_constant(side_b) {
                            return Some(Expr::Poly(side_a_poly.times(n)));
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
                if let Some(_) = get_constant(rhs) {
                    return Some(Expr::Op(Mul, rhs_rc.clone(), lhs_rc.clone()));
                }
                None
            }
            Div => {
                if let Some(n) = get_constant(lhs) {
                    if n == 0 {
                        return Some(Expr::constant(0));
                    }
                }
                if let Some(n) = get_constant(rhs) {
                    if n == 1 {
                        return Some(lhs.clone());
                    }
                    if let Expr::Poly(polynomial) = lhs {
                        if let Some(simpler_polynomial) = polynomial.div(n) {
                            return Some(Expr::Poly(simpler_polynomial));
                        }
                    }
                }
                if get_range(expr) == (0..=0) {
                    return Some(Expr::constant(0));
                }
                None
            }
            Mod => {
                if let Some(modulus) = get_constant(rhs) {
                    if let Expr::Poly(lhs_poly) = lhs {
                        return Some(Expr::Poly(lhs_poly.modulo(modulus)));
                    }
                    if let Some(simplified) = simplify_in_mod(lhs, modulus) {
                        return Some(Expr::Op(Mod, Rc::new(simplified), rhs_rc.clone()));
                    }
                }
                {
                    let lhs_range = get_range(lhs);
                    let rhs_range = get_range(rhs);
                    if 0 <= *lhs_range.start() && *lhs_range.end() < *rhs_range.start() {
                        println!("YYY from {:?} {:?}", expr, get_range(expr));
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
                    return Some(Expr::constant(0));
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
        let mut state = State::start();
        for line in lines {
            state = state.after(&Instruction::parse(line));
        }
        (*state.registers[0]).clone()
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
    registers: [Rc<Expr>; 4],
}

fn set_register(
    register_name: RegisterName,
    value: Expr,
    old_registers: &[Rc<Expr>; 4],
) -> [Rc<Expr>; 4] {
    let mut result = old_registers.clone();
    result[register_name.index()] = Rc::new(value);
    result
}

impl State {
    fn start() -> State {
        State {
            next_input: Some(InputName::first()),
            registers: [
                Rc::new(Expr::constant(0)),
                Rc::new(Expr::constant(0)),
                Rc::new(Expr::constant(0)),
                Rc::new(Expr::constant(0)),
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
                    Constant(n) => Rc::new(Expr::constant(*n)),
                };
                let mut expr = Expr::Op(*op_name, lhs, rhs);
                while let Some(simplified) = simplify(&expr) {
                    println!("SIMPLIFY {:?} => {:?}", expr, simplified);
                    expr = simplified;
                }
                State {
                    next_input: self.next_input,
                    registers: set_register(*lhs_register_name, expr, &self.registers),
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

fn print_tree(expr: &Expr, indentation: usize) {
    let range = get_range(expr);
    match expr {
        Expr::Poly(polynomial) => println!(
            "{:?} {{{:?} .. {:?}}}",
            polynomial,
            range.start(),
            range.end()
        ),
        Expr::Op(op_name, lhs_rc, rhs_rc) => {
            println!("{:?} {{{:?} .. {:?}}}", op_name, range.start(), range.end());
            indent(indentation + 1);
            print_tree(&**lhs_rc, indentation + 1);
            indent(indentation + 1);
            print_tree(&**rhs_rc, indentation + 1);
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

fn evaluate_one(instructions: &[Instruction], z_expr: &Expr, inputs: &[i64; 14]) {
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
        let instruction = Instruction::parse(line);
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
