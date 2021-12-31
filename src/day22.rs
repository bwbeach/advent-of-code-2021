use std::cmp::{max, min};
use std::collections::HashMap;

use crate::types::{AdventResult, Answer, Day, DayPart};

/// An inclusive span on one axis.  First number
/// is always lower than second number.
type Span = (i32, i32);

/// How many numbers in a span?
fn span_size(span: Span) -> i64 {
    (span.1 as i64) - (span.0 as i64) + 1
}

/// Computes the intersection of two spans.
fn intersect_spans(a: Span, b: Span) -> Option<Span> {
    let low = max(a.0, b.0);
    let high = min(a.1, b.1);
    if low <= high {
        Some((low, high))
    } else {
        None
    }
}

#[test]
fn test_intersect_spans() {
    assert_eq!(None, intersect_spans((1, 4), (5, 9)));
    assert_eq!(Some((5, 5)), intersect_spans((1, 5), (5, 9)));
    assert_eq!(Some((4, 6)), intersect_spans((1, 10), (4, 6)));
}

/// A cube
type Cube = (Span, Span, Span);

/// How many cubelets in a cube?
fn cube_size(cube: Cube) -> i64 {
    let (x, y, z) = cube;
    span_size(x) * span_size(y) * span_size(z)
}

/// Computes the intersection of two cubes
fn intersect_cubes(a: Cube, b: Cube) -> Option<Cube> {
    let (a_x, a_y, a_z) = a;
    let (b_x, b_y, b_z) = b;
    if let Some(x) = intersect_spans(a_x, b_x) {
        if let Some(y) = intersect_spans(a_y, b_y) {
            if let Some(z) = intersect_spans(a_z, b_z) {
                return Some((x, y, z));
            }
        }
    }
    None
}

#[test]
fn test_intersect_cubes() {
    let ten = ((1, 10), (1, 10), (1, 10));
    let lower = ((1, 2), (2, 3), (3, 4));
    let middle = ((4, 6), (4, 6), (4, 6));
    assert_eq!(None, intersect_cubes(lower, middle));
    assert_eq!(Some(middle), intersect_cubes(middle, ten));
}

fn parse_span(s: &str) -> Span {
    let mut numbers = s[2..].split("..");
    let low = numbers.next().unwrap().parse().unwrap();
    let high = numbers.next().unwrap().parse().unwrap();
    (low, high)
}

fn parse_line(line: &str) -> (bool, Cube) {
    let mut words = line.split_whitespace();
    let is_on = words.next().unwrap() == "on";
    let mut spans = words.next().unwrap().split(",");
    let x = parse_span(spans.next().unwrap());
    let y = parse_span(spans.next().unwrap());
    let z = parse_span(spans.next().unwrap());
    (is_on, (x, y, z))
}

#[test]
fn test_parse_line() {
    assert_eq!(
        (true, ((-20, 26), (-36, 17), (-47, 7))),
        parse_line("on x=-20..26,y=-36..17,z=-47..7")
    );
    assert_eq!(
        (false, ((-48, -32), (-32, -16), (-15, -5))),
        parse_line("off x=-48..-32,y=-32..-16,z=-15..-5")
    );
}

/// Builds a new cube-to-coefficient mapping that is the result of adding a
/// new instruction to an existing mapping.
fn add_one_instruction(
    instruction: (bool, Cube),
    before: &HashMap<Cube, i64>,
) -> HashMap<Cube, i64> {
    let (is_on, new_cube) = instruction;

    // The result starts out the same as the previous mapping.
    let mut result = before.clone();

    // The first step is to undo anything that affects the cube being added.
    for (old_cube, old_coefficient) in before {
        if let Some(intersection) = intersect_cubes(new_cube, *old_cube) {
            *result.entry(intersection).or_insert(0) -= old_coefficient;
        }
    }

    // Everything in the span of the new cube has been zeroed out now.
    // If we're turning this cube off, then we're done.
    // If we're turning it on, then do so.
    if is_on {
        *result.entry(new_cube).or_insert(0) += 1;
    }

    result
}

fn count_cubelets(cube_to_coefficient: &HashMap<Cube, i64>) -> usize {
    let mut result: i64 = 0;
    for (cube, coefficient) in cube_to_coefficient {
        result += coefficient * cube_size(*cube);
    }
    result as usize
}

#[test]
fn test_part_a() {
    let mut result: HashMap<Cube, i64> = HashMap::new();
    result = add_one_instruction(parse_line("on x=10..12,y=10..12,z=10..12"), &result);
    assert_eq!(27, count_cubelets(&result));
    result = add_one_instruction(parse_line("on x=11..13,y=11..13,z=11..13"), &result);
    assert_eq!(27 + 19, count_cubelets(&result));
    result = add_one_instruction(parse_line("off x=9..11,y=9..11,z=9..11"), &result);
    assert_eq!(27 + 19 - 8, count_cubelets(&result));
    result = add_one_instruction(parse_line("on x=10..10,y=10..10,z=10..10"), &result);
    assert_eq!(39, count_cubelets(&result));
}

fn day_22_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut result: HashMap<Cube, i64> = HashMap::new();
    for line in lines {
        println!("\nLINE: {:?}\n", line);
        let (is_on, cube_from_line) = parse_line(line);
        if let Some(cube_to_use) =
            intersect_cubes(cube_from_line, ((-50, 50), (-50, 50), (-50, 50)))
        {
            result = add_one_instruction((is_on, cube_to_use), &result);
            println!("\nso far: {:?}", count_cubelets(&result));
        } else {
            println!("SKIP: {:?}\n", line);
        }
    }

    let mut count: i64 = 0;
    for (cube, coefficient) in result {
        count += coefficient * cube_size(cube);
    }

    Ok(count as Answer)
}

fn day_22_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_22() -> Day {
    Day::new(
        22,
        DayPart::new(day_22_a, 590784, 564654),
        DayPart::new(day_22_b, 0, 0),
    )
}
