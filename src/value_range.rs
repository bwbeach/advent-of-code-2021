///
/// The ValueRange class holds the range of values that an expression can/should produce.
///
/// There are functions to calculate the possible range of values from each of
/// the operations in the day 24 ALU (inp, add, mul, div, mod, eql).  And for
/// the operations that take inputs from registers, there are functions to calculate
/// the possible input ranges given an output range and the range of the other
/// argument.
///
use std::cmp::{max, min};
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Eq, PartialEq)]
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

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.end
    }

    /// Does this range contain the given value?
    pub fn contains(&self, a: i64) -> bool {
        self.start <= a && a <= self.end
    }

    /// The range of values possible after adding two inputs with known ranges.
    pub fn add_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        let start = a.start + b.start;
        let end = a.end + b.end;
        ValueRange { start, end }
    }

    /// The range of possible inputs for the left input given the right input and output of add.
    /// And vice-versa -- add is commutative.
    pub fn add_backward(b: ValueRange, z: ValueRange) -> Option<ValueRange> {
        // The lowest possible start is the one that combines with b.end to get z.start
        let start = z.start - b.end;
        // The highest possible end is the one that combines with b.start to get z.end
        let end = z.end - b.start;
        Some(ValueRange { start, end })
    }

    /// The range of values possible after adding two inputs with known ranges.
    pub fn mul_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        let extreme_values = [
            a.start * b.start,
            a.start * b.end,
            a.end * b.start,
            a.end * b.end,
        ];
        ValueRange {
            start: extreme_values.into_iter().min().unwrap(),
            end: extreme_values.into_iter().max().unwrap(),
        }
    }

    /// The range of possible inputs for the left input given the right input and output of mul.
    /// And vice-versa -- mul is commutative.
    pub fn mul_backward(b: ValueRange, z: ValueRange) -> Option<ValueRange> {
        // If 0 is in the input range we know, then we don't know anything
        // about the other input range.
        if b.contains(0) {
            None
        } else {
            // TODO: negative numbers
            if z.start < 0 {
                panic!("negative numbers not implemented");
            }
            Some(ValueRange {
                start: (z.start + b.end - 1) / b.end,
                end: z.end / b.start,
            })
        }
    }

    /// The range of values possible after div-ing two inputs with known ranges.
    pub fn div_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        if b.contains(0) {
            panic!("division by ranges including 0 not supported");
        }
        if a.start < 0 || b.start < 0 {
            panic!("negative division not supported");
        }
        ValueRange {
            start: a.start / b.end,
            end: a.end / b.start,
        }
    }

    /// The range of possible numerators for `div`
    pub fn div_backward_left(b: ValueRange, z: ValueRange) -> Option<ValueRange> {
        if z.start < 0 {
            panic!("negative div not supported");
        }
        if b.start <= 0 {
            panic!("div rhs range includes 0 or is negative");
        }
        Some(ValueRange::new(
            b.start * z.start,
            b.end * z.end + b.end - 1,
        ))
    }

    /// The range of values possible after mod-ing two inputs with known ranges.
    pub fn mod_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        if b.contains(0) {
            panic!("mod by ranges including 0 not supported");
        }
        if a.start < 0 || b.start < 0 {
            panic!("negative mod not supported");
        }
        if a.end < b.start {
            // We know all of the a values are within the modulo,
            // and will come through unchanged.
            a
        } else {
            ValueRange {
                start: 0,
                end: b.end - 1,
            }
        }
    }

    /// The range of values possible after eql-ing two inputs with known ranges.
    pub fn eql_forward(a: ValueRange, b: ValueRange) -> ValueRange {
        match ValueRange::intersect(a, b) {
            // Some intersection; could be true or false
            Some(_) => {
                if a.start == a.end && a == b {
                    // both inputs are constants, and the same.  always equal
                    ValueRange::new(1, 1)
                } else {
                    ValueRange::new(0, 1)
                }
            }
            // no intersection; always false
            None => ValueRange::new(0, 0),
        }
    }

    /// The range of possible inputs to `eql` on the lift side given a right range and a result range.
    ///
    pub fn eql_backward(a: ValueRange, b: ValueRange, z: ValueRange) -> Option<ValueRange> {
        // There are some things we can do if we know the result is "not equal"
        if z == ValueRange::new(0, 0) {
            if b.start == b.end {
                if a.start + 1 == a.end {
                    if a.start == b.start {
                        return Some(ValueRange::new(a.end, a.end));
                    } else {
                        return Some(ValueRange::new(a.start, a.start));
                    }
                }
            }
        }
        // If the results are "equal", we know the range of the input
        if z == ValueRange::new(1, 1) {
            return Some(b);
        }
        None
    }

    /// Returns the intersection of the two ranges.  
    /// ValueRanges always contain at least one value, so None is returned
    /// if the intersection is empty.
    ///
    pub fn intersect(a: ValueRange, b: ValueRange) -> Option<ValueRange> {
        let start = max(a.start, b.start);
        let end = min(a.end, b.end);
        if start <= end {
            Some(ValueRange::new(start, end))
        } else {
            None
        }
    }
}

