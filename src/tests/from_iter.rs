use super::*;

#[test]
fn cases() {
    assert_eq!(0, [].into_iter().collect::<TestFlags>().bits());

    assert_eq!(1, [TestFlags::A,].into_iter().collect::<TestFlags>().bits());

    assert_eq!(
        1 | 1 << 1 | 1 << 2,
        [TestFlags::A, TestFlags::B | TestFlags::C,]
            .into_iter()
            .collect::<TestFlags>()
            .bits()
    );

    assert_eq!(
        1 | 1 << 3,
        [
            TestFlags::from_bits_retain(1 << 3),
            TestFlags::empty(),
            TestFlags::A,
        ]
        .into_iter()
        .collect::<TestFlags>()
        .bits()
    );
}
