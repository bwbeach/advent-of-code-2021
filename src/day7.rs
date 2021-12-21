use num::abs;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Finds the position with the least total cost for moving crabs.
///
/// The best place is the median.  If there's a tie because of an even number
/// of inputs, either answer (or anything in between them) will have the
/// same total cost.  (Proof left to reader. :-) )
///
fn day_7_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut positions: Vec<i32> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    positions.sort();
    let median = positions[positions.len() / 2];
    let total_cost: i32 = positions.iter().map(|&p| abs(p - median)).sum();
    Ok(total_cost as Answer)
}

fn day_7_b(lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_7() -> Day {
    Day::new(
        7,
        DayPart::new(day_7_a, 37, 353800),
        DayPart::new(day_7_b, 0, 0),
    )
}
