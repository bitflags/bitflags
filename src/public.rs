//! Generate the user-facing flags type.
//!
//! The code here belongs to the end-user, so new trait implementations and methods can't be
//! added without potentially breaking users.

/// Declare the user-facing bitflags struct.
///
/// This type is guaranteed to be a newtype with a `bitflags`-facing type as its single field.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __declare_public_bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident;
    ) => {
        $(#[$outer])*
        $vis struct $BitFlags(<Self as $crate::__private::PublicFlags>::Internal);
    };
}

/// Implement functions on the public (user-facing) bitflags type.
///
/// We need to be careful about adding new methods and trait implementations here because they
/// could conflict with items added by the end-user.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_public_bitflags {
    (
        $PublicBitFlags:ident: $T:ty, $InternalBitFlags:ident, $Iter:ident, $IterNames:ident;
    ) => {
        impl $crate::__private::core::fmt::Binary for $PublicBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::Binary::fmt(&self.0, f)
            }
        }

        impl $crate::__private::core::fmt::Octal for $PublicBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::Octal::fmt(&self.0, f)
            }
        }

        impl $crate::__private::core::fmt::LowerHex for $PublicBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl $crate::__private::core::fmt::UpperHex for $PublicBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl $PublicBitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub const fn empty() -> Self {
                Self($InternalBitFlags::empty())
            }

            /// Returns the set containing all flags.
            #[inline]
            pub const fn all() -> Self {
                Self($InternalBitFlags::all())
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub const fn bits(&self) -> $T {
                self.0.bits()
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub const fn from_bits(bits: $T) -> $crate::__private::core::option::Option<Self> {
                match $InternalBitFlags::from_bits(bits) {
                    $crate::__private::core::option::Option::Some(bits) => $crate::__private::core::option::Option::Some(Self(bits)),
                    $crate::__private::core::option::Option::None => $crate::__private::core::option::Option::None,
                }
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub const fn from_bits_truncate(bits: $T) -> Self {
                Self($InternalBitFlags::from_bits_truncate(bits))
            }

            /// Convert from underlying bit representation, preserving all
            /// bits (even those not corresponding to a defined flag).
            #[inline]
            pub const fn from_bits_retain(bits: $T) -> Self {
                Self($InternalBitFlags::from_bits_retain(bits))
            }

            /// Get the value for a flag from its stringified name.
            ///
            /// Names are _case-sensitive_, so must correspond exactly to
            /// the identifier given to the flag.
            #[inline]
            pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> {
                match $InternalBitFlags::from_name(name) {
                    $crate::__private::core::option::Option::Some(bits) => $crate::__private::core::option::Option::Some(Self(bits)),
                    $crate::__private::core::option::Option::None => $crate::__private::core::option::Option::None,
                }
            }

            /// Iterate over enabled flag values.
            #[inline]
            pub const fn iter(&self) -> $Iter {
                self.0.iter()
            }

            /// Iterate over enabled flag values with their stringified names.
            #[inline]
            pub const fn iter_names(&self) -> $IterNames {
                self.0.iter_names()
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                self.0.is_all()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                self.0.intersects(other.0)
            }

            /// Returns `true` if all of the flags in `other` are contained within `self`.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                self.0.contains(other.0)
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.0.insert(other.0)
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.0.remove(other.0)
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0.toggle(other.0)
            }

            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                self.0.set(other.0, value)
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
                Self(self.0.intersection(other.0))
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
                Self(self.0.union(other.0))
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
                Self(self.0.difference(other.0))
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
                Self(self.0.symmetric_difference(other.0))
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
                Self(self.0.complement())
            }
        }

        impl $crate::__private::core::ops::BitOr for $PublicBitFlags {
            type Output = Self;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $PublicBitFlags) -> Self {
                self.union(other)
            }
        }

        impl $crate::__private::core::ops::BitOrAssign for $PublicBitFlags {
            /// Adds the set of flags.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.0 = self.0.union(other.0);
            }
        }

        impl $crate::__private::core::ops::BitXor for $PublicBitFlags {
            type Output = Self;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }

        impl $crate::__private::core::ops::BitXorAssign for $PublicBitFlags {
            /// Toggles the set of flags.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.0 = self.0.symmetric_difference(other.0);
            }
        }

        impl $crate::__private::core::ops::BitAnd for $PublicBitFlags {
            type Output = Self;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }

        impl $crate::__private::core::ops::BitAndAssign for $PublicBitFlags {
            /// Disables all flags disabled in the set.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.0 = self.0.intersection(other.0);
            }
        }

        impl $crate::__private::core::ops::Sub for $PublicBitFlags {
            type Output = Self;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }

        impl $crate::__private::core::ops::SubAssign for $PublicBitFlags {
            /// Disables all flags enabled in the set.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.difference(other.0);
            }
        }

        impl $crate::__private::core::ops::Not for $PublicBitFlags {
            type Output = Self;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }

        impl $crate::__private::core::iter::Extend<$PublicBitFlags> for $PublicBitFlags {
            fn extend<T: $crate::__private::core::iter::IntoIterator<Item=Self>>(&mut self, iterator: T) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }

        impl $crate::__private::core::iter::FromIterator<$PublicBitFlags> for $PublicBitFlags {
            fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item=Self>>(iterator: T) -> Self {
                use $crate::__private::core::iter::Extend;

                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }

        impl $crate::__private::core::iter::IntoIterator for $PublicBitFlags {
            type Item = Self;
            type IntoIter = $Iter;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl $crate::BitFlags for $PublicBitFlags {
            type Bits = $T;

            type Iter = $Iter;
            type IterNames = $IterNames;

            fn empty() -> Self {
                $PublicBitFlags::empty()
            }

            fn all() -> Self {
                $PublicBitFlags::all()
            }

            fn bits(&self) -> $T {
                $PublicBitFlags::bits(self)
            }

            fn from_bits(bits: $T) -> $crate::__private::core::option::Option<$PublicBitFlags> {
                $PublicBitFlags::from_bits(bits)
            }

            fn from_bits_truncate(bits: $T) -> $PublicBitFlags {
                $PublicBitFlags::from_bits_truncate(bits)
            }

            fn from_bits_retain(bits: $T) -> $PublicBitFlags {
                $PublicBitFlags::from_bits_retain(bits)
            }

            fn from_name(name: &str) -> $crate::__private::core::option::Option<$PublicBitFlags> {
                $PublicBitFlags::from_name(name)
            }

            fn iter(&self) -> Self::Iter {
                $PublicBitFlags::iter(self)
            }

            fn iter_names(&self) -> Self::IterNames {
                $PublicBitFlags::iter_names(self)
            }

            fn is_empty(&self) -> bool {
                $PublicBitFlags::is_empty(self)
            }

            fn is_all(&self) -> bool {
                $PublicBitFlags::is_all(self)
            }

            fn intersects(&self, other: $PublicBitFlags) -> bool {
                $PublicBitFlags::intersects(self, other)
            }

            fn contains(&self, other: $PublicBitFlags) -> bool {
                $PublicBitFlags::contains(self, other)
            }

            fn insert(&mut self, other: $PublicBitFlags) {
                $PublicBitFlags::insert(self, other)
            }

            fn remove(&mut self, other: $PublicBitFlags) {
                $PublicBitFlags::remove(self, other)
            }

            fn toggle(&mut self, other: $PublicBitFlags) {
                $PublicBitFlags::toggle(self, other)
            }

            fn set(&mut self, other: $PublicBitFlags, value: bool) {
                $PublicBitFlags::set(self, other, value)
            }
        }

        impl $crate::__private::ImplementedByBitFlagsMacro for $PublicBitFlags {}
    };
}

/// Implement constants on the public (user-facing) bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_public_bitflags_consts {
    (
        $PublicBitFlags:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => {
        impl $PublicBitFlags {
            $(
                $(#[$attr $($args)*])*
                pub const $Flag: Self = Self::from_bits_retain($value);
            )*
        }
    };
}
