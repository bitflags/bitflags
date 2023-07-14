use bitflags::bitflags;

bitflags! {
    pub struct Unnamed: u8 {
        const A = 1;

        const _ = Self::A.bits();
        const _ = 2;
        const _ = !0;
    }

    pub struct External: u8 {
        const _ = !0;
    }

    pub struct Overlapping: u8 {
        const _ = 1;
        const _ = 2;
        const _ = !0;
    }
}

fn main() {}
