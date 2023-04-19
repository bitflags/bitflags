#[macro_use]
extern crate bitflags;

bitflags! {
    #[doc(alias = "FLAG")]
    pub struct Flags: u8 {
        #[doc(alias = "FLAG_A")]
        const A = 1;
    }
}

fn main() {
    
}
