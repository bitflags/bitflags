// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A typesafe bitmask flag generator useful for sets of C-style bitmask flags.
//! It can be used for creating typesafe wrappers around C APIs.
//!
//! The `bitflags!` macro generates types that manage a set of flags. The
//! flags should only be defined for integer types, otherwise unexpected type
//! errors may occur at compile time.
//!
//! # Example
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(PartialEq)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!         const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
//!     }
//! }
//!
//! fn main() {
//!     let e1 = Flags::A | Flags::C;
//!     let e2 = Flags::B | Flags::C;
//!     assert_eq!((e1 | e2), Flags::ABC);   // union
//!     assert_eq!((e1 & e2), Flags::C);     // intersection
//!     assert_eq!((e1 - e2), Flags::A);     // set difference
//!     assert_eq!(!e2, Flags::A);           // set complement
//! }
//! ```
//!
//! See [`example_generated::Flags`](./example_generated/struct.Flags.html) for documentation of code
//! generated by the above `bitflags!` expansion.
//!
//! # Visibility
//!
//! The generated bitflags types and their associated flag constants are not exported
//! out of the current module by default. A definition can be exported out of
//! the current module by adding standard visibility modifiers:
//!
//! ```compile_fail
//! mod example {
//!     use bitflags::bitflags;
//!
//!     bitflags! {
//!         pub struct Flags1: u32 {
//!             const A = 0b00000001;
//!         }
//!
//!         struct Flags2: u32 {
//!             const B = 0b00000010;
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let flag1 = example::Flags1::A;
//!     let flag2 = example::Flags2::B; // error: const `B` is private
//! }
//! ```
//!
//! # Attributes
//!
//! Attributes can be attached to the bitflags types by placing them
//! before the `struct` keyword.
//!
//! ## Representations
//!
//! It's valid to add a `#[repr(C)]` or `#[repr(transparent)]` attribute to a bitflags type.
//! In these cases, the type is guaranteed to be a struct with a single field. The type
//! of that field has the same ABI as the specified integer type:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[repr(transparent)]
//!     struct Flags: u8 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! let int = &0b00000001u8;
//!
//! // SAFETY: This is valid because `Flags` and `u8` have the same ABI
//! let flags = unsafe { &*(int as *const u8 as *const Flags) };
//! ```
//!
//! # Trait implementations
//!
//! The following traits are automatically implemented for all bitflags types:
//!
//! - `Extend` and `FromIterator`: union
//! - `Binary`: Format bits as binary
//! - `LowerHex`: Format bits as lower hex
//! - `Octal`: Format bits as octal
//! - `UpperHex`: Format bits as upper hex
//! - `BitOr` and `BitOrAssign`: union
//! - `BitAnd` and `BitAndAssign`: intersection
//! - `BitXor` and `BitXorAssign`: toggle
//! - `Sub` and `SubAssign`: set difference
//! - `Not`: set complement
//!
//! Traits that you can derive are not automatically implemented for bitflags types.
//! These can be derived or implemented manually:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//! ```
//! 
//! They may also be implemented manually with other semantics depending on your needs.
//!
//! ## `Default`
//!
//! The `Default` trait is not automatically implemented for bitflags types.
//!
//! If your default value is equal to `0` (which is the same value as calling `empty()`),
//! you can simply derive `Default`:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     // Results in default value with bits: 0
//!     #[derive(Default)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! fn main() {
//!     let derived_default: Flags = Default::default();
//!     assert_eq!(derived_default.bits(), 0);
//! }
//! ```
//!
//! If your default value is not equal to `0` you need to implement `Default` yourself:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! // explicit `Default` implementation
//! impl Default for Flags {
//!     fn default() -> Flags {
//!         Flags::A | Flags::C
//!     }
//! }
//!
//! fn main() {
//!     let implemented_default: Flags = Default::default();
//!     assert_eq!(implemented_default, (Flags::A | Flags::C));
//! }
//! ```
//!
//! ## `Borrow`
//!
//! The `Borrow` trait is not automatically implemented for bitflags types,
//! but it can be added so long as you also derive `Eq`, `Ord`, and `Hash`:
//!
//! ```
//! use std::borrow::Borrow;
//!
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! impl Borrow<u32> for Flags {
//!     fn borrow(&self) -> &u32 {
//!         self.as_bits()
//!     }
//! }
//! ```
//!
//! Deriving the comparison and hashing traits ensures the bitflags type
//! upholds the contract of `Borrow`: that it compares and hashes the same
//! as the target type.
//!
//! # Methods
//!
//! The following methods are defined for bitflags types:
//!
//! - `empty`: an empty set of flags
//! - `all`: the set of all defined flags
//! - `bits`: the raw value of the flags currently stored
//! - `from_bits`: convert from underlying bit representation, unless that
//!                representation contains bits that do not correspond to a
//!                defined flag
//! - `from_bits_truncate`: convert from underlying bit representation, dropping
//!                         any bits that do not correspond to defined flags
//! - `from_bits_preserve`: convert from underlying bit representation, keeping
//!                          all bits (even those not corresponding to defined
//!                          flags)
//! - `is_empty`: `true` if no flags are currently stored
//! - `is_all`: `true` if currently set flags exactly equal all defined flags
//! - `intersects`: `true` if there are flags common to both `self` and `other`
//! - `contains`: `true` if all of the flags in `other` are contained within `self`
//! - `insert`: inserts the specified flags in-place
//! - `remove`: removes the specified flags in-place
//! - `toggle`: the specified flags will be inserted if not present, and removed
//!             if they are.
//! - `set`: inserts or removes the specified flags depending on the passed value
//! - `intersection`: returns a new set of flags, containing only the flags present
//!                   in both `self` and `other` (the argument to the function).
//! - `union`: returns a new set of flags, containing any flags present in
//!            either `self` or `other` (the argument to the function).
//! - `difference`: returns a new set of flags, containing all flags present in
//!                 `self` without any of the flags present in `other` (the
//!                 argument to the function).
//! - `symmetric_difference`: returns a new set of flags, containing all flags
//!                           present in either `self` or `other` (the argument
//!                           to the function), but not both.
//! - `complement`: returns a new set of flags, containing all flags which are
//!                 not set in `self`, but which are allowed for this type.
//!
//! You can define other inherent methods on bitflags types:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!     }
//! }
//!
//! impl Flags {
//!     pub fn clear(&mut self) {
//!         *self = Flags::empty();
//!     }
//! }
//! ```
//!
//! # Zero Flags
//!
//! Flags with a value equal to zero will have some strange behavior that one should be aware of.
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const NONE = 0b00000000;
//!         const SOME = 0b00000001;
//!     }
//! }
//!
//! fn main() {
//!     let empty = Flags::empty();
//!     let none = Flags::NONE;
//!     let some = Flags::SOME;
//!
//!     // Zero flags are treated as always present
//!     assert!(empty.contains(Flags::NONE));
//!     assert!(none.contains(Flags::NONE));
//!     assert!(some.contains(Flags::NONE));
//!
//!     // Zero flags will be ignored when testing for emptiness
//!     assert!(none.is_empty());
//! }
//! ```
//!
//! Users should generally avoid defining a flag with a value of zero.

#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/bitflags/1.3.2")]

#[doc(hidden)]
pub extern crate core as _core;

#[doc(hidden)]
#[cfg(feature = "serde")]
pub extern crate serde as _serde;

#[doc(inline)]
pub use bitflags_trait::BitFlags;

mod bitflags_trait;

#[doc(hidden)]
pub mod _private {
    pub use crate::bitflags_trait::{BitFlags, ImplementedByBitFlagsMacro};

    /// Used to give a name to an otherwise unreachable type.
    pub trait BitFlagsField: BitFlags {
        type Field;
    }
}

/// The macro used to generate the flag structure.
///
/// See the [crate level docs](../bitflags/index.html) for complete documentation.
///
/// # Example
///
/// ```
/// use bitflags::bitflags;
///
/// bitflags! {
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///         const C = 0b00000100;
///         const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
///     }
/// }
///
/// fn main() {
///     let e1 = Flags::A | Flags::C;
///     let e2 = Flags::B | Flags::C;
///     assert_eq!((e1 | e2), Flags::ABC);   // union
///     assert_eq!((e1 & e2), Flags::C);     // intersection
///     assert_eq!((e1 - e2), Flags::A);     // set difference
///     assert_eq!(!e2, Flags::A);           // set complement
/// }
/// ```
///
/// The generated `struct`s can also be extended with type and trait
/// implementations:
///
/// ```
/// use std::fmt;
///
/// use bitflags::bitflags;
///
/// bitflags! {
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///     }
/// }
///
/// impl Flags {
///     pub fn clear(&mut self) {
///         *self = Flags::empty();
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
///     let mut flags = Flags::A | Flags::B;
///     flags.clear();
///     assert!(flags.is_empty());
///     assert_eq!(format!("{}", flags), "hi!");
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        $vis struct $BitFlags(#[deprecated(note = "accessing the internal field directly is deprecated. Use the `bits()` / `as_bits()` methods for the raw bits value or the private `bit_field()` / `as_bit_field()` / `as_bit_field_mut()` for bitflags-specific trait implementations.")] <Self as $crate::_private::BitFlagsField>::Field);

        const _: () = {
            _impl_bitflags! {
                $BitFlags: $T {
                    $(
                        $(#[$inner $($args)*])*
                        $Flag = $value;
                    )*
                }
            }
        };

        bitflags! {
            $($t)*
        }
    };
    () => {};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! _impl_bitflags {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => {
        #[allow(dead_code, deprecated)]
        impl $BitFlags {
            $(
                $(#[$attr $($args)*])*
                pub const $Flag: Self = Self(InternalBitField($value));
            )*

            /// Returns an empty set of flags.
            #[inline]
            pub const fn empty() -> Self {
                Self(InternalBitField(0))
            }

            /// Returns the set containing all flags.
            #[inline]
            pub const fn all() -> Self {
                _impl_all_bitflags! {
                    $BitFlags: $T {
                        $(
                            $(#[$attr $($args)*])*
                            $Flag = $value;
                        )*
                    }
                }
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub const fn bits(&self) -> $T {
                self.bit_field().0
            }

            /// Returns a reference to the raw value of the flags currently stored.
            #[inline]
            pub const fn as_bits(&self) -> &$T {
                &self.as_bit_field().0
            }

            /// Returns the internal bitflags-defined field.
            #[inline]
            const fn bit_field(&self) -> InternalBitField {
                #[allow(deprecated)]
                self.0
            }

            /// Returns a reference to the bitflags-defined field.
            #[inline]
            const fn as_bit_field(&self) -> &InternalBitField {
                #[allow(deprecated)]
                &self.0
            }

            /// Returns a mutable reference to the bitflags-defined field.
            #[inline]
            fn as_bit_field_mut(&mut self) -> &mut InternalBitField {
                #[allow(deprecated)]
                &mut self.0
            }

            /// Convert from a bitflags-defined field.
            #[inline]
            const fn from_bit_field(field: InternalBitField) -> Self {
                Self(field)
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub const fn from_bits(bits: $T) -> $crate::_core::option::Option<Self> {
                if (bits & !Self::all().bits()) == 0 {
                    $crate::_core::option::Option::Some(Self::from_bits_preserve(bits))
                } else {
                    $crate::_core::option::Option::None
                }
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub const fn from_bits_truncate(bits: $T) -> Self {
                Self::from_bits_preserve(bits & Self::all().bits())
            }

            /// Convert from underlying bit representation, preserving all
            /// bits (even those not corresponding to a defined flag).
            ///
            /// # Safety
            ///
            /// The caller of the `bitflags!` macro can chose to allow or
            /// disallow extra bits for their bitflags type.
            ///
            /// The caller of `from_bits_preserve()` has to ensure that
            /// all bits correspond to a defined flag or that extra bits
            /// are valid for this bitflags type.
            #[inline]
            pub const fn from_bits_preserve(bits: $T) -> Self {
                Self(InternalBitField(bits))
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.bits() == Self::empty().bits()
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().bits() | self.bits() == self.bits()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                !(Self::from_bits_preserve(self.bits() & other.bits())).is_empty()
            }

            /// Returns `true` if all of the flags in `other` are contained within `self`.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.bits() & other.bits()) == other.bits()
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.as_bit_field_mut().0 |= other.bits();
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.as_bit_field_mut().0 &= !other.bits();
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.as_bit_field_mut().0 ^= other.bits();
            }

            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }

            /// Returns the intersection between the flags in `self` and
            /// `other`.
            ///
            /// Specifically, the returned set contains only the flags which are
            /// present in *both* `self` *and* `other`.
            ///
            /// This is equivalent to using the `&` operator (e.g.
            /// [`ops::BitAnd`]), as in `flags & other`.
            ///
            /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() & other.bits())
            }

            /// Returns the union of between the flags in `self` and `other`.
            ///
            /// Specifically, the returned set contains all flags which are
            /// present in *either* `self` *or* `other`, including any which are
            /// present in both (see [`Self::symmetric_difference`] if that
            /// is undesirable).
            ///
            /// This is equivalent to using the `|` operator (e.g.
            /// [`ops::BitOr`]), as in `flags | other`.
            ///
            /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() | other.bits())
            }

            /// Returns the difference between the flags in `self` and `other`.
            ///
            /// Specifically, the returned set contains all flags present in
            /// `self`, except for the ones present in `other`.
            ///
            /// It is also conceptually equivalent to the "bit-clear" operation:
            /// `flags & !other` (and this syntax is also supported).
            ///
            /// This is equivalent to using the `-` operator (e.g.
            /// [`ops::Sub`]), as in `flags - other`.
            ///
            /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() & !other.bits())
            }

            /// Returns the [symmetric difference][sym-diff] between the flags
            /// in `self` and `other`.
            ///
            /// Specifically, the returned set contains the flags present which
            /// are present in `self` or `other`, but that are not present in
            /// both. Equivalently, it contains the flags present in *exactly
            /// one* of the sets `self` and `other`.
            ///
            /// This is equivalent to using the `^` operator (e.g.
            /// [`ops::BitXor`]), as in `flags ^ other`.
            ///
            /// [sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
            /// [`ops::BitXor`]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() ^ other.bits())
            }

            /// Returns the complement of this set of flags.
            ///
            /// Specifically, the returned set contains all the flags which are
            /// not set in `self`, but which are allowed for this type.
            ///
            /// Alternatively, it can be thought of as the set difference
            /// between [`Self::all()`] and `self` (e.g. `Self::all() - self`)
            ///
            /// This is equivalent to using the `!` operator (e.g.
            /// [`ops::Not`]), as in `!flags`.
            ///
            /// [`Self::all()`]: Self::all
            /// [`ops::Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.bits())
            }
        }

        impl $crate::_core::fmt::Binary for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Binary::fmt(self.as_bit_field(), f)
            }
        }

        impl $crate::_core::fmt::Octal for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Octal::fmt(self.as_bit_field(), f)
            }
        }

        impl $crate::_core::fmt::LowerHex for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::LowerHex::fmt(self.as_bit_field(), f)
            }
        }

        impl $crate::_core::fmt::UpperHex for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::UpperHex::fmt(self.as_bit_field(), f)
            }
        }

        impl $crate::_core::ops::BitOr for $BitFlags {
            type Output = Self;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $BitFlags) -> Self {
                Self::from_bits_preserve(self.bits() | other.bits())
            }
        }

        impl $crate::_core::ops::BitOrAssign for $BitFlags {
            /// Adds the set of flags.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.as_bit_field_mut().0 |= other.bits();
            }
        }

        impl $crate::_core::ops::BitXor for $BitFlags {
            type Output = Self;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() ^ other.bits())
            }
        }

        impl $crate::_core::ops::BitXorAssign for $BitFlags {
            /// Toggles the set of flags.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.as_bit_field_mut().0 ^= other.bits();
            }
        }

        impl $crate::_core::ops::BitAnd for $BitFlags {
            type Output = Self;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() & other.bits())
            }
        }

        impl $crate::_core::ops::BitAndAssign for $BitFlags {
            /// Disables all flags disabled in the set.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.as_bit_field_mut().0 &= other.bits();
            }
        }

        impl $crate::_core::ops::Sub for $BitFlags {
            type Output = Self;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: Self) -> Self {
                Self::from_bits_preserve(self.bits() & !other.bits())
            }
        }

        impl $crate::_core::ops::SubAssign for $BitFlags {
            /// Disables all flags enabled in the set.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.as_bit_field_mut().0 &= !other.bits();
            }
        }

        impl $crate::_core::ops::Not for $BitFlags {
            type Output = Self;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> Self {
                Self::from_bits_preserve(!self.bits()) & Self::all()
            }
        }

        impl $crate::_core::iter::Extend<$BitFlags> for $BitFlags {
            fn extend<T: $crate::_core::iter::IntoIterator<Item=Self>>(&mut self, iterator: T) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }

        impl $crate::_core::iter::FromIterator<$BitFlags> for $BitFlags {
            fn from_iter<T: $crate::_core::iter::IntoIterator<Item=Self>>(iterator: T) -> Self {
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }

        impl $crate::BitFlags for $BitFlags {
            type Bits = $T;

            fn empty() -> Self {
                $BitFlags::empty()
            }

            fn all() -> Self {
                $BitFlags::all()
            }

            fn bits(&self) -> $T {
                $BitFlags::bits(self)
            }

            fn from_bits(bits: $T) -> $crate::_core::option::Option<$BitFlags> {
                $BitFlags::from_bits(bits)
            }

            fn from_bits_truncate(bits: $T) -> $BitFlags {
                $BitFlags::from_bits_truncate(bits)
            }

            fn from_bits_preserve(bits: $T) -> $BitFlags {
                $BitFlags::from_bits_preserve(bits)
            }

            fn is_empty(&self) -> bool {
                $BitFlags::is_empty(self)
            }

            fn is_all(&self) -> bool {
                $BitFlags::is_all(self)
            }

            fn intersects(&self, other: $BitFlags) -> bool {
                $BitFlags::intersects(self, other)
            }

            fn contains(&self, other: $BitFlags) -> bool {
                $BitFlags::contains(self, other)
            }

            fn insert(&mut self, other: $BitFlags) {
                $BitFlags::insert(self, other)
            }

            fn remove(&mut self, other: $BitFlags) {
                $BitFlags::remove(self, other)
            }

            fn toggle(&mut self, other: $BitFlags) {
                $BitFlags::toggle(self, other)
            }

            fn set(&mut self, other: $BitFlags, value: bool) {
                $BitFlags::set(self, other, value)
            }
        }

        impl $crate::_private::ImplementedByBitFlagsMacro for $BitFlags {}

        /// The underlying implementation, determined by the `bitflags` crate.
        ///
        /// This field type carries trait implementations that end-users can opt-in to
        /// using `#[derive]`. If they don't, then they're free to choose their own semantics.
        #[repr(transparent)]
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct InternalBitField($T);

        impl $crate::_private::BitFlagsField for $BitFlags {
            type Field = InternalBitField;
        }

        impl From<$T> for InternalBitField {
            fn from(value: $T) -> InternalBitField {
                InternalBitField(value)
            }
        }

        _impl_serde_bitfield! {
            $BitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag = $value;
                )*
            }
        }

        _impl_fmt_bitfield! {
            $BitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag = $value;
                )*
            }
        }
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use something else.")]
    //     ? #[doc = r"High quality documentation."]
    //     fn f() -> i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     fn f() -> i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        _impl_bitflags! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        _impl_bitflags! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        fn $($item:tt)*
    ) => {
        $(#[$filtered])*
        fn $($item)*
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // const version
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use something else.")]
    //     ? #[doc = r"High quality documentation."]
    //     const f: i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     const f: i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        const $($item:tt)*
    ) => {
        _impl_bitflags! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            const $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        const $($item:tt)*
    ) => {
        _impl_bitflags! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            const $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        const $($item:tt)*
    ) => {
        $(#[$filtered])*
        const $($item)*
    };
}

// A helper macro to implement the `all` function.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! _impl_all_bitflags {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )+
        }
    ) => {
        // See `Debug::fmt` for why this approach is taken.
        #[allow(non_snake_case)]
        trait VisitFlags {
            $(
                const $Flag: $T = 0;
            )+
        }
        #[allow(non_snake_case)]
        impl VisitFlags for $BitFlags {
            $(
                _impl_bitflags! {
                    #[allow(deprecated)]
                    $(? #[$attr $($args)*])*
                    const $Flag: $T = Self::$Flag.bits();
                }
            )+
        }

        Self::from_bits_preserve($(<Self as VisitFlags>::$Flag)|+)
    };
    (
        $BitFlags:ident: $T:ty { }
    ) => {
        Self::from_bits_preserve(0)
    };
}

