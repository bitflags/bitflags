#![deny(clippy::pedantic)]

use bitflags::bitflags;

bitflags! {
    pub struct MyFlags: u32 {
        const A = 1;
        const B = 2;
    }
}

fn main() {}
