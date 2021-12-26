use ndarray::{Array, ArrayBase, Dim, OwnedRepr, ShapeBuilder};
use std::fmt;

/// One point in a grid
pub type Point = (usize, usize);

pub fn parse_point(s: &str) -> Point {
    let mut parts = s.split(",");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

#[test]
fn test_parse_point() {
    assert_eq!((3, 7), parse_point("3,7"));
}

/// State for the iterator over the neighbors of a cell in a grid
pub struct Neighbors {
    // the size of the grid
    width: usize,
    height: usize,

    // the point whose neighbors we want
    x: usize,
    y: usize,

    // include diagonals?
    include_diagonals: bool,

    // how many neighbors we've returned so far
    i: usize,
}

impl Iterator for Neighbors {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let at_left = self.x == 0;
        let at_right = self.x == self.width - 1;
        let at_top = self.y == 0;
        let at_bottom = self.y == self.height - 1;
        loop {
            self.i += 1;
            match self.i {
                // the cell to the left
                1 => {
                    if !at_left {
                        return Some((self.x - 1, self.y));
                    }
                }

                // diagonal: up and left
                2 => {
                    if self.include_diagonals && !at_left && !at_top {
                        return Some((self.x - 1, self.y - 1));
                    }
                }

                // the cell above
                3 => {
                    if !at_top {
                        return Some((self.x, self.y - 1));
                    }
                }

                // diagonal: up and right
                4 => {
                    if self.include_diagonals && !at_right && !at_top {
                        return Some((self.x + 1, self.y - 1));
                    }
                }

                // the cell to the right
                5 => {
                    if !at_right {
                        return Some((self.x + 1, self.y));
                    }
                }

                // diagonal: down and right
                6 => {
                    if self.include_diagonals && !at_right && !at_bottom {
                        return Some((self.x + 1, self.y + 1));
                    }
                }

                // the cell below
                7 => {
                    if !at_bottom {
                        return Some((self.x, self.y + 1));
                    }
                }

                // diagonal: down and right
                8 => {
                    if self.include_diagonals && !at_left && !at_bottom {
                        return Some((self.x - 1, self.y + 1));
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
fn test_neighbors_no_diagonals() {
    fn run_one(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        Neighbors {
            width,
            height,
            x,
            y,
            include_diagonals: false,
            i: 0,
        }
        .collect()
    }
    assert_eq!(vec![(1, 0), (0, 1)], run_one(2, 2, 0, 0));
    assert_eq!(vec![(0, 1), (1, 0)], run_one(2, 2, 1, 1));
    assert_eq!(vec![(0, 1), (1, 0), (2, 1), (1, 2)], run_one(3, 3, 1, 1));
}

#[test]
fn test_neighbors_with_diagonals() {
    fn run_one(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        Neighbors {
            width,
            height,
            x,
            y,
            include_diagonals: true,
            i: 0,
        }
        .collect()
    }
    assert_eq!(vec![(1, 0), (1, 1), (0, 1)], run_one(2, 2, 0, 0));
    assert_eq!(vec![(0, 1), (0, 0), (1, 0)], run_one(2, 2, 1, 1));
    assert_eq!(
        vec![
            (0, 1),
            (0, 0),
            (1, 0),
            (2, 0),
            (2, 1),
            (2, 2),
            (1, 2),
            (0, 2)
        ],
        run_one(3, 3, 1, 1)
    );
}

#[derive(PartialEq)]
pub struct Grid {
    values: ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>,
}

impl Grid {
    pub fn zeros(shape: (usize, usize)) -> Grid {
        let values = ArrayBase::zeros(shape);
        Grid { values }
    }

    pub fn get(&self, pos: Point) -> u8 {
        self.values[pos]
    }

    pub fn set(&mut self, pos: Point, new_value: u8) {
        self.values[pos] = new_value;
    }

    pub fn shape(&self) -> (usize, usize) {
        let shape = self.values.shape();
        let columns = shape[0];
        let rows = shape[1];
        (columns, rows)
    }

    pub fn neigbors(&self, pos: (usize, usize)) -> Neighbors {
        let shape = self.values.shape();
        Neighbors {
            width: shape[0],
            height: shape[1],
            x: pos.0,
            y: pos.1,
            include_diagonals: false,
            i: 0,
        }
    }

    pub fn neigbors_with_diagonals(&self, pos: (usize, usize)) -> Neighbors {
        let shape = self.values.shape();
        Neighbors {
            width: shape[0],
            height: shape[1],
            x: pos.0,
            y: pos.1,
            include_diagonals: true,
            i: 0,
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

pub fn parse_grid(lines: &Vec<String>) -> Grid {
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
