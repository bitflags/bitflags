use core as _core;

pub trait BitFlags<T>:
    _core::ops::BitOr<Output = Self>
    + _core::ops::BitOrAssign
    + _core::ops::BitXor<Output = Self>
    + _core::ops::BitXorAssign
    + _core::ops::BitAnd<Output = Self>
    + _core::ops::BitAndAssign
    + _core::ops::Sub<Output = Self>
    + _core::ops::SubAssign
    + _core::ops::Not<Output = Self>
    + _core::iter::Extend<Self>
    + _core::iter::FromIterator<Self>
    + _core::fmt::Debug
    + _core::cmp::PartialEq<Self>
    + _core::cmp::Eq
    + _core::cmp::PartialOrd<Self>
    + _core::cmp::Ord
    + _core::hash::Hash
    + _core::fmt::Octal
    + _core::fmt::Binary
    + _core::fmt::LowerHex
    + _core::fmt::UpperHex
    + Clone
    + Copy
{
    /// Returns an empty set of flags.
    fn empty() -> Self;
    /// Returns the set containing all flags.
    fn all() -> Self;
    /// Returns the raw value of the flags currently stored.
    fn bits(&self) -> T;
    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    fn from_bits(bits: T) -> _core::option::Option<Self>;
    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    unsafe fn from_bits_truncate(bits: T) -> Self;
    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    ///
    /// # Safety
    ///
    /// The caller of the `bitflags!` macro can chose to allow or
    /// disallow extra bits for their bitflags type.
    ///
    /// The caller of `from_bits_unchecked()` has to ensure that
    /// all bits correspond to a defined flag or that extra bits
    /// are valid for this bitflags type.
    unsafe fn from_bits_unchecked(bits: T) -> Self;
    /// Returns `true` if no flags are currently stored.
    fn is_empty(&self) -> bool;
    /// Returns `true` if all flags are currently set.
    fn is_all(&self) -> bool;
    /// Returns `true` if there are flags common to both `self` and `other`.
    fn intersects(&self, other: Self) -> bool;
    /// Returns `true` all of the flags in `other` are contained within `self`.
    fn contains(&self, other: Self) -> bool;
    /// Inserts the specified flags in-place.
    fn insert(&mut self, other: Self);
    /// Removes the specified flags in-place.
    fn remove(&mut self, other: Self);
    /// Toggles the specified flags in-place.
    fn toggle(&mut self, other: Self);
    /// Inserts or removes the specified flags depending on the passed value.
    fn set(&mut self, other: Self, value: bool);
}
