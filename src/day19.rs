use std::collections::HashSet;
use std::fmt;
use std::ops;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A point in 3-D space, with integer coordinates
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?},{:?},{:?}", self.x, self.y, self.z)
    }
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[test]
fn test_point_math() {
    assert_eq!(
        Point::new(9, 18, 36),
        Point::new(1, 2, 4) + Point::new(8, 16, 32)
    );
    assert_eq!(
        Point::new(1, 2, 4),
        Point::new(9, 18, 36) - Point::new(8, 16, 32)
    );
}

fn parse_point(s: &str) -> Point {
    let mut words = s.split(",");
    let x: i32 = words.next().unwrap().parse().unwrap();
    let y: i32 = words.next().unwrap().parse().unwrap();
    let z: i32 = words.next().unwrap().parse().unwrap();
    if !words.next().is_none() {
        panic!("too many numbers in Point")
    }
    Point { x, y, z }
}

#[test]
fn test_parse_point() {
    assert_eq!(Point::new(1, -2, 3), parse_point("1,-2,3"));
}

/// A rotation of a point
type Rotation = fn(Point) -> Point;

fn rotation_plus_x_1(p: Point) -> Point {
    Point {
        x: p.x,
        y: p.y,
        z: p.z,
    }
}

fn rotation_plus_x_2(p: Point) -> Point {
    Point {
        x: p.x,
        y: -p.z,
        z: p.y,
    }
}

fn rotation_plus_x_3(p: Point) -> Point {
    Point {
        x: p.x,
        y: -p.y,
        z: -p.z,
    }
}

fn rotation_plus_x_4(p: Point) -> Point {
    Point {
        x: p.x,
        y: p.z,
        z: -p.y,
    }
}

fn rotation_minus_x_1(p: Point) -> Point {
    Point {
        x: -p.x,
        y: p.y,
        z: -p.z,
    }
}

fn rotation_minus_x_2(p: Point) -> Point {
    Point {
        x: -p.x,
        y: p.z,
        z: p.y,
    }
}

fn rotation_minus_x_3(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
        z: p.z,
    }
}

fn rotation_minus_x_4(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.z,
        z: -p.y,
    }
}

fn rotation_plus_y_1(p: Point) -> Point {
    Point {
        x: p.y,
        y: -p.x,
        z: p.z,
    }
}

fn rotation_plus_y_2(p: Point) -> Point {
    Point {
        x: p.y,
        y: -p.z,
        z: -p.x,
    }
}

fn rotation_plus_y_3(p: Point) -> Point {
    Point {
        x: p.y,
        y: p.x,
        z: -p.z,
    }
}

fn rotation_plus_y_4(p: Point) -> Point {
    Point {
        x: p.y,
        y: p.z,
        z: p.x,
    }
}

fn rotation_minus_y_1(p: Point) -> Point {
    Point {
        x: -p.y,
        y: p.x,
        z: p.z,
    }
}

fn rotation_minus_y_2(p: Point) -> Point {
    Point {
        x: -p.y,
        y: -p.z,
        z: p.x,
    }
}

fn rotation_minus_y_3(p: Point) -> Point {
    Point {
        x: -p.y,
        y: -p.x,
        z: -p.z,
    }
}

fn rotation_minus_y_4(p: Point) -> Point {
    Point {
        x: -p.y,
        y: p.z,
        z: -p.x,
    }
}

fn rotation_plus_z_1(p: Point) -> Point {
    Point {
        x: p.z,
        y: p.y,
        z: -p.x,
    }
}

fn rotation_plus_z_2(p: Point) -> Point {
    Point {
        x: p.z,
        y: p.x,
        z: p.y,
    }
}

fn rotation_plus_z_3(p: Point) -> Point {
    Point {
        x: p.z,
        y: -p.y,
        z: p.x,
    }
}

fn rotation_plus_z_4(p: Point) -> Point {
    Point {
        x: p.z,
        y: -p.x,
        z: -p.y,
    }
}

fn rotation_minus_z_1(p: Point) -> Point {
    Point {
        x: -p.z,
        y: p.y,
        z: p.x,
    }
}

fn rotation_minus_z_2(p: Point) -> Point {
    Point {
        x: -p.z,
        y: -p.x,
        z: p.y,
    }
}

fn rotation_minus_z_3(p: Point) -> Point {
    Point {
        x: -p.z,
        y: -p.y,
        z: -p.x,
    }
}

fn rotation_minus_z_4(p: Point) -> Point {
    Point {
        x: -p.z,
        y: p.x,
        z: -p.y,
    }
}

static ALL_ROTATIONS: [Rotation; 24] = [
    rotation_plus_x_1,
    rotation_plus_x_2,
    rotation_plus_x_3,
    rotation_plus_x_4,
    rotation_minus_x_1,
    rotation_minus_x_2,
    rotation_minus_x_3,
    rotation_minus_x_4,
    rotation_plus_y_1,
    rotation_plus_y_2,
    rotation_plus_y_3,
    rotation_plus_y_4,
    rotation_minus_y_1,
    rotation_minus_y_2,
    rotation_minus_y_3,
    rotation_minus_y_4,
    rotation_plus_z_1,
    rotation_plus_z_2,
    rotation_plus_z_3,
    rotation_plus_z_4,
    rotation_minus_z_1,
    rotation_minus_z_2,
    rotation_minus_z_3,
    rotation_minus_z_4,
];

#[test]
fn test_all_rotations() {
    let p0 = Point::new(1, 2, 3);
    let rotated_p: HashSet<_> = ALL_ROTATIONS.iter().map(|r| r(p0)).collect();
    // The rotated points should all be different
    assert_eq!(24, rotated_p.len());
    // All rotations of all of those points should be in the set
    for rotated in &rotated_p {
        for rotation in &ALL_ROTATIONS {
            assert_eq!(true, rotated_p.contains(&rotation(*rotated)));
        }
    }
}

/// Parse the locatons of the beacons from one scanner
fn parse_beacons(lines: &[&str]) -> HashSet<Point> {
    lines.iter().skip(1).map(|&s| parse_point(s)).collect()
}

/// Parse the input file, containing reports from all scanners
fn parse_input(lines: &[&str]) -> Vec<HashSet<Point>> {
    lines
        .split(|line| *line == "")
        .map(|sub_lines| parse_beacons(sub_lines))
        .collect()
}

#[test]
fn test_parse_input() {
    let mut set1 = HashSet::new();
    set1.insert(Point::new(1, 2, 3));
    set1.insert(Point::new(4, 5, 6));
    let mut set2 = HashSet::new();
    set2.insert(Point::new(7, 8, 9));
    assert_eq!(
        vec![set1, set2],
        parse_input(&[
            "--- sensor 0 ---",
            "1,2,3",
            "4,5,6",
            "",
            "--- sensor 1 ---",
            "7,8,9",
        ])
    );
}

fn day_19_a(lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

fn day_19_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_19() -> Day {
    Day::new(
        19,
        DayPart::new(day_19_a, 0, 0),
        DayPart::new(day_19_b, 0, 0),
    )
}
