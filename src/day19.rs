use std::collections::HashSet;
use std::fmt;
use std::ops;

use itertools::iproduct;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// A point in 3-D space, with integer coordinates
#[derive(Clone, Copy, Eq, Hash, PartialOrd, Ord, PartialEq)]
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

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
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
    rotation_plus_x_1, // The first rotation must be the identity
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

/// Returns all rotations of a set of points, with each one being a sorted
/// vector of points.
fn all_rotations_of_set(set: &HashSet<Point>) -> Vec<Vec<Point>> {
    let mut result = Vec::new();
    for rotation in ALL_ROTATIONS {
        let mut rotated_points: Vec<_> = set.iter().map(|&p| rotation(p)).collect();
        rotated_points.sort();
        result.push(rotated_points);
    }
    result
}

/// Returns a vector containing one thing per input set.  Each thing is a list
/// of all of the possible rotations of that set; each rotation is a sorted
/// list of points.
fn pre_process_input(sets: &Vec<HashSet<Point>>) -> Vec<Vec<Vec<Point>>> {
    sets.iter().map(|set| all_rotations_of_set(set)).collect()
}

/// Given slices of two lists of sorted points, find the number that match
/// after adding the given offset to the second one.
fn count_matching_points(a: &[Point], b: &[Point], offset: Point) -> usize {
    let mut i_a = 0;
    let mut i_b = 0;
    let mut match_count = 0;
    while i_a < a.len() && i_b < b.len() {
        let p_a = a[i_a];
        let p_b = b[i_b] + offset;
        if p_a == p_b {
            match_count += 1;
            i_a += 1;
            i_b += 1;
        } else {
            if p_a < p_b {
                i_a += 1;
            } else {
                i_b += 1;
            }
        }
    }
    match_count
}

#[test]
fn test_count_matching_points() {
    assert_eq!(
        2,
        count_matching_points(
            &[
                Point::new(0, 0, 0),
                Point::new(1, 2, 3),
                Point::new(6, 7, 8),
                Point::new(9, 9, 9)
            ],
            &[
                Point::new(0, 0, 0),
                Point::new(0, 1, 2),
                Point::new(4, 5, 6),
                Point::new(5, 6, 7)
            ],
            Point::new(1, 1, 1)
        )
    )
}

/// Try to find a match in two sorted point lists without rotation, returning
/// the offset to add to 'b' to get them to match at least 'count' points.
fn match_point_lists(a: &Vec<Point>, b: &Vec<Point>, count: usize) -> Option<Point> {
    // Loop through all of the starting pairings that determine the offset by
    // which we'll move b to try and match a.
    for i_first_a in 0..(a.len() - count + 1) {
        for i_first_b in 0..(b.len() - count + 1) {
            let first_a = a[i_first_a];
            let first_b = b[i_first_b];
            let offset = first_a - first_b;
            if count_matching_points(&a[i_first_a..], &b[i_first_b..], offset) == count {
                return Some(offset);
            }
        }
    }
    None
}

/// Given the output of two sensors, returns the transform for
/// the second one to make it match the first one.
///
/// For the first sensor, we alredy know the orientation because
/// the search starts with an unrotated sensor 0, and then matches
/// things against that.
fn find_match(a_points: &Vec<Point>, b_rotations: &Vec<Vec<Point>>) -> Option<(Point, Vec<Point>)> {
    for b_points in b_rotations {
        if let Some(offset) = match_point_lists(a_points, b_points, 12) {
            let moved_b_points: Vec<_> = b_points.iter().map(|p| *p + offset).collect();
            return Some((offset, moved_b_points));
        }
    }
    None
}

#[test]
fn test_find_match() {
    let lines_in_file =
        crate::util::lines_in_file(std::path::Path::new("input/day-19/sample.txt")).unwrap();
    let strs_in_file: Vec<&str> = lines_in_file.iter().map(|s| &s[..]).collect();
    let sets = pre_process_input(&parse_input(&strs_in_file[..]));

    let (sensor_1_position, sensor_1_points) = find_match(&sets[0][0], &sets[1]).unwrap();
    assert_eq!(Point::new(68, -1246, -43), sensor_1_position);
    assert_eq!(
        true,
        sensor_1_points.contains(&Point::new(-618, -824, -621))
    );
    assert_eq!(true, sensor_1_points.contains(&Point::new(404, -588, -901)));

    assert_eq!(true, find_match(&sets[0][0], &sets[4]).is_none());

    let (sensor_4_position, sensor_4_points) = find_match(&sensor_1_points, &sets[4]).unwrap();
    assert_eq!(Point::new(-20, -1133, 1061), sensor_4_position);
    assert_eq!(true, sensor_1_points.contains(&Point::new(459, -707, 401)));
    assert_eq!(
        true,
        sensor_1_points.contains(&Point::new(-739, -1745, 668))
    );

    let (sensor_2_position, _) = find_match(&sensor_4_points, &sets[2]).unwrap();
    assert_eq!(Point::new(1105, -1205, 1229), sensor_2_position);

    let (sensor_3_position, _) = find_match(&sensor_1_points, &sets[3]).unwrap();
    assert_eq!(Point::new(-92, -2380, -20), sensor_3_position);
}

