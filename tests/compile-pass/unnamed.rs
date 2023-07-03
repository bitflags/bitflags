use bitflags::bitflags;

bitflags! {
    pub struct Unnamed: u8 {
        const A = 1;

        const _ = Self::A.bits();
        const _ = 2;
        const _ = !0;
    }
}

fn main() {}
