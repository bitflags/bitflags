use bitflags::bitflags;

// Checks for possible errors caused by overriding names used by `bitflags!` internally.

mod arbitrary {}
mod _arbitrary {}

bitflags! {
    struct Test: u8 {
        const A = 1;
    }
}

fn main() {
    #[cfg(feature = "arbitrary")]
    {
        let mut unstructured = arbitrary::Unstructured::new(&[0_u8; 256]);
        let _test = Test::arbitrary(&mut unstructured);
    }
}
