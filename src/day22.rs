use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_22_a(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

fn day_22_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_22() -> Day {
    Day::new(
        22,
        DayPart::new(day_22_a, 0, 0),
        DayPart::new(day_22_b, 0, 0),
    )
}
