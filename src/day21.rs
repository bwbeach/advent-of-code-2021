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
}

// Represents the current state of one player
#[derive(Debug)]
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

    fn one_move(&mut self, die: &mut Die) -> usize {
        let total_roll = die.roll() + die.roll() + die.roll();
        self.position = wrap(self.position + total_roll, 1, 10);
        self.score += self.position;
        self.score
    }
}

/// The current state is the game.
#[derive(Debug)]
struct State {
    // the players
    players: Vec<Player>,

    // the index of the player to play next
    next: usize,

    // the die
    die: Die,
}

impl State {
    // One player moves.  Returns the new score of that player
    fn one_move(&mut self) -> usize {
        let player_count = self.players.len();
        let player = &mut self.players[self.next];
        self.next = (self.next + 1) % player_count;
        player.one_move(&mut self.die)
    }

    // The score that the loser had after somebody won.
    fn loser_score(&self) -> usize {
        self.players.iter().map(|p| p.score).min().unwrap()
    }
}

fn parse_input(lines: &[&str]) -> State {
    let mut iter = lines.iter();
    let players = lines
        .iter()
        .map(|&line| Player::from_input_line(line))
        .collect();
    State {
        players,
        next: 0,
        die: Die::new(),
    }
}

fn day_21_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut state = parse_input(lines);
    while state.one_move() < 1000 {}
    Ok((state.loser_score() * state.die.count) as Answer)
}

fn day_21_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_21() -> Day {
    Day::new(
        21,
        DayPart::new(day_21_a, 739785, 805932),
        DayPart::new(day_21_b, 0, 0),
    )
}
