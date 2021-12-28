use std::collections::HashMap;

use crate::grid::{parse_grid, Grid, Point};
use crate::types::{AdventResult, Answer, Day, DayPart};

fn is_low_spot(grid: &Grid, pos: Point) -> bool {
    // The value at the position in question
    let value = grid.get(pos);

    // Check each neighbor cell
    grid.neigbors(pos)
        .all(|neighbor| value < grid.get(neighbor))
}

#[test]
fn test_is_low_spot() {
    let grid = parse_grid(&["123", "303", "321"]);
    for x in 0..3 {
        for y in 0..3 {
            assert_eq!(x == y, is_low_spot(&grid, (x, y)));
        }
    }
}

fn day_9_a(lines: &[&str]) -> AdventResult<Answer> {
    let grid = parse_grid(lines);
    let (columns, rows) = grid.shape();
    let mut score = 0;
    for y in 0..rows {
        for x in 0..columns {
            if is_low_spot(&grid, (x, y)) {
                score += 1 + (grid.get((x, y)) as u64);
            }
        }
    }
    Ok(score)
}

/// Given a point, keeps going down to find the low point in
/// the basin, and return that.
fn find_basin(grid: &Grid, point: Point) -> Option<Point> {
    if grid.get(point) == 9 {
        return None;
    }
    let mut current = point;
    loop {
        let current_value = grid.get(current);
        // The problem doesn't explicitly say what to do if there
        // are multiple neighbors that are lower.  We'll just assume
        // that they all go to the same low point, and use the firt one.
        let lower: Option<Point> = grid
            .neigbors(current)
            .filter(|&p| grid.get(p) < current_value)
            .next();
        match lower {
            Some(p) => current = p,
            None => return Some(current),
        }
    }
}

#[test]
fn test_find_basin() {
    let grid = parse_grid(&vec!["123", "994", "129"]);
    println!("{:?}", grid);
    assert_eq!(Some((0, 0)), find_basin(&grid, (0, 0)));
    assert_eq!(Some((0, 0)), find_basin(&grid, (1, 0)));
    assert_eq!(Some((0, 0)), find_basin(&grid, (2, 0)));
    assert_eq!(Some((0, 0)), find_basin(&grid, (2, 1)));
    assert_eq!(Some((0, 2)), find_basin(&grid, (1, 2)));
    assert_eq!((None), find_basin(&grid, (1, 1)));
}

fn day_9_b(lines: &[&str]) -> AdventResult<Answer> {
    let grid = parse_grid(lines);
    let (width, height) = grid.shape();
    let mut basin_to_count: HashMap<Point, usize> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            if let Some(point) = find_basin(&grid, (x, y)) {
                let entry = basin_to_count.entry(point).or_insert(0);
                *entry += 1;
            }
        }
    }
    let mut counts: Vec<Answer> = basin_to_count.values().map(|&n| n as Answer).collect();
    counts.sort();
    Ok(counts.iter().rev().take(3).product())
}

pub fn make_day_9() -> Day {
    Day::new(
        9,
        DayPart::new(day_9_a, 15, 506),
        DayPart::new(day_9_b, 1134, 931200),
    )
}
