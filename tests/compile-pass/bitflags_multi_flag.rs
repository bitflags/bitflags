use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = Self::A.complement().bits();
    }
}

fn main() {}
