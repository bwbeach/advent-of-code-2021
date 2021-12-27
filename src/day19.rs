use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_19_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_19_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_19() -> Day {
    Day::new(
        19,
        DayPart::new(day_19_a, 0, 0),
        DayPart::new(day_19_b, 0, 0),
    )
}
