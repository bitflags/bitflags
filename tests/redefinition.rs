use bitflags::bitflags;

// Checks for possible errors caused by overriding names used by `bitflags!` internally.

mod core {}
mod _core {}

#[allow(unused_macros)]
macro_rules! stringify {
    ($($t:tt)*) => { "..." };
}

bitflags! {
    struct Test: u8 {
        const A = 1;
    }
}

#[test]
fn stringify() {
    assert_eq!(format!("{:?}", Test::A), "A");
}
