//! Generate the internal `bitflags`-facing flags type.
//!
//! The code generated here is owned by `bitflags`, but still part of its public API.
//! Changes to the types generated here need to be considered like any other public API change.

/// Declare the `bitflags`-facing bitflags struct.
///
/// This type is part of the `bitflags` crate's public API, but not part of the user's.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __declare_internal_bitflags {
    (
        $vis:vis struct $InternalBitFlags:ident: $T:ty;
        $iter_vis:vis struct $Iter:ident;
        $iter_raw_vis:vis struct $IterRaw:ident;
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $InternalBitFlags {
            bits: $T,
        }

        $iter_vis struct $Iter($IterRaw);

        $iter_raw_vis struct $IterRaw {
            idx: usize,
            source: $InternalBitFlags,
            state: $InternalBitFlags,
        }
    };
}

/// Implement functions on the private (bitflags-facing) bitflags type.
///
/// Methods and trait implementations can be freely added here without breaking end-users.
/// If we want to expose new functionality to `#[derive]`, this is the place to do it.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_internal_bitflags {
    (
        $InternalBitFlags:ident: $T:ty, $BitFlags:ident, $Iter:ident, $IterRaw:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {
        impl $crate::__private::PublicFlags for $BitFlags {
            type Internal = $InternalBitFlags;
        }

        impl $crate::__private::core::default::Default for $InternalBitFlags {
            #[inline]
            fn default() -> Self {
                $InternalBitFlags::empty()
            }
        }

        impl $crate::__private::core::fmt::Debug for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                // Iterate over the valid flags
                let mut first = true;
                for (name, _) in self.iter_raw() {
                    if !first {
                        f.write_str(" | ")?;
                    }

                    first = false;
                    f.write_str(name)?;
                }

                // Append any extra bits that correspond to flags to the end of the format
                let extra_bits = self.bits & !Self::all().bits;

                if extra_bits != <$T as $crate::__private::Bits>::EMPTY {
                    if !first {
                        f.write_str(" | ")?;
                    }
                    first = false;
                    $crate::__private::core::write!(f, "{:#x}", extra_bits)?;
                }

                if first {
                    f.write_str("empty")?;
                }

                $crate::__private::core::fmt::Result::Ok(())
            }
        }

        impl $crate::__private::core::fmt::Binary for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::Binary::fmt(&self.bits(), f)
            }
        }

        impl $crate::__private::core::fmt::Octal for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::Octal::fmt(&self.bits(), f)
            }
        }

        impl $crate::__private::core::fmt::LowerHex for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::LowerHex::fmt(&self.bits(), f)
            }
        }

        impl $crate::__private::core::fmt::UpperHex for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result {
                $crate::__private::core::fmt::UpperHex::fmt(&self.bits(), f)
            }
        }

        impl $InternalBitFlags {
            #[inline]
            pub const fn empty() -> Self {
                Self { bits: <$T as $crate::__private::Bits>::EMPTY }
            }

            #[inline]
            pub const fn all() -> Self {
                Self::from_bits_truncate(<$T as $crate::__private::Bits>::ALL)
            }

            #[inline]
            pub const fn bits(&self) -> $T {
                self.bits
            }

            #[inline]
            pub fn bits_mut(&mut self) -> &mut $T {
                &mut self.bits
            }

            #[inline]
            pub const fn from_bits(bits: $T) -> $crate::__private::core::option::Option<Self> {
                let truncated = Self::from_bits_truncate(bits).bits;

                if truncated == bits {
                    $crate::__private::core::option::Option::Some(Self { bits })
                } else {
                    $crate::__private::core::option::Option::None
                }
            }

            #[inline]
            pub const fn from_bits_truncate(bits: $T) -> Self {
                if bits == <$T as $crate::__private::Bits>::EMPTY {
                    return Self { bits }
                }

                let mut truncated = <$T as $crate::__private::Bits>::EMPTY;

                $(
                    $(#[$attr $($args)*])*
                    if bits & $BitFlags::$Flag.bits() == $BitFlags::$Flag.bits() {
                        truncated |= $BitFlags::$Flag.bits()
                    }
                )*

                Self { bits: truncated }
            }

            #[inline]
            pub const fn from_bits_retain(bits: $T) -> Self {
                Self { bits }
            }

            #[inline]
            pub const fn iter(&self) -> $Iter {
                $Iter(self.iter_raw())
            }

            #[inline]
            pub const fn iter_raw(&self) -> $IterRaw {
                $IterRaw {
                    idx: 0,
                    source: *self,
                    state: *self,
                }
            }

            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.bits == Self::empty().bits
            }

            #[inline]
            pub const fn is_all(&self) -> bool {
                Self::all().bits | self.bits == self.bits
            }

            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                !(Self { bits: self.bits & other.bits}).is_empty()
            }

            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            #[inline]
            pub fn insert(&mut self, other: Self) {
                self.bits |= other.bits;
            }

            #[inline]
            pub fn remove(&mut self, other: Self) {
                self.bits &= !other.bits;
            }

            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.bits ^= other.bits;
            }

            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }

            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }

            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }

            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }

            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }

            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                Self::from_bits_truncate(!self.bits)
            }
        }

        impl $crate::__private::core::iter::Iterator for $Iter {
            type Item = $BitFlags;

            fn next(&mut self) -> $crate::__private::core::option::Option<Self::Item> {
                self.0.next().map(|(_, value)| $BitFlags::from_bits_retain(value))
            }
        }

        impl $crate::__private::core::iter::Iterator for $IterRaw {
            type Item = (&'static str, $T);

            fn next(&mut self) -> $crate::__private::core::option::Option<Self::Item> {
                const NUM_FLAGS: usize = {
                    let mut num_flags = 0;

                    $(
                        $(#[$attr $($args)*])*
                        {
                            num_flags += 1;
                        }
                    )*

                    num_flags
                };

                const OPTIONS: [$T; NUM_FLAGS] = [
                    $(
                        $(#[$attr $($args)*])*
                        $BitFlags::$Flag.bits(),
                    )*
                ];

                const OPTIONS_NAMES: [&'static str; NUM_FLAGS] = [
                    $(
                        $(#[$attr $($args)*])*
                        $crate::__private::core::stringify!($Flag),
                    )*
                ];

                if self.state.is_empty() || NUM_FLAGS == 0 {
                    $crate::__private::core::option::Option::None
                } else {
                    for (flag, flag_name) in OPTIONS[self.idx..NUM_FLAGS].iter().copied()
                        .zip(OPTIONS_NAMES[self.idx..NUM_FLAGS].iter().copied())
                    {
                        self.idx += 1;

                        // NOTE: We check whether the flag exists in self, but remove it from
                        // a different value. This ensure that overlapping flags are handled
                        // properly. Take the following example:
                        //
                        // const A: 0b00000001;
                        // const B: 0b00000101;
                        //
                        // Given the bits 0b00000101, both A and B are set. But if we removed A
                        // as we encountered it we'd be left with 0b00000100, which doesn't
                        // correspond to a valid flag on its own.
                        if self.source.contains($InternalBitFlags { bits: flag }) {
                            self.state.remove($InternalBitFlags { bits: flag });

                            return $crate::__private::core::option::Option::Some((flag_name, flag))
                        }
                    }

                    $crate::__private::core::option::Option::None
                }
            }
        }
    };
}