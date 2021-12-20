use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use itertools::{all, any};
use ndarray::{arr2, s, Array2}; // TODO: fix unused warning, and keep available for tests

mod types;
mod util;

use types::{AdventResult, Answer, Day, DayPart};
use util::lines_in_file;

/// Takes a vector of strings and converts them to u64
fn lines_to_numbers(lines: &Vec<String>) -> AdventResult<Vec<u64>> {
    let result: Result<Vec<u64>, std::num::ParseIntError> =
        lines.iter().map(|s| s.parse()).collect();
    Ok(result?)
}

#[test]
fn test_lines_to_numbers() {
    assert_eq!(
        vec![1, 456],
        lines_to_numbers(&vec!["1".to_string(), "456".to_string()]).unwrap()
    );
}

/// 1a: Counts lines containin numbers bigger than the line before
fn day_1_a(lines: &Vec<String>) -> AdventResult<Answer> {
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
fn day_1_b(lines: &Vec<String>) -> AdventResult<Answer> {
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

fn make_day_1() -> Day {
    Day::new(
        1,
        DayPart::new(day_1_a, 7, 1233),
        DayPart::new(day_1_b, 5, 1275),
    )
}

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
            _ => Err(AdventError {
                message: format!("unknown submarine direction: {}", s),
            }),
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

fn make_day_2() -> Day {
    Day::new(
        2,
        DayPart::new(day_2_a, 150, 1383564),
        DayPart::new(day_2_b, 900, 1488311643),
    )
}

fn day_3_a(lines: &Vec<String>) -> AdventResult<Answer> {
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

/// Returns the most common bit in a sequence of binary numbers
/// represented as strings of '0' and '1'.
///
/// TODO: switch from String to &str
fn most_common_bit_in_column(numbers: &Vec<String>, index: usize) -> char {
    let number_of_ones = numbers
        .iter()
        .filter(|s| s.as_bytes()[index] == '1' as u8)
        .count();
    if numbers.len() <= number_of_ones * 2 {
        '1'
    } else {
        '0'
    }
}

#[test]
fn test_most_common_bit_in_column() {
    let data = vec![
        "0001".to_string(),
        "0011".to_string(),
        "0111".to_string(),
        "1111".to_string(),
    ];
    assert_eq!('0', most_common_bit_in_column(&data, 0));
    assert_eq!('1', most_common_bit_in_column(&data, 1));
    assert_eq!('1', most_common_bit_in_column(&data, 2));
    assert_eq!('1', most_common_bit_in_column(&data, 3));
}

fn day_3_b_helper(lines: &Vec<String>, index: usize, keep_common: bool) -> String {
    if lines.len() == 0 {
        panic!("no lines in input");
    } else if lines.len() == 1 {
        lines[0].clone()
    } else {
        let most_common = most_common_bit_in_column(lines, index);
        let matching = lines
            .iter()
            .filter(|s| (s.as_bytes()[index] == most_common as u8) == keep_common)
            .map(|s| s.clone())
            .collect();
        day_3_b_helper(&matching, index + 1, keep_common)
    }
}
fn day_3_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let oxygen_line = day_3_b_helper(lines, 0, true);
    let oxygen = u64::from_str_radix(&oxygen_line, 2).unwrap();
    let co2_line = day_3_b_helper(lines, 0, false);
    let co2 = u64::from_str_radix(&co2_line, 2).unwrap();
    Ok(oxygen * co2)
}

fn make_day_3() -> Day {
    Day::new(
        3,
        DayPart::new(day_3_a, 198, 693486),
        DayPart::new(day_3_b, 230, 3379326),
    )
}

/// A number on a Day 4 bingo card
type BingoCardNumber = u8;

/// Finds the sequences of non-empty lines in a list of lines.
///
/// TODO: figure out how to use group_by, either the new experimental
/// feature in std, or the one in the itertools package.
fn group_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_group = Vec::new();
    for line in lines {
        if line.len() == 0 {
            if current_group.len() != 0 {
                result.push(current_group.clone());
                current_group.clear();
            }
        } else {
            current_group.push(line.clone());
        }
    }
    if current_group.len() != 0 {
        result.push(current_group.clone());
    }
    result
}

#[test]
fn test_group_lines() {
    assert_eq!(
        vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()]
        ],
        group_lines(&vec![
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "".to_string(),
            "".to_string(),
            "c".to_string(),
            "".to_string()
        ])
    )
}

/// Holds one bingo card and the numbers on it.
///
/// This is just the card, and does not track which
/// numbers have been drawn.
#[derive(Debug, PartialEq)]
struct BingoCard {
    grid: Array2<BingoCardNumber>,
}

impl BingoCard {
    /// Returns the number of rows/columns in the card
    fn size(&self) -> usize {
        // this assumes that the grid is square.  TODO: add asserting to struct to say that
        self.grid.shape()[0]
    }

