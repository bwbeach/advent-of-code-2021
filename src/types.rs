use std::fmt::{Display, Formatter};

/// Result type used throughout Advent of Code
pub type AdventResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Error with a message
#[derive(Debug, Clone)]
pub struct AdventError {
    message: String,
}

impl AdventError {
    pub fn new(message: &str) -> AdventError {
        AdventError {
            message: message.to_string(),
        }
    }
}

impl Display for AdventError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "AdventError: {}", self.message)
    }
}

impl std::error::Error for AdventError {}

/// The answer to each problem is a positive integer
pub type Answer = u64;

/// Solutions know how to take the input lines for a problem and produce the answer.
pub type Solver = fn(&Vec<String>) -> AdventResult<Answer>;

/// The implementation for each day contains a solution for part A and
/// part B of the problem.
#[derive(Clone)]
pub struct DayPart {
    pub solver: Solver,
    pub sample_answer: Answer,
    pub full_answer: Answer,
}

impl DayPart {
    pub fn new(solver: Solver, sample_answer: Answer, full_answer: Answer) -> DayPart {
        DayPart {
            solver,
            sample_answer,
            full_answer,
        }
    }

    pub fn solve(&self, lines: &Vec<String>) -> AdventResult<Answer> {
        let s = self.solver;
        s(lines)
    }
}

/// The implementation for each day contains a solution for part A and
/// part B of the problem.
#[derive(Clone)]
pub struct Day {
    pub number: usize,
    pub part_a: DayPart,
    pub part_b: DayPart,
}

impl Day {
    pub fn new(number: usize, part_a: DayPart, part_b: DayPart) -> Day {
        Day {
            number,
            part_a,
            part_b,
        }
    }

    pub fn input_dir(&self) -> String {
        format!("input/day-{}", self.number)
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day-{}", self.number)
    }
}
