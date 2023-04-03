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
        $iter_names_vis:vis struct $IterNames:ident;
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $InternalBitFlags {
            bits: $T,
        }

        $iter_vis struct $Iter {
            inner: $IterNames,
            done: bool,
        }

        $iter_names_vis struct $IterNames {
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
        $InternalBitFlags:ident: $T:ty, $BitFlags:ident, $Iter:ident, $IterNames:ident {
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
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result {
                if self.is_empty() {
                    // If no flags are set then write an empty hex flag to avoid
                    // writing an empty string. In some contexts, like serialization,
                    // an empty string is preferrable, but it may be unexpected in
                    // others for a format not to produce any output.
                    //
                    // We can remove this `0x0` and remain compatible with `FromStr`,
                    // because an empty string will still parse to an empty set of flags,
                    // just like `0x0` does.
                    $crate::__private::core::write!(f, "{:#x}", <$T as $crate::__private::Bits>::EMPTY)
                } else {
                    $crate::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }

        impl $crate::__private::core::fmt::Display for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result {
                // A formatter for bitflags that produces text output like:
                //
                // A | B | 0xf6
                //
                // The names of set flags are written in a bar-separated-format,
                // followed by a hex number of any remaining bits that are set
                // but don't correspond to any flags.

                // Iterate over the valid flags
                let mut first = true;
                let mut iter = self.iter_names();
                for (name, _) in &mut iter {
                    if !first {
                        f.write_str(" | ")?;
                    }

                    first = false;
                    f.write_str(name)?;
                }

                // Append any extra bits that correspond to flags to the end of the format
                let extra_bits = iter.state.bits();
                if extra_bits != <$T as $crate::__private::Bits>::EMPTY {
                    if !first {
                        f.write_str(" | ")?;
                    }

                    $crate::__private::core::write!(f, "{:#x}", extra_bits)?;
                }

                $crate::__private::core::fmt::Result::Ok(())
            }
        }

        // The impl for `FromStr` should parse anything produced by `Display`
        impl $crate::__private::core::str::FromStr for $InternalBitFlags {
            type Err = $crate::parser::ParseError;

            fn from_str(s: &str) -> $crate::__private::core::result::Result<Self, Self::Err> {
                let s = s.trim();

                let mut parsed_flags = Self::empty();

                // If the input is empty then return an empty set of flags
                if s.is_empty() {
                    return $crate::__private::core::result::Result::Ok(parsed_flags);
                }

                for flag in s.split('|') {
                    let flag = flag.trim();

                    // If the flag is empty then we've got missing input
                    if flag.is_empty() {
                        return $crate::__private::core::result::Result::Err($crate::parser::ParseError::empty_flag());
                    }

                    // If the flag starts with `0x` then it's a hex number
                    // Parse it directly to the underlying bits type
                    let parsed_flag = if let $crate::__private::core::option::Option::Some(flag) = flag.strip_prefix("0x") {
                        let bits = <$T>::from_str_radix(flag, 16).map_err(|_| $crate::parser::ParseError::invalid_hex_flag(flag))?;

                        Self::from_bits_retain(bits)
                    }
                    // Otherwise the flag is a name
                    // The generated flags type will determine whether
                    // or not it's a valid identifier
                    else {
                        Self::from_name(flag).ok_or_else(|| $crate::parser::ParseError::invalid_named_flag(flag))?
                    };

                    parsed_flags.insert(parsed_flag);
                }

                $crate::__private::core::result::Result::Ok(parsed_flags)
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
            pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> {
                match name {
                    $(
                        $(#[$attr $($args)*])*
                        $crate::__private::core::stringify!($Flag) => $crate::__private::core::option::Option::Some(Self { bits: $BitFlags::$Flag.bits() }),
                    )*
                    _ => $crate::__private::core::option::Option::None,
                }
            }

            #[inline]
            pub const fn iter(&self) -> $Iter {
                $Iter {
                    inner: self.iter_names(),
                    done: false,
                }
            }

            #[inline]
            pub const fn iter_names(&self) -> $IterNames {
                $IterNames {
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

        impl $crate::__private::core::convert::AsRef<$T> for $InternalBitFlags {
            fn as_ref(&self) -> &$T {
                &self.bits
            }
        }

        impl $crate::__private::core::convert::From<$T> for $InternalBitFlags {
            fn from(bits: $T) -> Self {
                Self::from_bits_retain(bits)
            }
        }

        impl $crate::__private::core::iter::Iterator for $Iter {
            type Item = $BitFlags;

            fn next(&mut self) -> $crate::__private::core::option::Option<Self::Item> {
                match self.inner.next().map(|(_, value)| value) {
                    $crate::__private::core::option::Option::Some(value) => $crate::__private::core::option::Option::Some(value),
                    $crate::__private::core::option::Option::None if !self.done => {
                        self.done = true;

                        // After iterating through valid names, if there are any bits left over
                        // then return one final value that includes them. This makes `into_iter`
                        // and `from_iter` roundtrip
                        if self.inner.state != $InternalBitFlags::empty() {
                            $crate::__private::core::option::Option::Some($BitFlags::from_bits_retain(self.inner.state.bits()))
                        } else {
                            $crate::__private::core::option::Option::None
                        }
                    },
                    _ => $crate::__private::core::option::Option::None,
                }
            }
        }

        impl $crate::__private::core::iter::Iterator for $IterNames {
            type Item = (&'static str, $BitFlags);

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
                    #[allow(clippy::indexing_slicing)]
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

                            return $crate::__private::core::option::Option::Some((flag_name, $BitFlags::from_bits_retain(flag)))
                        }
                    }

                    $crate::__private::core::option::Option::None
                }
            }
        }
    };
}
