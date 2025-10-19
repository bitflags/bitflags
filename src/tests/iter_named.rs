use crate::Flags;

#[test]
fn test_iter_named() {
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

    // Test all named flags produced by the iterator
    let all_named: Vec<TestFlags> = TestFlags::iter_named().collect();

    // Verify all named flags are included
    let expected_flags = vec![
        TestFlags::A,
        TestFlags::ZERO,
        TestFlags::B,
        TestFlags::C,
        TestFlags::CC, // Note: CC and C have the same bit value, but both are named flags
        TestFlags::D,
        TestFlags::ABC,
        TestFlags::AB,
        TestFlags::AC,
        TestFlags::CB,
    ];

    assert_eq!(
        all_named.len(),
        expected_flags.len(),
        "Should have 10 named flags"
    );

    // Verify each expected flag is in the result
    for expected_flag in &expected_flags {
        assert!(
            all_named.contains(expected_flag),
            "Missing flag: {:?}",
            expected_flag
        );
    }

    // Test if iterator order is consistent with definition order
    let flags_in_order: Vec<TestFlags> = TestFlags::iter_named().collect();
    assert_eq!(
        flags_in_order, expected_flags,
        "Flag order should match definition order"
    );

    // Test that iterator can be used multiple times
    let first_iteration: Vec<TestFlags> = TestFlags::iter_named().collect();
    let second_iteration: Vec<TestFlags> = TestFlags::iter_named().collect();
    assert_eq!(
        first_iteration, second_iteration,
        "Multiple iterations should produce the same result"
    );

    // Test consistency with FLAGS constant
    let flags_from_iter: std::collections::HashSet<u32> =
        TestFlags::iter_named().map(|f| f.bits()).collect();

    let flags_from_const: std::collections::HashSet<u32> = TestFlags::FLAGS
        .iter()
        .filter(|f| f.is_named())
        .map(|f| f.value().bits())
        .collect();

    assert_eq!(
        flags_from_iter, flags_from_const,
        "iter_named() should be consistent with named flags in FLAGS"
    );

    // Test flag bit values
    for flag in TestFlags::iter_named() {
        let bits = flag.bits();
        match bits {
            0b00000001 => {} // A
            0 => {}          // ZERO
            0b00000010 => {} // B
            0b00000100 => {} // C or CC (they have the same bit value)
            0b10000100 => {} // D
            0b00000111 => {} // ABC
            0b00000011 => {} // AB
            0b00000101 => {} // AC
            0b00000110 => {} // CB
            _ => panic!("Unexpected bit value: {:08b} from flag: {:?}", bits, flag),
        }
    }

    // Verify specific flag bit values
    assert_eq!(TestFlags::A.bits(), 0b00000001);
    assert_eq!(TestFlags::ZERO.bits(), 0);
    assert_eq!(TestFlags::B.bits(), 0b00000010);
    assert_eq!(TestFlags::C.bits(), 0b00000100);
    assert_eq!(TestFlags::CC.bits(), 0b00000100); // Same as C
    assert_eq!(TestFlags::D.bits(), 0b10000100);
    assert_eq!(TestFlags::ABC.bits(), 0b00000111);
    assert_eq!(TestFlags::AB.bits(), 0b00000011);
    assert_eq!(TestFlags::AC.bits(), 0b00000101);
    assert_eq!(TestFlags::CB.bits(), 0b00000110);

    // Test iterator usage with other iterator methods
    let non_zero_flags: Vec<TestFlags> =
        TestFlags::iter_named().filter(|f| !f.is_empty()).collect();
    assert_eq!(non_zero_flags.len(), 9, "Should have 9 non-zero flags");

    let single_bit_flags: Vec<TestFlags> = TestFlags::iter_named()
        .filter(|f| f.bits().count_ones() == 1)
        .collect();
    // A, B, C, CC are all single-bit flags (CC and C have same bit value but both are enumerated)
    assert_eq!(
        single_bit_flags.len(),
        4,
        "Should have 4 single-bit flags (A, B, C, CC)"
    );

    // Test composite flags (multiple bits set)
    let composite_flags: Vec<TestFlags> = TestFlags::iter_named()
        .filter(|f| f.bits().count_ones() > 1)
        .collect();
    assert_eq!(
        composite_flags.len(),
        5,
        "Should have 5 composite flags (D, ABC, AB, AC, CB)"
    );

    println!("All iter_named() tests passed!");

    // Optional: print all flags for debugging
    for flag in TestFlags::iter_named() {
        println!(
            "Flag: {:?}, bit value: {:08b} ({})",
            flag,
            flag.bits(),
            flag.bits()
        );
    }

    // Additional edge case tests

    // Test if iterator implements the Iterator trait correctly
    let mut iter = TestFlags::iter_named();
    let first = iter.next();
    assert!(
        first.is_some(),
        "Iterator should produce at least one element"
    );

    // Test size_hint (if implemented)
    let iter = TestFlags::iter_named();
    let (lower, upper) = iter.size_hint();
    println!("Iterator size_hint: lower={}, upper={:?}", lower, upper);

    // Test usage with collect
    let collected: std::collections::BTreeSet<u32> =
        TestFlags::iter_named().map(|f| f.bits()).collect();
    println!("Deduplicated bit value set: {:?}", collected);

    // Test fold and reduce operations
    let all_bits_or = TestFlags::iter_named().fold(TestFlags::empty(), |acc, flag| acc.union(flag));
    println!(
        "Union of all flags: {:?} (bit value: {:08b})",
        all_bits_or,
        all_bits_or.bits()
    );

    // Test usage with enumerate
    for (index, flag) in TestFlags::iter_named().enumerate() {
        println!("Flag #{}: {:?}", index, flag);
        if index >= 2 {
            // Only print first few to avoid excessive output
            println!("... (more flags)");
            break;
        }
    }

    // Test handling of empty flags
    let zero_flags: Vec<TestFlags> = TestFlags::iter_named().filter(|f| f.is_empty()).collect();
    assert_eq!(
        zero_flags.len(),
        1,
        "Should have only one empty flag (ZERO)"
    );
    assert_eq!(zero_flags[0], TestFlags::ZERO);

    // Test flags with duplicate bit values (C and CC)
    let c_value_flags: Vec<TestFlags> = TestFlags::iter_named()
        .filter(|f| f.bits() == 0b00000100)
        .collect();
    assert_eq!(
        c_value_flags.len(),
        2,
        "Should have two flags with the same bit value 0b00000100"
    );
}
