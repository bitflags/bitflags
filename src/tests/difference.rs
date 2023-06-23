use super::*;

use crate::Flags;

#[track_caller]
fn case<T: Flags + std::fmt::Debug + std::ops::Sub<Output = T> + std::ops::SubAssign + Copy>(
    value: T,
    inputs: &[(T, T::Bits)],
    mut inherent: impl FnMut(T, T) -> T,
) where
    T::Bits: std::fmt::Debug + PartialEq + Copy,
{
    for (input, expected) in inputs {
        assert_eq!(
            *expected,
            inherent(value, *input).bits(),
            "{:?}.difference({:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            Flags::difference(value, *input).bits(),
            "Flags::difference({:?}, {:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            (value - *input).bits(),
            "{:?} - {:?}",
            value,
            input
        );
        assert_eq!(
            *expected,
            {
                let mut value = value;
                value -= *input;
                value
            }
            .bits(),
            "{:?} -= {:?}",
            value,
            input,
        );
    }
}

#[test]
fn cases() {
    case(
        TestFlags::A | TestFlags::B,
        &[
            (TestFlags::A, 1 << 1),
            (TestFlags::B, 1),
            (TestFlags::from_bits_retain(1 << 3), 1 | 1 << 1),
        ],
        TestFlags::difference,
    );

    case(
        TestFlags::from_bits_retain(1 | 1 << 3),
        &[
            (TestFlags::A, 1 << 3),
            (TestFlags::from_bits_retain(1 << 3), 1),
        ],
        TestFlags::difference,
    );
}
