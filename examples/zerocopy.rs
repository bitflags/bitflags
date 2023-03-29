#[cfg(feature = "zerocopy_0_6")]
fn main() {
    use bitflags::bitflags;
    use zerocopy::{AsBytes, FromBytes};

    bitflags! {
        #[derive(AsBytes, Debug, Eq, FromBytes, PartialEq)]
        #[repr(transparent)]
        pub struct Flags: u32 {
            const A = 1;
            const B = 2;
            const C = 4;
            const D = 8;
        }
    }

    let flags = Flags::A | Flags::B;
    let bytes = flags.as_bytes();
    println!("{:?} -> {:?}", flags, bytes);

    let flags_from_bytes = Flags::read_from(bytes).unwrap();
    println!("{:?} -> {:?}", bytes, flags_from_bytes);

    assert_eq!(flags, flags_from_bytes);
}

#[cfg(not(feature = "zerocopy_0_6"))]
fn main() {}
