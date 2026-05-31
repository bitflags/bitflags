#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags: u32 {
        #[cfg(target_os = "linux")]
        const FOO = 1;

        #[cfg(not(target_os = "linux"))]
        const FOO = 2;
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        assert_eq!(1, Flags::FOO.bits());
    }

    #[cfg(not(target_os = "linux"))]
    {
        assert_eq!(2, Flags::FOO.bits());
    }
}
