use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_24_a(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

fn day_24_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_24() -> Day {
    Day::new(
        24,
        DayPart::new(day_24_a, 0, 0),
        DayPart::new(day_24_b, 0, 0),
    )
}