fn match_with_done(
    done: &Vec<Option<(Point, Vec<Point>)>>,
    to_check: &HashSet<usize>,
    rotations_u: &Vec<Vec<Point>>,
) -> Option<(usize, Point, Vec<Point>)> {
    for (d, d_state) in done.iter().enumerate() {
        if to_check.contains(&d) {
            if let Some((_, points_d)) = d_state {
                if let Some((offset_u, points_u)) = find_match(points_d, rotations_u) {
                    return Some((d, offset_u, points_u.clone()));
                }
            }
        }
    }
    None
}

fn find_all_matches(lines: &[&str]) -> Vec<(Point, Vec<Point>)> {
    let sets = pre_process_input(&parse_input(lines));

    // The 'done' vector is parallel to sets, and tracks which ones
    // have been matched and located.  For each one that's done, we
    // keep the offset to it (the sensor's position), and the matching
    // points after they were rotated and translated.
    let mut done: Vec<Option<(Point, Vec<Point>)>> = Vec::new();
    for _ in 0..sets.len() {
        done.push(None);
    }
    let mut done_count = 1;

    // We want to know the position of every sensor in relation to
    // sensor 0.  Initially, we only know where sensor 0 is.
    done[0] = Some((Point::new(0, 0, 0), sets[0][0].clone()));

    // For efficiency, we track which indices have just been added
    // to done. These are the only ones we need to match against.
    let mut to_check: HashSet<usize> = HashSet::new();
    to_check.insert(0);

    // We'll keep trying to match until they're all done.
    // TODO: optimize to reduce time from 4 minutes: avoid re-comparisons, maybe parallelize
    while done_count < sets.len() {
        let mut new_to_check = HashSet::new();
        for (u, rotations_u) in sets.iter().enumerate() {
            if done[u].is_none() {
                if let Some((d, offset_u, points_u)) =
                    match_with_done(&done, &to_check, rotations_u)
                {
                    println!("    Sensor {:?} is at {:?} matches {:?}", u, offset_u, d);
                    done[u] = Some((offset_u, points_u.clone()));
                    done_count += 1;
                    new_to_check.insert(u);
                }
            }
        }
        if new_to_check.len() == 0 {
            panic!("no progress");
        }
        to_check = new_to_check;
    }

    done.into_iter().map(|item| item.unwrap()).collect()
}

#[test]
fn test_find_all_matches() {
    let lines_in_file =
        crate::util::lines_in_file(std::path::Path::new("input/day-19/sample.txt")).unwrap();
    let strs_in_file: Vec<&str> = lines_in_file.iter().map(|s| &s[..]).collect();
    let answers = find_all_matches(&strs_in_file);
    assert_eq!(Point::new(0, 0, 0), answers[0].0);
    assert_eq!(Point::new(68, -1246, -43), answers[1].0);
    assert_eq!(Point::new(1105, -1205, 1229), answers[2].0);
    assert_eq!(Point::new(-92, -2380, -20), answers[3].0);
    assert_eq!(Point::new(-20, -1133, 1061), answers[4].0);
}

fn day_19_a(lines: &[&str]) -> AdventResult<Answer> {
    let all_probes: HashSet<_> = find_all_matches(lines)
        .iter()
        .map(|(_, points)| points)
        .flatten()
        .map(|&p| p)
        .collect();
    Ok(all_probes.len() as Answer)
}

fn day_19_b(lines: &[&str]) -> AdventResult<Answer> {
    let all_locations: Vec<_> = find_all_matches(lines)
        .iter()
        .map(|(location, _)| *location)
        .collect();
    let max_distance = iproduct!(&all_locations, &all_locations)
        .map(|(a, b)| manhattan_distance(a, b))
        .max()
        .unwrap();
    Ok(max_distance as Answer)
}

pub fn make_day_19() -> Day {
    Day::new(
        19,
        DayPart::new(day_19_a, 79, 350),
        DayPart::new(day_19_b, 3621, 10895),
    )
}
