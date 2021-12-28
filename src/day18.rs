use std::fmt;
use std::iter;
use std::rc::Rc;
use std::str::FromStr;

use itertools::Itertools;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

/// At the top level, every Snailfish Number is a pair.
///
/// The left and right pairts of a pair are either pairs or
/// regular numbers.
#[derive(Clone, PartialEq)]
enum SnailfishDetails {
    Pair(SnailfishNumber, SnailfishNumber),
    Regular(u8),
}

use SnailfishDetails::{Pair, Regular};

#[derive(Clone, PartialEq)]
struct SnailfishNumber {
    details: Rc<SnailfishDetails>,
}

impl SnailfishNumber {
    fn regular(n: u8) -> SnailfishNumber {
        SnailfishNumber {
            details: Rc::new(Regular(n)),
        }
    }

    fn pair(left: &SnailfishNumber, right: &SnailfishNumber) -> SnailfishNumber {
        SnailfishNumber {
            details: Rc::new(Pair(left.clone(), right.clone())),
        }
    }

    fn details(&self) -> &SnailfishDetails {
        &*self.details
    }

    /// Parsing from an iterable over the input characters.
    ///
    /// For all reduced numbers, we could parse without peeking ahead
    /// because all of the numbers are single digits.  For tests, though,
    /// we want to be able to parse non-reduced numbers, so we need to
    /// be able to peek ahead and see if there's more of the number.
    fn parse<I>(iter: &mut iter::Peekable<I>) -> SnailfishNumber
    where
        I: Iterator<Item = char>,
    {
        let c: char = iter.next().unwrap();
        if c.is_digit(10) {
            let mut n = c.to_digit(10).unwrap() as u8;
            loop {
                if let Some(c) = iter.peek() {
                    if let Some(next_n) = c.to_digit(10) {
                        iter.next();
                        n = n * 10 + (next_n as u8);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            SnailfishNumber::regular(n)
        } else if c == '[' {
            let left = SnailfishNumber::parse(iter);
            if iter.next().unwrap() != ',' {
                panic!("expected comma");
            }
            let right = SnailfishNumber::parse(iter);
            if iter.next().unwrap() != ']' {
                panic!("expected comma");
            }
            SnailfishNumber::pair(&left, &right)
        } else {
            panic!("bad number: {:?}", c);
        }
    }
}

impl FromStr for SnailfishNumber {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<SnailfishNumber, AdventError> {
        let mut iter = s.chars().peekable();
        let result = SnailfishNumber::parse(&mut iter);
        Ok(result)
    }
}

#[test]
fn test_from_str() {
    assert_eq!(
        SnailfishNumber::regular(8),
        SnailfishNumber::from_str("8").unwrap()
    );
    assert_eq!(
        SnailfishNumber::regular(12),
        SnailfishNumber::from_str("12").unwrap()
    );
    assert_eq!(
        SnailfishNumber::pair(
            &SnailfishNumber::regular(1),
            &SnailfishNumber::pair(&SnailfishNumber::regular(2), &SnailfishNumber::regular(10))
        ),
        SnailfishNumber::from_str("[1,[2,10]]").unwrap()
    );
    // Check that equality goes inside the Rc
    assert_ne!(
        SnailfishNumber::pair(
            &SnailfishNumber::regular(1),
            &SnailfishNumber::pair(&SnailfishNumber::regular(2), &SnailfishNumber::regular(9))
        ),
        SnailfishNumber::from_str("[1,[2,10]]").unwrap()
    );
}

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.details {
            Pair(left, right) => {
                write!(f, "[{:?},{:?}]", left, right)
            }
            Regular(n) => write!(f, "{:?}", n),
        }
    }
}

fn get_regular(number: &SnailfishNumber) -> u8 {
    match number.details() {
        Regular(n) => *n,
        _ => panic!("expected regular"),
    }
}

fn add_to_leftmost(number: &SnailfishNumber, delta: u8) -> SnailfishNumber {
    if delta == 0 {
        number.clone()
    } else {
        match number.details() {
            Regular(n) => SnailfishNumber::regular(*n + delta),
            Pair(left, right) => SnailfishNumber::pair(&add_to_leftmost(left, delta), right),
        }
    }
}

#[test]
fn test_add_to_leftmost() {
    assert_eq! {
        SnailfishNumber::from_str("[[3,4],8]").unwrap(),
        add_to_leftmost(&SnailfishNumber::from_str("[[1,4],8]").unwrap(), 2)
    }
}

fn add_to_rightmost(number: &SnailfishNumber, delta: u8) -> SnailfishNumber {
    if delta == 0 {
        number.clone()
    } else {
        match number.details() {
            Regular(n) => SnailfishNumber::regular(*n + delta),
            Pair(left, right) => SnailfishNumber::pair(left, &add_to_rightmost(right, delta)),
        }
    }
}

#[test]
fn test_add_to_rightmost() {
    assert_eq! {
        SnailfishNumber::from_str("[[1,4],10]").unwrap(),
        add_to_rightmost(&SnailfishNumber::from_str("[[1,4],8]").unwrap(), 2)
    }
}

/// Walks down a given depth from the current number and explodes there.
///
/// Caller must ensure that there are no pairs at (depth + 1).
///
/// Returns the None of nothing to explode was found.  
/// Returns Some((add_left, new_number, add_right)) if a number to explode was
/// found.
///
fn explode(number: &SnailfishNumber, depth: usize) -> Option<(u8, SnailfishNumber, u8)> {
    match number.details() {
        Regular(_) => None,
        Pair(left, right) => {
            if depth == 0 {
                // We're going to explode this one.
                // Anything below this level should be a Regular number.
                let n_left = get_regular(left);
                let n_right = get_regular(right);
                Some((n_left, SnailfishNumber::regular(0), n_right))
            } else {
                if let Some((add_left, new_left, add_right)) = explode(left, depth - 1) {
                    let new_number =
                        SnailfishNumber::pair(&new_left, &add_to_leftmost(right, add_right));
                    Some((add_left, new_number, 0))
                } else if let Some((add_left, new_right, add_right)) = explode(right, depth - 1) {
                    let new_number =
                        SnailfishNumber::pair(&add_to_rightmost(left, add_left), &new_right);
                    Some((0, new_number, add_right))
                } else {
                    None
                }
            }
        }
    }
}

/// Replaces the first number bigger than 9 by splitting it.
fn split(number: &SnailfishNumber) -> Option<SnailfishNumber> {
    match number.details() {
        Regular(n) => {
            if 9 < *n {
                Some(SnailfishNumber::pair(
                    &SnailfishNumber::regular((*n) / 2),
                    &SnailfishNumber::regular((*n + 1) / 2),
                ))
            } else {
                None
            }
        }
        Pair(left, right) => {
            if let Some(new_left) = split(left) {
                Some(SnailfishNumber::pair(&new_left, right))
            } else if let Some(new_right) = split(right) {
                Some(SnailfishNumber::pair(left, &new_right))
            } else {
                None
            }
        }
    }
}

fn one_reduce(number: &SnailfishNumber) -> Option<SnailfishNumber> {
    if let Some((_, new_number, _)) = explode(number, 4) {
        Some(new_number)
    } else if let Some(new_number) = split(number) {
        Some(new_number)
    } else {
        None
    }
}

#[test]
fn test_one_reduce() {
    fn check_one_reduce(expected: &str, initial: &str) {
        assert_eq!(
            SnailfishNumber::from_str(expected).unwrap(),
            one_reduce(&SnailfishNumber::from_str(initial).unwrap()).unwrap()
        )
    }
    check_one_reduce("[[[[0,9],2],3],4]", "[[[[[9,8],1],2],3],4]");
    check_one_reduce("[7,[6,[5,[7,0]]]]", "[7,[6,[5,[4,[3,2]]]]]");
    check_one_reduce("[[6,[5,[7,0]]],3]", "[[6,[5,[4,[3,2]]]],1]");
    check_one_reduce(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    );
    check_one_reduce(
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );

    let sequence = [
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    ];
    for (initial, expected) in sequence.iter().tuple_windows() {
        check_one_reduce(expected, initial);
    }
}

fn reduce(number: &SnailfishNumber) -> SnailfishNumber {
    let mut result = number.clone();
    while let Some(reduced) = one_reduce(&result) {
        result = reduced;
    }
    result
}

#[test]
fn test_reduce() {
    assert_eq!(
        SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap(),
        reduce(&SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap())
    )
}

/// Computes the magnitude of a SnailfishNumber
fn magnitude(number: &SnailfishNumber) -> Answer {
    match number.details() {
        Regular(n) => *n as Answer,
        Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

#[test]
fn test_magnitude() {
    assert_eq!(
        143,
        magnitude(&SnailfishNumber::from_str("[[1,2],[[3,4],5]]").unwrap())
    );
    assert_eq!(
        3488,
        magnitude(
            &SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .unwrap()
        )
    );
}

/// Adds two SnailfishNumbers
fn add(a: &SnailfishNumber, b: &SnailfishNumber) -> SnailfishNumber {
    reduce(&SnailfishNumber::pair(a, b))
}

#[test]
fn test_add() {
    assert_eq!(
        SnailfishNumber::from_str("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
            .unwrap(),
        add(
            &SnailfishNumber::from_str(
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
            )
            .unwrap(),
            &SnailfishNumber::from_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")
                .unwrap()
        )
    )
}

fn day_18_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let sum = lines
        .iter()
        .map(|line| SnailfishNumber::from_str(line).unwrap())
        .reduce(|a, b| add(&a, &b))
        .unwrap();

    Ok(magnitude(&sum))
}

fn day_18_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_18() -> Day {
    Day::new(
        18,
        DayPart::new(day_18_a, 4140, 3494),
        DayPart::new(day_18_b, 0, 0),
    )
}
