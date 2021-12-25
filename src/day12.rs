use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_12_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_12_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_12() -> Day {
    Day::new(
        12,
        DayPart::new(day_12_a, 0, 0),
        DayPart::new(day_12_b, 0, 0),
    )
}
