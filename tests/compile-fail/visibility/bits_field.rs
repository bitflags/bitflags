#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags1: u32 {
        const A = 1;
    }
}

fn main() {
    let _ = Flags1::A.0.bits;
}
