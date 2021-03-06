use std::cmp::{max, min};
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

    fn get_range(&self) -> ValueRange {
        // Start with the constant part
        let mut start = self.coefficients[14];
        let mut end = self.coefficients[14];

        // Update based on min/max values for each input times that input's coefficient
        for i in 0..14 {
            let coefficient = self.coefficients[i];
            start += coefficient * 1;
            end += coefficient * 9;
        }
        ValueRange::new(start, end)
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
enum ExprDetails {
    Poly(Polynomial),
    Op(OpName, Expr, Expr),
}

impl fmt::Debug for ExprDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprDetails::Poly(polynomial) => write!(f, "{:?}", polynomial),
            ExprDetails::Op(op_name, a, b) => write!(f, "({:?} {:?} {:?})", a, op_name, b),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Expr {
    details: Rc<ExprDetails>,
}

impl Expr {
    /// Matchable details of this expression.
    fn details<'a>(&'a self) -> &'a ExprDetails {
        &*self.details
    }

    /// Returns an expression holding a constant value
    fn constant(n: i64) -> Expr {
        Expr::poly(Polynomial::constant(n))
    }

    /// Returns an expression holding one of the input values
    fn input(input_name: InputName) -> Expr {
        Expr::poly(Polynomial::input(input_name))
    }

    /// Returns an expression holding a polynomial
    fn poly(polynomial: Polynomial) -> Expr {
        let details = Rc::new(ExprDetails::Poly(polynomial));
        Expr { details }
    }

    /// Returns an expression holding an operation
    fn op(op_name: OpName, lhs: Expr, rhs: Expr) -> Expr {
        let details = Rc::new(ExprDetails::Op(op_name, lhs, rhs));
        Expr { details }
    }

    /// Returns the constant value of an expression if it's a polynomial
    /// with only a constant part.
    fn get_constant(&self) -> Option<i64> {
        match self.details() {
            ExprDetails::Poly(polynomial) => polynomial.get_constant(),
            _ => None,
        }
    }

    fn evaluate(&self, inputs: &[i64; 14]) -> i64 {
        match self.details() {
            ExprDetails::Poly(polynomial) => {
                let mut result = polynomial.coefficients[14]; // constant part
                for (input, coefficient) in inputs.iter().zip(polynomial.coefficients) {
                    result += input * coefficient;
                }
                result
            }
            ExprDetails::Op(op_name, lhs, rhs) => {
                op_name.perform(lhs.evaluate(inputs), rhs.evaluate(inputs))
            }
        }
    }

    fn get_range(&self) -> ValueRange {
        match self.details() {
            ExprDetails::Poly(polynomial) => polynomial.get_range(),
            ExprDetails::Op(op_name, lhs, rhs) => {
                op_name.perform_on_range(lhs.get_range(), rhs.get_range())
            }
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self.details)
    }
}

fn both_ways<T: Copy>(a: T, b: T) -> [(T, T); 2] {
    [(a, b), (b, a)]
}

