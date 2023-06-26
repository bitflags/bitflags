use super::*;

use crate::Flags;

#[track_caller]
fn case<T: Flags + std::fmt::Debug + std::ops::BitAnd<Output = T> + std::ops::BitAndAssign + Copy>(
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
            "{:?}.intersection({:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            Flags::intersection(value, *input).bits(),
            "Flags::intersection({:?}, {:?})",
            value,
            input
        );
        assert_eq!(
            *expected,
            (value & *input).bits(),
            "{:?} & {:?}",
            value,
            input
        );
        assert_eq!(
            *expected,
            {
                let mut value = value;
                value &= *input;
                value
            }
            .bits(),
            "{:?} &= {:?}",
            value,
            input,
        );
    }
}

#[test]
fn cases() {
    todo!()
}
