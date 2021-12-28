use crate::types::{AdventResult, Answer, Day, DayPart};

/// Takes a vector of strings and converts them to u64
fn lines_to_numbers(lines: &[&str]) -> AdventResult<Vec<u64>> {
    let result: Result<Vec<u64>, std::num::ParseIntError> =
        lines.iter().map(|s| s.parse()).collect();
    Ok(result?)
}

#[test]
fn test_lines_to_numbers() {
    assert_eq!(vec![1, 456], lines_to_numbers(&["1", "456"]).unwrap());
}

/// 1a: Counts lines containin numbers bigger than the line before
fn day_1_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut prev: Option<u64> = None;
    let mut count: u64 = 0;
    for value in lines_to_numbers(&lines)? {
        let is_increase = match prev {
            Some(prev_value) => prev_value < value,
            None => false,
        };
        if is_increase {
            count += 1;
        }
        prev = Some(value)
    }
    Ok(count)
}

/// 1b: Counts groups of three lines containin numbers bigger than the line before
fn day_1_b(lines: &[&str]) -> AdventResult<Answer> {
    let mut a;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut num_seen: u64 = 0;
    let mut prev_sum: u64 = 0;
    let mut count: u64 = 0;
    for line in lines {
        a = b;
        b = c;
        c = line.parse()?;
        num_seen += 1;
        if 3 <= num_seen {
            let sum = a + b + c;
            if 4 <= num_seen && prev_sum < sum {
                count += 1;
            }
            prev_sum = sum;
        }
    }
    Ok(count)
}

pub fn make_day_1() -> Day {
    Day::new(
        1,
        DayPart::new(day_1_a, 7, 1233),
        DayPart::new(day_1_b, 5, 1275),
    )
}
