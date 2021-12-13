use std::env;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Returns a vector containing all of the lines in a file.Iterator
///
fn lines_in_file(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        result.push(line?);
    }
    Ok(result)
}

fn day_1_a(lines: &Vec<String>) -> Result<String> {
    let mut prev: Option<u64> = None;
    let mut count: u64 = 0;
    for line in lines {
        let value: u64 = line.parse()?;
        let is_increase = match prev {
            Some(prev_value) => prev_value < value,
            None => false,
        };
        if is_increase {
            count += 1;
        }
        prev = Some(value)
    }
    Ok(format!("{}", count))
}

/// Solutions know how to take the input lines for a problem and produce the answer.
type Solution = fn(&Vec<String>) -> Result<String>;

/// Error that indicates there is no such problem.
#[derive(Debug, Clone)]
struct NoSuchProblemError;

impl fmt::Display for NoSuchProblemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function for problem not implemented")
    }
}

impl std::error::Error for NoSuchProblemError {}

fn function_for_problem(problem_name: &str) -> Result<Solution> {
    if problem_name == "day-1-a" {
        Ok(day_1_a)
    } else {
        Err(Box::new(NoSuchProblemError))
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: advent <problemName>");
        std::process::exit(1);
    }
    let problem_name = &args[1];
    let solution = function_for_problem(problem_name)?;
    let input_dir = format!("input/{}", problem_name);
    println!("Input dir: {}", input_dir);
    for entry in std::fs::read_dir(input_dir)? {
        let path = entry?.path();
        println!("Reading file: {}", path.display());
        let lines = lines_in_file(&path)?;
        let answer = solution(&lines);
        println!("answer: {}", answer?);
    }
    Ok(())
}
