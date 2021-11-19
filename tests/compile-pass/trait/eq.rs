use std::borrow::Borrow;

use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Flags: u32 {
        const A = 0b00000001;
    }
}

impl Borrow<u32> for Flags {
    fn borrow(&self) -> &u32 {
        self.bits_ref()
    }
}

fn main() {}
