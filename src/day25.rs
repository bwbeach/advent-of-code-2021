use crate::types::{AdventResult, Answer, Day, DayPart};
use ndarray::{Array2, ArrayBase};

type Grid = Array2<u8>;
type Point = (usize, usize);
type Move = (Point, Point);

fn parse_input(lines: &[&str]) -> Grid {
    let width = lines[0].as_bytes().len();
    let height = lines.len();
    let mut grid: Grid = ArrayBase::zeros((width, height));
    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            grid[(x, y)] = *b;
        }
    }
    grid
}

fn print_grid(message: &str, grid: &Grid) {
    let width = grid.shape()[0];
    let height = grid.shape()[1];
    println!("{}", message);
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[(x, y)] as char);
        }
        println!("");
    }
    println!("");
}

fn apply_moves(move_type: u8, moves: &[Move], grid: &mut Grid) {
    for (src, dest) in moves {
        if grid[*src] != move_type {
            panic!(
                "source does not match {:?} for {:?}",
                move_type,
                (src, dest)
            );
        }
        if grid[*dest] != b'.' {
            panic!(
                "dest is not empty for {:?} it has {:?}",
                (src, dest),
                (grid[*dest] as char)
            );
        }
        grid[*src] = b'.';
        grid[*dest] = move_type;
    }
}

fn one_step(grid: &Grid) -> Grid {
    let width = grid.shape()[0];
    let height = grid.shape()[1];
    let mut result = grid.clone();

    // First, move east
    {
        let moves: Vec<Move> = result
            .indexed_iter()
            .filter_map(|((x, y), b)| {
                let east = ((x + 1) % width, y);
                if *b == b'>' && result[east] == b'.' {
                    Some(((x, y), east))
                } else {
                    None
                }
            })
            .collect();
        apply_moves(b'>', &moves, &mut result);
    }

    // Then, move south
    {
        let moves: Vec<Move> = result
            .indexed_iter()
            .filter_map(|((x, y), b)| {
                let south = (x, (y + 1) % height);
                if *b == b'v' && result[south] == b'.' {
                    Some(((x, y), south))
                } else {
                    None
                }
            })
            .collect();
        apply_moves(b'v', &moves, &mut result);
    }

    result
}

fn day_25_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut grid = parse_input(lines);
    for i in 0.. {
        let new_grid = one_step(&grid);
        if new_grid == grid {
            print_grid("answer", &new_grid);
            return Ok((i + 1) as Answer);
        }
        grid = new_grid;
    }
    panic!("EEK!")
}

fn day_25_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_25() -> Day {
    Day::new(
        25,
        DayPart::new(day_25_a, 58, 471),
        DayPart::new(day_25_b, 0, 0),
    )
}
