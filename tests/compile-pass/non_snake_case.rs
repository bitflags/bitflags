#![deny(warnings)]

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags: u32 {
        const CamelCase = 0b00000001;
        const B = 0b00000010;
    }
}

fn main() {}
