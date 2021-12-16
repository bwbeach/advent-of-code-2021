use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The answer to each problem is a positive integer
type Answer = u64;

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

/// Takes a vector of strings and converts them to u64
fn lines_to_numbers(lines: &Vec<String>) -> Result<Vec<u64>> {
    let result: std::result::Result<Vec<u64>, std::num::ParseIntError> =
        lines.iter().map(|s| s.parse()).collect();
    Ok(result?)
}

/// 1a: Counts lines containin numbers bigger than the line before
fn day_1_a(lines: &Vec<String>) -> Result<Answer> {
    let mut prev: Option<u64> = None;
    let mut count: u64 = 0;
    for value in lines_to_numbers(&lines)? {
        let is_increase = match prev {
            Some(prev_value) => prev_value < value,
            None => false,
        };
        if is_increase {
            count += 1;
        }
        prev = Some(value)
    }
    Ok(count)
}

/// 1b: Counts groups of three lines containin numbers bigger than the line before
fn day_1_b(lines: &Vec<String>) -> Result<Answer> {
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
    Ok(count)
}

enum SubmarineDirection {
    Up,
    Down,
    Forward,
}

impl FromStr for SubmarineDirection {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<SubmarineDirection, Self::Err> {
        match s {
            "up" => Ok(SubmarineDirection::Up),
            "down" => Ok(SubmarineDirection::Down),
            "forward" => Ok(SubmarineDirection::Forward),
            _ => Err(AdventError {
                message: format!("unknown submarine direction: {}", s),
            }),
        }
    }
}

struct SubmarineCommand {
    direction: SubmarineDirection,
    distance: u64,
}

impl FromStr for SubmarineCommand {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<SubmarineCommand, Self::Err> {
        let mut iter = s.split_whitespace();
        let direction: SubmarineDirection = iter.next().unwrap().parse()?;
        // TODO: translate error
        let distance: u64 = iter.next().unwrap().parse().expect("parsing distance");
        Ok(SubmarineCommand {
            direction,
            distance,
        })
    }
}

// TODO: unit tests for parsing

fn day_2_a(lines: &Vec<String>) -> Result<Answer> {
    let mut distance = 0;
    let mut depth = 0;
    for line in lines {
        let command: SubmarineCommand = line.parse()?;
        match command.direction {
            SubmarineDirection::Up => depth -= command.distance,
            SubmarineDirection::Down => depth += command.distance,
            SubmarineDirection::Forward => distance += command.distance,
        }
    }
    Ok(distance * depth)
}

fn day_2_b(lines: &Vec<String>) -> Result<Answer> {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let command: SubmarineCommand = line.parse()?;
        match command.direction {
            SubmarineDirection::Up => aim -= command.distance,
            SubmarineDirection::Down => aim += command.distance,
            SubmarineDirection::Forward => {
                distance += command.distance;
                depth += aim * command.distance;
            }
        }
    }
    Ok(distance * depth)
}

fn day_3_a(lines: &Vec<String>) -> Result<Answer> {
    let number_of_bits = lines[0].len();
    let numbers: Vec<u64> = lines
        .iter()
        .map(|s| u64::from_str_radix(s, 2).unwrap())
        .collect();
    let mut epsilon: u64 = 0;
    let mut gamma: u64 = 0;
    for i in 0..number_of_bits {
        let mask: u64 = 1 << i;
        let number_of_ones = numbers.iter().filter(|n| *n & mask != 0).count();
        if number_of_ones < numbers.len() / 2 {
            epsilon += mask;
        } else {
            gamma += mask;
        }
    }
    Ok(epsilon * gamma)
}

fn day_3_b(_lines: &Vec<String>) -> Result<Answer> {
    Ok(0)
}

/// Solutions know how to take the input lines for a problem and produce the answer.
type Solution = fn(&Vec<String>) -> Result<Answer>;

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
    match problem_name {
        "day-1-a" => Ok(day_1_a),
        "day-1-b" => Ok(day_1_b),
        "day-2-a" => Ok(day_2_a),
        "day-2-b" => Ok(day_2_b),
        "day-3-a" => Ok(day_3_a),
        "day-3-b" => Ok(day_3_b),
        _ => Err(Box::new(AdventError {
            message: format!("no such problem: {}", problem_name.escape_debug()),
        })),
    }
}

/// Returns a mapping from problem/input to expected answer
///
/// Answers are added here after being checked against the advent of code
/// web site.  These are used as regression tests when refactoring code.
fn build_expected_answers() -> HashMap<String, Answer> {
    let mut result = HashMap::new();
    let mut add = |name: &str, answer: u64| {
        result.insert(name.to_string(), answer);
    };
    add("input/day-1-a/sample.txt", 7);
    add("input/day-1-a/input.txt", 1233);
    add("input/day-1-b/sample.txt", 5);
    add("input/day-1-b/input.txt", 1275);
    add("input/day-2-a/sample.txt", 150);
    add("input/day-2-a/input.txt", 1383564);
    add("input/day-2-b/sample.txt", 900);
    add("input/day-2-b/input.txt", 1488311643);
    add("input/day-3-a/sample.txt", 198);
    add("input/day-3-a/input.txt", 693486);
    result
}

/// Returns a list of all of the days we have input data sets for.
fn all_days() -> Result<Vec<String>> {
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

/// Returns a list of all of the prbolems, assuming each day
/// has a "-a" and a "-b" version.
fn all_problems() -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for day in all_days()? {
        result.push(format!("{}-a", day));
        result.push(format!("{}-b", day));
    }
    Ok(result)
}

fn run_problem(problem_name: &str) -> Result<()> {
    println!("\n########");
    println!("# {}", problem_name);
    println!("########\n");
    let solution = function_for_problem(problem_name)?;
    let end = problem_name.len() - 2;
    let day_name = &problem_name[0..end];
    let input_dir = format!("input/{}", day_name.to_string());
    println!("Input dir: {}", input_dir);
    for entry in std::fs::read_dir(input_dir)? {
        let path = entry?.path();
        println!("Reading file: {}", path.display());
        let lines = lines_in_file(&path)?;
        let answer = solution(&lines)?;
        println!("answer: {}", answer);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let expected_answer_name = format!("input/{}/{}", problem_name, file_name);
        let expected_answers = build_expected_answers();
        match expected_answers.get(&expected_answer_name) {
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
