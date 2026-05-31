use bitflags::bitflags;

pub const A: u16 = 1;

bitflags! {
    pub struct Flags: u16 {
        const A = A;
    }
}

fn main() {}
