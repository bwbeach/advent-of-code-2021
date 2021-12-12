use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn day_1_a(lines: Box<dyn Iterator<Item = std::io::Result<String>>>) -> Result<String> {
    let mut prev: Option<u64> = None;
    let mut count: u64 = 0;
    for line in lines {
        let value: u64 = line?.parse()?;
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: advent <problemName>");
        std::process::exit(1);
    }
    let problem_name = &args[1];
    let input_file = format!("input/{}/sample.txt", problem_name);
    println!("Reading file: {}", input_file);
    let file = File::open(input_file)?;
    let input_lines = BufReader::new(file).lines();
    let answer = day_1_a(Box::new(input_lines));
    println!("answer: {}", answer?);
    Ok(())
}
