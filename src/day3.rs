use crate::types::{AdventResult, Answer, Day, DayPart};

fn day_3_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let number_of_bits = lines[0].len();
    let numbers: Vec<u64> = lines
        .iter()
        .map(|s| u64::from_str_radix(s, 2).unwrap())
        .collect();
    let mut epsilon: u64 = 0;
    let mut gamma: u64 = 0;
    for i in 0..number_of_bits {
        let mask: u64 = 1 << i;
        let number_of_ones = numbers.iter().filter(|n| *n & mask != 0).count();
        if number_of_ones < numbers.len() / 2 {
            epsilon += mask;
        } else {
            gamma += mask;
        }
    }
    Ok(epsilon * gamma)
}

/// Returns the most common bit in a sequence of binary numbers
/// represented as strings of '0' and '1'.
///
/// TODO: switch from String to &str
fn most_common_bit_in_column(numbers: &Vec<String>, index: usize) -> char {
    let number_of_ones = numbers
        .iter()
        .filter(|s| s.as_bytes()[index] == '1' as u8)
        .count();
    if numbers.len() <= number_of_ones * 2 {
        '1'
    } else {
        '0'
    }
}

#[test]
fn test_most_common_bit_in_column() {
    let data = vec![
        "0001".to_string(),
        "0011".to_string(),
        "0111".to_string(),
        "1111".to_string(),
    ];
    assert_eq!('0', most_common_bit_in_column(&data, 0));
    assert_eq!('1', most_common_bit_in_column(&data, 1));
    assert_eq!('1', most_common_bit_in_column(&data, 2));
    assert_eq!('1', most_common_bit_in_column(&data, 3));
}

fn day_3_b_helper(lines: &Vec<String>, index: usize, keep_common: bool) -> String {
    if lines.len() == 0 {
        panic!("no lines in input");
    } else if lines.len() == 1 {
        lines[0].clone()
    } else {
        let most_common = most_common_bit_in_column(lines, index);
        let matching = lines
            .iter()
            .filter(|s| (s.as_bytes()[index] == most_common as u8) == keep_common)
            .map(|s| s.clone())
            .collect();
        day_3_b_helper(&matching, index + 1, keep_common)
    }
}
fn day_3_b(lines: &Vec<String>) -> AdventResult<Answer> {
    let oxygen_line = day_3_b_helper(lines, 0, true);
    let oxygen = u64::from_str_radix(&oxygen_line, 2).unwrap();
    let co2_line = day_3_b_helper(lines, 0, false);
    let co2 = u64::from_str_radix(&co2_line, 2).unwrap();
    Ok(oxygen * co2)
}

pub fn make_day_3() -> Day {
    Day::new(
        3,
        DayPart::new(day_3_a, 198, 693486),
        DayPart::new(day_3_b, 230, 3379326),
    )
}