    /// Returns true iff the numbers given complete any row or column
    fn is_bingo(&self, drawn: &HashSet<BingoCardNumber>) -> bool {
        for i in 0..self.size() {
            if all(self.grid.slice(s![i, ..]), |n| drawn.contains(n)) {
                return true;
            }
            if all(self.grid.slice(s![.., i]), |n| drawn.contains(n)) {
                return true;
            }
        }
        false
    }

    /// Returns the store of a winning board
    fn score(&self, all_drawn: &HashSet<BingoCardNumber>, last_draw: BingoCardNumber) -> u64 {
        let unpicked_sum: u64 = self
            .grid
            .iter()
            .filter(|&n| !all_drawn.contains(n))
            .map(|&n| n as u64)
            .sum();
        unpicked_sum * (last_draw as u64)
    }
}

#[test]
fn test_is_bingo() {
    fn make_set(items: &[BingoCardNumber]) -> HashSet<BingoCardNumber> {
        items.iter().map(|&n| n).collect()
    }
    let card = BingoCard {
        grid: arr2(&[[1, 2], [3, 4]]),
    };
    assert_eq!(false, card.is_bingo(&make_set(&[])));
    assert_eq!(false, card.is_bingo(&make_set(&[1])));
    assert_eq!(false, card.is_bingo(&make_set(&[1, 4])));
    assert_eq!(true, card.is_bingo(&make_set(&[1, 2])));
    assert_eq!(true, card.is_bingo(&make_set(&[1, 3])));
}

fn parse_bingo_card(lines: &Vec<String>) -> BingoCard {
    let size = lines.len();
    let mut grid = Array2::<BingoCardNumber>::zeros((size, size));
    for (y, line) in lines.iter().enumerate() {
        for (x, num_str) in line.split_whitespace().enumerate() {
            let number: BingoCardNumber = num_str.parse().unwrap();
            grid[(y, x)] = number;
        }
    }
    BingoCard { grid }
}

#[test]
fn test_parse_bingo_card() {
    assert_eq!(
        BingoCard {
            grid: arr2(&[[1, 2], [3, 4]])
        },
        parse_bingo_card(&vec!("1 2".to_string(), " 3  4 ".to_string()))
    )
}

/// Holds the input to Day 4 problems
#[derive(Debug, PartialEq)]
struct Day4Input {
    // The list of numbers called
    called: Vec<BingoCardNumber>,

    // The collection of cards
    cards: Vec<BingoCard>,
}

fn parse_day_4_input(lines: &Vec<String>) -> Day4Input {
    let called: Vec<BingoCardNumber> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    // TODO: use slice of &str
    let remaining_lines = lines[1..].iter().map(|line| line.clone()).collect();
    let cards: Vec<BingoCard> = group_lines(&remaining_lines)
        .iter()
        .map(|g| parse_bingo_card(g))
        .collect();
    Day4Input { called, cards }
}

#[test]
fn test_parse_day_4_input() {
    let input: Vec<String> = r"13,15

    1 2
    3 4

    5 6
    7 8
"
    .split("\n")
    .map(|s| s.to_string())
    .collect();

    assert_eq!(
        Day4Input {
            called: vec![13, 15],
            cards: vec![
                BingoCard {
                    grid: arr2(&[[1, 2], [3, 4]])
                },
                BingoCard {
                    grid: arr2(&[[5, 6], [7, 8]])
                }
            ]
        },
        parse_day_4_input(&input)
    )
}

fn day_4_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let input = parse_day_4_input(lines);
    let mut picked_so_far = HashSet::<BingoCardNumber>::new();
    for &draw in input.called.iter() {
        picked_so_far.insert(draw);
        for card in input.cards.iter() {
            if card.is_bingo(&picked_so_far) {
                return Ok(card.score(&picked_so_far, draw));
            }
        }
    }
    Ok(0)
}

fn day_4_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let input = parse_day_4_input(lines);
    let mut picked_so_far = HashSet::<BingoCardNumber>::new();
    // all of the cards that have won so far
    let mut winners = HashSet::<usize>::new();
    for &draw in input.called.iter() {
        picked_so_far.insert(draw);
        for (i, card) in input.cards.iter().enumerate() {
            if !winners.contains(&i) {
                if card.is_bingo(&picked_so_far) {
                    winners.insert(i);
                    if winners.len() == input.cards.len() {
                        return Ok(card.score(&picked_so_far, draw));
                    }
                }
            }
        }
    }
    Ok(0)
}

fn make_day_4() -> Day {
    Day::new(
        4,
        DayPart::new(day_4_a, 4512, 58374),
        DayPart::new(day_4_b, 1924, 11377),
    )
}

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
    let days = vec![make_day_1(), make_day_2(), make_day_3(), make_day_4()];

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
