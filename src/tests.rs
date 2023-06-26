mod all;
mod bits;
mod complement;
mod contains;
mod difference;
mod empty;
mod eq;
mod extend;
mod flags;
mod fmt;
mod from_bits;
mod from_bits_retain;
mod from_bits_truncate;
mod from_iter;
mod from_name;
mod from_str;
mod insert;
mod intersection;
mod intersects;
mod is_all;
mod is_empty;
mod iter;
mod iter_names;
mod remove;
mod set;
mod symmetric_difference;
mod toggle;
mod union;

bitflags! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestFlags: u8 {
        /// 1
        const A = 1;

        /// 1 << 1
        const B = 1 << 1;

        /// 1 << 2
        const C = 1 << 2;

        /// 1 | (1 << 1) | (1 << 2)
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestZero: u8 {
        /// 0
        const ZERO = 0;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestEmpty: u8 {}

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TestOverlapping: u8 {
        /// 1 | (1 << 1)
        const AB = 1 | (1 << 1);

        /// (1 << 1) | (1 << 2)
        const BC = (1 << 1) | (1 << 2);
    }
}
