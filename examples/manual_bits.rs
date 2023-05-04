//! An example of defining a custom flags storage type.

use std::ops::{Not, BitAnd, BitOr, BitXor};

use bitflags::{BitFlags, Bits};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ManualBits([bool; 3]);

impl Bits for ManualBits {
    const EMPTY: Self = ManualBits([false; 3]);

    const ALL: Self = ManualBits([true; 3]);
}

impl Not for ManualBits {
    type Output = Self;

    fn not(self) -> Self::Output {
        ManualBits([!self.0[0], !self.0[1], !self.0[2]])
    }
}

impl BitAnd for ManualBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ManualBits([self.0[0] & rhs.0[0], self.0[1] & rhs.0[1], self.0[2] & rhs.0[2]])
    }
}

impl BitOr for ManualBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ManualBits([self.0[0] | rhs.0[0], self.0[1] | rhs.0[1], self.0[2] | rhs.0[2]])
    }
}

impl BitXor for ManualBits {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        ManualBits([self.0[0] ^ rhs.0[0], self.0[1] ^ rhs.0[1], self.0[2] ^ rhs.0[2]])
    }
}

#[derive(Debug)]
pub struct Flags(ManualBits);

impl Flags {
    pub const A: Self = Flags(ManualBits([true, false, false]));
    pub const B: Self = Flags(ManualBits([false, true, false]));
    pub const C: Self = Flags(ManualBits([false, false, true]));
}

impl BitFlags for Flags {
    const NAMES: &'static [(&'static str, Self)] = &[
        ("A", Self::A),
        ("B", Self::B),
        ("C", Self::C),
    ];

    type Bits = ManualBits;

    type Iter = bitflags::iter::Iter<Self>;

    type IterNames = bitflags::iter::IterNames<Self>;

    fn bits(&self) -> Self::Bits {
        self.0
    }

    fn from_bits_retain(bits: Self::Bits) -> Self {
        Flags(bits)
    }

    fn iter(&self) -> Self::Iter {
        bitflags::iter::Iter::new(self)
    }

    fn iter_names(&self) -> Self::IterNames {
        bitflags::iter::IterNames::new(self)
    }
}

fn main() {
    let flags = Flags::A.union(Flags::B);
    
    println!("{:?}", flags);
    println!("{:?}", flags.iter_names().collect::<Vec<_>>());
}
