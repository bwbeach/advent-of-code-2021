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

    fn one_move(&mut self, roll: usize) -> usize {
        self.position = wrap(self.position + roll, 1, 10);
        self.score += self.position;
        self.score
    }
}

/// The current state is the game.
#[derive(Debug)]
struct State {
    // the players
    players: [Player; 2],

    // the index of the player to play next: 0 or 1
    next: usize,
}

impl State {
    // One player moves.  Returns the new score of that player
    fn one_move(&mut self, roll: usize) -> usize {
        let player = &mut self.players[self.next];
        self.next = 1 - self.next;
        player.one_move(roll)
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
    let mut state = parse_input(lines);
    let mut die = Die::new();
    while state.one_move(die.roll3()) < 1000 {}
    Ok((state.loser_score() * die.count) as Answer)
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
