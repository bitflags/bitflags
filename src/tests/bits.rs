use super::*;

use crate::Flags;

fn case<T: Flags>(expected: T::Bits, value: T, inherent: impl FnOnce(&T) -> T::Bits)
where
    T::Bits: std::fmt::Debug + PartialEq,
{
    assert_eq!(expected, inherent(&value));
    assert_eq!(expected, Flags::bits(&value));
}

#[test]
fn cases() {
    case(0, TestFlags::empty(), TestFlags::bits);

    case(1, TestFlags::A, TestFlags::bits);

    case(1 | 1 << 1 | 1 << 2, TestFlags::ABC, TestFlags::bits);

    case(1 << 3, TestFlags::from_bits_retain(1 << 3), TestFlags::bits);

    case(1 << 3, TestZero::from_bits_retain(1 << 3), TestZero::bits);

    case(1 << 3, TestEmpty::from_bits_retain(1 << 3), TestEmpty::bits);
}
