use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_17_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_17_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_17() -> Day {
    Day::new(
        17,
        DayPart::new(day_17_a, 0, 0),
        DayPart::new(day_17_b, 0, 0),
    )
}
