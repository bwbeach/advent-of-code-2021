use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_15_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_15_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_15() -> Day {
    Day::new(
        15,
        DayPart::new(day_15_a, 0, 0),
        DayPart::new(day_15_b, 0, 0),
    )
}
