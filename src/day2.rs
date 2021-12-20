use std::str::FromStr;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

#[derive(Debug, PartialEq)]
enum SubmarineDirection {
    Up,
    Down,
    Forward,
}

impl FromStr for SubmarineDirection {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<SubmarineDirection, Self::Err> {
        match s {
            "up" => Ok(SubmarineDirection::Up),
            "down" => Ok(SubmarineDirection::Down),
            "forward" => Ok(SubmarineDirection::Forward),
            _ => Err(AdventError::new(&format!(
                "unknown submarine direction: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SubmarineCommand {
    direction: SubmarineDirection,
    distance: u64,
}

impl FromStr for SubmarineCommand {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<SubmarineCommand, Self::Err> {
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

#[test]
fn test_submarine_command() {
    assert_eq!(
        SubmarineCommand {
            direction: SubmarineDirection::Forward,
            distance: 45
        },
        SubmarineCommand::from_str("forward 45").unwrap()
    )
}

// TODO: unit tests for parsing

fn day_2_a(lines: &Vec<String>) -> AdventResult<Answer> {
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

fn day_2_b(lines: &Vec<String>) -> AdventResult<Answer> {
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

pub fn make_day_2() -> Day {
    Day::new(
        2,
        DayPart::new(day_2_a, 150, 1383564),
        DayPart::new(day_2_b, 900, 1488311643),
    )
}
