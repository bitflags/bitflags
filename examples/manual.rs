//! An example of implementing the `BitFlags` trait manually for a flags type.

use std::{fmt, str};

use bitflags::BitFlags;

// First: Define your flags type. It just needs to be `Sized + 'static`.
pub struct ManualFlags(u32);

// Not required: Define some constants for valid flags
impl ManualFlags {
    pub const A: ManualFlags = ManualFlags(0b00000001);
    pub const B: ManualFlags = ManualFlags(0b00000010);
    pub const C: ManualFlags = ManualFlags(0b00000100);
    pub const ABC: ManualFlags = ManualFlags(0b00000111);
}

// Next: Implement the `BitFlags` trait, specifying your set of valid flags
// and iterators
impl BitFlags for ManualFlags {
    const FLAGS: &'static [(&'static str, Self)] = &[
        ("A", Self::A),
        ("B", Self::B),
        ("C", Self::C),
    ];

    type Bits = u32;

    type Iter = bitflags::iter::Iter<Self>;
    type IterNames = bitflags::iter::IterNames<Self>;

    fn bits(&self) -> u32 {
        self.0
    }

    fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    fn iter(&self) -> Self::Iter {
        bitflags::iter::Iter::new(self)
    }

    fn iter_names(&self) -> Self::IterNames {
        bitflags::iter::IterNames::new(self)
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
        bitflags::fmt::to_writer(self, f)
    }
}

fn main() {
    println!("{}", ManualFlags::A.union(ManualFlags::B).union(ManualFlags::C));
}
