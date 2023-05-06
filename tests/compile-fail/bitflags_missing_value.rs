#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags1: u32 {
        const A;
    }
}
