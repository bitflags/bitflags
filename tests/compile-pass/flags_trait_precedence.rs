use bitflags::{bitflags, Flags};

bitflags! {
    struct MyFlags: u32 {
        const A = 0b00000001;
    }
}

fn all_from_trait<F: Flags>() {
    let _ = F::all();
}

fn main() {
    all_from_trait::<MyFlags>();
    let _ = MyFlags::all();
}
