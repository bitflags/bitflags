extern crate bitflags;

struct Example(u64);

bitflags::bitflags! {
    /// Docs on the `impl` block.
    #[allow(dead_code)]
    impl Example: u64 {
        const flag = 0b01;
    }
}

fn main() {}
