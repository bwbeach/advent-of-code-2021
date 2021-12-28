use std::fmt;
use std::iter;
use std::rc::Rc;
use std::str::FromStr;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

/// At the top level, every Snailfish Number is a pair.
///
/// The left and right pairts of a pair are either pairs or
/// regular numbers.
#[derive(Clone, PartialEq)]
enum SnailfishNumber {
    Pair(SnailfishRc, SnailfishRc),
    Regular(u8),
}

use SnailfishNumber::{Pair, Regular};

#[derive(Clone, PartialEq)]
struct SnailfishRc {
    details: Rc<SnailfishNumber>,
}

impl SnailfishRc {
    fn regular(n: u8) -> SnailfishRc {
        SnailfishRc {
            details: Rc::new(Regular(n)),
        }
    }

    fn pair(left: &SnailfishRc, right: &SnailfishRc) -> SnailfishRc {
        SnailfishRc {
            details: Rc::new(Pair(left.clone(), right.clone())),
        }
    }
    /// Parsing from an iterable over the input characters.
    ///
    /// For all reduced numbers, we could parse without peeking ahead
    /// because all of the numbers are single digits.  For tests, though,
    /// we want to be able to parse non-reduced numbers, so we need to
    /// be able to peek ahead and see if there's more of the number.
    fn parse<I>(iter: &mut iter::Peekable<I>) -> SnailfishRc
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

            SnailfishRc::regular(n)
        } else if c == '[' {
            let left = SnailfishRc::parse(iter);
            if iter.next().unwrap() != ',' {
                panic!("expected comma");
            }
            let right = SnailfishRc::parse(iter);
            SnailfishRc::pair(&left, &right)
        } else {
            panic!("bad number: {:?}", c);
        }
    }
}

impl FromStr for SnailfishRc {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<SnailfishRc, AdventError> {
        let mut iter = s.chars().peekable();
        let result = SnailfishRc::parse(&mut iter);
        Ok(result)
    }
}

#[test]
fn test_from_str() {
    assert_eq!(SnailfishRc::regular(8), SnailfishRc::from_str("8").unwrap());
    assert_eq!(
        SnailfishRc::regular(12),
        SnailfishRc::from_str("12").unwrap()
    );
    assert_eq!(
        SnailfishRc::pair(
            &SnailfishRc::regular(1),
            &SnailfishRc::pair(&SnailfishRc::regular(2), &SnailfishRc::regular(10))
        ),
        SnailfishRc::from_str("[1,[2,10]]").unwrap()
    );
    // Check that equality goes inside the Rc
    assert_ne!(
        SnailfishRc::pair(
            &SnailfishRc::regular(1),
            &SnailfishRc::pair(&SnailfishRc::regular(2), &SnailfishRc::regular(9))
        ),
        SnailfishRc::from_str("[1,[2,10]]").unwrap()
    );
}

impl fmt::Debug for SnailfishRc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.details {
            Pair(left, right) => {
                write!(f, "[{:?},{:?}]", left, right)
            }
            Regular(n) => write!(f, "{:?}", n),
        }
    }
}

// enum PathElem {
//     Left,
//     Right,
// }

// enum Path {
//     Null,
//     Cons(PathElem, Box<Path>),
// }

// fn first_pair_at_depth(number: &SnailfishNumber, depth: usize) -> Option<Path> {
//     match number {
//         Regular(_) => None,
//         Pair(pair) => {
//             if depth == 0 {
//                 Some(Path::Null)
//             } else {
//                 let (left, right) = &**pair;
//                 if let Some(left_path) = first_pair_at_depth(left, depth - 1) {
//                     Some(Path::Cons(PathElem::Left, Box::new(left_path)))
//                 } else if let Some(right_path) = first_pair_at_depth(right, depth - 1) {
//                     Some(Path::Cons(PathElem::Left, Box::new(right_path)))
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }

// fn one_left(path: &Path) -> Option<Path> {
//     match path {
//         Path::Null => None,
//         Path::Cons(first, rest) => {
//             if let Some(rest_path) = one_left(&*rest) {
//                 Some(Path::Cons(first, Box::new(rest_path)))
//             } else {
//                 None
//             }
//         }
//     }
// }

// fn get_regular(number: &SnailfishNumber) -> u8 {
//     match number {
//         Regular(n) => *n,
//         _ => panic!("expected regular"),
//     }
// }

// fn add_to_leftmost(number: &SnailfishNumber, delta: u8) -> SnailfishNumber {
//     match number {
//         Regular(n) => Regular(*n + 1),
//         Pair(pair) => {
//             let (left, right) = &**pair;
//             Pair(Box::new((add_to_leftmost(left, delta), right.clone())))
//         }
//     }
// }

/// Walks down a given depth from the current number and explodes there.
///
/// Caller must ensure that there are no pairs at (depth + 1).
///
/// Returns the None of nothing to explode was found.  
/// Returns Some((add_left, new_number, add_right)) if a number to explode was
/// found.
///
// fn explode_helper(number: &SnailfishNumber, depth: usize) -> Option<(u8, SnailfishNumber, u8)> {
//     match number {
//         Regular(_) => None,
//         Pair(left, right) => {
//             if depth == 0 {
//                 // We're going to explode this one.
//                 // Anything below this level should be a Regular number.
//                 let n_left = get_regular(left);
//                 let n_right = get_regular(right);
//                 Some((n_left, Regular(0), n_right))
//             } else {
//                 if let Some((add_left, new_left, add_right)) = explode_helper(left, depth - 1) {
//                     let new_number = Pair(new_left, add_to_leftmost(right, add_right))));
//                     Some((add_left, new_number, new_number))
//                 } else if let Some((add_left, new_right, add_right)) =
//                     explode_helper(right, depth - 1)
//                 {
//                     let new_number = Pair(add_to_rightmost())
//                     if let Some(new_left) = add_to_rightmost(left) {
//                         Some((0, new_right, add_right))
//                     } else {
//                         Some((add_left, new_right, add_right))
//                     }
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }

// fn reduce(number: &SnailfishNumber) -> SnailfishNumber {}

// #[test]
// fn test_reduce() {
//     assert_eq!(
//         SnailfishNumber::from_str("[[[[0,9],2],3],4]").unwrap(),
//         reduce(&SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]").unwrap())
//     )
// }

fn day_18_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_18_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_18() -> Day {
    Day::new(
        18,
        DayPart::new(day_18_a, 0, 0),
        DayPart::new(day_18_b, 0, 0),
    )
}
