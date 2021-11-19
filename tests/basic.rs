#![no_std]

use bitflags::bitflags;

bitflags! {
    /// A set of flags.
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Flags: u32 {
        /// A flag for the first bit.
        const A = 0b00000001;
        
        /// A flag for the second bit.
        const B = 0b00000010;

        /// A flag for the third bit.
        const C = 0b00000100;
        
        /// A combination of `A`, `B`, and `C`.
        const ABC = Flags::A.bits() | Flags::B.bits() | Flags::C.bits();
    }
}

#[test]
fn basic() {
    assert_eq!(Flags::ABC, Flags::A | Flags::B | Flags::C);
}

#[test]
fn test_bits() {
    assert_eq!(Flags::empty().bits(), 0b00000000);
    assert_eq!(Flags::A.bits(), 0b00000001);
    assert_eq!(Flags::ABC.bits(), 0b00000111);

    assert_eq!(AnotherSetOfFlags::empty().bits(), 0b00);
    assert_eq!(AnotherSetOfFlags::ANOTHER_FLAG.bits(), !0_i8);

    assert_eq!(EmptyFlags::empty().bits(), 0b00000000);
}

#[test]
fn test_from_bits() {
    assert_eq!(Flags::from_bits(0), Some(Flags::empty()));
    assert_eq!(Flags::from_bits(0b1), Some(Flags::A));
    assert_eq!(Flags::from_bits(0b10), Some(Flags::B));
    assert_eq!(Flags::from_bits(0b11), Some(Flags::A | Flags::B));
    assert_eq!(Flags::from_bits(0b1000), None);

    assert_eq!(
        AnotherSetOfFlags::from_bits(!0_i8),
        Some(AnotherSetOfFlags::ANOTHER_FLAG)
    );

    assert_eq!(EmptyFlags::from_bits(0), Some(EmptyFlags::empty()));
    assert_eq!(EmptyFlags::from_bits(0b1), None);
}

#[test]
fn test_from_bits_truncate() {
    assert_eq!(Flags::from_bits_truncate(0), Flags::empty());
    assert_eq!(Flags::from_bits_truncate(0b1), Flags::A);
    assert_eq!(Flags::from_bits_truncate(0b10), Flags::B);
    assert_eq!(Flags::from_bits_truncate(0b11), (Flags::A | Flags::B));
    assert_eq!(Flags::from_bits_truncate(0b1000), Flags::empty());
    assert_eq!(Flags::from_bits_truncate(0b1001), Flags::A);

    assert_eq!(
        AnotherSetOfFlags::from_bits_truncate(0_i8),
        AnotherSetOfFlags::empty()
    );

    assert_eq!(EmptyFlags::from_bits_truncate(0), EmptyFlags::empty());
    assert_eq!(EmptyFlags::from_bits_truncate(0b1), EmptyFlags::empty());
}

#[test]
fn test_from_bits_preserve() {
    let extra = Flags::from_bits_preserve(0b1000);
    assert_eq!(Flags::from_bits_preserve(0), Flags::empty());
    assert_eq!(Flags::from_bits_preserve(0b1), Flags::A);
    assert_eq!(Flags::from_bits_preserve(0b10), Flags::B);

    assert_eq!(
        Flags::from_bits_preserve(0b11),
        (Flags::A | Flags::B)
    );
    assert_eq!(
        Flags::from_bits_preserve(0b1000),
        (extra | Flags::empty())
    );
    assert_eq!(
        Flags::from_bits_preserve(0b1001),
        (extra | Flags::A)
    );

    let extra = EmptyFlags::from_bits_preserve(0b1000);
    assert_eq!(
        EmptyFlags::from_bits_preserve(0b1000),
        (extra | EmptyFlags::empty())
    );
}

#[test]
fn test_is_empty() {
    assert!(Flags::empty().is_empty());
    assert!(!Flags::A.is_empty());
    assert!(!Flags::ABC.is_empty());

    assert!(!AnotherSetOfFlags::ANOTHER_FLAG.is_empty());

    assert!(EmptyFlags::empty().is_empty());
    assert!(EmptyFlags::all().is_empty());
}

