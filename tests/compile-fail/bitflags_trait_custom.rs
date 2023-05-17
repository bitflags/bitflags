use bitflags::BitFlags;

pub struct BootlegFlags(u32);

impl BitFlags for BootlegFlags {
    type Bits = u32;
}

fn main() {}
