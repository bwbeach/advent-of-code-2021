use crate::types::{AdventResult, Answer, Day, DayPart};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Bit {
    Zero,
    One,
}

use Bit::{One, Zero};

fn value_of_hex(c: u8) -> u8 {
    if b'0' <= c && c <= b'9' {
        c - b'0'
    } else if b'A' <= c && c <= b'F' {
        10 + (c - b'A')
    } else {
        panic!("bad hex digit: {:?}", c);
    }
}

#[test]
fn test_value_of_hex() {
    assert_eq!(3, value_of_hex(b'3'));
    assert_eq!(13, value_of_hex(b'D'));
}

#[derive(Debug)]
struct Biterator {
    // the string of hex digits to decode
    hex_values: Vec<u8>,

    // the next bit index to return
    bit_index: usize,
}

impl Biterator {
    fn new(hex_digits: &str) -> Biterator {
        Biterator {
            hex_values: hex_digits
                .as_bytes()
                .iter()
                .map(|&c| value_of_hex(c))
                .collect(),
            bit_index: 0,
        }
    }
    fn next_number(&mut self, bit_count: usize) -> usize {
        let mut result = 0;
        for _ in 0..bit_count {
            result <<= 1;
            if self.next().unwrap() == One {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test_next_number() {
    let mut biterator = Biterator::new("57");
    assert_eq!(2, biterator.next_number(3));
    assert_eq!(23, biterator.next_number(5));
}

impl Iterator for Biterator {
    type Item = Bit;

    fn next(&mut self) -> Option<Bit> {
        let byte_index = self.bit_index / 4;
        if self.hex_values.len() <= byte_index {
            None
        } else {
            let hex_value = self.hex_values[byte_index];
            let mask = 1 << (3 - self.bit_index % 4);
            if hex_value & mask == 0 {
                self.bit_index += 1;
                Some(Zero)
            } else {
                self.bit_index += 1;
                Some(One)
            }
        }
    }
}

#[test]
fn test_biterator() {
    assert_eq!(
        vec![Zero, One, Zero, One, One, Zero, Zero, One],
        Biterator::new("59").collect::<Vec<Bit>>()
    )
}

#[derive(Debug, PartialEq)]
enum Contents {
    Literal(usize),
    Operator(Vec<Packet>),
}

use Contents::{Literal, Operator};

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    type_id: usize,
    contents: Contents,
}

fn parse_literal(biterator: &mut Biterator) -> Contents {
    let mut literal_value = 0;
    loop {
        let last_group_bit = biterator.next().unwrap();
        literal_value = (literal_value << 4) + biterator.next_number(4);
        if last_group_bit == Zero {
            return Literal(literal_value);
        }
    }
}

fn parse_operator(biterator: &mut Biterator) -> Contents {
    let mut sub_packets = Vec::new();
    match biterator.next().unwrap() {
        Zero => {
            let bit_length = biterator.next_number(15);
            let target = biterator.bit_index + bit_length;
            while biterator.bit_index < target {
                sub_packets.push(parse_packet(biterator));
            }
            if biterator.bit_index != target {
                panic!("length mismatch in sub packets");
            }
        }
        One => {
            let subpacket_count = biterator.next_number(11);
            for _ in 0..subpacket_count {
                sub_packets.push(parse_packet(biterator));
            }
        }
    }
    Operator(sub_packets)
}

fn parse_packet(biterator: &mut Biterator) -> Packet {
    let version = biterator.next_number(3);
    let type_id = biterator.next_number(3);
    let contents = match type_id {
        4 => parse_literal(biterator),
        _ => parse_operator(biterator),
    };
    Packet {
        version,
        type_id,
        contents,
    }
}

fn parse_string(s: &str) -> Packet {
    let mut biterator = Biterator::new(s);
    let result = parse_packet(&mut biterator);
    for bit in biterator {
        match bit {
            Zero => {}
            One => panic!("One bit following end of packet"),
        }
    }
    result
}

#[test]
fn test_parse_packet() {
    assert_eq!(
        Packet {
            version: 6,
            type_id: 4,
            contents: Literal(2021)
        },
        parse_string("D2FE28")
    );
    assert_eq!(
        Packet {
            version: 1,
            type_id: 6,
            contents: Operator(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    contents: Literal(10)
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    contents: Literal(20)
                }
            ])
        },
        parse_string("38006F45291200")
    );
    assert_eq!(
        Packet {
            version: 7,
            type_id: 3,
            contents: Operator(vec![
                Packet {
                    version: 2,
                    type_id: 4,
                    contents: Literal(1)
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    contents: Literal(2)
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    contents: Literal(3)
                }
            ])
        },
        parse_string("EE00D40C823060")
    );
}

fn sum_versions(packet: &Packet) -> usize {
    let mut result = packet.version;
    if let Operator(sub_packets) = &packet.contents {
        for sub_packet in sub_packets {
            result += sum_versions(&sub_packet);
        }
    }
    result
}

#[test]
fn test_sum_versions() {
    assert_eq!(16, sum_versions(&parse_string("8A004A801A8002F478")));
    assert_eq!(
        12,
        sum_versions(&parse_string("620080001611562C8802118E34"))
    );
    assert_eq!(
        23,
        sum_versions(&parse_string("C0015000016115A2E0802F182340"))
    );
    assert_eq!(
        31,
        sum_versions(&parse_string("A0016C880162017C3686B18A3D4780"))
    );
}

fn day_16_a(lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(sum_versions(&parse_string(&lines[0])) as Answer)
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.contents {
        Literal(n) => *n,
        Operator(sub_packets) => {
            let mut sub_values = sub_packets.iter().map(|p| evaluate(p));
            match packet.type_id {
                0 => sub_values.sum(),
                1 => sub_values.product(),
                2 => sub_values.min().unwrap(),
                3 => sub_values.max().unwrap(),
                5 => {
                    let a = sub_values.next().unwrap();
                    let b = sub_values.next().unwrap();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = sub_values.next().unwrap();
                    let b = sub_values.next().unwrap();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = sub_values.next().unwrap();
                    let b = sub_values.next().unwrap();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }

                _ => panic!("unexpected type_id: {:?}", packet.type_id),
            }
        }
    }
}

#[test]
fn test_evaluate() {
    assert_eq!(3, evaluate(&parse_string("C200B40A82")));
    assert_eq!(54, evaluate(&parse_string("04005AC33890")));
    assert_eq!(7, evaluate(&parse_string("880086C3E88112")));
    assert_eq!(9, evaluate(&parse_string("CE00C43D881120")));
    assert_eq!(1, evaluate(&parse_string("D8005AC2A8F0")));
    assert_eq!(0, evaluate(&parse_string("F600BC2D8F")));
    assert_eq!(0, evaluate(&parse_string("9C005AC2F8F0")));
    assert_eq!(1, evaluate(&parse_string("9C0141080250320F1802104A08")));
}

fn day_16_b(lines: &Vec<String>) -> AdventResult<Answer> {
    Ok(evaluate(&parse_string(&lines[0])) as Answer)
}

pub fn make_day_16() -> Day {
    Day::new(
        16,
        DayPart::new(day_16_a, 31, 977),
        DayPart::new(day_16_b, 54, 101501020883),
    )
}
