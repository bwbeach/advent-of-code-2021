use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_18_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_18_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_18() -> Day {
    Day::new(
        18,
        DayPart::new(day_18_a, 0, 0),
        DayPart::new(day_18_b, 0, 0),
    )
}
