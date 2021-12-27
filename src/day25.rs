use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_25_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_25_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_25() -> Day {
    Day::new(
        25,
        DayPart::new(day_25_a, 0, 0),
        DayPart::new(day_25_b, 0, 0),
    )
}
