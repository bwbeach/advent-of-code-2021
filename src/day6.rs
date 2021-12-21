use crate::types::{AdventResult, Answer, Day, DayPart};

type Counter = u8;

fn parse_input(lines: &Vec<String>) -> Vec<Counter> {
    if lines.len() != 1 {
        panic!("expected exactly one input line");
    }
    lines[0].split(",").map(|s| s.parse().unwrap()).collect()
}
fn day_6_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut state = parse_input(lines);
    for _ in 0..80 {
        let initial_len = state.len();
        for i in 0..initial_len {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }
    }
    Ok(state.len() as u64)
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
