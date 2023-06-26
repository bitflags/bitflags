use super::*;

#[test]
fn cases() {
    let mut flags = TestFlags::empty();

    flags.extend(TestFlags::A);

    assert_eq!(TestFlags::A, flags);

    flags.extend(TestFlags::A | TestFlags::B | TestFlags::C);

    assert_eq!(TestFlags::ABC, flags);

    flags.extend(TestFlags::from_bits_retain(1 << 5));

    assert_eq!(TestFlags::ABC | TestFlags::from_bits_retain(1 << 5), flags);
}
