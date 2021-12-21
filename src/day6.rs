use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_6_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_6_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_6() -> Day {
    Day::new(6, DayPart::new(day_6_a, 0, 0), DayPart::new(day_6_b, 0, 0))
}
