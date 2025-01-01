bitflags::bitflags! {
    #[derive(PartialEq)]
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

fn main() {
    match Flags::A {
        Flags::A => (),
        Flags::B => (),
        Flags::C => (),
        _ => (),
    }
}
