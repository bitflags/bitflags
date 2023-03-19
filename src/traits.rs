use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A trait that is automatically implemented for all bitflags.
///
/// It should not be implemented manually.
pub trait BitFlags: ImplementedByBitFlagsMacro {
    /// The underlying integer type.
    type Bits: Bits;

    /// An iterator over enabled flags in an instance of the type.
    type Iter: Iterator<Item = Self>;

    /// An iterator over the raw names and bits for enabled flags in an instance of the type.
    type IterNames: Iterator<Item = (&'static str, Self)>;

    /// Returns an empty set of flags.
    fn empty() -> Self;

    /// Returns the set containing all flags.
    fn all() -> Self;

    /// Returns the raw value of the flags currently stored.
    fn bits(&self) -> Self::Bits;

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    fn from_bits(bits: Self::Bits) -> Option<Self>
    where
        Self: Sized;

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    fn from_bits_truncate(bits: Self::Bits) -> Self;

    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    fn from_bits_retain(bits: Self::Bits) -> Self;

    /// Get the flag for a particular name.
    fn from_name(name: &str) -> Option<Self>
    where
        Self: Sized;

    /// Iterate over enabled flag values.
    fn iter(&self) -> Self::Iter;

    /// Iterate over the raw names and bits for enabled flag values.
    fn iter_names(&self) -> Self::IterNames;

    /// Returns `true` if no flags are currently stored.
    fn is_empty(&self) -> bool;

    /// Returns `true` if all flags are currently set.
    fn is_all(&self) -> bool;

    /// Returns `true` if there are flags common to both `self` and `other`.
    fn intersects(&self, other: Self) -> bool;

    /// Returns `true` if all of the flags in `other` are contained within `self`.
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

/// A marker trait that signals that an implementation of `BitFlags` came from the `bitflags!` macro.
///
/// There's nothing stopping an end-user from implementing this trait, but we don't guarantee their
/// manual implementations won't break between non-breaking releases.
#[doc(hidden)]
pub trait ImplementedByBitFlagsMacro {}

// Not re-exported
pub trait Sealed {}

// Private implementation details
//
// The `Bits`, `PublicFlags`, and `InternalFlags` traits are implementation details of the `bitflags!`
// macro that we're free to change here. They work with the `bitflags!` macro to separate the generated
// code that belongs to end-users, and the generated code that belongs to this library.

/// A private trait that encodes the requirements of underlying bits types that can hold flags.
///
/// This trait may be made public at some future point, but it presents a compatibility hazard
/// so is left internal for now.
#[doc(hidden)]
pub trait Bits:
    Clone
    + Copy
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Not
    + Sized
    + Sealed
{
    /// The value of `Self` where no bits are set.
    const EMPTY: Self;

    /// The value of `Self` where all bits are set.
    const ALL: Self;
}

macro_rules! impl_bits {
    ($($u:ty, $i:ty,)*) => {
        $(
            impl Bits for $u {
                const EMPTY: $u = 0;
                const ALL: $u = <$u>::MAX;
            }

            impl Bits for $i {
                const EMPTY: $i = 0;
                const ALL: $i = <$u>::MAX as $i;
            }

            impl Sealed for $u {}
            impl Sealed for $i {}
        )*
    }
}

impl_bits! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize,
}

/// A trait for referencing the `bitflags`-owned internal type
/// without exposing it publicly.
pub trait PublicFlags {
    /// The type of the internal field on the generated flags type.
    type Internal;
}
