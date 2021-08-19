#![cfg(feature = "arbitrary")]

extern crate arbitrary as a;

use a::Arbitrary;
use bitflags::bitflags;

// Checks for possible errors caused by overriding names used by `bitflags!` internally.

mod arbitrary {}
mod _arbitrary {}

bitflags! {
    struct Test: u8 {
        const A = 1;
    }
}

#[test]
fn test_arbitrary_redefinition() {
    let mut unstructured = a::Unstructured::new(&[0_u8; 256]);
    let _test = Test::arbitrary(&mut unstructured);
}
