use bitflags::bitflags;

// Checks for possible errors caused by overriding names used by `bitflags!` internally.

// bug #265 (https://github.com/bitflags/bitflags/issues/265)

pub struct Ok<T>(T);

bitflags! {
    pub struct Flags: u16{
        const FOO = 0x0001;
    }
}

fn main() {}
