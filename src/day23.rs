use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_23_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_23_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_23() -> Day {
    Day::new(
        23,
        DayPart::new(day_23_a, 0, 0),
        DayPart::new(day_23_b, 0, 0),
    )
}
