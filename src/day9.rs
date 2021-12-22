use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_9_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_9_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_9() -> Day {
    Day::new(9, DayPart::new(day_9_a, 0, 0), DayPart::new(day_9_b, 0, 0))
}
