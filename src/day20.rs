use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

use itertools::{iproduct, Itertools};

use crate::types::{AdventResult, Answer, Day, DayPart};

// A two-dimensional point that is the address of a pixel.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors { p: *self, i: 0 }
    }
}

// Iterator over the neighbors of a point, including the point itself
struct Neighbors {
    p: Point,
    i: usize,
}

impl Iterator for Neighbors {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        let index = self.i;
        if index < 9 {
            self.i += 1;
            Some(Point::new(
                self.p.x - 1 + ((index % 3) as i32),
                self.p.y - 1 + ((index / 3) as i32),
            ))
        } else {
            None
        }
    }
}

#[test]
fn test_neighbors() {
    let neighbors: Vec<_> = (Neighbors {
        p: Point::new(5, 5),
        i: 0,
    })
    .collect();
    assert_eq!(
        vec![4, 5, 6, 4, 5, 6, 4, 5, 6],
        neighbors.iter().map(|p| p.x).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![4, 4, 4, 5, 5, 5, 6, 6, 6],
        neighbors.iter().map(|p| p.y).collect::<Vec<_>>()
    );
}

// An algorithm is an array of 0s/1s indicating whether an
// output pixel is on based on the nine pixels in the block
// around an input pixel.
type Algorithm = [u8; 512];

#[derive(Clone)]
struct Image {
    // 0 means '.', and 1 means '#'
    background: u8,

    // all pixels that do not have the background value
    different: HashSet<Point>,
}

impl Image {
    // Returns a new image that is all background
    fn blank(background: u8) -> Image {
        if background != 0 && background != 1 {
            panic!("bad background: {:?}", background);
        }
        Image {
            background,
            different: HashSet::new(),
        }
    }

    // Returns (inclusive) rectangular bounds on different pixels:
    // (min_x, max_x, min_y, max_y), plus one on each side
    fn bounds(&self) -> (i32, i32, i32, i32) {
        let min_x = self.different.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = self.different.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = self.different.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = self.different.iter().map(|p| p.y).max().unwrap() + 1;
        (min_x, max_x, min_y, max_y)
    }

    // Returns the pixel at the given coordinates
    fn get(&self, pos: &Point) -> u8 {
        let mut result = self.background;
        if self.different.contains(pos) {
            result = 1 - result;
        }
        result
    }

    // Sets the pixel at the given coordinates
    fn set(&mut self, pos: &Point, value: u8) {
        if value == self.background {
            self.different.remove(pos);
        } else {
            self.different.insert(*pos);
        }
    }

    // Returns the number of pixels that are on
    fn pixel_on_count(&self) -> usize {
        if self.background == 0 {
            self.different.len()
        } else {
            panic!("can't count pixel when background is on");
        }
    }
}

fn print_image(image: &Image) {
    let (min_x, max_x, min_y, max_y) = image.bounds();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel = if image.get(&Point::new(x, y)) == 1 {
                '#'
            } else {
                '.'
            };
            print!("{}", pixel);
        }
        println!("");
    }
    println!("");
}

struct Input {
    algorithm: Algorithm,
    image: Image,
}

fn parse_algorithm(lines: &[&str]) -> Algorithm {
    fn map_char(c: char) -> Option<u8> {
        match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        }
    }
    let mut result = [0; 512];
    for (i, bit) in lines
        .iter()
        .map(|line| line.chars())
        .flatten()
        .filter_map(map_char)
        .enumerate()
    {
        result[i] = bit;
    }
    result
}

#[test]
fn test_parse_algorithm() {
    let mut expected: Algorithm = [0; 512];
    expected[1] = 1;
    expected[3] = 1;
    assert_eq!(expected, parse_algorithm(&[".#", ".#"]));
}

fn parse_image(lines: &[&str]) -> Image {
    let mut pixels = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                pixels.insert(Point::new(x as i32, y as i32));
            }
        }
    }
    Image {
        background: 0,
        different: pixels,
    }
}

#[test]
fn test_parse_image() {
    let mut expected: HashSet<Point> = HashSet::new();
    expected.insert(Point::new(1, 0));
    expected.insert(Point::new(1, 1));
    assert_eq!(expected, parse_image(&[".#", ".#"]).different);
}

fn parse_input(lines: &[&str]) -> Input {
    let mut blocks = lines.split(|&line| line == "");
    let algorithm = parse_algorithm(blocks.next().unwrap());
    let image = parse_image(blocks.next().unwrap());
    Input { algorithm, image }
}

/// Returns the pixel if it's on in the new image.
fn compute_one_pixel(p: Point, original: &Image, algorithm: &Algorithm) -> u8 {
    let address: usize = p.neighbors().fold(0, |left, neighbor| {
        (left << 1) + (original.get(&neighbor) as usize)
    });
    algorithm[address]
}

/// Runs one image processing step, producing a new image
fn one_step(original: &Image, algorithm: &Algorithm) -> Image {
    // Make the maximum bounds of the output image, which can be one pixel bigger
    // along each edge.
    let (min_x, max_x, min_y, max_y) = original.bounds();

    // Create a new image, and figure out what the background is.
    let old_background_address = if original.background == 0 { 0 } else { 511 };
    let new_background = algorithm[old_background_address];
    let mut new_image = Image::blank(new_background);

    // Check each possible pixel in the new image, and decide whether
    // its on or not.
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Point::new(x, y);
            new_image.set(&p, compute_one_pixel(p, original, algorithm));
        }
    }

    // all done
    new_image
}

fn day_20_a(lines: &[&str]) -> AdventResult<Answer> {
    let input = parse_input(lines);
    let mut current_image = input.image.clone();
    print_image(&current_image);
    for _ in 0..2 {
        current_image = one_step(&current_image, &input.algorithm);
        print_image(&current_image);
    }
    Ok(current_image.pixel_on_count() as Answer)
}

fn day_20_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_20() -> Day {
    Day::new(
        20,
        DayPart::new(day_20_a, 35, 5663),
        DayPart::new(day_20_b, 0, 0),
    )
}
