mod example {
    use bitflags::bitflags;

    bitflags! {
        #[derive(Clone, Copy)]
        pub struct Flags1: u32 {
            const FLAG_A = 0b00000001;
        }
    }
}

fn main() {
    let mut flag = example::Flags1::FLAG_A;

    let flag1 = flag.0;
    let flag1_bits = flag.bit_field();
    let flag1_bits = flag.as_bit_field();
    let flag1_bits = flag.as_bit_field_mut();
}
