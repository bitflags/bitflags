use bitflags::BitFlags;

pub struct BootlegFlags(u32);

impl BitFlags for BootlegFlags {
    type Bits = u32;

    fn empty() -> Self {
        unimplemented!()
    }

    fn all() -> Self {
        unimplemented!()
    }

    fn bits(&self) -> u32 {
        unimplemented!()
    }

    fn from_bits(_: u32) -> Option<BootlegFlags> {
        unimplemented!()
    }

    fn from_bits_truncate(_: u32) -> BootlegFlags {
        unimplemented!()
    }

    unsafe fn from_bits_unchecked(_: u32) -> BootlegFlags {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn is_all(&self) -> bool {
        unimplemented!()
    }

    fn intersects(&self, _: BootlegFlags) -> bool {
        unimplemented!()
    }

    fn contains(&self, _: BootlegFlags) -> bool {
        unimplemented!()
    }

    fn insert(&mut self, _: BootlegFlags) {
        unimplemented!()
    }

    fn remove(&mut self, _: BootlegFlags) {
        unimplemented!()
    }

    fn toggle(&mut self, _: BootlegFlags) {
        unimplemented!()
    }

    fn set(&mut self, _: BootlegFlags, value: bool) {
        unimplemented!()
    }
}

fn main() { }
