#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
    }
}

pub struct WrappedIter(<Flags as IntoIterator>::IntoIter);

impl Iterator for WrappedIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|flag| flag.bits())
    }
}

fn main() {
    assert_eq!(1, WrappedIter(Flags::A.into_iter()).count());
}
