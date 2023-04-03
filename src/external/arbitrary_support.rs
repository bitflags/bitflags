#[cfg(test)]
mod tests {
    use arbitrary::Arbitrary;

    bitflags! {
        #[derive(Arbitrary)]
        struct Color: u32 {
            const RED = 0x1;
            const GREEN = 0x2;
            const BLUE = 0x4;
        }
    }

    #[test]
    fn test_arbitrary() {
        let mut unstructured = arbitrary::Unstructured::new(&[0_u8; 256]);
        let _color = Color::arbitrary(&mut unstructured);
    }
}
