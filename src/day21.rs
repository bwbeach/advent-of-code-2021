use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_21_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_21_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_21() -> Day {
    Day::new(
        21,
        DayPart::new(day_21_a, 0, 0),
        DayPart::new(day_21_b, 0, 0),
    )
}
