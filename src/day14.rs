use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_14_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_14_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_14() -> Day {
    Day::new(
        14,
        DayPart::new(day_14_a, 0, 0),
        DayPart::new(day_14_b, 0, 0),
    )
}
