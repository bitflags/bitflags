#![allow(deprecated)]

use bitflags::{bitflags, BitFlags};

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
    }
}

impl From<u32> for Flags {
    fn from(v: u32) -> Flags {
        Flags::from_bits_truncate(v)
    }
}

fn all_from_trait<F: BitFlags>() {
    let _ = F::all();
}

fn main() {
    all_from_trait::<Flags>();
    let _ = Flags::all();
}
