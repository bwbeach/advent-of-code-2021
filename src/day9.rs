use ndarray::{Array, ArrayBase, Dim, OwnedRepr, ShapeBuilder};
use std::fmt;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Represents the neighbors of a cell in a grid
struct Neighbors {
    // the size of the grid
    width: usize,
    height: usize,

    // the point whose neighbors we want
    x: usize,
    y: usize,

    // how many neighbors we've returned so far
    i: usize,
}

impl Iterator for Neighbors {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        loop {
            self.i += 1;
            match self.i {
                // the cell to the left
                1 => {
                    if self.x != 0 {
                        return Some((self.x - 1, self.y));
                    }
                }

                // the cell to the right
                2 => {
                    if self.x != self.width - 1 {
                        return Some((self.x + 1, self.y));
                    }
                }

                // the cell above
                3 => {
                    if self.y != 0 {
                        return Some((self.x, self.y - 1));
                    }
                }

                // the cell below
                4 => {
                    if self.y != self.height - 1 {
                        return Some((self.x, self.y + 1));
                    }
                }

                // all done
                _ => {
                    return None;
                }
            }
        }
    }
}

#[test]
fn test_neighbors() {
    fn run_one(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        Neighbors {
            width,
            height,
            x,
            y,
            i: 0,
        }
        .collect()
    }
    assert_eq!(vec![(1, 0), (0, 1)], run_one(2, 2, 0, 0));
    assert_eq!(vec![(0, 1), (2, 1), (1, 0), (1, 2)], run_one(3, 3, 1, 1));
}

#[derive(PartialEq)]
struct Grid {
    values: ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>,
}

impl Grid {
    fn shape(&self) -> (usize, usize) {
        let shape = self.values.shape();
        let columns = shape[0];
        let rows = shape[1];
        (columns, rows)
    }

    fn neigbors(&self, pos: (usize, usize)) -> Neighbors {
        let shape = self.values.shape();
        Neighbors {
            width: shape[0],
            height: shape[1],
            x: pos.0,
            y: pos.1,
            i: 0,
        }
    }

    fn is_low_spot(&self, x: usize, y: usize) -> bool {
        // The value at the position in question
        let value = self.values[(x, y)];

        // Check each neighbor cell
        self.neigbors((x, y))
            .all(|neighbor| value < self.values[neighbor])
    }
}

#[test]
fn test_is_low_spot() {
    let values = ndarray::arr2(&[[1, 2, 3], [3, 0, 3], [3, 2, 1]]);
    let grid = Grid { values };
    for x in 0..3 {
        for y in 0..3 {
            assert_eq!(x == y, grid.is_low_spot(x, y));
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (columns, rows) = self.shape();
        for y in 0..rows {
            for x in 0..columns {
                if 0 < x {
                    write!(f, " ")?;
                }
                write!(f, "{:?}", self.values[(x, y)])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_grid(lines: &Vec<String>) -> Grid {
    let value_vector: Vec<u8> = lines
        .iter()
        .map(|line| line.chars())
        .flatten()
        .map(|c| (c as u8) - b'0')
        .collect();
    let rows = lines.len();
    let columns = value_vector.len() / rows;
    if value_vector.len() != rows * columns {
        panic!("size didn't work");
    }
    let values =
        Array::from_shape_vec((columns, rows).strides((1, columns)), value_vector).unwrap();
    Grid { values }
}

#[test]
fn test_parse_format_grid() {
    let grid = parse_grid(&vec!["123".to_string(), "456".to_string()]);
    assert_eq!("1 2 3\n4 5 6\n", format!("{:?}", grid));
}

fn day_9_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let grid = parse_grid(lines);
    let (columns, rows) = grid.shape();
    let mut score = 0;
    for y in 0..rows {
        for x in 0..columns {
            if grid.is_low_spot(x, y) {
                score += 1 + (grid.values[(x, y)] as u64);
            }
        }
    }
    Ok(score)
}

fn day_9_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_9() -> Day {
    Day::new(
        9,
        DayPart::new(day_9_a, 15, 506),
        DayPart::new(day_9_b, 0, 0),
    )
}
