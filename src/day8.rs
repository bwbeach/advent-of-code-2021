use std::fmt;
use std::ops::BitAnd;
use std::str::FromStr;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

/// A displayed digit, with some subset of the seven segments lit up.
///
/// We don't know which segment is which.
///
#[derive(Clone, Copy, PartialEq)]
struct Display {
    // Segment n is lit up if bit (1 << n) is on
    bits: u8,

    // Number of segments lit up
    count: u8,
}

impl Display {
    fn new(bits: u8) -> Display {
        let count = (0..=7).filter(|n| bits & (1u8 << n) != 0).count() as u8;
        Display { bits, count }
    }

    fn contains(self, other: &Display) -> bool {
        (self.bits & other.bits) == other.bits
    }
}

#[test]
fn test_contains() {
    assert_eq!(true, Display::new(7).contains(&Display::new(3)));
    assert_eq!(false, Display::new(7).contains(&Display::new(12)));
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..7 {
            if self.bits & (1 << i) != 0 {
                write!(f, "{}", (b'a' + i) as char)?
            }
        }
        Ok(())
    }
}

#[test]
fn test_debug_display() {
    assert_eq!("ace", format!("{:?}", Display::new(21)))
}

impl BitAnd<Display> for Display {
    type Output = Display;

    fn bitand(self, rhs: Display) -> Display {
        Display::new(self.bits & rhs.bits)
    }
}

#[test]
fn test_bitand_display() {
    assert_eq!(Display::new(5), Display::new(7) & Display::new(13))
}

impl FromStr for Display {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Display, Self::Err> {
        let mut bits = 0;
        for &c in s.as_bytes() {
            if c < b'a' || b'g' < c {
                return Err(AdventError::new(&format!("Illegal character: {:?}", c)));
            }
            bits |= 1 << (c - b'a');
        }
        Ok(Display::new(bits))
    }
}

#[test]
fn test_parse_display() {
    assert_eq!(
        Display { bits: 5, count: 2 },
        Display::from_str("ca").unwrap()
    );
}

/// Parses a list of digits separated by spaces
fn parse_display_list(s: &str) -> Vec<Display> {
    s.split_whitespace()
        .map(|word| Display::from_str(word).unwrap())
        .collect()
}

/// Input line with ten sample digits, and the four digits of output
///
#[derive(Debug, PartialEq)]
struct InputLine {
    // The 10 sample displays on the left of the separator
    samples: Vec<Display>,

    // The four digits displayed after the separator
    output: Vec<Display>,
}

impl FromStr for InputLine {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<InputLine, Self::Err> {
        let parts: Vec<&str> = s.split("|").collect();
        let samples = parse_display_list(parts[0]);
        let output = parse_display_list(parts[1]);
        Ok(InputLine { samples, output })
    }
}

#[test]
fn test_parse_input_line() {
    assert_eq!(
        InputLine {
            samples: vec![
                Display::from_str("acedgfb").unwrap(),
                Display::from_str("cdfbe").unwrap()
            ],
            output: vec![Display::from_str("cdfeb").unwrap()]
        },
        InputLine::from_str("acedgfb cdfbe | cdfeb").unwrap()
    )
}

/// Maps from the count of lit LEDs to the digit, if the
/// LED count is sufficient info to know.
fn count_to_digit(n: u8) -> Option<usize> {
    match n {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

/// Maps from the count of lit LEDs to the digit, if the
/// LED count is sufficient info to know.
fn sample_and_mapping_to_digit(sample: Display, mapping: &[Display; 10]) -> Option<usize> {
    match sample.count {
        5 => {
            if sample.contains(&mapping[1]) {
                Some(3)
            } else {
                let share_with_4 = (sample & mapping[4]).count;
                if share_with_4 == 3 {
                    Some(5)
                } else {
                    Some(2)
                }
            }
        }
        6 => {
            if sample.contains(&mapping[4]) {
                Some(9)
            } else if !sample.contains(&mapping[1]) {
                Some(6)
            } else {
                Some(0)
            }
        }
        _ => None,
    }
}

/// Figures out the digit mapping on one line, and translates the output
fn solve_one_line(input: &InputLine) -> Vec<u8> {
    // For each digit which of the samples is used to represent it
    let mut mapping: [Display; 10] = [Display::new(0); 10];

    // First, assign the mappings to the digits that have unique
    // counts of lit LEDs.
    for &sample in input.samples.iter() {
        match count_to_digit(sample.count) {
            Some(digit) => mapping[digit] = sample,
            _ => {}
        }
    }

    // Now we can assign the rest, based on the mappings for 2, 3, 4, and 7.
    for &sample in input.samples.iter() {
        match sample_and_mapping_to_digit(sample, &mapping) {
            Some(digit) => mapping[digit] = sample,
            _ => {}
        }
    }

    // Function to map from an output display to a digit
    fn output_to_digit(output: Display, mapping: &[Display; 10]) -> u8 {
        for i in 0..10 {
            if mapping[i] == output {
                return i as u8;
            }
        }
        panic!("No mapping found for: {:?}", output);
    }

    input
        .output
        .iter()
        .map(|&out| output_to_digit(out, &mapping))
        .collect()
}

#[test]
fn test_solve_one_line() {
    assert_eq! {
        vec![5, 3, 5, 3],
        solve_one_line(&InputLine::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf").unwrap())
    }
}

fn day_8_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let count: usize = lines
        .iter()
        .map(|line| InputLine::from_str(line).unwrap())
        .map(|input_line| solve_one_line(&input_line))
        .flatten()
        .filter(|&n| n == 1 || n == 4 || n == 7 || n == 8)
        .count();
    Ok(count as u64)
}

/// Converts a vector of base 10 digits into a number.
///
/// [1, 2 ,3] becomes 123
///
fn vector_to_number(digits: &Vec<u8>) -> u64 {
    let mut result = 0;
    for &d in digits {
        result = result * 10 + (d as u64);
    }
    result
}

#[test]
fn test_vector_to_number() {
    assert_eq!(1234, vector_to_number(&vec![1, 2, 3, 4]));
}

fn day_8_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let total: u64 = lines
        .iter()
        .map(|line| InputLine::from_str(line).unwrap())
        .map(|input_line| solve_one_line(&input_line))
        .map(|v| vector_to_number(&v))
        .sum();
    Ok(total)
}

pub fn make_day_8() -> Day {
    Day::new(
        8,
        DayPart::new(day_8_a, 26, 383),
        DayPart::new(day_8_b, 61229, 998900),
    )
}