impl IntoIterator for ValueRange {
    type Item = i64;
    type IntoIter = RangeInclusive<i64>;

    fn into_iter(self) -> RangeInclusive<i64> {
        self.start..=self.end
    }
}

impl fmt::Debug for ValueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}..{:?}]", self.start, self.end)
    }
}

#[test]
fn test_ops() {
    // When checking forward calculations, the range computed should include
    // all of the input combinations, and the input combinations should hit
    // the min and max of the range.
    fn check_forward(
        a_range: ValueRange,
        b_range: ValueRange,
        op: fn(i64, i64) -> i64,
        range_op: fn(ValueRange, ValueRange) -> ValueRange,
    ) {
        let z_range = range_op(a_range, b_range);
        let mut z_min = None;
        let mut z_max = None;
        for a in a_range {
            for b in b_range {
                let z = op(a, b);
                assert_eq!(true, z_range.contains(z));
                z_min = Some(z_min.map(|x| min(x, z)).unwrap_or(z));
                z_max = Some(z_max.map(|x| max(x, z)).unwrap_or(z));
            }
        }
        assert_eq!(z_min.unwrap(), z_range.start);
        assert_eq!(z_max.unwrap(), z_range.end);
    }
    // When checking backward calculations, the extremes of the range
    // should produce at least one value that is in the target range,
    // and one step outside the range on each end should not.
    fn check_backward_left(
        b_range: ValueRange,
        z_range: ValueRange,
        op: fn(i64, i64) -> i64,
        range_op: fn(ValueRange, ValueRange) -> Option<ValueRange>,
    ) {
        let a_range = range_op(b_range, z_range).unwrap();
        assert_eq!(
            true,
            b_range
                .into_iter()
                .any(|b| z_range.contains(op(a_range.start, b)))
        );
        assert_eq!(
            false,
            b_range
                .into_iter()
                .any(|b| z_range.contains(op(a_range.start - 1, b)))
        );
        assert_eq!(
            true,
            b_range
                .into_iter()
                .any(|b| z_range.contains(op(a_range.end, b)))
        );
        assert_eq!(
            false,
            b_range
                .into_iter()
                .any(|b| z_range.contains(op(a_range.end + 1, b)))
        );
    }
    check_forward(
        ValueRange::new(-2, 4),
        ValueRange::new(-8, 16),
        |a, b| a + b,
        ValueRange::add_forward,
    );
    check_backward_left(
        ValueRange::new(-8, 16),
        ValueRange::new(-10, 20),
        |a, b| a + b,
        ValueRange::add_backward,
    );
    check_forward(
        ValueRange::new(2, 3),
        ValueRange::new(5, 7),
        |a, b| a * b,
        ValueRange::mul_forward,
    );
    check_forward(
        ValueRange::new(-2, 3),
        ValueRange::new(5, 7),
        |a, b| a * b,
        ValueRange::mul_forward,
    );
    check_forward(
        ValueRange::new(-3, -2),
        ValueRange::new(5, 7),
        |a, b| a * b,
        ValueRange::mul_forward,
    );
    check_backward_left(
        ValueRange::new(5, 7),
        ValueRange::new(13, 41),
        |a, b| a * b,
        ValueRange::mul_backward,
    );
    check_forward(
        ValueRange::new(13, 29),
        ValueRange::new(5, 7),
        |a, b| a / b,
        ValueRange::div_forward,
    );
    check_backward_left(
        ValueRange::new(5, 7),
        ValueRange::new(11, 13),
        |a, b| a / b,
        ValueRange::div_backward_left,
    );
    check_forward(
        ValueRange::new(13, 29),
        ValueRange::new(5, 7),
        |a, b| a % b,
        ValueRange::mod_forward,
    );
    check_forward(
        ValueRange::new(5, 7),
        ValueRange::new(13, 29),
        |a, b| a % b,
        ValueRange::mod_forward,
    );
    check_forward(
        ValueRange::new(5, 7),
        ValueRange::new(13, 15),
        |a, b| if a == b { 1 } else { 0 },
        ValueRange::eql_forward,
    );
    check_forward(
        ValueRange::new(5, 7),
        ValueRange::new(6, 8),
        |a, b| if a == b { 1 } else { 0 },
        ValueRange::eql_forward,
    );
    check_forward(
        ValueRange::new(5, 5),
        ValueRange::new(5, 5),
        |a, b| if a == b { 1 } else { 0 },
        ValueRange::eql_forward,
    );
    assert_eq!(
        ValueRange::eql_backward(
            ValueRange::new(2, 3),
            ValueRange::new(3, 3),
            ValueRange::new(0, 0)
        ),
        Some(ValueRange::new(2, 2))
    );
    assert_eq!(
        ValueRange::eql_backward(
            ValueRange::new(2, 3),
            ValueRange::new(2, 2),
            ValueRange::new(0, 0)
        ),
        Some(ValueRange::new(3, 3))
    );
    assert_eq!(
        ValueRange::eql_backward(
            ValueRange::new(2, 3),
            ValueRange::new(2, 2),
            ValueRange::new(1, 1)
        ),
        Some(ValueRange::new(2, 2))
    );
}