#[test]
fn test_is_all() {
    assert!(Flags::all().is_all());
    assert!(!Flags::A.is_all());
    assert!(Flags::ABC.is_all());

    let extra = Flags::from_bits_preserve(0b1000);
    assert!(!extra.is_all());
    assert!(!(Flags::A | extra).is_all());
    assert!((Flags::ABC | extra).is_all());

    assert!(AnotherSetOfFlags::ANOTHER_FLAG.is_all());

    assert!(EmptyFlags::all().is_all());
    assert!(EmptyFlags::empty().is_all());
}

#[test]
fn test_two_empties_do_not_intersect() {
    let e1 = Flags::empty();
    let e2 = Flags::empty();
    assert!(!e1.intersects(e2));

    assert!(AnotherSetOfFlags::ANOTHER_FLAG.intersects(AnotherSetOfFlags::ANOTHER_FLAG));
}

#[test]
fn test_empty_does_not_intersect_with_full() {
    let e1 = Flags::empty();
    let e2 = Flags::ABC;
    assert!(!e1.intersects(e2));
}

#[test]
fn test_disjoint_intersects() {
    let e1 = Flags::A;
    let e2 = Flags::B;
    assert!(!e1.intersects(e2));
}

#[test]
fn test_overlapping_intersects() {
    let e1 = Flags::A;
    let e2 = Flags::A | Flags::B;
    assert!(e1.intersects(e2));
}

#[test]
fn test_contains() {
    let e1 = Flags::A;
    let e2 = Flags::A | Flags::B;
    assert!(!e1.contains(e2));
    assert!(e2.contains(e1));
    assert!(Flags::ABC.contains(e2));

    assert!(AnotherSetOfFlags::ANOTHER_FLAG.contains(AnotherSetOfFlags::ANOTHER_FLAG));

    assert!(EmptyFlags::empty().contains(EmptyFlags::empty()));
}

#[test]
fn test_insert() {
    let mut e1 = Flags::A;
    let e2 = Flags::A | Flags::B;
    e1.insert(e2);
    assert_eq!(e1, e2);

    let mut e3 = AnotherSetOfFlags::empty();
    e3.insert(AnotherSetOfFlags::ANOTHER_FLAG);
    assert_eq!(e3, AnotherSetOfFlags::ANOTHER_FLAG);
}

#[test]
fn test_remove() {
    let mut e1 = Flags::A | Flags::B;
    let e2 = Flags::A | Flags::C;
    e1.remove(e2);
    assert_eq!(e1, Flags::B);

    let mut e3 = AnotherSetOfFlags::ANOTHER_FLAG;
    e3.remove(AnotherSetOfFlags::ANOTHER_FLAG);
    assert_eq!(e3, AnotherSetOfFlags::empty());
}

#[test]
fn test_operators() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC); // union
    assert_eq!((e1 & e2), Flags::C); // intersection
    assert_eq!((e1 - e2), Flags::A); // set difference
    assert_eq!(!e2, Flags::A); // set complement
    assert_eq!(e1 ^ e2, Flags::A | Flags::B); // toggle
    let mut e3 = e1;
    e3.toggle(e2);
    assert_eq!(e3, Flags::A | Flags::B);

    let mut m4 = AnotherSetOfFlags::empty();
    m4.toggle(AnotherSetOfFlags::empty());
    assert_eq!(m4, AnotherSetOfFlags::empty());
}

#[test]
fn test_operators_unchecked() {
    let extra = Flags::from_bits_preserve(0b1000);
    let e1 = Flags::A | Flags::C | extra;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), (Flags::ABC | extra)); // union
    assert_eq!((e1 & e2), Flags::C); // intersection
    assert_eq!((e1 - e2), (Flags::A | extra)); // set difference
    assert_eq!(!e2, Flags::A); // set complement
    assert_eq!(!e1, Flags::B); // set complement
    assert_eq!(e1 ^ e2, Flags::A | Flags::B | extra); // toggle
    let mut e3 = e1;
    e3.toggle(e2);
    assert_eq!(e3, Flags::A | Flags::B | extra);
}

