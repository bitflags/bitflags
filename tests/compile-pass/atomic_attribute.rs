use bitflags::{bitflags, AtomicFlags};

bitflags! {
    #[atomic_attr(derive(Default))]
    #[atomic(MyAtomicFlags)]
    pub struct MyFlags: u32 {
        const A = 1;
        const B = 2;
    }
}

fn main() {
    let flags = MyAtomicFlags::default();
    flags.fetch_insert(MyFlags::A, core::sync::atomic::Ordering::Relaxed);
}