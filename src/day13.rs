use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_13_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_13_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_13() -> Day {
    Day::new(
        13,
        DayPart::new(day_13_a, 0, 0),
        DayPart::new(day_13_b, 0, 0),
    )
}