fn simplify_in_mod_helper(expr: &Expr, modulus: i64) -> Option<Expr> {
    match expr.details() {
        ExprDetails::Poly(polynomial) => {
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
        ExprDetails::Op(op_name, lhs, rhs) => {
            match op_name {
                Add => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(Expr::op(*op_name, simplified_lhs, rhs.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(rhs, modulus) {
                        Some(Expr::op(*op_name, lhs.clone(), simplified_rhs))
                    } else {
                        None
                    }
                }
                Mul => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(Expr::op(*op_name, simplified_lhs, rhs.clone()))
                    } else if let Some(simplified_rhs) = simplify_in_mod(&rhs, modulus) {
                        Some(Expr::op(*op_name, lhs.clone(), simplified_rhs))
                    } else {
                        None
                    }
                }
                // Div => {
                //     // In the context of a mod operation, we can recursively look at addends and multiplicands.
                //     if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                //         Some(ExprDetails::Op(*op_name, Rc::new(simplified_lhs), rhs_rc.clone()))
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
            Some(Expr::from(even_simpler))
        } else {
            Some(Expr::from(simpler))
        }
    } else {
        None
    }
}

fn simplify(expr: &Expr) -> Option<Expr> {
    if let ExprDetails::Op(op_name, lhs, rhs) = expr.details() {
        // operating on two constants can be done now
        if let Some(lhs_value) = lhs.get_constant() {
            if let Some(rhs_value) = rhs.get_constant() {
                return Some(Expr::poly(Polynomial::constant(
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
                    if let ExprDetails::Poly(poly_a) = side_a.details() {
                        if let ExprDetails::Poly(poly_b) = side_b.details() {
                            return Some(Expr::poly(*poly_a + *poly_b));
                        }
                    }
                }
                None
            }
            Mul => {
                for (side_a, side_b) in both_ways(lhs, rhs) {
                    if let Some(n) = side_a.get_constant() {
                        if n == 0 {
                            return Some(Expr::constant(0));
                        }
                        if n == 1 {
                            return Some(side_b.clone());
                        }
                    }
                    if let ExprDetails::Poly(side_a_poly) = side_a.details() {
                        if let Some(n) = side_b.get_constant() {
                            return Some(Expr::poly(side_a_poly.times(n)));
                        }
                    }
                    // if let ExprDetails::Op(Add, addend_1_rc, addend_2_rc) = side_a {
                    //     let addend_1 = &**addend_1_rc;
                    //     let addend_2 = &**addend_2_rc;
                    //     return Some(ExprDetails::Op(
                    //         Add,
                    //         Rc::new(ExprDetails::Op(
                    //             Mul,
                    //             Rc::new(addend_1.clone()),
                    //             Rc::new(side_b.clone()),
                    //         )),
                    //         Rc::new(ExprDetails::Op(
                    //             Mul,
                    //             Rc::new(addend_2.clone()),
                    //             Rc::new(side_b.clone()),
                    //         )),
                    //     ));
                    // }
                }
                // Put constants on the left if they can't be folded in
                if let Some(_) = rhs.get_constant() {
                    return Some(Expr::op(Mul, rhs.clone(), lhs.clone()));
                }
                None
            }
            Div => {
                if let Some(n) = lhs.get_constant() {
                    if n == 0 {
                        return Some(Expr::constant(0));
                    }
                }
                if let Some(n) = rhs.get_constant() {
                    if n == 1 {
                        return Some(lhs.clone());
                    }
                    if let ExprDetails::Poly(polynomial) = lhs.details() {
                        if let Some(simpler_polynomial) = polynomial.div(n) {
                            return Some(Expr::poly(simpler_polynomial));
                        }
                    }
                }
                if expr.get_range() == ValueRange::new(0, 0) {
                    return Some(Expr::constant(0));
                }
                None
            }
            Mod => {
                if let Some(modulus) = rhs.get_constant() {
                    if let ExprDetails::Poly(lhs_poly) = lhs.details() {
                        return Some(Expr::poly(lhs_poly.modulo(modulus)));
                    }
                    if let Some(simplified) = simplify_in_mod(lhs, modulus) {
                        return Some(Expr::op(Mod, simplified, rhs.clone()));
                    }
                }
                {
                    let lhs_range = lhs.get_range();
                    let rhs_range = rhs.get_range();
                    if 0 <= lhs_range.start() && lhs_range.end() < rhs_range.start() {
                        return Some(lhs.clone());
                    }
                }

                None
            }
            Eql => {
                let lhs_range = lhs.get_range();
                let rhs_range = rhs.get_range();
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
            let instruction = line.parse().unwrap();
            state = state.after(&instruction);
        }
        state.registers[0].clone()
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
    registers: [Expr; 4],
}

fn set_register(register_name: RegisterName, value: Expr, old_registers: &[Expr; 4]) -> [Expr; 4] {
    let mut result = old_registers.clone();
    result[register_name.index()] = value;
    result
}

impl State {
    fn start() -> State {
        State {
            next_input: Some(InputName::first()),
            registers: [
                Expr::constant(0),
                Expr::constant(0),
                Expr::constant(0),
                Expr::constant(0),
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
                    Constant(n) => Expr::constant(*n),
                };
                let mut expr = Expr::op(*op_name, lhs, rhs);
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
        println!("{:?} = {:?}   {:?}", r, expr.get_range(), *expr);
    }
    println!("");
}

fn indent(indentation: usize) {
    for _ in 0..indentation {
        print!("  ");
    }
}

fn print_tree(expr: &Expr, indentation: usize) {
    let range = expr.get_range();
    match expr.details() {
        ExprDetails::Poly(polynomial) => println!(
            "{:?} {{{:?} .. {:?}}}",
            polynomial,
            range.start(),
            range.end()
        ),
        ExprDetails::Op(op_name, lhs, rhs) => {
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

fn day_24_a_old(lines: &[&str]) -> AdventResult<Answer> {
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

type Ranges = [ValueRange; 4];
type Limits = [Option<ValueRange>; 4];

/// Info about each instruction, and what we know about the
/// state of the registers after it runs.
struct Info {
    instruction: Instruction,
    ranges: Ranges,
    limits: Limits,
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}  {:?}    limits: {:?}",
            self.instruction, self.ranges, self.limits
        )
    }
}

fn ranges_after(prev_ranges: &[ValueRange; 4], instruction: &Instruction) -> Ranges {
    let mut new_ranges = prev_ranges.clone();
    match instruction {
        Inp(register_name) => new_ranges[register_name.index()] = ValueRange::new(1, 9),
        Op(op_name, register_name, rhs) => {
            let lhs_range = prev_ranges[register_name.index()];
            let rhs_range = match rhs {
                Register(rhs_reg_name) => prev_ranges[rhs_reg_name.index()],
                Constant(n) => ValueRange::new(*n, *n),
            };
            new_ranges[register_name.index()] = op_name.perform_on_range(lhs_range, rhs_range)
        }
    }
    new_ranges
}

fn left_limit(
    op_name: OpName,
    left: ValueRange,
    right: ValueRange,
    result: ValueRange,
) -> Option<ValueRange> {
    match op_name {
        Add => ValueRange::add_backward(right, result),
        Mul => ValueRange::mul_backward(right, result),
        Div => ValueRange::div_backward_left(right, result),
        Eql => ValueRange::eql_backward(left, right, result),
        _ => None,
    }
}

fn right_limit(
    op_name: OpName,
    left: ValueRange,
    right: ValueRange,
    result: ValueRange,
) -> Option<ValueRange> {
    match op_name {
        Add => ValueRange::add_backward(left, result),
        Mul => ValueRange::mul_backward(left, result),
        Eql => ValueRange::eql_backward(right, left, result),
        _ => None,
    }
}

fn limit_range(range: ValueRange, limit: Option<ValueRange>) -> ValueRange {
    match limit {
        Some(lim) => ValueRange::intersect(range, lim).unwrap(),
        None => range,
    }
}

fn search(
    mut registers: [i64; 4],
    infos: &[Info],
    starting_pc: usize,
    bindings: [i64; 14],
    input_counter: usize,
    search_order: &[i64],
) -> Option<Answer> {
    // Run instructions as long they are not input instructions, and we're not at the end.
    for pc in starting_pc..infos.len() {
        match &infos[pc].instruction {
            Inp(reg_name) => {
                // try all of the possible inputs, and then search the rest of the program
                for &value in search_order {
                    let mut new_registers = registers.clone();
                    new_registers[reg_name.index()] = value;
                    let mut new_bindings = bindings.clone();
                    new_bindings[input_counter] = value;
                    if let Some(answer) = search(
                        new_registers,
                        infos,
                        pc + 1,
                        new_bindings,
                        input_counter + 1,
                        search_order,
                    ) {
                        return Some(answer);
                    }
                }
                // none of the input values worked
                return None;
            }
            Op(op_name, register, rhs) => {
                let r = register.index();
                let lhs_value = registers[r];
                let rhs_value = match rhs {
                    Constant(n) => *n,
                    Register(rhs_reg) => registers[rhs_reg.index()],
                };
                let new_value = op_name.perform(lhs_value, rhs_value);
                if let Some(limit) = infos[pc].limits[r] {
                    if !limit.contains(new_value) {
                        return None;
                    }
                }
                registers[r] = op_name.perform(lhs_value, rhs_value);
            }
        }
    }
    // We've reached the end of the problem, we need to stop.
    if registers[3] == 0 {
        // we found an answer!
        let mut result = 0;
        for b in bindings {
            result = result * 10 + b;
        }
        Some(result as Answer)
    } else {
        // not zero; need caller to keep searching
        None
    }
}

fn day_24(lines: &[&str], search_order: &[i64]) -> AdventResult<Answer> {
    if false {
        let _ = day_24_a_old(lines);
    }

    let mut infos = Vec::new();

    // Collect the instructions, and calculate the possible value ranges
    // for each
    {
        let mut ranges = [ValueRange::new(0, 0); 4];
        for line in lines {
            let instruction = line.parse().unwrap();
            ranges = ranges_after(&ranges, &instruction);
            let limits = [None; 4];
            let info = Info {
                instruction,
                ranges: ranges.clone(),
                limits,
            };
            infos.push(info)
        }
    }

    // Working backwards from the last instruction, propagate the limits
    // on what each register can hold.  At the end, we know each register
    // is limited to the range of values possible.  And z must be 0.
    let mut limits = [None; 4];
    limits[3] = Some(ValueRange::new(0, 0));
    for i in (0..infos.len()).rev() {
        infos[i].limits = limits.clone();
        let info = &infos[i];
        let instruction = &infos[i].instruction;
        match instruction {
            Inp(r) => {
                limits[r.index()] = None;
            }
            Op(op_name, lhs, rhs) => {
                // The range of values possible on the left-hand side is the range
                // coming in from the previous instruction.  We can't apply the limits
                // on the register directly, because they have to be translated through
                // the operation first.
                let lhs_range = if i == 0 {
                    ValueRange::new(0, 0)
                } else {
                    infos[i - 1].ranges[lhs.index()]
                };
                // The range of input values possible on the right-hand side
                let rhs_range = match rhs {
                    // constant ranges are easy
                    Constant(n) => ValueRange::new(*n, *n),
                    // the range of values for the right-hand side comes from
                    // combining the know range product from earlier, with any
                    // limits we know about.
                    Register(r) => {
                        let prev_range = info.ranges[r.index()];
                        let limited_range = limit_range(prev_range, info.limits[r.index()]);
                        limited_range
                    }
                };
                let result_range = info.limits[lhs.index()].unwrap_or(info.ranges[lhs.index()]);

                // Limits in the input value on the left side
                let lhs_limited_range = limit_range(
                    lhs_range,
                    left_limit(*op_name, lhs_range, rhs_range, result_range),
                );
                let left_limit = if lhs_limited_range != lhs_range {
                    Some(lhs_limited_range)
                } else {
                    None
                };
                limits[lhs.index()] = left_limit;

                // Limits on the input value on the right side
                if let Register(rhs_reg) = rhs {
                    let rhs_original_limited_range =
                        limit_range(rhs_range, limits[rhs_reg.index()]);
                    let rhs_limited_range = limit_range(
                        rhs_original_limited_range,
                        right_limit(*op_name, lhs_range, rhs_range, result_range),
                    );
                    let right_limit = if rhs_limited_range != info.ranges[rhs_reg.index()] {
                        Some(rhs_limited_range)
                    } else {
                        None
                    };
                    if right_limit != limits[rhs_reg.index()] {
                        limits[rhs_reg.index()] = right_limit;
                    }
                }
            }
        }
    }

    // Now do the search
    Ok(search([0; 4], &infos, 0, [0; 14], 0, search_order).unwrap())
}

fn day_24_a(lines: &[&str]) -> AdventResult<Answer> {
    let search_order: Vec<i64> = (1..=9).rev().collect();
    day_24(lines, &search_order[..])
}

fn day_24_b(lines: &[&str]) -> AdventResult<Answer> {
    let search_order: Vec<i64> = (1..=9).collect();
    day_24(lines, &search_order[..])
}

pub fn make_day_24() -> Day {
    Day::new(
        24,
        DayPart::new(day_24_a, 80000000000000, 12996997829399),
        DayPart::new(day_24_b, 20000000000000, 11841231117189),
    )
}
