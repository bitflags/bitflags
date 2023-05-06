#![allow(deprecated)]

use bitflags::{bitflags, BitFlags};

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
    }
}

fn count_flags<F: BitFlags>(flags: &F) -> usize {
    flags.iter().count()
}

fn main() {
    assert_eq!(2, count_flags(&(Flags::A | Flags::B)));
}
