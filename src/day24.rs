use std::cmp::{max, min};
use std::fmt;
use std::ops;
use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Registers are named 'w' through 'z'
#[derive(Clone, Copy, Eq, PartialEq)]
struct RegisterName {
    name: char,
}

impl RegisterName {
    fn all() -> Vec<RegisterName> {
        ('w'..='z').map(|c| RegisterName::new(c)).collect()
    }

    fn new(name: char) -> RegisterName {
        if name < 'w' || 'z' < name {
            panic!("bad register name: {}", name);
        }
        RegisterName { name }
    }

    fn parse(s: &str) -> RegisterName {
        if s.len() != 1 {
            panic!("register name too long");
        }
        RegisterName::new(s.chars().next().unwrap())
    }

    fn index(&self) -> usize {
        (self.name as usize) - ('w' as usize)
    }
}

impl fmt::Debug for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Inputs are named 'a' through 'n'
#[derive(Clone, Copy, Eq, PartialEq)]
struct InputName {
    name: char,
}

impl InputName {
    fn all() -> Vec<InputName> {
        ('a'..='n').map(|c| InputName::new(c)).collect()
    }

    fn new(name: char) -> InputName {
        if name < 'a' || 'n' < name {
            panic!("bad input name: {}", name);
        }
        InputName { name }
    }

    // fn parse(s: &str) -> InputName {
    //     if s.len() != 1 {
    //         panic!("input name too long");
    //     }
    //     InputName::new(s.chars().next().unwrap())
    // }

    fn index(&self) -> usize {
        (self.name as usize) - ('a' as usize)
    }

    fn next(&self) -> Option<InputName> {
        if self.name == 'n' {
            None
        } else {
            Some(InputName::new(((self.name as u8) + 1) as char))
        }
    }
}

impl fmt::Debug for InputName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

enum RegisterOrConstant {
    Register(RegisterName),
    Constant(i64),
}

use RegisterOrConstant::*;

impl RegisterOrConstant {
    fn parse(s: &str) -> RegisterOrConstant {
        if let Ok(n) = s.parse::<i64>() {
            Constant(n)
        } else {
            Register(RegisterName::parse(s))
        }
    }
}

impl fmt::Debug for RegisterOrConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register(register_name) => write!(f, "{:?}", register_name),
            Constant(n) => write!(f, "{:?}", n),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum OpName {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

use OpName::*;

impl OpName {
    fn parse(s: &str) -> OpName {
        match s {
            "add" => Add,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "eql" => Eql,
            _ => panic!("bad op name: {:?}", s),
        }
    }
}

impl fmt::Debug for OpName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_name = match self {
            Add => '+',
            Mul => '*',
            Div => '/',
            Mod => '%',
            Eql => '=',
        };
        write!(f, "{}", short_name)
    }
}

fn perform_op(op_name: OpName, a: i64, b: i64) -> i64 {
    match op_name {
        Add => a + b,
        Mul => a * b,
        Div => {
            if b == 0 {
                panic!("division by 0");
            } else {
                a / b
            }
        }
        Mod => {
            if b == 0 {
                panic!("mod by 0");
            } else if a < 0 || b < 0 {
                panic!("mod with negative");
            } else {
                a % b
            }
        }
        Eql => {
            if a == b {
                1
            } else {
                0
            }
        }
    }
}

#[test]
fn test_perform_op() {
    assert_eq!(10, perform_op(Add, 2, 8));
    assert_eq!(16, perform_op(Mul, 2, 8));
    assert_eq!(3, perform_op(Div, 7, 2));
    assert_eq!(3, perform_op(Div, -7, -2));
    assert_eq!(-3, perform_op(Div, -7, 2));
    assert_eq!(-3, perform_op(Div, 7, -2));
    assert_eq!(1, perform_op(Mod, 7, 2));
    assert_eq!(0, perform_op(Eql, 3, 5));
    assert_eq!(1, perform_op(Eql, 5, 5));
}

#[derive(Debug)]
enum Instruction {
    Inp(RegisterName),
    Op(OpName, RegisterName, RegisterOrConstant),
}

use Instruction::*;

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let words: Vec<_> = s.split_whitespace().collect();
        if words[0] == "inp" {
            if words.len() != 2 {
                panic!("wrong length of inp instruction");
            }
            Inp(RegisterName::parse(words[1]))
        } else {
            Op(
                OpName::parse(words[0]),
                RegisterName::parse(words[1]),
                RegisterOrConstant::parse(words[2]),
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
            let c = coefficients[i];
            if c < 0 || scalar <= 0 {
                panic!("bad mod: {:?} {:?}", c, scalar);
            }
            coefficients[i] = self.coefficients[i] % scalar;
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
    let a = Polynomial::input(InputName::new('a'));
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
                Mul => RangeInclusive::new(
                    lhs_range.start() * rhs_range.start(),
                    lhs_range.end() * rhs_range.end(),
                ),
                Div => RangeInclusive::new(
                    lhs_range.start() / rhs_range.end(),
                    lhs_range.end() / rhs_range.start(),
                ),
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
    println!("SIM {:?}", expr);
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
                Div => {
                    // In the context of a mod operation, we can recursively look at addends and multiplicands.
                    if let Some(simplified_lhs) = simplify_in_mod(lhs, modulus) {
                        Some(Expr::Op(*op_name, Rc::new(simplified_lhs), rhs_rc.clone()))
                    } else {
                        None
                    }
                }
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
                return Some(Expr::Poly(Polynomial::constant(perform_op(
                    *op_name, lhs_value, rhs_value,
                ))));
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
                }
                if let Expr::Poly(lhs_poly) = lhs {
                    if let Some(n) = get_constant(rhs) {
                        return Some(Expr::Poly(lhs_poly.times(n)));
                    }
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
            _ => None,
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
    )
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
            next_input: Some(InputName::new('a')),
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
    match expr {
        Expr::Poly(polynomial) => println!("{:?}", polynomial),
        Expr::Op(op_name, lhs_rc, rhs_rc) => {
            print!("{:?} ", op_name);
            print_tree(&**lhs_rc, indentation + 1);
            indent(indentation + 1);
            print_tree(&**rhs_rc, indentation + 1);
        }
    }
}

fn day_24_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut state = State::start();
    for line in lines {
        println!("INSTRUCTION: {:?}\n", line);
        let instruction = Instruction::parse(line);
        state = state.after(&instruction);
        print_state(&state);
    }
    println!("\n\n\n\n\n\n");
    print_tree(&state.registers[3], 0);
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
