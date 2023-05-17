use bitflags::{bitflags, Flags};

bitflags! {
    struct MyFlags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
    }
}

fn count_flags<F: Flags>(flags: &F) -> usize {
    flags.iter().count()
}

fn main() {
    assert_eq!(2, count_flags(&(MyFlags::A | MyFlags::B)));
}
