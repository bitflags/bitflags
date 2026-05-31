extern crate bitflags;

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Example: u64 {
        #[flag_name = "custom"]
        const FLAG = 0b01;
    }
}

fn main() {
    assert_eq!(Example::FLAG, Example::from_name("custom").unwrap());
    assert_eq!("custom", Example::FLAG.iter_names().next().unwrap().0);
}
