use bitflags::bitflags;

// Checks for possible errors caused by overriding names used by `bitflags!` internally.

#[allow(unused_macros)]
macro_rules! stringify {
    ($($t:tt)*) => { "..." };
}

#[allow(unused_macros)]
macro_rules! write {
    ($($t:tt)*) => { "..." };
}

bitflags! {
    struct Test: u8 {
        const A = 1;
    }
}

fn main() {
    // Just make sure we don't call the redefined `stringify` or `write` macro
    assert_eq!(format!("{:?}", unsafe { Test::from_bits_unchecked(0b11) }), "A | 0x2");
}
