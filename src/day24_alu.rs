use std::fmt;

/// Registers are named 'w' through 'z'
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RegisterName {
    name: char,
}

impl RegisterName {
    pub fn all() -> Vec<RegisterName> {
        ('w'..='z').map(|c| RegisterName::new(c)).collect()
    }

    pub fn new(name: char) -> RegisterName {
        if name < 'w' || 'z' < name {
            panic!("bad register name: {}", name);
        }
        RegisterName { name }
    }

    pub fn parse(s: &str) -> RegisterName {
        if s.len() != 1 {
            panic!("register name too long");
        }
        RegisterName::new(s.chars().next().unwrap())
    }

    pub fn index(&self) -> usize {
        (self.name as usize) - ('w' as usize)
    }
}

impl fmt::Debug for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
