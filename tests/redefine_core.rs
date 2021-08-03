use bitflags::bitflags;

// ensure that no naming conflicts happen
mod core {}
mod _core {}

bitflags! {
    struct Test: u8 {}
}
