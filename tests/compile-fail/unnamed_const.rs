use bitflags::bitflags;

bitflags! {
    pub struct Unnamed: u8 {
        const _ = 1;

        const A = Self::_.bits();
    }
}

fn main() {}
