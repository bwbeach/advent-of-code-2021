use crate::grid::{parse_grid, Grid, Point};
use crate::types::{AdventResult, Answer, Day, DayPart};

/// Updates a grid by incrementing the value at one spot,
/// and doing the same to neighbors if this one incremented
/// to 9.  Returns the number of flashes, including ones
/// we trigger in neighbors.
fn increment_one(grid: &mut Grid, pos: Point) -> Answer {
    let new_value = grid.get(pos) + 1;
    grid.set(pos, new_value);
    let mut flash_count = 0;
    if new_value == 10 {
        flash_count += 1;
        for neighbor in grid.neigbors_with_diagonals(pos) {
            flash_count += increment_one(grid, neighbor);
        }
    }
    flash_count
}

#[test]
fn test_increment_one() {
    let mut grid = parse_grid(&vec![
        "111".to_string(),
        "199".to_string(),
        "191".to_string(),
        "111".to_string(),
    ]);
    assert_eq!(3, increment_one(&mut grid, (1, 1)));
    let mut expected = parse_grid(&vec![
        "233".to_string(),
        "300".to_string(),
        "304".to_string(),
        "222".to_string(),
    ]);
    expected.set((1, 1), 12);
    expected.set((2, 1), 11);
    expected.set((1, 2), 11);
    assert_eq!(expected, grid);
}

/// Takes the entire grid to the next step, returning
/// the number of flashes that happened.
fn one_step(grid: &mut Grid) -> Answer {
    let (width, height) = grid.shape();
    let mut flash_count = 0;
    for x in 0..width {
        for y in 0..height {
            flash_count += increment_one(grid, (x, y));
        }
    }
    for x in 0..width {
        for y in 0..height {
            if 10 <= grid.get((x, y)) {
                grid.set((x, y), 0);
            }
        }
    }
    flash_count
}

#[test]
fn test_one_step() {
    let mut grid = parse_grid(&vec![
        "11111".to_string(),
        "19991".to_string(),
        "19191".to_string(),
        "19991".to_string(),
        "11111".to_string(),
    ]);
    assert_eq!(9, one_step(&mut grid));
    let expected = parse_grid(&vec![
        "34543".to_string(),
        "40004".to_string(),
        "50005".to_string(),
        "40004".to_string(),
        "34543".to_string(),
    ]);
    assert_eq!(expected, grid);
}

fn day_11_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut grid = parse_grid(lines);
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += one_step(&mut grid);
    }
    Ok(flash_count)
}

fn day_11_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut grid = parse_grid(lines);
    let (width, height) = grid.shape();
    let octopus_count = (width * height) as Answer;
    let mut step_count = 0;
    loop {
        step_count += 1;
        if one_step(&mut grid) == octopus_count {
            return Ok(step_count);
        }
    }
}

pub fn make_day_11() -> Day {
    Day::new(
        11,
        DayPart::new(day_11_a, 1656, 1617),
        DayPart::new(day_11_b, 195, 258),
    )
}