#[test]
fn test_set_ops_basic() {
    let ab = Flags::A.union(Flags::B);
    let ac = Flags::A.union(Flags::C);
    let bc = Flags::B.union(Flags::C);
    assert_eq!(ab.bits(), 0b011);
    assert_eq!(bc.bits(), 0b110);
    assert_eq!(ac.bits(), 0b101);

    assert_eq!(ab, Flags::B.union(Flags::A));
    assert_eq!(ac, Flags::C.union(Flags::A));
    assert_eq!(bc, Flags::C.union(Flags::B));

    assert_eq!(ac, Flags::A | Flags::C);
    assert_eq!(bc, Flags::B | Flags::C);
    assert_eq!(ab.union(bc), Flags::ABC);

    assert_eq!(ac, Flags::A | Flags::C);
    assert_eq!(bc, Flags::B | Flags::C);

    assert_eq!(ac.union(bc), ac | bc);
    assert_eq!(ac.union(bc), Flags::ABC);
    assert_eq!(bc.union(ac), Flags::ABC);

    assert_eq!(ac.intersection(bc), ac & bc);
    assert_eq!(ac.intersection(bc), Flags::C);
    assert_eq!(bc.intersection(ac), Flags::C);

    assert_eq!(ac.difference(bc), ac - bc);
    assert_eq!(bc.difference(ac), bc - ac);
    assert_eq!(ac.difference(bc), Flags::A);
    assert_eq!(bc.difference(ac), Flags::B);

    assert_eq!(bc.complement(), !bc);
    assert_eq!(bc.complement(), Flags::A);
    assert_eq!(ac.symmetric_difference(bc), Flags::A.union(Flags::B));
    assert_eq!(bc.symmetric_difference(ac), Flags::A.union(Flags::B));
}

#[test]
fn test_set_ops_const() {
    // These just test that these compile and don't cause use-site panics
    // (would be possible if we had some sort of UB)
    const INTERSECT: Flags = Flags::all().intersection(Flags::C);
    const UNION: Flags = Flags::A.union(Flags::C);
    const DIFFERENCE: Flags = Flags::all().difference(Flags::A);
    const COMPLEMENT: Flags = Flags::C.complement();
    const SYM_DIFFERENCE: Flags = UNION.symmetric_difference(DIFFERENCE);
    assert_eq!(INTERSECT, Flags::C);
    assert_eq!(UNION, Flags::A | Flags::C);
    assert_eq!(DIFFERENCE, Flags::all() - Flags::A);
    assert_eq!(COMPLEMENT, !Flags::C);
    assert_eq!(SYM_DIFFERENCE, (Flags::A | Flags::C) ^ (Flags::all() - Flags::A));
}

#[test]
fn test_set_ops_unchecked() {
    let extra = Flags::from_bits_preserve(0b1000);
    let e1 = Flags::A.union(Flags::C).union(extra);
    let e2 = Flags::B.union(Flags::C);
    assert_eq!(e1.bits(), 0b1101);
    assert_eq!(e1.union(e2), (Flags::ABC | extra));
    assert_eq!(e1.intersection(e2), Flags::C);
    assert_eq!(e1.difference(e2), Flags::A | extra);
    assert_eq!(e2.difference(e1), Flags::B);
    assert_eq!(e2.complement(), Flags::A);
    assert_eq!(e1.complement(), Flags::B);
    assert_eq!(e1.symmetric_difference(e2), Flags::A | Flags::B | extra); // toggle
}

#[test]
fn test_set() {
    let mut e1 = Flags::A | Flags::C;
    e1.set(Flags::B, true);
    e1.set(Flags::C, false);

    assert_eq!(e1, Flags::A | Flags::B);
}

#[test]
fn test_assignment_operators() {
    let mut m1 = Flags::empty();
    let e1 = Flags::A | Flags::C;
    // union
    m1 |= Flags::A;
    assert_eq!(m1, Flags::A);
    // intersection
    m1 &= e1;
    assert_eq!(m1, Flags::A);
    // set difference
    m1 -= m1;
    assert_eq!(m1, Flags::empty());
    // toggle
    m1 ^= e1;
    assert_eq!(m1, e1);
}

#[test]
fn test_extend() {
    let mut flags;

    flags = Flags::empty();
    flags.extend([].iter().cloned());
    assert_eq!(flags, Flags::empty());

    flags = Flags::empty();
    flags.extend([Flags::A, Flags::B].iter().cloned());
    assert_eq!(flags, Flags::A | Flags::B);

    flags = Flags::A;
    flags.extend([Flags::A, Flags::B].iter().cloned());
    assert_eq!(flags, Flags::A | Flags::B);

    flags = Flags::B;
    flags.extend([Flags::A, Flags::ABC].iter().cloned());
    assert_eq!(flags, Flags::ABC);
}

