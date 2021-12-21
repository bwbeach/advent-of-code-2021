use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

#[test]
fn test_set_of_points() {
    let mut points: HashSet<Point> = HashSet::new();
    points.insert(Point::new(1, 2));
    assert_eq!(true, points.contains(&Point::new(1, 2)));
    assert_eq!(false, points.contains(&Point::new(99, 99)));
}

impl FromStr for Point {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let parts: Vec<String> = s.split(",").map(|s| s.to_string()).collect();
        if parts.len() != 2 {
            Err(AdventError::new("too many commas in point"))
        } else {
            Ok(Point::new(
                u16::from_str(&parts[0]).unwrap(),
                u16::from_str(&parts[1]).unwrap(),
            ))
        }
    }
}

#[test]
fn test_parse_point() {
    assert_eq!(Point::new(1, 2), Point::from_str("1,2").unwrap());
}

#[derive(Debug, Eq, PartialEq)]
struct PointRange {
    p1: Point,
    p2: Point,
}

fn range_inclusive(a: u16, b: u16) -> Box<dyn Iterator<Item = u16>> {
    if a <= b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

impl PointRange {
    fn new(p1: Point, p2: Point) -> PointRange {
        PointRange { p1, p2 }
    }

    fn is_horiz_or_vertical(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn points(&self) -> Vec<Point> {
        if self.p1.x == self.p2.x {
            range_inclusive(self.p1.y, self.p2.y)
                .map(|y| Point::new(self.p1.x, y))
                .collect()
        } else if self.p1.y == self.p2.y {
            range_inclusive(self.p1.x, self.p2.x)
                .map(|x| Point::new(x, self.p1.y))
                .collect()
        } else {
            let x_range = range_inclusive(self.p1.x, self.p2.x);
            let y_range = range_inclusive(self.p1.y, self.p2.y);
            // TODO: panic if ranges are not the same length
            x_range
                .zip(y_range)
                .map(|(x, y)| Point::new(x, y))
                .collect()
        }
    }
}

impl FromStr for PointRange {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<PointRange, Self::Err> {
        let parts: Vec<String> = s.split(" -> ").map(|s| s.to_string()).collect();
        if parts.len() != 2 {
            Err(AdventError::new("bad point range"))
        } else {
            Ok(PointRange {
                p1: Point::from_str(&parts[0]).unwrap(),
                p2: Point::from_str(&parts[1]).unwrap(),
            })
        }
    }
}

#[test]
fn test_parse_point_range() {
    assert_eq!(
        PointRange::new(Point::new(1, 2), Point::new(3, 4)),
        PointRange::from_str("1,2 -> 3,4").unwrap()
    )
}

#[test]
fn test_points_in_range() {
    assert_eq!(
        vec![Point::new(1, 5), Point::new(2, 5)],
        PointRange::from_str("1,5 -> 2,5").unwrap().points()
    );
    assert_eq!(
        vec![Point::new(2, 5), Point::new(1, 5)],
        PointRange::from_str("2,5 -> 1,5").unwrap().points()
    );
    assert_eq!(
        vec![Point::new(5, 1), Point::new(5, 2)],
        PointRange::from_str("5,1 -> 5,2").unwrap().points()
    );
    assert_eq!(
        vec![Point::new(1, 9), Point::new(2, 8), Point::new(3, 7)],
        PointRange::from_str("1,9 -> 3,7").unwrap().points()
    );
}

fn day_5_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut point_to_count: HashMap<Point, u32> = HashMap::new();
    for line in lines.iter() {
        let point_range = PointRange::from_str(line)?;
        if point_range.is_horiz_or_vertical() {
            for point in point_range.points().iter() {
                point_to_count.insert(*point, point_to_count.get(point).unwrap_or(&0) + 1);
            }
        }
    }
    let count = point_to_count.iter().filter(|(_, &v)| 1 < v).count();
    Ok(count as u64)
}

fn day_5_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let mut point_to_count: HashMap<Point, u32> = HashMap::new();
    for line in lines.iter() {
        let point_range = PointRange::from_str(line)?;
        for point in point_range.points().iter() {
            point_to_count.insert(*point, point_to_count.get(point).unwrap_or(&0) + 1);
        }
    }
    let count = point_to_count.iter().filter(|(_, &v)| 1 < v).count();
    Ok(count as u64)
}

pub fn make_day_5() -> Day {
    Day::new(
        5,
        DayPart::new(day_5_a, 5, 6311),
        DayPart::new(day_5_b, 12, 19929),
    )
}
