use crate::types::{AdventResult, Answer, Day, DayPart};

#[derive(Debug, PartialEq)]
enum Char {
    Open(u8),
    Close(u8),
}

fn parse_char(c: u8) -> Char {
    match c {
        b'(' => Char::Open(b'('),
        b')' => Char::Close(b'('),
        b'[' => Char::Open(b'['),
        b']' => Char::Close(b'['),
        b'{' => Char::Open(b'{'),
        b'}' => Char::Close(b'{'),
        b'<' => Char::Open(b'<'),
        b'>' => Char::Close(b'<'),
        _ => {
            panic!("bad char: {:?}", c);
        }
    }
}

/// The result of checking a line.
#[derive(Debug, PartialEq)]
enum LineStatus {
    // There was a mismatch; holds score of the mismatch
    Mismatch(Answer),

    // Everything was fine; holds the score of un-closed opening chars
    Incomplete(Answer),
}

fn score_for_char(b: u8) -> Answer {
    match b {
        b'(' => 3,
        b'[' => 57,
        b'{' => 1197,
        b'<' => 25137,
        _ => {
            panic!("Unknown closing char: {:?}", b);
        }
    }
}

/// Compute the score for an incomplete line
fn score_incomplete(stack: &Vec<u8>) -> Answer {
    let mut result = 0;
    for b in stack.iter().rev() {
        let char_score = match b {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => panic!("Unknown incomplete char: {:?}", b),
        };
        result = result * 5 + char_score;
    }
    result
}

/// Check a line and return its status
fn check_line(line: &str) -> LineStatus {
    let mut stack: Vec<u8> = Vec::new();
    for c in line.bytes().map(parse_char) {
        match c {
            Char::Open(b) => {
                stack.push(b);
            }
            Char::Close(b) => {
                if b == *stack.last().unwrap() {
                    stack.pop();
                } else {
                    return LineStatus::Mismatch(score_for_char(b));
                }
            }
        }
    }
    LineStatus::Incomplete(score_incomplete(&stack))
}

#[test]
fn test_check_line() {
    assert_eq!(
        LineStatus::Mismatch(1197),
        check_line("{([(<{}[<>[]}>{[]{[(<()>")
    );
    assert_eq!(
        LineStatus::Mismatch(3),
        check_line("[[<[([]))<([[{}[[()]]]")
    );
    assert_eq!(
        LineStatus::Incomplete(288957),
        check_line("[({(<(())[]>[[{[]{<()<>>")
    );
    assert_eq!(
        LineStatus::Incomplete(5566),
        check_line("[(()[<>])]({[<{<<[]>>(")
    );
}

fn day_10_a(lines: &[&str]) -> AdventResult<Answer> {
    let answer: Answer = lines
        .iter()
        .filter_map(|line| match check_line(line) {
            LineStatus::Mismatch(score) => Some(score),
            LineStatus::Incomplete(_) => None,
        })
        .sum();
    Ok(answer)
}

fn day_10_b(lines: &[&str]) -> AdventResult<Answer> {
    let mut answers: Vec<Answer> = lines
        .iter()
        .filter_map(|line| match check_line(line) {
            LineStatus::Mismatch(_) => None,
            LineStatus::Incomplete(score) => Some(score),
        })
        .collect();
    answers.sort();
    Ok(answers[answers.len() / 2])
}

pub fn make_day_10() -> Day {
    Day::new(
        10,
        DayPart::new(day_10_a, 26397, 364389),
        DayPart::new(day_10_b, 288957, 2870201088),
    )
}
