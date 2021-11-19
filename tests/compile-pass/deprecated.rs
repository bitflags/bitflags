use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        #[deprecated(note = "Use something else.")]
        const ONE = 1;
    }
}
