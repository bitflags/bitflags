use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    struct Flags: u8 {
        const TWO = 0x2;
    }
}

fn main() {
    // bug #267 (https://github.com/bitflags/bitflags/issues/267)
    let flags = Flags::from_bits_retain(0b11);
    assert_eq!(format!("{:?}", flags), "Flags(TWO | 0x1)");
    assert_eq!(format!("{:#?}", flags), "Flags(\n    TWO | 0x1,\n)");
}
