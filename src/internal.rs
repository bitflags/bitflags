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
    ) => {
        // NOTE: The ABI of this type is _guaranteed_ to be the same as `T`
        // This is relied on by some external libraries like `bytemuck` to make
        // its `unsafe` trait impls sound.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $InternalBitFlags {
            bits: $T,
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
        $InternalBitFlags:ident: $T:ty, $BitFlags:ident {
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
                $crate::parser::to_writer(&$BitFlags(*self), f)
            }
        }

        // The impl for `FromStr` should parse anything produced by `Display`
        impl $crate::__private::core::str::FromStr for $InternalBitFlags {
            type Err = $crate::parser::ParseError;

            fn from_str(s: &str) -> $crate::__private::core::result::Result<Self, Self::Err> {
                $crate::parser::from_str::<$BitFlags>(s).map(|flags| flags.0)
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

        __impl_bitflags! {
            $InternalBitFlags: $T {
                fn empty() {
                    Self { bits: <$T as $crate::__private::Bits>::EMPTY }
                }

                fn all() {
                    Self::from_bits_truncate(<$T as $crate::__private::Bits>::ALL)
                }

                fn bits(f) {
                    f.bits
                }

                fn from_bits(bits) {
                    let truncated = Self::from_bits_truncate(bits).bits;

                    if truncated == bits {
                        $crate::__private::core::option::Option::Some(Self { bits })
                    } else {
                        $crate::__private::core::option::Option::None
                    }
                }

                fn from_bits_truncate(bits) {
                    if bits == <$T as $crate::__private::Bits>::EMPTY {
                    return Self { bits }
                }

                let mut truncated = <$T as $crate::__private::Bits>::EMPTY;

                $(
                    __bitflags_expr_safe_attrs!(
                        $(#[$attr $($args)*])*
                        {
                            if bits & $BitFlags::$Flag.bits() == $BitFlags::$Flag.bits() {
                                truncated |= $BitFlags::$Flag.bits()
                            }
                        }
                    );
                )*

                Self { bits: truncated }
                }

                fn from_bits_retain(bits) {
                    Self { bits }
                }

                fn from_name(name) {
                    $(
                        __bitflags_expr_safe_attrs!(
                            $(#[$attr $($args)*])*
                        {
                            if name == $crate::__private::core::stringify!($Flag) {
                                return $crate::__private::core::option::Option::Some(Self { bits: $BitFlags::$Flag.bits() });
                            }
                        }
                    );
                        )*

                let _ = name;
                    $crate::__private::core::option::Option::None
                }

                fn is_empty(f) {
                    f.bits == Self::empty().bits
                }

                fn is_all(f) {
                    Self::all().bits | f.bits == f.bits
                }

                fn intersects(f, other) {
                    !(Self { bits: f.bits & other.bits}).is_empty()
                }

                fn contains(f, other) {
                    (f.bits & other.bits) == other.bits
                }

                fn insert(f, other) {
                    f.bits |= other.bits;
                }

                fn remove(f, other) {
                    f.bits &= !other.bits;
                }

                fn toggle(f, other) {
                    f.bits ^= other.bits;
                }

                fn set(f, other, value) {
                    if value {
                        f.insert(other);
                    } else {
                        f.remove(other);
                    }
                }

                fn intersection(f, other) {
                    Self { bits: f.bits & other.bits }
                }

                fn union(f, other) {
                    Self { bits: f.bits | other.bits }
                }

                fn difference(f, other) {
                    Self { bits: f.bits & !other.bits }
                }

                fn symmetric_difference(f, other) {
                    Self { bits: f.bits ^ other.bits }
                }

                fn complement(f) {
                    Self::from_bits_truncate(!f.bits)
                }
            }
        }

        impl $InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut $T {
                &mut self.bits
            }

            /// Iterate over enabled flag values.
            #[inline]
            pub const fn iter(&self) -> $crate::iter::Iter<$BitFlags> {
                $crate::iter::Iter::__private_const_new(<$BitFlags as $crate::Flags>::FLAGS, $BitFlags::from_bits_retain(self.bits()), $BitFlags::from_bits_retain(self.bits()))
            }

            /// Iterate over enabled flag values with their stringified names.
            #[inline]
            pub const fn iter_names(&self) -> $crate::iter::IterNames<$BitFlags> {
                $crate::iter::IterNames::__private_const_new(<$BitFlags as $crate::Flags>::FLAGS, $BitFlags::from_bits_retain(self.bits()), $BitFlags::from_bits_retain(self.bits()))
            }
        }
    };
}
