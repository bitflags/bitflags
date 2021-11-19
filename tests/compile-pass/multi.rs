#![no_std]

use bitflags::bitflags;

bitflags! {
    struct Flags: i8 {
        const ANOTHER_FLAG = -1_i8;
    }

    struct LongFlags: u32 {
        const LONG_A = 0b1111111111111111;
    }
}
