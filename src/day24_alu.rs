use std::fmt;
use std::str;

/// Error type used for a variety of things in this day24_alu module.
#[derive(Debug, Eq, PartialEq)]
pub enum AluError {
    BadRegisterName(String),
    NotRegisterOrConstant(String),
}

/// The name of a register in the ALU
///
/// Registers are named 'w' through 'z'
///
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RegisterName {
    name: char,
}

impl RegisterName {
    /// Returns the names of all registers in the ALU
    pub fn all() -> Vec<RegisterName> {
        ('w'..='z').map(|name| (RegisterName { name })).collect()
    }

    /// Returns the index of this register in the ALU.
    ///
    /// Registers are indexed starting at 0.
    ///
    pub fn index(&self) -> usize {
        (self.name as usize) - ('w' as usize)
    }
}

impl fmt::Debug for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl str::FromStr for RegisterName {
    type Err = AluError;

    fn from_str(s: &str) -> Result<RegisterName, AluError> {
        if s.len() != 1 {
            return Err(AluError::BadRegisterName(s.to_string()));
        }
        let name = s.chars().next().unwrap();
        if name < 'w' || 'z' < name {
            return Err(AluError::BadRegisterName(s.to_string()));
        }
        Ok(RegisterName { name })
    }
}

#[test]
fn test_register_name() {
    assert_eq!(
        AluError::BadRegisterName("bad".to_string()),
        "bad".parse::<RegisterName>().err().unwrap()
    );
    assert_eq!(
        AluError::BadRegisterName("m".to_string()),
        "m".parse::<RegisterName>().err().unwrap()
    );
    assert_eq!(
        RegisterName { name: 'x' },
        "x".parse::<RegisterName>().unwrap()
    );
}

/// The name of one of the inputs
///
/// Inputs are named 'a' through 'n'
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct InputName {
    name: char,
}

impl InputName {
    pub fn all() -> Vec<InputName> {
        ('a'..='n').map(|name| InputName { name }).collect()
    }

    /// Returns the index of this input.
    ///
    /// The first input value is 'a', then 'b', etc., and they
    /// are indexed starting at 0.
    ///
    pub fn index(&self) -> usize {
        (self.name as usize) - ('a' as usize)
    }

    /// Returns the name of the next input, if there is one.
    ///
    pub fn next(&self) -> Option<InputName> {
        if self.name == 'n' {
            None
        } else {
            let name = ((self.name as u8) + 1) as char;
            Some(InputName { name })
        }
    }

    /// Returns the name of the first input.  (Used mostly for tests)
    pub fn first() -> InputName {
        let name = 'a';
        InputName { name }
    }
}

impl fmt::Debug for InputName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Holds the right-hand side of many instructions, which can be
/// either an integer constant or a register name.
///
pub enum RegisterOrConstant {
    Register(RegisterName),
    Constant(i64),
}

use RegisterOrConstant::*;

impl fmt::Debug for RegisterOrConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register(register_name) => write!(f, "{:?}", register_name),
            Constant(n) => write!(f, "{:?}", n),
        }
    }
}

impl str::FromStr for RegisterOrConstant {
    type Err = AluError;
    fn from_str(s: &str) -> Result<RegisterOrConstant, AluError> {
        if let Ok(register_name) = s.parse::<RegisterName>() {
            Ok(Register(register_name))
        } else if let Ok(n) = s.parse::<i64>() {
            Ok(Constant(n))
        } else {
            Err(AluError::NotRegisterOrConstant(s.to_string()))
        }
    }
}

/// The names of the arithmetic operations the ALU can perform
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum OpName {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

use OpName::*;

impl OpName {
    pub fn parse(s: &str) -> OpName {
        match s {
            "add" => Add,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "eql" => Eql,
            _ => panic!("bad op name: {:?}", s),
        }
    }

    pub fn perform(self, a: i64, b: i64) -> i64 {
        match self {
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

#[test]
fn test_perform_op() {
    assert_eq!(10, Add.perform(2, 8));
    assert_eq!(16, Mul.perform(2, 8));
    assert_eq!(3, Div.perform(7, 2));
    assert_eq!(3, Div.perform(-7, -2));
    assert_eq!(-3, Div.perform(-7, 2));
    assert_eq!(-3, Div.perform(7, -2));
    assert_eq!(1, Mod.perform(7, 2));
    assert_eq!(0, Eql.perform(3, 5));
    assert_eq!(1, Eql.perform(5, 5));
}
