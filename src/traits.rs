use core::{ops::{BitAnd, BitOr, BitXor, Not}, fmt::LowerHex};

use crate::parser::{ParseError, FromHex};

/// A set of flags.
///
/// This trait is automatically implemented for flags types defined using the `bitflags!` macro.
/// It can also be implemented manually for custom flags types.
pub trait BitFlags: Sized + 'static {
    /// The set of available flags and their names.
    const FLAGS: &'static [(&'static str, Self)];

    /// The underlying storage type.
    type Bits: Bits;

    /// An iterator over enabled flags in an instance of the type.
    type Iter: Iterator<Item = Self>;

    /// An iterator over the raw names and bits for enabled flags in an instance of the type.
    type IterNames: Iterator<Item = (&'static str, Self)>;

    /// Returns an empty set of flags.
    fn empty() -> Self {
        Self::from_bits_retain(Self::Bits::EMPTY)
    }

    /// Returns the set containing all flags.
    fn all() -> Self {
        Self::from_bits_retain(Self::Bits::ALL)
    }

    /// Returns the raw value of the flags currently stored.
    fn bits(&self) -> Self::Bits;

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    ///
    /// Note that each [multi-bit flag] is treated as a unit for this comparison.
    ///
    /// [multi-bit flag]: index.html#multi-bit-flags
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        let truncated = Self::from_bits_truncate(bits);

        if truncated.bits() == bits {
            Some(truncated)
        } else {
            None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    ///
    /// Note that each [multi-bit flag] is treated as a unit for this comparison.
    ///
    /// [multi-bit flag]: index.html#multi-bit-flags
    fn from_bits_truncate(bits: Self::Bits) -> Self {
        if bits == Self::Bits::EMPTY {
            return Self::empty();
        }

        let mut truncated = Self::Bits::EMPTY;

        for (_, flag) in Self::FLAGS.iter() {
            if bits & flag.bits() == flag.bits() {
                truncated = truncated | flag.bits();
            }
        }

        Self::from_bits_retain(truncated)
    }

    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    fn from_bits_retain(bits: Self::Bits) -> Self;

    /// Get the flag for a particular name.
    fn from_name(name: &str) -> Option<Self> {
        for (known, flag) in Self::FLAGS {
            if *known == name {
                return Some(Self::from_bits_retain(flag.bits()))
            }
        }

        None
    }

    /// Iterate over enabled flag values.
    fn iter(&self) -> Self::Iter;

    /// Iterate over the raw names and bits for enabled flag values.
    fn iter_names(&self) -> Self::IterNames;

    /// Returns `true` if no flags are currently stored.
    fn is_empty(&self) -> bool {
        self.bits() == Self::Bits::EMPTY
    }

    /// Returns `true` if all flags are currently set.
    fn is_all(&self) -> bool {
        // NOTE: We check against `Self::all` here, not `Self::Bits::ALL`
        // because the set of all flags may not use all bits
        Self::all().bits() | self.bits() == self.bits()
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    fn intersects(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() != Self::Bits::EMPTY
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    fn contains(&self, other: Self) -> bool
    where
        Self: Sized,
    {
        self.bits() & other.bits() == other.bits()
    }

    /// Inserts the specified flags in-place.
    fn insert(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits() | other.bits());
    }

    /// Removes the specified flags in-place.
    fn remove(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits() & !other.bits());
    }

    /// Toggles the specified flags in-place.
    fn toggle(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = Self::from_bits_retain(self.bits() ^ other.bits());
    }

    /// Inserts or removes the specified flags depending on the passed value.
    fn set(&mut self, other: Self, value: bool)
    where
        Self: Sized,
    {
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
    #[must_use]
    fn intersection(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & other.bits())
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both (see [`Self::symmetric_difference`] if that
    /// is undesirable).
    #[must_use]
    fn union(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() | other.bits())
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    #[must_use]
    fn difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & !other.bits())
    }

    /// Returns the [symmetric difference][sym-diff] between the flags
    /// in `self` and `other`.
    ///
    /// Specifically, the returned set contains the flags present which
    /// are present in `self` or `other`, but that are not present in
    /// both. Equivalently, it contains the flags present in *exactly
    /// one* of the sets `self` and `other`.
    ///
    /// [sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[must_use]
    fn symmetric_difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() ^ other.bits())
    }

    /// Returns the complement of this set of flags.
    ///
    /// Specifically, the returned set contains all the flags which are
    /// not set in `self`, but which are allowed for this type.
    #[must_use]
    fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}

/// Underlying storage for a flags type.
pub trait Bits:
    Clone
    + Copy
    + PartialEq
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + LowerHex
    + FromHex
    + Sized
    + 'static
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

            impl FromHex for $u {
                fn from_hex(input: &str) -> Result<Self, ParseError> {
                    <$u>::from_str_radix(input, 16).map_err(|_| ParseError::invalid_hex_flag(input))
                }
            }

            impl FromHex for $i {
                fn from_hex(input: &str) -> Result<Self, ParseError> {
                    <$i>::from_str_radix(input, 16).map_err(|_| ParseError::invalid_hex_flag(input))
                }
            }
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
