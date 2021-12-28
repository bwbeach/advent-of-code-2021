use std::cmp::max;
use std::ops::RangeInclusive;

use regex::Regex;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Holds the x and y ranges that are the target area
#[derive(Debug, PartialEq)]
struct Target {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn parse_target(line: &str) -> Target {
    let pattern =
        Regex::new(r"^target area: x=(-?[0-9]+)[.][.](-?[0-9]+), y=(-?[0-9]+)[.][.](-?[0-9]+)$")
            .unwrap();
    let captures = pattern.captures(line).unwrap();
    let x_min = captures[1].parse().unwrap();
    let x_max = captures[2].parse().unwrap();
    let y_min = captures[3].parse().unwrap();
    let y_max = captures[4].parse().unwrap();
    Target {
        x_range: x_min..=x_max,
        y_range: y_min..=y_max,
    }
}

#[test]
fn test_parse_target() {
    assert_eq!(
        Target {
            x_range: 20..=30,
            y_range: -10..=-5
        },
        parse_target("target area: x=20..30, y=-10..-5")
    )
}

/// Does the given initial velocity hit the target?
fn hits_target(initial_vx: i32, initial_vy: i32, target: &Target) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut vx = initial_vx;
    let mut vy = initial_vy;
    while x <= *target.x_range.end() && *target.y_range.start() <= y {
        if target.x_range.contains(&x) && target.y_range.contains(&y) {
            return true;
        }
        x += vx;
        y += vy;
        vx = max(0, vx - 1);
        vy = vy - 1;
    }
    false
}

/// Returns all of the initial velocities that hit the target.
fn all_velocities(target: &Target) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    // This assumes that the target is off to the right, and so
    // the initial x velocity must be positive.
    //
    // The initial velocity can't be bigger than the far side of
    // the target; if it were, the first position would be past the
    // target.
    for vx in 1..=*target.x_range.end() {
        // This assumes that the target is below the starting position.
        //
        // The initial y velocity can't be more negative than the
        // bottom of the target; if it were, the first position would
        // be past the target.
        //
        // The initial y velocity can't be more positive than the distance
        // to the bottom of the target; if it were, it would go up, come
        // back to a y of 0 with the same velocity, but negative, and then
        // go past the target on the next step.
        for vy in *target.y_range.start()..=-*target.y_range.start() {
            if hits_target(vx, vy, target) {
                result.push((vx, vy));
            }
        }
    }
    result
}

fn day_17_a(lines: &[&str]) -> AdventResult<Answer> {
    let target = parse_target(&lines[0]);
    let all = all_velocities(&target);
    let max_vy = all.iter().map(|(_, vy)| vy).max().unwrap();
    let max_y = (max_vy + max_vy * max_vy) / 2;
    Ok(max_y as Answer)
}

fn day_17_b(lines: &[&str]) -> AdventResult<Answer> {
    let target = parse_target(&lines[0]);
    let all = all_velocities(&target);
    Ok(all.len() as Answer)
}

pub fn make_day_17() -> Day {
    Day::new(
        17,
        DayPart::new(day_17_a, 45, 7750),
        DayPart::new(day_17_b, 112, 4120),
    )
}
