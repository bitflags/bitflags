#[doc(hidden)]
pub trait ImplementedByBitFlagsMacro {}

/// A trait that is automatically implemented for all bitflags.
///
/// It should not be implemented manually.
pub trait BitFlags: ImplementedByBitFlagsMacro {
    type Bits;
    const NUM_FLAGS: usize;

    /// Returns an empty set of flags.
    fn empty() -> Self;
    /// Returns the set containing all flags.
    fn all() -> Self;
    /// Returns the raw value of the flags currently stored.
    fn bits(&self) -> Self::Bits;
    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    fn from_bits(bits: Self::Bits) -> Option<Self>
    where Self: Sized;
    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    fn from_bits_truncate(bits: Self::Bits) -> Self;
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
    unsafe fn from_bits_unchecked(bits: Self::Bits) -> Self;
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
