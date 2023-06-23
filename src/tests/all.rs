use super::*;

use crate::Flags;

#[track_caller]
fn case<T: Flags + std::fmt::Debug + PartialEq>(expected: T, inherent: impl FnOnce() -> T) {
    assert_eq!(expected, inherent(), "T::all()");
    assert_eq!(expected, T::all(), "Flags::all()");
}

#[test]
fn cases() {
    case(TestFlags::A | TestFlags::B | TestFlags::C, TestFlags::all);

    case(TestZero::empty(), TestZero::all);

    case(TestEmpty::empty(), TestEmpty::all);
}
