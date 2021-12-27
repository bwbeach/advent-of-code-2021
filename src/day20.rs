use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_20_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_20_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_20() -> Day {
    Day::new(
        20,
        DayPart::new(day_20_a, 0, 0),
        DayPart::new(day_20_b, 0, 0),
    )
}
