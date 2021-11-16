#![no_std]

use bitflags::bitflags;

bitflags! {
    /// A set of flags.
    #[derive(Debug, PartialEq)]
    struct Flags: u32 {
        /// A flag for the first bit.
        const A = 0b00000001;
        
        /// A flag for the second bit.
        const B = 0b00000010;

        /// A flag for the third bit.
        const C = 0b00000100;
        
        /// A combination of `A`, `B`, and `C`.
        const ABC = Flags::A.bits() | Flags::B.bits() | Flags::C.bits();
    }
}

#[test]
fn basic() {
    assert_eq!(Flags::ABC, Flags::A | Flags::B | Flags::C);
}
