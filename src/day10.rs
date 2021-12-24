use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_10_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_10_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_10() -> Day {
    Day::new(
        10,
        DayPart::new(day_10_a, 0, 0),
        DayPart::new(day_10_b, 0, 0),
    )
}
