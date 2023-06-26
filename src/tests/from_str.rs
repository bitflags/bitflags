use super::*;

use crate::parser::from_str;

#[test]
fn valid() {
    assert_eq!(0, from_str::<TestFlags>("").unwrap().bits());

    assert_eq!(1, from_str::<TestFlags>("A").unwrap().bits());
    assert_eq!(1, from_str::<TestFlags>(" A ").unwrap().bits());
    assert_eq!(
        1 | 1 << 1 | 1 << 2,
        from_str::<TestFlags>("A | B | C").unwrap().bits()
    );
    assert_eq!(
        1 | 1 << 1 | 1 << 2,
        from_str::<TestFlags>("A\n|\tB\r\n|   C ").unwrap().bits()
    );
    assert_eq!(
        1 | 1 << 1 | 1 << 2,
        from_str::<TestFlags>("A|B|C").unwrap().bits()
    );

    assert_eq!(1 << 3, from_str::<TestFlags>("0x8").unwrap().bits());
    assert_eq!(1 | 1 << 3, from_str::<TestFlags>("A | 0x8").unwrap().bits());
    assert_eq!(
        1 | 1 << 1 | 1 << 3,
        from_str::<TestFlags>("0x1 | 0x8 | B").unwrap().bits()
    );

    assert_eq!(
        1 | 1 << 1,
        from_str::<TestUnicode>("一 | 二").unwrap().bits()
    );
}

#[test]
fn invalid() {
    assert_eq!(
        "unrecognized named flag",
        from_str::<TestFlags>("a").unwrap_err().to_string()
    );
    assert_eq!(
        "unrecognized named flag",
        from_str::<TestFlags>("A & B").unwrap_err().to_string()
    );

    assert_eq!(
        "invalid hex flag",
        from_str::<TestFlags>("0xg").unwrap_err().to_string()
    );
    assert_eq!(
        "invalid hex flag",
        from_str::<TestFlags>("0xffffffffffff")
            .unwrap_err()
            .to_string()
    );
}
