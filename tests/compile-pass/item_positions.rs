#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags1: u32 {
        const A = 1;
    }
}

bitflags! {
    pub struct Flags2: u32 {
        const A = 1;
    }
}

pub mod nested {
    bitflags! {
        pub struct Flags1: u32 {
            const A = 1;
        }
    }

    bitflags! {
        pub struct Flags2: u32 {
            const A = 1;
        }
    }
}

pub const _: () = {
    bitflags! {
        pub struct Flags1: u32 {
            const A = 1;
        }
    }
};

fn main() {
    bitflags! {
        pub struct Flags1: u32 {
            const A = 1;
        }
    }

    let _ = {
        bitflags! {
            pub struct Flags2: u32 {
                const A = 1;
            }
        }
    };
}
