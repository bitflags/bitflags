bitflags::bitflags! {
    #[derive(Debug)]
    /// Example Flags
    pub struct Flags: u32 {
        /// A
        const A = 0b0000_0001;
        /// B
        const B = 0b0000_0010;
        /// C
        const C = 0b0000_0100;
        /// ABC
        const ABC = Flags::A.bits() | Flags::B.bits() | Flags::C.bits();

        const _ = !0;
    }
}

fn main() {
    bitflags::bitflags_match!(Flags::A | Flags::B, {
        Flags::A => (),
    });
}
