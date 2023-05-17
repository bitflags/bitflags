#![allow(deprecated)]

use bitflags::{bitflags, Flags, BitFlags};

fn uses_flags1<B: Flags>(f: B) -> usize {
    uses_bitflags2(f)
}

fn uses_bitflags1<B: BitFlags>(f: B) -> usize {
    uses_flags2(f)
}

fn uses_flags2<B: Flags>(f: B) -> usize {
    f.iter().count()
}

fn uses_bitflags2<B: BitFlags>(f: B) -> usize {
    f.iter().count()
}

bitflags! {
    pub struct MyFlags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

fn main() {
    assert_eq!(uses_flags1(MyFlags::A | MyFlags::B), uses_bitflags1(MyFlags::A | MyFlags::B));
}
