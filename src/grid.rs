use ndarray::{Array, ArrayBase, Dim, OwnedRepr, ShapeBuilder};
use std::fmt;

/// One point in a grid
pub type Point = (usize, usize);

/// State for the iterator over the neighbors of a cell in a grid
pub struct Neighbors {
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
pub struct Grid {
    pub values: ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>,
}

impl Grid {
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
