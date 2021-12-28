use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A pair of two adjacent letters
type Pair = (char, char);

/// Internal representation of a string of characters.
///
/// We don't keep the actual string because it gets too long
/// when running a full forty iterations for part two of this
/// problem.  Instead, we keep a count of each pair of characters.
///
/// Each character in the string appears twice in the map, once
/// as the left half of the pair, and once as the right half.
/// The exceptions are the first and last character of the string,
/// which each appear just once; the first character as the left
/// half of one pair, an the last character as the second half of
/// one pair.
///
/// To count the number of occurrences of a letter, we can count
/// just the right half of each pair, which will count each letter
/// once, but exclude the first char.  So we also explicitly keep
/// the first char in the string.
///
#[derive(Clone, Debug, PartialEq)]
struct State {
    first_char: char,
    pair_to_count: HashMap<(char, char), u64>,
}

/// Parses a string of chars and turns it into a State.
fn parse_state(line: &str) -> State {
    let first_char = line.chars().next().unwrap();
    let mut pair_to_count = HashMap::new();
    for pair in line.chars().tuple_windows() {
        *(pair_to_count.entry(pair).or_insert(0)) += 1;
    }
    State {
        first_char,
        pair_to_count,
    }
}

#[test]
fn test_parse_state() {
    let mut pair_to_count = HashMap::new();
    pair_to_count.insert(('A', 'B'), 1);
    pair_to_count.insert(('B', 'B'), 2);
    let expected = State {
        first_char: 'A',
        pair_to_count,
    };
    assert_eq!(expected, parse_state("ABBB"));
}

/// Holds the contents of the input file.
#[derive(Debug)]
struct Input {
    // Internal representation of the initial string
    initial_state: State,

    // The rules that specify each pair that gets a char inserted
    // in the middle at each step.
    rules: HashMap<Pair, char>,
}

// Regex for parsing rules
lazy_static! {
    static ref RULE_PATTERN: Regex = Regex::new(r"^(.)(.) -> (.)$").expect("fold regex");
}

/// Parses one rule
fn parse_rule(line: &str) -> (Pair, char) {
    match RULE_PATTERN.captures(line) {
        None => {
            panic!("bad rule: {:?}", line);
        }
        Some(captures) => {
            let lhs = (
                captures[1].chars().next().unwrap(),
                captures[2].chars().next().unwrap(),
            );
            let rhs = captures[3].chars().next().unwrap();
            (lhs, rhs)
        }
    }
}

/// Parses the entire input file
fn parse_input(lines: &[&str]) -> Input {
    let mut iter = lines.iter();
    let initial_string = iter.next().unwrap().clone();
    let initial_state = parse_state(&initial_string);
    if *iter.next().unwrap() != "" {
        panic!("expected blank second line");
    }
    let rules = iter.map(|line| parse_rule(line)).collect();
    Input {
        initial_state,
        rules,
    }
}

/// Adds a delta to a counter stored in a hash map.
fn add_to_count<T>(key: T, delta: u64, key_to_count: &mut HashMap<T, u64>)
where
    T: Eq,
    T: std::hash::Hash,
{
    *(key_to_count.entry(key).or_insert(0)) += delta;
}

fn one_step(before: &State, rules: &HashMap<(char, char), char>) -> State {
    let first_char = before.first_char;
    let mut pair_to_count = HashMap::new();
    for (&from_pair, &from_count) in before.pair_to_count.iter() {
        if let Some(&b) = rules.get(&from_pair) {
            let (a, c) = from_pair;
            add_to_count((a, b), from_count, &mut pair_to_count);
            add_to_count((b, c), from_count, &mut pair_to_count);
        } else {
            add_to_count(from_pair, from_count, &mut pair_to_count);
        }
    }
    State {
        first_char,
        pair_to_count,
    }
}

#[test]
fn test_one_step() {
    let mut rules = HashMap::new();
    rules.insert(('B', 'D'), 'C');
    assert_eq!(parse_state("ABCD"), one_step(&parse_state("ABD"), &rules));
}

fn day_14(lines: &[&str], step_count: usize) -> AdventResult<Answer> {
    let input = parse_input(lines);
    let mut current = input.initial_state.clone();
    for _ in 0..step_count {
        current = one_step(&current, &input.rules);
    }

    // Count each char
    let mut char_to_count: HashMap<char, Answer> = HashMap::new();
    add_to_count(current.first_char, 1, &mut char_to_count);
    for ((_, c), count) in current.pair_to_count {
        add_to_count(c, count, &mut char_to_count);
    }

    // Sort by frequency
    let mut count_and_char: Vec<_> = char_to_count.iter().map(|(c, n)| (n, c)).collect();
    count_and_char.sort();
    let min_count = count_and_char[0].0;
    let max_count = count_and_char.last().unwrap().0;
    Ok(max_count - min_count)
}

fn day_14_a(lines: &[&str]) -> AdventResult<Answer> {
    day_14(lines, 10)
}

fn day_14_b(lines: &[&str]) -> AdventResult<Answer> {
    day_14(lines, 40)
}

pub fn make_day_14() -> Day {
    Day::new(
        14,
        DayPart::new(day_14_a, 1588, 2112),
        DayPart::new(day_14_b, 2188189693529, 3243771149914),
    )
}
