use super::*;

use crate::Flags;

#[test]
fn cases() {
    todo!()
}

#[track_caller]
fn case<T: Flags + std::fmt::Debug + std::ops::BitOr<Output = T> + std::ops::BitOrAssign + Copy>(
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
            "{:?}.union({:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            Flags::union(value, *input).bits(),
            "Flags::union({:?}, {:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            (value | *input).bits(),
            "{:?} | {:?}",
            value,
            input
        );
        assert_eq!(
            *expected,
            {
                let mut value = value;
                value |= *input;
                value
            }
            .bits(),
            "{:?} |= {:?}",
            value,
            input,
        );
    }
}
