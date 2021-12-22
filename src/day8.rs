use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_8_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_8_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_8() -> Day {
    Day::new(8, DayPart::new(day_8_a, 0, 0), DayPart::new(day_8_b, 0, 0))
}
