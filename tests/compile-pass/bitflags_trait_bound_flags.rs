#![allow(deprecated)]

use bitflags::{BitFlags, Flags};

pub trait MyCustomFlagsTrait {
    fn uses_flags<B: BitFlags>(flags: B);
}

pub struct MyCustomFlags;

impl MyCustomFlagsTrait for MyCustomFlags {
    fn uses_flags<B: Flags>(_: B) {

    }
}

fn main() {

}
