extern crate bitflags;

#[repr(packed)]
struct Example(u64);

bitflags::bitflags! {
    impl Example: u64 {
        const FLAG_1 = 0b01;
        const FLAG_2 = 0b10;
    }
}

fn main() {}
