use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const SOME_FLAG = 1 << Self::SOME_FLAG_SHIFT;
    }
}

impl Flags {
    const SOME_FLAG_SHIFT: u32 = 5;
}

fn main() {
    
}
