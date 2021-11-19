bitflags! {
    #[derive(Debug)]
    struct Flags: u32 {
        const NONE = 0b0;
        const SOME = 0b1;
    }
}

#[test]
fn test_zero_value_flags() {
    assert!(Flags::empty().contains(Flags::NONE));
    assert!(Flags::SOME.contains(Flags::NONE));
    assert!(Flags::NONE.is_empty());

    assert_eq!(format!("{:?}", Flags::empty()), "Flags(NONE)");
    assert_eq!(format!("{:?}", Flags::SOME), "Flags(SOME)");
}
