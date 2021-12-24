use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_11_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_11_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_11() -> Day {
    Day::new(
        11,
        DayPart::new(day_11_a, 0, 0),
        DayPart::new(day_11_b, 0, 0),
    )
}
