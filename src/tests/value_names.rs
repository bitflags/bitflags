#[test]
fn test_value_names() {
    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct TestFlags: u32 {
            const A = 0b00000001;
            const ZERO = 0;
            const B = 0b00000010;
            const C = 0b00000100;
            const CC = Self::C.bits();
            const D = 0b10000100;
            const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
            const AB = Self::A.bits() | Self::B.bits();
            const AC = Self::A.bits() | Self::C.bits();
            const CB = Self::B.bits() | Self::C.bits();
        }
    }

    assert_eq!(TestFlags::A.names().collect::<Vec<_>>(), vec!["A"]);
    assert_eq!(TestFlags::ZERO.names().collect::<Vec<_>>(), vec!["ZERO"]);
    assert_eq!(TestFlags::B.names().collect::<Vec<_>>(), vec!["B"]);

    assert_eq!(TestFlags::C.names().collect::<Vec<_>>(), vec!["C", "CC"]);
    assert!(TestFlags::C.names().any(|n| n == "CC" || n == "C"));

    assert_eq!(TestFlags::CC.names().collect::<Vec<_>>(), vec!["C", "CC"]);
    assert!(TestFlags::CC.names().any(|n| n == "CC" || n == "C"));

    assert_eq!(TestFlags::D.names().collect::<Vec<_>>(), vec!["D"]);
    assert_eq!(TestFlags::ABC.names().collect::<Vec<_>>(), vec!["ABC"]);
    assert_eq!(TestFlags::AB.names().collect::<Vec<_>>(), vec!["AB"]);
    assert_eq!(TestFlags::AC.names().collect::<Vec<_>>(), vec!["AC"]);
    assert_eq!(TestFlags::CB.names().collect::<Vec<_>>(), vec!["CB"]);

    let xyz = TestFlags::from_bits_retain(123456);
    assert!(xyz.names().collect::<Vec<_>>().is_empty());
}
