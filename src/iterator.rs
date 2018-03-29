// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num_traits::int::PrimInt;
use num_traits::ops::wrapping::WrappingSub;
use num_traits::{One, Zero};

pub trait BitFlags {
    type Bits;
    type Flags;

    fn as_bits(&self) -> Self::Bits;
    fn from_bits(Self::Bits) -> Self::Flags;
}

/// An iterator over each flag set in a `BitFlags` struct.
///
/// This `struct` is create by the [`iter`] function.
#[derive(Debug)]
pub struct BitFlagsIter<B>
where
    B: BitFlags,
{
    flags: B,
    bit: B::Bits,
}

impl<B> BitFlagsIter<B>
where
    B: BitFlags,
    B::Bits: PrimInt,
{
    pub fn new(flags: B) -> Self {
        BitFlagsIter {
            flags,
            bit: B::Bits::one(),
        }
    }
}

impl<B> Iterator for BitFlagsIter<B>
where
    B: BitFlags,
    B::Bits: PrimInt + WrappingSub,
{
    type Item = B::Flags;

    fn next(&mut self) -> Option<Self::Item> {
        let flags = self.flags.as_bits();
        let t = self.bit.wrapping_sub(&B::Bits::one());
        while (flags & !t) != B::Bits::zero() {
            let test_bit = self.bit;
            self.bit = self.bit.unsigned_shl(1);
            if (flags & test_bit) != B::Bits::zero() {
                return Some(B::from_bits(test_bit));
            }
        }
        None
    }
}