// A helper macro to implement `Debug`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! _impl_fmt_bitfield {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => {
        impl $crate::_core::fmt::Debug for InternalBitField {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag
                // omission correctly. For example it needs to support:
                //
                //    #[cfg(unix)] const A: Flag = /* ... */;
                //    #[cfg(windows)] const B: Flag = /* ... */;

                // Unconditionally define a check for every flag, even disabled
                // ones.
                #[allow(non_snake_case)]
                trait VisitFlags {
                    $(
                        #[inline]
                        fn $Flag(&self) -> bool { false }
                    )*
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                #[allow(non_snake_case)]
                impl VisitFlags for $BitFlags {
                    $(
                        _impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag(&self) -> bool {
                                if Self::$Flag.bits() == 0 && self.bits() != 0 {
                                    false
                                } else {
                                    self.bits() & Self::$Flag.bits() == Self::$Flag.bits()
                                }
                            }
                        }
                    )*
                }

                let mut first = true;
                $(
                    if <$BitFlags as VisitFlags>::$Flag(&$BitFlags(*self)) {
                        if !first {
                            f.write_str(" | ")?;
                        }

                        first = false;
                        f.write_str($crate::_core::stringify!($Flag))?;
                    }
                )*

                // Handle any extra bits that don't correspond to a flag
                let extra_bits = $BitFlags(*self).bits() & !$BitFlags::all().bits();
                if extra_bits != 0 {
                    if !first {
                        f.write_str(" | ")?;
                    }

                    first = false;
                    f.write_str("0x")?;
                    $crate::_core::fmt::LowerHex::fmt(&extra_bits, f)?;
                }
                if first {
                    f.write_str("<empty>")?;
                }

                Ok(())
            }
        }

        impl $crate::_core::fmt::Binary for InternalBitField {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Binary::fmt(&self.0, f)
            }
        }

        impl $crate::_core::fmt::Octal for InternalBitField {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Octal::fmt(&self.0, f)
            }
        }

        impl $crate::_core::fmt::LowerHex for InternalBitField {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl $crate::_core::fmt::UpperHex for InternalBitField {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
    }
}

// A helper macro to implement `serde` traits.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! _impl_serde_bitfield {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => {
        impl $crate::_serde::Serialize for InternalBitField {
            fn serialize<S>(&self, serializer: S) -> $crate::_core::result::Result<S::Ok, S::Error>
            where
                S: $crate::_serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        impl<'de> $crate::_serde::Deserialize<'de> for InternalBitField {
            fn deserialize<D>(deserializer: D) -> $crate::_core::result::Result<InternalBitField, D::Error>
            where
                D: $crate::_serde::Deserializer<'de>,
            {
                let bits = $T::deserialize(deserializer)?;

                Ok(InternalBitField(bits))
            }
        }
    };
}
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! _impl_serde_bitfield {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => { };
}

#[cfg(feature = "example_generated")]
pub mod example_generated;
