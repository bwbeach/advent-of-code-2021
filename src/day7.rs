use math::round::half_up;
use num::abs;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Finds the position with the least total cost for moving crabs.
///
/// The cost function is the sum of the distances that individual
/// crabs move.
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

/// Cost function for part B for one crab.
///
fn part_b_one_cost(chosen: i32, pos: i32) -> i32 {
    let d = abs(chosen - pos);
    (d + d * d) / 2
}

/// Cost of moving all the crabs in part B.
fn part_b_total_cost(positions: &Vec<i32>, chosen: i32) -> i32 {
    positions.iter().map(|&p| part_b_one_cost(chosen, p)).sum()
}
/// Same as day_7_a, but the cost function is (n + n^2) / 2
///
/// I don't know how to derive the answer mathematically, so we'll
/// just try all the possibilities until we find the answer.
///
fn day_7_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let positions: Vec<i32> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let min: i32 = *(positions.iter().min().unwrap());
    let max: i32 = *(positions.iter().max().unwrap());
    let mut prev_cost = part_b_total_cost(&positions, min);
    // println!("{:?} - {:?}", min, prev_cost);
    for p in (min + 1)..max {
        let this_cost = part_b_total_cost(&positions, p);
        // println!("{:?} - {:?}", p, this_cost);
        if this_cost <= prev_cost {
            prev_cost = this_cost;
        } else {
            return Ok(prev_cost as Answer);
        }
    }
    panic!("cost did not go back up");
}

pub fn make_day_7() -> Day {
    Day::new(
        7,
        DayPart::new(day_7_a, 37, 353800),
        DayPart::new(day_7_b, 168, 0),
    )
}
