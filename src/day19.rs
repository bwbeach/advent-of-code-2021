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

impl ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

/// The transform that moves the data for a sensor into place.
///
/// Order of operations is always:
///    (1) rotate
///    (2) translate
///
#[derive(Clone, Copy, Debug)]
struct SensorTransform {
    rotation: Rotation,
    translation: Point,
}

impl SensorTransform {
    fn apply(self, p: Point) -> Point {
        (self.rotation)(p) + self.translation
    }
}

/// Given the output of two sensors, returns the transform for
/// the second one to make it match the first one.
fn find_match(a: &HashSet<Point>, b: &HashSet<Point>) -> Option<SensorTransform> {
    // We can skip the first 11 when finding a point to match on.
    // If 12 points match, we can afford to miss the first 11
    for &a_start in a.iter().skip(11) {
        for &b_start in b {
            for rotation in ALL_ROTATIONS {
                let transform = SensorTransform {
                    rotation: rotation,
                    translation: a_start - rotation(b_start),
                };
                let match_count = b
                    .iter()
                    .map(|&pb| (&transform).apply(pb))
                    .filter(|pbr| a.contains(pbr))
                    .count();
                if 12 <= match_count {
                    return Some(transform);
                }
            }
        }
    }
    None
}

#[test]
fn test_find_match() {
    let lines_in_file =
        crate::util::lines_in_file(std::path::Path::new("input/day-19/sample.txt")).unwrap();
    let strs_in_file: Vec<&str> = lines_in_file.iter().map(|s| &s[..]).collect();
    let sets = parse_input(&strs_in_file[..]);

    assert_eq!(
        Point::new(68, -1246, -43),
        find_match(&sets[0], &sets[1]).unwrap().translation
    );
    assert_eq!(true, find_match(&sets[0], &sets[4]).is_none());
    assert_eq!(true, find_match(&sets[1], &sets[4]).is_some());
}

fn day_19_a(lines: &[&str]) -> AdventResult<Answer> {
    let sets = parse_input(lines);
    let mut done = HashSet::new(); // indices of sets that are done
    let mut all_probes_from_sensor_0 = HashSet::new();

    // We want to know the position of every sensor in relation to
    // sensor 0.  Initially, we only know where sensor 0 is.
    done.insert(0);
    for p in &sets[0] {
        all_probes_from_sensor_0.insert(*p);
    }

    // We'll keep trying to match until they're all done.
    // TODO: optimize to reduce time from 4 minutes: avoid re-comparisons, maybe parallelize
    while (&done).len() < (&sets).len() {
        let mut done_this_time = HashSet::new();
        for (u, set_u) in sets.iter().enumerate() {
            if !done.contains(&u) {
                for &d in &done {
                    let set_d = &sets[d];
                    if let Some(transform) = find_match(set_d, set_u) {
                        println!("sensor {:?} matches sensor {:?}: {:?}", u, d, transform);
                        // Find the transform from sensor 0.
                        // TODO: add ability to combine transforms directly
                        let full_transform = find_match(&all_probes_from_sensor_0, set_u).unwrap();
                        for p in set_u {
                            all_probes_from_sensor_0.insert(full_transform.apply(*p));
                        }
                        done_this_time.insert(u);
                    }
                }
            }
        }
        for dtt in done_this_time {
            done.insert(dtt);
        }
    }
    Ok(all_probes_from_sensor_0.len() as Answer)
}

fn day_19_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_19() -> Day {
    Day::new(
        19,
        DayPart::new(day_19_a, 79, 350),
        DayPart::new(day_19_b, 0, 0),
    )
}