#[test]
fn test_from_iterator() {
    assert_eq!([].iter().cloned().collect::<Flags>(), Flags::empty());
    assert_eq!(
        [Flags::A, Flags::B].iter().cloned().collect::<Flags>(),
        Flags::A | Flags::B
    );
    assert_eq!(
        [Flags::A, Flags::ABC].iter().cloned().collect::<Flags>(),
        Flags::ABC
    );
}

#[test]
fn test_lt() {
    let mut a = Flags::empty();
    let mut b = Flags::empty();

    assert!(!(a < b) && !(b < a));
    b = Flags::B;
    assert!(a < b);
    a = Flags::C;
    assert!(!(a < b) && b < a);
    b = Flags::C | Flags::B;
    assert!(a < b);
}

#[test]
fn test_ord() {
    let mut a = Flags::empty();
    let mut b = Flags::empty();

    assert!(a <= b && a >= b);
    a = Flags::A;
    assert!(a > b && a >= b);
    assert!(b < a && b <= a);
    b = Flags::B;
    assert!(b > a && b >= a);
    assert!(a < b && a <= b);
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn test_hash() {
    let mut x = Flags::empty();
    let mut y = Flags::empty();
    assert_eq!(hash(&x), hash(&y));
    x = Flags::all();
    y = Flags::ABC;
    assert_eq!(hash(&x), hash(&y));
}

#[test]
fn test_default() {
    assert_eq!(Flags::empty(), Flags::default());
}

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", Flags::A | Flags::B), "Flags(A | B)");
    assert_eq!(format!("{:?}", Flags::empty()), "Flags(<empty>)");
    assert_eq!(format!("{:?}", Flags::ABC), "Flags(A | B | C | ABC)");
    let extra = Flags::from_bits_preserve(0xb8);
    assert_eq!(format!("{:?}", extra), "Flags(0xb8)");
    assert_eq!(format!("{:?}", Flags::A | extra), "Flags(A | 0xb8)");

    assert_eq!(
        format!("{:?}", Flags::ABC | extra),
        "Flags(A | B | C | ABC | 0xb8)"
    );

    assert_eq!(format!("{:?}", EmptyFlags::empty()), "EmptyFlags(<empty>)");
}

#[test]
fn test_binary() {
    assert_eq!(format!("{:b}", Flags::ABC), "111");
    assert_eq!(format!("{:#b}", Flags::ABC), "0b111");
    let extra = Flags::from_bits_preserve(0b1010000);
    assert_eq!(format!("{:b}", Flags::ABC | extra), "1010111");
    assert_eq!(format!("{:#b}", Flags::ABC | extra), "0b1010111");
}

#[test]
fn test_octal() {
    assert_eq!(format!("{:o}", Flags::A), "177777");
    assert_eq!(format!("{:#o}", Flags::A), "0o177777");
    let extra = Flags::from_bits_preserve(0o5000000);
    assert_eq!(format!("{:o}", Flags::A | extra), "5177777");
    assert_eq!(format!("{:#o}", Flags::A | extra), "0o5177777");
}

#[test]
fn test_lowerhex() {
    assert_eq!(format!("{:x}", Flags::A), "ffff");
    assert_eq!(format!("{:#x}", Flags::A), "0xffff");
    let extra = Flags::from_bits_preserve(0xe00000);
    assert_eq!(format!("{:x}", Flags::A | extra), "e0ffff");
    assert_eq!(format!("{:#x}", Flags::A | extra), "0xe0ffff");
}

#[test]
fn test_upperhex() {
    assert_eq!(format!("{:X}", Flags::A), "FFFF");
    assert_eq!(format!("{:#X}", Flags::A), "0xFFFF");
    let extra = Flags::from_bits_preserve(0xe00000);
    assert_eq!(format!("{:X}", Flags::A | extra), "E0FFFF");
    assert_eq!(format!("{:#X}", Flags::A | extra), "0xE0FFFF");
}
