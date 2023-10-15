/*!
Crate-level doc
*/

#![deny(missing_docs)]

use bitflags::bitflags;

bitflags! {
    #[allow(missing_docs)]
    pub struct MyFlags: u32 {
        #[allow(missing_docs)]
        const A = 1;
        #[allow(missing_docs)]
        const B = 2;
    }
}

fn main() {}
