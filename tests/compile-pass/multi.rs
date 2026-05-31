#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct FlagsA: u32 {
        const A = 1;
    }

    pub struct FlagsB: u32 {
        const A = 1;
    }
}

bitflags! {
    pub struct FlagsC: u32 {
        const A = 1;
    }
}

fn main() {}
