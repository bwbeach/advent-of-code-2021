///
/// The ValueRange class holds the range of values that an expression can/should produce.
///
/// There are functions to calculate the possible range of values from each of
/// the operations in the day 24 ALU (inp, add, mul, div, mod, eql).  And for
/// the operations that take inputs from registers, there are functions to calculate
/// the possible input ranges given an output range and the range of the other
/// argument.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueRange {
    start: i64,
    end: i64,
}

impl ValueRange {
    pub fn new(start: i64, end: i64) -> ValueRange {
        if end < start {
            panic!("ValueRange not in order: {:?} {:?}", start, end)
        }
        ValueRange { start, end }
    }

    /// The range of values possible after adding two inputs with known ranges.
    pub fn add_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        let start = a.start + b.start;
        let end = a.end + b.end;
        ValueRange { start, end }
    }

    /// The range of possible inputs for the left input given the right input and output of add
    /// And vice-versa -- add is commutative
    pub fn add_backward(b: ValueRange, z: ValueRange) -> ValueRange {
        // The lowest possible start is the one that combines with b.end to get z.start
        let start = z.start - b.end;
        // The highest possible end is the one that combines with b.start to get z.end
        let end = z.end - b.start;
        ValueRange { start, end }
    }
}

#[test]
fn test_ops() {
    let a = ValueRange::new(2, 4);
    let b = ValueRange::new(8, 16);
    let z = ValueRange::new(10, 20); // a + b
    assert_eq!(ValueRange::new(10, 20), ValueRange::add_forward(a, b));
    assert_eq!(ValueRange::new(-6, 12), ValueRange::add_backward(b, z));
}
