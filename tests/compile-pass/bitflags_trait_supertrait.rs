#![allow(deprecated)]

// From: https://github.com/bitflags/bitflags/issues/293#issuecomment-1493296383
use core::{
    fmt::{Binary, LowerHex, Octal, UpperHex},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
        Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    }
};

use bitflags::{bitflags, BitFlags};

pub trait BitFlagsExt {
    fn complement_retain(self) -> Self;
    fn has_uncorresponding_bits(self) -> bool;
}
impl<T, U> BitFlagsExt for T
where
    T: BitFlags<Bits = U>
        + BitAnd<Output = T>
        + BitAndAssign
        + BitOr<Output = T>
        + BitOrAssign
        + BitXor<Output = T>
        + BitXorAssign
        + Not<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Extend<T>
        + FromIterator<T>
        + IntoIterator
        + Binary
        + LowerHex
        + Octal
        + UpperHex,
    U: BitAnd<Output = U>
        + BitAndAssign
        + BitOr<Output = U>
        + BitOrAssign
        + BitXor<Output = U>
        + BitXorAssign
        + Not<Output = U>
        + Shl<Output = U>
        + ShlAssign
        + Shr<Output = U>
        + ShrAssign
        + Add<Output = U>
        + AddAssign
        + Div<Output = U>
        + DivAssign
        + Mul<Output = U>
        + MulAssign
        + Rem<Output = U>
        + RemAssign
        + Sub<Output = U>
        + SubAssign,
{
    fn complement_retain(self) -> Self {
        Self::from_bits_retain(!self.bits())
    }

    fn has_uncorresponding_bits(self) -> bool {
        !(self & Self::all().complement_retain()).is_empty()
    }
}

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
    }
}

fn has_uncorresponding_bits<B: BitFlagsExt>(flags: B) -> bool {
    flags.has_uncorresponding_bits()
}

fn main() {
    has_uncorresponding_bits(Flags::A);
}
