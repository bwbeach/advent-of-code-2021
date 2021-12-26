use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::types::{AdventResult, Answer, Day, DayPart};

#[derive(Debug)]
struct Input {
    initial_string: String,
    rules: HashMap<(char, char), char>,
}

lazy_static! {
    static ref RULE_PATTERN: Regex = Regex::new(r"^(.)(.) -> (.)$").expect("fold regex");
}

fn parse_rule(line: &str) -> ((char, char), char) {
    match RULE_PATTERN.captures(line) {
        None => {
            panic!("bad rule: {:?}", line);
        }
        Some(captures) => {
            let lh1 = captures[1].chars().next().unwrap();
            let lh2 = captures[2].chars().next().unwrap();
            let rh = captures[3].chars().next().unwrap();
            ((lh1, lh2), rh)
        }
    }
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut iter = lines.iter();
    let initial_string = iter.next().unwrap().clone();
    if iter.next().unwrap() != "" {
        panic!("expected blank second line");
    }
    let rules = iter.map(|line| parse_rule(line)).collect();
    Input {
        initial_string,
        rules,
    }
}

fn one_step(before: &str, rules: &HashMap<(char, char), char>) -> String {
    let mut after = String::new();
    let mut prev_c = None;
    for c in before.chars() {
        if let Some(p) = prev_c {
            if let Some(&to_insert) = rules.get(&(p, c)) {
                after.push(to_insert);
            }
        }
        after.push(c);
        prev_c = Some(c);
    }
    after
}

#[test]
fn test_one_step() {
    let mut rules = HashMap::new();
    rules.insert(('B', 'D'), 'C');
    assert_eq!("ABCD", one_step("ABD", &rules));
}

fn day_14_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let input = parse_input(lines);
    let mut current = input.initial_string.clone();
    for _ in 0..10 {
        current = one_step(&current, &input.rules);
    }

    // Count each char
    let mut char_to_count: HashMap<char, Answer> = HashMap::new();
    for c in current.chars() {
        *(char_to_count.entry(c).or_insert(0)) += 1;
    }

    // Sort by frequency
    let mut count_and_char: Vec<_> = char_to_count.iter().map(|(c, n)| (n, c)).collect();
    count_and_char.sort();
    let min_count = count_and_char[0].0;
    let max_count = count_and_char.last().unwrap().0;
    Ok(max_count - min_count)
}

fn day_14_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_14() -> Day {
    Day::new(
        14,
        DayPart::new(day_14_a, 1588, 2112),
        DayPart::new(day_14_b, 0, 0),
    )
}
