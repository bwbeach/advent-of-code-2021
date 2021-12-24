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

fn score_line(line: &str) -> Answer {
    let mut stack: Vec<Char> = Vec::new();
    for c in line.bytes().map(parse_char) {
        match c {
            Char::Open(_) => {
                stack.push(c);
            }
            Char::Close(b) => {
                if Char::Open(b) == *stack.last().unwrap() {
                    stack.pop();
                } else {
                    return score_for_char(b);
                }
            }
        }
    }
    0
}

#[test]
fn test_score_line() {
    assert_eq!(1197, score_line("{([(<{}[<>[]}>{[]{[(<()>"));
    assert_eq!(3, score_line("[[<[([]))<([[{}[[()]]]"));
}

fn day_10_a(lines: &Vec<String>) -> AdventResult<Answer> {
    let answer: Answer = lines.iter().map(|s| score_line(s)).sum();
    Ok(answer)
}

fn day_10_b(_lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_10() -> Day {
    Day::new(
        10,
        DayPart::new(day_10_a, 26397, 364389),
        DayPart::new(day_10_b, 0, 0),
    )
}
