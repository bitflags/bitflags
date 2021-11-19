use bitflags::bitflags;

// Define a flag that contains gaps to help exercise edge-cases,
// especially around "unknown" flags (e.g. ones outside of `all()`
// `from_bits_preserve`).
// - when lhs and rhs both have different sets of unknown flags.
// - unknown flags at both ends, and in the middle
// - cases with "gaps".
bitflags! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Flags: u16 {
        // Intentionally no `A`
        const B = 0b000000010;
        // Intentionally no `C`
        const D = 0b000001000;
        const E = 0b000010000;
        const F = 0b000100000;
        const G = 0b001000000;
        // Intentionally no `H`
        const I = 0b100000000;
    }
}

#[test]
fn test_set_ops_exhaustive() {
    let iter_test_flags =
        || (0..=0b111_1111_1111).map(|bits| Flags::from_bits_preserve(bits));

    for a in iter_test_flags() {
        assert_eq!(
            a.complement(),
            Flags::from_bits_truncate(!a.bits()),
            "wrong result: !({:?})",
            a,
        );
        assert_eq!(a.complement(), !a, "named != op: !({:?})", a);
        for b in iter_test_flags() {
            // Check that the named operations produce the expected bitwise
            // values.
            assert_eq!(
                a.union(b).bits(),
                a.bits() | b.bits(),
                "wrong result: `{:?}` | `{:?}`",
                a,
                b,
            );
            assert_eq!(
                a.intersection(b).bits(),
                a.bits() & b.bits(),
                "wrong result: `{:?}` & `{:?}`",
                a,
                b,
            );
            assert_eq!(
                a.symmetric_difference(b).bits(),
                a.bits() ^ b.bits(),
                "wrong result: `{:?}` ^ `{:?}`",
                a,
                b,
            );
            assert_eq!(
                a.difference(b).bits(),
                a.bits() & !b.bits(),
                "wrong result: `{:?}` - `{:?}`",
                a,
                b,
            );
            // Note: Difference is checked as both `a - b` and `b - a`
            assert_eq!(
                b.difference(a).bits(),
                b.bits() & !a.bits(),
                "wrong result: `{:?}` - `{:?}`",
                b,
                a,
            );
            // Check that the named set operations are equivalent to the
            // bitwise equivalents
            assert_eq!(a.union(b), a | b, "named != op: `{:?}` | `{:?}`", a, b,);
            assert_eq!(
                a.intersection(b),
                a & b,
                "named != op: `{:?}` & `{:?}`",
                a,
                b,
            );
            assert_eq!(
                a.symmetric_difference(b),
                a ^ b,
                "named != op: `{:?}` ^ `{:?}`",
                a,
                b,
            );
            assert_eq!(a.difference(b), a - b, "named != op: `{:?}` - `{:?}`", a, b,);
            // Note: Difference is checked as both `a - b` and `b - a`
            assert_eq!(b.difference(a), b - a, "named != op: `{:?}` - `{:?}`", b, a,);
            // Verify that the operations which should be symmetric are
            // actually symmetric.
            assert_eq!(a.union(b), b.union(a), "asymmetry: `{:?}` | `{:?}`", a, b,);
            assert_eq!(
                a.intersection(b),
                b.intersection(a),
                "asymmetry: `{:?}` & `{:?}`",
                a,
                b,
            );
            assert_eq!(
                a.symmetric_difference(b),
                b.symmetric_difference(a),
                "asymmetry: `{:?}` ^ `{:?}`",
                a,
                b,
            );
        }
    }
}
