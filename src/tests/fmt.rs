use super::*;

#[test]
fn cases() {
    assert_eq!("TestFlags(0x0)", format!("{:?}", TestFlags::empty()));
    assert_eq!("TestFlags(A)", format!("{:?}", TestFlags::A));
    assert_eq!("TestFlags(A | B | C)", format!("{:?}", TestFlags::all()));
    assert_eq!(
        "TestFlags(0x8)",
        format!("{:?}", TestFlags::from_bits_retain(1 << 3))
    );
    assert_eq!(
        "TestFlags(A | 0x8)",
        format!("{:?}", TestFlags::A | TestFlags::from_bits_retain(1 << 3))
    );

    assert_eq!("TestZero(0x0)", format!("{:?}", TestZero::ZERO));
    assert_eq!(
        "TestZero(0x1)",
        format!("{:?}", TestZero::ZERO | TestZero::from_bits_retain(1))
    );

    assert_eq!("TestZeroOne(0x0)", format!("{:?}", TestZeroOne::ZERO));
    assert_eq!(
        "TestZeroOne(ONE)",
        format!("{:?}", TestZeroOne::ZERO | TestZeroOne::from_bits_retain(1))
    );

    assert_eq!(
        "TestOverlapping(0x2)",
        format!("{:?}", TestOverlapping::from_bits_retain(1 << 1))
    );
}
