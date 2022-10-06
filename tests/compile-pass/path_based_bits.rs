#[macro_use]
extern crate bitflags;

pub mod bits {
    pub type Bits = i32;
}

bitflags! {
    pub struct Flags1: bits::Bits {
        const A = 1;
    }
}

fn main() {}
