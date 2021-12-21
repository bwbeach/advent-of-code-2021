use crate::types::{AdventResult, Answer, Day, DayPart};

/// Counter for the number of fish at a given age (countdown number)
type FishCount = u64;

/// The state, holding the number of fish for each count-down value.
type State = [FishCount; 9];

fn parse_input(lines: &Vec<String>) -> State {
    if lines.len() != 1 {
        panic!("expected exactly one input line");
    }
    let counters: Vec<usize> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let mut state: State = [0; 9];
    for c in counters.iter() {
        state[*c] += 1;
    }
    state
}
fn day_6_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut state = parse_input(lines);
    for _ in 0..80 {
        let zeros = state[0];
        state[0] = state[1];
        state[1] = state[2];
        state[2] = state[3];
        state[3] = state[4];
        state[4] = state[5];
        state[5] = state[6];
        state[6] = state[7] + zeros;
        state[7] = state[8];
        state[8] = zeros;
    }
    let sum: u64 = state.iter().sum();
    Ok(sum)
}

fn day_6_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_6() -> Day {
    Day::new(
        6,
        DayPart::new(day_6_a, 5934, 350149),
        DayPart::new(day_6_b, 0, 0),
    )
}
