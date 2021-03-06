use std::collections::{HashMap, HashSet};

use crate::grid::{parse_grid, Grid, Point};
use crate::types::{AdventResult, Answer, Day, DayPart};

fn lowest_cost(cost_to_enter: &Grid) -> AdventResult<Answer> {
    // The input grid is the cost to enter each cell
    let (width, height) = cost_to_enter.shape();
    let bottom_right = (width - 1, height - 1);

    // This map keeps the total cost from a given cell to get
    // to the bottom right.  At the beginning, we just know that
    // the cost to get from the bottom right is 0, because
    // you're already there.
    let mut done: HashMap<Point, usize> = HashMap::new();
    done.insert(bottom_right, 0);

    // This set keeps track of the cells that are adjacent to
    // cells with known cost, which are the ones we'll know the
    // answers for next.  At the start, the cells next to the
    // bottom right are the ones that need answers.
    let mut to_do: HashSet<Point> = HashSet::new();
    for n in cost_to_enter.neigbors(bottom_right) {
        to_do.insert(n);
    }

    // Expand the are of known answers by checking whether any
    // of the cells to do next has the target score.  We keep
    // raising the target score until the entire grid has answers.
    // NOTE: This assumes that no cell has a score of 0.
    let mut target_score = 1;
    while !to_do.is_empty() {
        let mut new_to_do = HashSet::new();
        for cell in to_do {
            let this_one_matches = cost_to_enter.neigbors(cell).any(|n| {
                if let Some(cost_from_there) = done.get(&n) {
                    let cost_to_enter_there = cost_to_enter.get(n);
                    (cost_to_enter_there as usize) + cost_from_there == target_score
                } else {
                    false
                }
            });
            if this_one_matches {
                // This cell has the score we're looking for.  It's done.
                done.insert(cell, target_score);
                // If one of its neighbors already put this cell in the
                // new to do list, we can take it out.
                new_to_do.remove(&cell);
                // All of its neighbors that are NOT done are now candidates
                // for the next target score.
                for n in cost_to_enter.neigbors(cell) {
                    if !done.contains_key(&n) {
                        new_to_do.insert(n);
                    }
                }
            } else {
                // We need to try this cell again next time.
                new_to_do.insert(cell);
            }
        }
        to_do = new_to_do;
        target_score += 1;
    }
    Ok(*done.get(&(0, 0)).unwrap() as Answer)
}

fn day_15_a(lines: &[&str]) -> AdventResult<Answer> {
    // The input grid is the cost to enter each cell
    let cost_to_enter = parse_grid(lines);
    lowest_cost(&cost_to_enter)
}

fn day_15_b(lines: &[&str]) -> AdventResult<Answer> {
    // The input grid is the cost to enter each cell
    let original = parse_grid(lines);
    let original_shape = original.shape();
    let original_width = original_shape.0;
    let original_height = original_shape.1;

    // When adding one to get an expanded value, the numbers wrap from 9 back to 1
    fn next_value(v: u8) -> u8 {
        if v == 9 {
            1
        } else {
            v + 1
        }
    }

    // Expand by a factor of 5 in each direction
    let width = original_width * 5;
    let height = original_height * 5;
    let shape = (width, height);
    let mut expanded = Grid::zeros(shape);
    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            if original_height <= y {
                expanded.set(pos, next_value(expanded.get((x, y - original_height))));
            } else if original_width <= x {
                expanded.set(pos, next_value(expanded.get((x - original_width, y))));
            } else {
                expanded.set(pos, original.get((x, y)));
            }
        }
    }
    lowest_cost(&expanded)
}

pub fn make_day_15() -> Day {
    Day::new(
        15,
        DayPart::new(day_15_a, 40, 589),
        DayPart::new(day_15_b, 315, 2885),
    )
}
