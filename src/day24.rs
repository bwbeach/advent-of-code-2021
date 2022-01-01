use std::fmt;
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
    fn new(name: char) -> InputName {
        if name < 'a' || 'n' < name {
            panic!("bad input name: {}", name);
        }
        InputName { name }
    }

    fn parse(s: &str) -> InputName {
        if s.len() != 1 {
            panic!("input name too long");
        }
        InputName::new(s.chars().next().unwrap())
    }

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

#[derive(Clone, Eq, PartialEq)]
enum Expr {
    Constant(i64),
    Input(InputName),
    Op(OpName, Rc<Expr>, Rc<Expr>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Constant(n) => write!(f, "{:?}", n),
            Expr::Input(register_name) => write!(f, "{:?}", register_name),
            Expr::Op(op_name, a, b) => write!(f, "({:?} {:?} {:?})", a, op_name, b),
        }
    }
}

fn simplify(expr: &Expr) -> Option<Expr> {
    if let Expr::Op(op_name, lhs_rc, rhs_rc) = expr {
        let lhs = &**lhs_rc;
        let rhs = &**rhs_rc;
        if let Expr::Constant(a) = lhs {
            if let Expr::Constant(b) = rhs {
                return Some(Expr::Constant(perform_op(*op_name, *a, *b)));
            }
        }
        match op_name {
            Add => {
                if let Expr::Constant(n) = lhs {
                    if *n == 0 {
                        return Some(rhs.clone());
                    }
                }
                if let Expr::Constant(n) = rhs {
                    if *n == 0 {
                        return Some(lhs.clone());
                    }
                }
                None
            }
            Mul => {
                if let Expr::Constant(n) = lhs {
                    if *n == 0 {
                        return Some(Expr::Constant(0));
                    }
                    if *n == 1 {
                        return Some(rhs.clone());
                    }
                }
                if let Expr::Constant(n) = rhs {
                    if *n == 0 {
                        return Some(Expr::Constant(0));
                    }
                    if *n == 1 {
                        return Some(lhs.clone());
                    }
                }
                None
            }
            Div => {
                if let Expr::Constant(n) = lhs {
                    if *n == 0 {
                        return Some(Expr::Constant(0));
                    }
                }
                if let Expr::Constant(n) = rhs {
                    if *n == 1 {
                        return Some(lhs.clone());
                    }
                }
                None
            }
            Eql => {
                if let Expr::Constant(n) = lhs {
                    if let Expr::Input(_) = rhs {
                        if *n < 1 || 9 < *n {
                            return Some(Expr::Constant(0));
                        }
                    }
                }
                if let Expr::Constant(n) = rhs {
                    if let Expr::Input(_) = lhs {
                        if *n < 1 || 9 < *n {
                            return Some(Expr::Constant(0));
                        }
                    }
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
    assert_eq!(Expr::Constant(0), get_w_expression(&[]));

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
                Rc::new(Expr::Constant(0)),
                Rc::new(Expr::Constant(0)),
                Rc::new(Expr::Constant(0)),
                Rc::new(Expr::Constant(0)),
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
                            Expr::Input(input_name),
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
                    Constant(n) => Rc::new(Expr::Constant(*n)),
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
        println!("{:?} = {:?}", r, *expr);
    }
    println!("");
}

fn day_24_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut state = State::start();
    for line in lines {
        println!("INSTRUCTION: {:?}\n", line);
        let instruction = Instruction::parse(line);
        state = state.after(&instruction);
        print_state(&state);
    }
    print_state(&state);
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
