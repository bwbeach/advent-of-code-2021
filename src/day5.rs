use std::collections::HashSet;
use std::str::FromStr;

use crate::types::{AdventError, AdventResult, Answer, Day, DayPart};

#[derive(Debug, Eq, Hash, PartialEq)]
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

fn test_parse_point() {
    assert_eq!(Point::new(1, 2), Point::from_str("1,2").unwrap());
}

fn day_5_a(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

fn day_5_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_5() -> Day {
    Day::new(
        5,
        DayPart::new(day_5_a, 4512, 58374),
        DayPart::new(day_5_b, 1924, 11377),
    )
}
