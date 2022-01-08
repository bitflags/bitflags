use bitflags::bitflags;

bitflags! {
    struct Flags: u8 {
        const TWO = 0x2;
    }
}

fn main() {
    // bug #267 (https://github.com/bitflags/bitflags/issues/267)
    let flags = unsafe { Flags::from_bits_unchecked(0b11) };
    assert_eq!(format!("{:?}", flags), "TWO | 0x1");
    assert_eq!(format!("{:#?}", flags), "TWO | 0x1");
}
