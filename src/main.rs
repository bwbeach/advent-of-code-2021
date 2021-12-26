use std::env;
use std::path::Path;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod grid;
mod types;
mod util;

use types::{AdventResult, Answer, Day, DayPart};
use util::lines_in_file;

fn run_once(
    day_part: &DayPart,
    input_dir: &str,
    file_name: &str,
    expected: Answer,
) -> AdventResult<Answer> {
    let path = format!("{}/{}", input_dir, file_name);
    let lines = lines_in_file(Path::new(&path))?;
    let answer = day_part.solve(&lines)?;
    println!("{} -> {}", path, answer);
    if answer != expected {
        panic!("MISMATCH");
    }
    Ok(answer)
}

fn run_day_part(day: &Day, is_first_part: bool) -> AdventResult<()> {
    println!("\n########");
    println!("# {} part {}", day, if is_first_part { "A" } else { "B" });
    println!("########\n");
    let input_dir = day.input_dir();
    let day_part = if is_first_part {
        &day.part_a
    } else {
        &day.part_b
    };
    run_once(day_part, &input_dir, "sample.txt", day_part.sample_answer)?;
    run_once(day_part, &input_dir, "input.txt", day_part.full_answer)?;
    Ok(())
}

fn run_day(day: &Day) -> AdventResult<()> {
    run_day_part(day, true)?;
    run_day_part(day, false)?;
    Ok(())
}

fn main() -> AdventResult<()> {
    // All the days
    let days = vec![
        day1::make_day_1(),
        day2::make_day_2(),
        day3::make_day_3(),
        day4::make_day_4(),
        day5::make_day_5(),
        day6::make_day_6(),
        day7::make_day_7(),
        day8::make_day_8(),
        day9::make_day_9(),
        day10::make_day_10(),
        day11::make_day_11(),
        day12::make_day_12(),
        day13::make_day_13(),
        day14::make_day_14(),
        day15::make_day_15(),
    ];

    // Parse the command-line argument to get the problem name to run, or "all"
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: advent [<dayNumber>|all]");
        std::process::exit(1);
    }

    // Figure out which problems to run
    let problem_name = &args[1];
    let problems_to_run: Vec<&Day> = if problem_name == "all" {
        days.iter().collect()
    } else {
        let day_number: usize = args[1].parse().unwrap();
        vec![&days[day_number - 1]]
    };

    // Run them
    for day in problems_to_run.iter() {
        match run_day(day) {
            Err(x) => return Err(x),
            Ok(_) => {}
        }
    }
    Ok(())
}
