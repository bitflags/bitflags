// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A typesafe bitmask flag generator.

/// The `bitflags!` macro generates a `struct` that holds a set of C-style
/// bitmask flags. It is useful for creating typesafe wrappers for C APIs.
///
/// The flags should only be defined for integer types, otherwise unexpected
/// type errors may occur at compile time.
///
/// # Example
///
/// ```{.rust}
/// #[macro_use]
/// extern crate bitflags;
///
/// bitflags! {
///     flags Flags: u32 {
///         const FLAG_A       = 0b00000001,
///         const FLAG_B       = 0b00000010,
///         const FLAG_C       = 0b00000100,
///         const FLAG_ABC     = FLAG_A.bits
///                            | FLAG_B.bits
///                            | FLAG_C.bits,
///     }
/// }
///
/// fn main() {
///     let e1 = FLAG_A | FLAG_C;
///     let e2 = FLAG_B | FLAG_C;
///     assert!((e1 | e2) == FLAG_ABC);   // union
///     assert!((e1 & e2) == FLAG_C);     // intersection
///     assert!((e1 - e2) == FLAG_A);     // set difference
///     assert!(!e2 == FLAG_A);           // set complement
/// }
/// ```
///
/// The generated `struct`s can also be extended with type and trait
/// implementations:
///
/// ```{.rust}
/// #[macro_use]
/// extern crate bitflags;
///
/// use std::fmt;
///
/// bitflags! {
///     flags Flags: u32 {
///         const FLAG_A   = 0b00000001,
///         const FLAG_B   = 0b00000010,
///     }
/// }
///
/// impl Flags {
///     pub fn clear(&mut self) {
///         self.bits = 0;  // The `bits` field can be accessed from within the
///                         // same module where the `bitflags!` macro was invoked.
///     }
/// }
///
/// impl fmt::Display for Flags {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "hi!")
///     }
/// }
///
/// fn main() {
///     let mut flags = FLAG_A | FLAG_B;
///     flags.clear();
///     assert!(flags.is_empty());
///     assert_eq!(format!("{}", flags), "hi!");
///     assert_eq!(format!("{:?}", FLAG_A | FLAG_B), "FLAG_A | FLAG_B");
///     assert_eq!(format!("{:?}", FLAG_B), "FLAG_B");
/// }
/// ```
///
/// # Attributes
///
/// Attributes can be attached to the generated `struct` by placing them
/// before the `flags` keyword.
///
/// # Trait implementations
///
/// The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash`
/// traits automatically derived for the `struct` using the `derive` attribute.
/// Additional traits can be derived by providing an explicit `derive`
/// attribute on `flags`.
///
/// The `FromIterator` trait is implemented for the `struct`, too, calculating
/// the union of the instances of the `struct` iterated over.
///
/// The `Debug` trait is also implemented by displaying the bits value of the
/// internal struct.
///
/// ## Operators
///
/// The following operator traits are implemented for the generated `struct`:
///
/// - `BitOr`: union
/// - `BitAnd`: intersection
/// - `BitXor`: toggle
/// - `Sub`: set difference
/// - `Not`: set complement
///
/// # Methods
///
/// The following methods are defined for the generated `struct`:
///
/// - `empty`: an empty set of flags
/// - `all`: the set of all flags
/// - `bits`: the raw value of the flags currently stored
/// - `from_bits`: convert from underlying bit representation, unless that
///                representation contains bits that do not correspond to a flag
/// - `from_bits_truncate`: convert from underlying bit representation, dropping
///                         any bits that do not correspond to flags
/// - `is_empty`: `true` if no flags are currently stored
/// - `is_all`: `true` if all flags are currently set
/// - `intersects`: `true` if there are flags common to both `self` and `other`
/// - `contains`: `true` all of the flags in `other` are contained within `self`
/// - `insert`: inserts the specified flags in-place
/// - `remove`: removes the specified flags in-place
/// - `toggle`: the specified flags will be inserted if not present, and removed
///             if they are.
#[macro_export]
macro_rules! bitflags {
    ($(#[$attr:meta])* flags $BitFlags:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+
    }) => {
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub struct $BitFlags {
            bits: $T,
        }

        $($(#[$Flag_attr])* pub const $Flag: $BitFlags = $BitFlags { bits: $value };)+

        impl ::std::fmt::Debug for $BitFlags {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag omission correctly.
                mod dummy {
                    // Now we define the "undefined" versions of the flags.
                    // This way, all the names exist, even if some are #[cfg]ed out.
                    $(const $Flag: super::$BitFlags = super::$BitFlags { bits: 0 };)+

                    #[inline]
                    pub fn fmt(self_: &super::$BitFlags, f: &mut ::std::fmt::Formatter)
                              -> ::std::fmt::Result {
                        // Now we import the real values for the flags.
                        // Only ones that are #[cfg]ed out will be 0.
                        use super::*;

                        let mut first = true;
                        $(
                            // $Flag.bits == 0 means that $Flag doesn't exist
                            if $Flag.bits != 0 && self_.contains($Flag) {
                                if !first {
                                    try!(f.write_str(" | "));
                                }
                                first = false;
                                try!(f.write_str(stringify!($Flag)));
                            }
                        )+
                        let _ = first;  // Silence the unused_assignments warning
                        Ok(())
                    }
                }
                dummy::fmt(self, f)
            }
        }

        impl $BitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }

            /// Returns the set containing all flags.
            #[inline]
            pub fn all() -> $BitFlags {
                // This convoluted approach is to handle #[cfg]-based flag omission correctly.
                mod dummy {
                    // Now we define the "undefined" versions of the flags.
                    // This way, all the names exist, even if some are #[cfg]ed out.
                    $(const $Flag: super::$BitFlags = super::$BitFlags { bits: 0 };)+

                    #[inline]
                    pub fn all() -> super::$BitFlags {
                        // Now we import the real values for the flags.
                        // Only ones that are #[cfg]ed out will be 0.
                        use super::*;

                        $BitFlags { bits: $($Flag.bits)|+ }
                    }
                }
                dummy::all()
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub fn bits(&self) -> $T {
                self.bits
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub fn from_bits(bits: $T) -> ::std::option::Option<$BitFlags> {
                if (bits & !$BitFlags::all().bits()) != 0 {
                    ::std::option::Option::None
                } else {
                    ::std::option::Option::Some($BitFlags { bits: bits })
                }
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub fn from_bits_truncate(bits: $T) -> $BitFlags {
                $BitFlags { bits: bits } & $BitFlags::all()
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub fn is_empty(&self) -> bool {
                *self == $BitFlags::empty()
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub fn is_all(&self) -> bool {
                *self == $BitFlags::all()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub fn intersects(&self, other: $BitFlags) -> bool {
                !(*self & other).is_empty()
            }

            /// Returns `true` all of the flags in `other` are contained within `self`.
            #[inline]
            pub fn contains(&self, other: $BitFlags) -> bool {
                (*self & other) == other
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }
        }

        impl ::std::ops::BitOr for $BitFlags {
            type Output = $BitFlags;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl ::std::ops::BitXor for $BitFlags {
            type Output = $BitFlags;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits ^ other.bits }
            }
        }

        impl ::std::ops::BitAnd for $BitFlags {
            type Output = $BitFlags;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl ::std::ops::Sub for $BitFlags {
            type Output = $BitFlags;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }

        impl ::std::ops::Not for $BitFlags {
            type Output = $BitFlags;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> $BitFlags {
                $BitFlags { bits: !self.bits } & $BitFlags::all()
            }
        }

        impl ::std::iter::FromIterator<$BitFlags> for $BitFlags {
            fn from_iter<T: ::std::iter::IntoIterator<Item=$BitFlags>>(iterator: T) -> $BitFlags {
                let mut result = Self::empty();
                for item in iterator {
                    result.insert(item)
                }
                result
            }
        }
    };
    ($(#[$attr:meta])* flags $BitFlags:ident: $T:ty {
        $($(#[$Flag_attr:meta])* const $Flag:ident = $value:expr),+,
    }) => {
        bitflags! {
            $(#[$attr])*
            flags $BitFlags: $T {
                $($(#[$Flag_attr])* const $Flag = $value),+
            }
        }
    };
}

#[cfg(test)]
#[allow(non_upper_case_globals, dead_code)]
mod tests {
    use std::hash::{SipHasher, Hash, Hasher};

    bitflags! {
        #[doc = "> The first principle is that you must not fool yourself — and"]
        #[doc = "> you are the easiest person to fool."]
        #[doc = "> "]
        #[doc = "> - Richard Feynman"]
        flags Flags: u32 {
            const FlagA       = 0b00000001,
            #[doc = "<pcwalton> macros are way better at generating code than trans is"]
            const FlagB       = 0b00000010,
            const FlagC       = 0b00000100,
            #[doc = "* cmr bed"]
            #[doc = "* strcat table"]
            #[doc = "<strcat> wait what?"]
            const FlagABC     = FlagA.bits
                               | FlagB.bits
                               | FlagC.bits,
        }
    }

    bitflags! {
        flags _CfgFlags: u32 {
            #[cfg(windows)]
            const _CfgA = 0b01,
            #[cfg(unix)]
            const _CfgB = 0b01,
            #[cfg(windows)]
            const _CfgC = _CfgA.bits | 0b10,
        }
    }

    bitflags! {
        flags AnotherSetOfFlags: i8 {
            const AnotherFlag = -1_i8,
        }
    }

    #[test]
    fn test_bits(){
        assert_eq!(Flags::empty().bits(), 0b00000000);
        assert_eq!(FlagA.bits(), 0b00000001);
        assert_eq!(FlagABC.bits(), 0b00000111);

        assert_eq!(AnotherSetOfFlags::empty().bits(), 0b00);
        assert_eq!(AnotherFlag.bits(), !0_i8);
    }

    #[test]
    fn test_from_bits() {
        assert!(Flags::from_bits(0) == Some(Flags::empty()));
        assert!(Flags::from_bits(0b1) == Some(FlagA));
        assert!(Flags::from_bits(0b10) == Some(FlagB));
        assert!(Flags::from_bits(0b11) == Some(FlagA | FlagB));
        assert!(Flags::from_bits(0b1000) == None);

        assert!(AnotherSetOfFlags::from_bits(!0_i8) == Some(AnotherFlag));
    }

    #[test]
    fn test_from_bits_truncate() {
        assert!(Flags::from_bits_truncate(0) == Flags::empty());
        assert!(Flags::from_bits_truncate(0b1) == FlagA);
        assert!(Flags::from_bits_truncate(0b10) == FlagB);
        assert!(Flags::from_bits_truncate(0b11) == (FlagA | FlagB));
        assert!(Flags::from_bits_truncate(0b1000) == Flags::empty());
        assert!(Flags::from_bits_truncate(0b1001) == FlagA);

        assert!(AnotherSetOfFlags::from_bits_truncate(0_i8) == AnotherSetOfFlags::empty());
    }

    #[test]
    fn test_is_empty(){
        assert!(Flags::empty().is_empty());
        assert!(!FlagA.is_empty());
        assert!(!FlagABC.is_empty());

        assert!(!AnotherFlag.is_empty());
    }

    #[test]
    fn test_is_all() {
        assert!(Flags::all().is_all());
        assert!(!FlagA.is_all());
        assert!(FlagABC.is_all());

        assert!(AnotherFlag.is_all());
    }

    #[test]
    fn test_two_empties_do_not_intersect() {
        let e1 = Flags::empty();
        let e2 = Flags::empty();
        assert!(!e1.intersects(e2));

        assert!(AnotherFlag.intersects(AnotherFlag));
    }

    #[test]
    fn test_empty_does_not_intersect_with_full() {
        let e1 = Flags::empty();
        let e2 = FlagABC;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_disjoint_intersects() {
        let e1 = FlagA;
        let e2 = FlagB;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_overlapping_intersects() {
        let e1 = FlagA;
        let e2 = FlagA | FlagB;
        assert!(e1.intersects(e2));
    }

    #[test]
    fn test_contains() {
        let e1 = FlagA;
        let e2 = FlagA | FlagB;
        assert!(!e1.contains(e2));
        assert!(e2.contains(e1));
        assert!(FlagABC.contains(e2));

        assert!(AnotherFlag.contains(AnotherFlag));
    }

    #[test]
    fn test_insert(){
        let mut e1 = FlagA;
        let e2 = FlagA | FlagB;
        e1.insert(e2);
        assert!(e1 == e2);

        let mut e3 = AnotherSetOfFlags::empty();
        e3.insert(AnotherFlag);
        assert!(e3 == AnotherFlag);
    }

    #[test]
    fn test_remove(){
        let mut e1 = FlagA | FlagB;
        let e2 = FlagA | FlagC;
        e1.remove(e2);
        assert!(e1 == FlagB);

        let mut e3 = AnotherFlag;
        e3.remove(AnotherFlag);
        assert!(e3 == AnotherSetOfFlags::empty());
    }

    #[test]
    fn test_operators() {
        let e1 = FlagA | FlagC;
        let e2 = FlagB | FlagC;
        assert!((e1 | e2) == FlagABC);     // union
        assert!((e1 & e2) == FlagC);       // intersection
        assert!((e1 - e2) == FlagA);       // set difference
        assert!(!e2 == FlagA);             // set complement
        assert!(e1 ^ e2 == FlagA | FlagB); // toggle
        let mut e3 = e1;
        e3.toggle(e2);
        assert!(e3 == FlagA | FlagB);

        let mut m4 = AnotherSetOfFlags::empty();
        m4.toggle(AnotherSetOfFlags::empty());
        assert!(m4 == AnotherSetOfFlags::empty());
    }

    #[test]
    fn test_from_iterator() {
        assert_eq!([].iter().cloned().collect::<Flags>(), Flags::empty());
        assert_eq!([FlagA, FlagB].iter().cloned().collect::<Flags>(), FlagA | FlagB);
        assert_eq!([FlagA, FlagABC].iter().cloned().collect::<Flags>(), FlagABC);
    }

    #[test]
    fn test_lt() {
        let mut a = Flags::empty();
        let mut b = Flags::empty();

        assert!(!(a < b) && !(b < a));
        b = FlagB;
        assert!(a < b);
        a = FlagC;
        assert!(!(a < b) && b < a);
        b = FlagC | FlagB;
        assert!(a < b);
    }

    #[test]
    fn test_ord() {
        let mut a = Flags::empty();
        let mut b = Flags::empty();

        assert!(a <= b && a >= b);
        a = FlagA;
        assert!(a > b && a >= b);
        assert!(b < a && b <= a);
        b = FlagB;
        assert!(b > a && b >= a);
        assert!(a < b && a <= b);
    }

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = SipHasher::new_with_keys(0, 0);
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_hash() {
        let mut x = Flags::empty();
        let mut y = Flags::empty();
        assert!(hash(&x) == hash(&y));
        x = Flags::all();
        y = FlagABC;
        assert!(hash(&x) == hash(&y));
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", FlagA | FlagB), "FlagA | FlagB");
        assert_eq!(format!("{:?}", FlagABC), "FlagA | FlagB | FlagC | FlagABC");
    }
}
