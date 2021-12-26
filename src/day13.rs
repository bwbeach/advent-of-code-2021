use std::collections::HashSet;
use std::str::FromStr;

use crate::grid::{parse_point, Point};
use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

lazy_static! {
    static ref FOLD_PATTERN: Regex =
        Regex::new(r"fold along ([x|y])=([0-9]+)").expect("fold regex");
}

impl FromStr for FoldInstruction {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<FoldInstruction, AdventError> {
        match FOLD_PATTERN.captures(s) {
            None => Err(AdventError::new("bad fold instruction")),
            Some(captures) => {
                let axis = &captures[1];
                let ordinate: usize = captures[2].parse().unwrap();
                match axis {
                    "x" => Ok(FoldInstruction::X(ordinate)),
                    "y" => Ok(FoldInstruction::Y(ordinate)),
                    _ => Err(AdventError::new("bug in fold regex")),
                }
            }
        }
    }
}

#[test]
fn test_parse_fold_instruction() {
    assert_eq!(
        FoldInstruction::X(10),
        FoldInstruction::from_str("fold along x=10").unwrap()
    )
}

/// Returns the new location of a point after folding
fn fold_point(p: Point, fold: FoldInstruction) -> Point {
    match fold {
        FoldInstruction::X(n) => {
            if p.0 < n {
                p
            } else {
                if 2 * p.0 < n {
                    panic!("fold overflow");
                }
                (2 * n - p.0, p.1)
            }
        }
        FoldInstruction::Y(n) => {
            if p.1 < n {
                p
            } else {
                if 2 * p.1 < n {
                    panic!("fold overflow");
                }
                (p.0, 2 * n - p.1)
            }
        }
    }
}

#[test]
fn test_fold_point() {
    assert_eq!((1, 3), fold_point((1, 3), FoldInstruction::X(5)));
    assert_eq!((1, 3), fold_point((9, 3), FoldInstruction::X(5)));
    assert_eq!((3, 1), fold_point((3, 1), FoldInstruction::Y(5)));
    assert_eq!((3, 1), fold_point((3, 9), FoldInstruction::Y(5)));
}

#[derive(Debug)]
struct Input {
    points: HashSet<Point>,
    folds: Vec<FoldInstruction>,
}

fn parse_input(lines: &Vec<String>) -> Input {
    let iter = &mut lines.iter();
    let points: HashSet<_> = iter
        .take_while(|&line| line != "")
        .map(|line| parse_point(line))
        .collect();
    let folds: Vec<_> = iter
        .map(|line| FoldInstruction::from_str(line).unwrap())
        .collect();
    Input { points, folds }
}

fn fold(points: &HashSet<Point>, f: FoldInstruction) -> HashSet<Point> {
    points.iter().map(|&p| fold_point(p, f)).collect()
}

fn day_13_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let input = parse_input(lines);
    let points = fold(&input.points, input.folds[0]);
    Ok(points.len() as u64)
}

fn day_13_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_13() -> Day {
    Day::new(
        13,
        DayPart::new(day_13_a, 17, 592),
        DayPart::new(day_13_b, 0, 0),
    )
}
