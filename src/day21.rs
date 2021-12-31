use std::cmp::max;
use std::collections::HashMap;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Treats a range as a wraparound, and wraps
/// to get the given number into the range.
fn wrap(n: usize, min: usize, max_inclusive: usize) -> usize {
    min + ((n - min) % (max_inclusive + 1 - min))
}

#[test]
fn test_wrap() {
    assert_eq!(1, wrap(1, 1, 10));
    assert_eq!(10, wrap(10, 1, 10));
    assert_eq!(1, wrap(11, 1, 10));
    assert_eq!(2, wrap(22, 1, 10));
}

/// A 100-sided deterministic die
#[derive(Debug)]
struct Die {
    // the next number to roll
    next: usize,

    // the number of times die has been rolled
    count: usize,
}

impl Die {
    fn new() -> Die {
        Die { next: 1, count: 0 }
    }

    fn roll(&mut self) -> usize {
        let result = self.next;
        self.next = wrap(self.next + 1, 1, 100);
        self.count += 1;
        result
    }

    fn roll3(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }
}

// Represents the current state of one player
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Player {
    // position is always in the range 1..=10
    position: usize,

    // total score so far
    score: usize,
}

impl Player {
    fn start(initial_position: usize) -> Player {
        Player {
            position: initial_position,
            score: 0,
        }
    }

    fn from_input_line(line: &str) -> Player {
        let pos = line.split_whitespace().last().unwrap().parse().unwrap();
        Player::start(pos)
    }

    fn one_move(&self, roll: usize) -> Player {
        let new_position = wrap(self.position + roll, 1, 10);
        Player {
            position: new_position,
            score: self.score + new_position,
        }
    }
}

/// The current state is the game.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    // the players
    players: [Player; 2],

    // the index of the player to play next: 0 or 1
    next: usize,
}

impl State {
    // One player moves.  Returns the new score of that player
    fn one_move(&self, roll: usize) -> State {
        if self.next == 0 {
            State {
                players: [self.players[0].one_move(roll), self.players[1].clone()],
                next: 1,
            }
        } else {
            State {
                players: [self.players[0].clone(), self.players[1].one_move(roll)],
                next: 0,
            }
        }
    }

    // Is the game over?
    fn game_over(&self, winning_score: usize) -> bool {
        winning_score <= self.players[0].score || winning_score <= self.players[1].score
    }

    // The score that the loser had after somebody won.
    fn loser_score(&self) -> usize {
        self.players.iter().map(|p| p.score).min().unwrap()
    }
}

fn parse_input(lines: &[&str]) -> State {
    let mut iter = lines.iter();
    let players = [
        Player::from_input_line(iter.next().unwrap()),
        Player::from_input_line(iter.next().unwrap()),
    ];
    State { players, next: 0 }
}

fn day_21_a(lines: &[&str]) -> AdventResult<Answer> {
    let winning_score = 1000;
    let mut state = parse_input(lines);
    let mut die = Die::new();
    while !state.game_over(winning_score) {
        state = state.one_move(die.roll3());
    }
    Ok((state.loser_score() * die.count) as Answer)
}

fn day_21_b(lines: &[&str]) -> AdventResult<Answer> {
    let winning_score = 21;
    // the number of ways each total can come up using three three-side dice
    let roll_and_count: [(usize, usize); 7] =
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    // Mapping from state to the number of universes that have that state
    // We start off with a single universe holding the initial state.
    let mut state_to_universes: HashMap<State, usize> = HashMap::new();
    state_to_universes.insert(parse_input(lines), 1);

    // Run until all universes have complete games.
    loop {
        let mut new_state_to_universes = HashMap::new();
        let mut all_complete = true;
        for (state, universe_count) in state_to_universes {
            if !state.game_over(winning_score) {
                for (roll, count) in roll_and_count {
                    let new_state = state.one_move(roll);
                    if !new_state.game_over(winning_score) {
                        all_complete = false;
                    }
                    (*new_state_to_universes.entry(new_state).or_insert(0)) +=
                        universe_count * count;
                }
            } else {
                (*new_state_to_universes.entry(state).or_insert(0)) += universe_count;
            }
        }
        state_to_universes = new_state_to_universes;
        if all_complete {
            break;
        }
    }

    // Count the wins for each player
    let mut player_1_wins = 0;
    let mut player_2_wins = 0;
    for (state, universe_count) in state_to_universes {
        if winning_score <= state.players[0].score {
            player_1_wins += universe_count;
        } else {
            player_2_wins += universe_count;
        }
    }
    println!("win counts: {:?} {:?}", player_1_wins, player_2_wins);

    Ok(max(player_1_wins, player_2_wins) as Answer)
}

pub fn make_day_21() -> Day {
    Day::new(
        21,
        DayPart::new(day_21_a, 739785, 805932),
        DayPart::new(day_21_b, 444356092776315, 133029050096658),
    )
}
