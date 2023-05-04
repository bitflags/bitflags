//! An example of implementing the `BitFlags` trait manually for a flags type.

use std::{fmt, str};

use bitflags::bitflags;

// First: Define your flags type. It needs to be a newtype over its underlying bits type
pub struct ManualFlags(u32);

bitflags! {
    impl ManualFlags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }
}

// Not required: Add parsing support
impl str::FromStr for ManualFlags {
    type Err = bitflags::parser::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(input)
    }
}

// Not required: Add formatting support
impl fmt::Display for ManualFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

fn main() {
    println!("{}", ManualFlags::A | ManualFlags::B | ManualFlags::C);
}
