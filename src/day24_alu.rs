use std::fmt;
use std::str;

/// Error type used for a variety of things in this day24_alu module.
#[derive(Debug, Eq, PartialEq)]
pub enum AluError {
    BadRegisterName(String),
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
