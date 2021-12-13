use std::collections::HashMap;
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

/// 1a: Counts lines containin numbers bigger than the line before
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

/// 1b: Counts groups of three lines containin numbers bigger than the line before
fn day_1_b(lines: &Vec<String>) -> Result<String> {
    let mut a;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut num_seen: u64 = 0;
    let mut prev_sum: u64 = 0;
    let mut count: u64 = 0;
    for line in lines {
        a = b;
        b = c;
        c = line.parse()?;
        num_seen += 1;
        if 3 <= num_seen {
            let sum = a + b + c;
            if 4 <= num_seen && prev_sum < sum {
                count += 1;
            }
            prev_sum = sum;
        }
    }
    Ok(format!("{}", count))
}

/// Solutions know how to take the input lines for a problem and produce the answer.
type Solution = fn(&Vec<String>) -> Result<String>;

/// Error that indicates there is no such problem.
#[derive(Debug, Clone)]
struct AdventError {
    message: String,
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AdventError: {}", self.message)
    }
}

impl std::error::Error for AdventError {}

fn function_for_problem(problem_name: &str) -> Result<Solution> {
    if problem_name == "day-1-a" {
        Ok(day_1_a)
    } else if problem_name == "day-1-b" {
        Ok(day_1_b)
    } else {
        Err(Box::new(AdventError {
            message: format!("no such problem: {}", problem_name.escape_debug()),
        }))
    }
}

/// Returns a mapping from problem/input to expected answer
///
/// Answers are added here after being checked against the advent of code
/// web site.  These are used as regression tests when refactoring code.
fn build_expected_answers() -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut add = |name: &str, answer: &str| {
        result.insert(name.to_string(), answer.to_string());
    };
    add("input/day-1-a/sample.txt", "7");
    add("input/day-1-a/input.txt", "1233");
    add("input/day-1-b/sample.txt", "5");
    add("input/day-1-b/input.txt", "1275");
    result
}

/// Returns a list of all of the problems we have, in alphabetical order.
fn all_problems() -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for entry in std::fs::read_dir("input")? {
        result.push(
            entry?
                .path()
                .file_name()
                .expect("file without name")
                .to_str()
                .expect("invalid file name")
                .to_string(),
        )
    }
    result.sort();
    Ok(result)
}

fn run_problem(problem_name: &str) -> Result<()> {
    println!("\n########");
    println!("# {}", problem_name);
    println!("########\n");
    let solution = function_for_problem(problem_name)?;
    let input_dir = format!("input/{}", problem_name);
    println!("Input dir: {}", input_dir);
    for entry in std::fs::read_dir(input_dir)? {
        let path = entry?.path();
        println!("Reading file: {}", path.display());
        let lines = lines_in_file(&path)?;
        let answer = solution(&lines)?;
        println!("answer: {}", answer);
        let expected_answers = build_expected_answers();
        let path_str = format!("{}", path.display());
        match expected_answers.get(&path_str) {
            Some(expected_answer) => {
                println!("Expected {}", *expected_answer);
                if *expected_answer != answer {
                    println!("Mismatch: {} {}", answer, *expected_answer);
                    return Err(Box::new(AdventError {
                        message: "wrong answer".to_string(),
                    }));
                } else {
                    println!("match");
                }
            }
            None => {}
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // Parse the command-line argument to get the problem name to run, or "all"
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: advent <problemName>");
        std::process::exit(1);
    }

    // Figure out which problems to run
    let problem_name = &args[1];
    let problems_to_run = if problem_name == "all" {
        all_problems()?
    } else {
        vec![problem_name.clone()]
    };

    // Run them
    for name in problems_to_run {
        match run_problem(&name) {
            Err(x) => return Err(x),
            Ok(_) => {}
        }
    }
    Ok(())
}
