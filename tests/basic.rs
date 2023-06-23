#![allow(unused_attributes)]
#![no_std]

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TestFlags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;

        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TestZero: u8 {
        const ZERO = 0;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TestEmpty: u8 {}
}

bitflags! {
    pub struct I8: i8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct I16: i16 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct I32: i32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct I64: i64 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct I128: i128 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct Isize: isize {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

bitflags! {
    pub struct U8: u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct U16: u16 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct U32: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct U64: u64 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct U128: u128 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }

    pub struct Usize: usize {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
