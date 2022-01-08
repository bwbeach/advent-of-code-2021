use std::collections::HashSet;

use itertools::all;
use ndarray::{s, Array2}; // TODO: fix unused warning, and keep available for tests

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A number on a Day 4 bingo card
type BingoCardNumber = u8;

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
        grid: ndarray::arr2(&[[1, 2], [3, 4]]),
    };
    assert_eq!(false, card.is_bingo(&make_set(&[])));
    assert_eq!(false, card.is_bingo(&make_set(&[1])));
    assert_eq!(false, card.is_bingo(&make_set(&[1, 4])));
    assert_eq!(true, card.is_bingo(&make_set(&[1, 2])));
    assert_eq!(true, card.is_bingo(&make_set(&[1, 3])));
}

fn parse_bingo_card(lines: &[&str]) -> BingoCard {
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
            grid: ndarray::arr2(&[[1, 2], [3, 4]])
        },
        parse_bingo_card(&["1 2", " 3  4 "])
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

fn parse_day_4_input(lines: &[&str]) -> Day4Input {
    let called: Vec<BingoCardNumber> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let cards: Vec<BingoCard> = lines[1..]
        .split(|line| *line == "")
        .filter(|lines| lines.len() != 0)
        .map(|g| parse_bingo_card(g))
        .collect();
    Day4Input { called, cards }
}

#[test]
fn test_parse_day_4_input() {
    assert_eq!(
        Day4Input {
            called: vec![13, 15],
            cards: vec![
                BingoCard {
                    grid: ndarray::arr2(&[[1, 2], [3, 4]])
                },
                BingoCard {
                    grid: ndarray::arr2(&[[5, 6], [7, 8]])
                }
            ]
        },
        parse_day_4_input(&["13,15", "", "1 2", "3 4", "", "5 6", "7 8"])
    )
}

fn day_4_a(lines: &[&str]) -> AdventResult<Answer> {
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

fn day_4_b(lines: &[&str]) -> AdventResult<Answer> {
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

pub fn make_day_4() -> Day {
    Day::new(
        4,
        DayPart::new(day_4_a, 4512, 58374),
        DayPart::new(day_4_b, 1924, 11377),
    )
}
