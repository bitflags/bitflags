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
                $crate::fmt::to_writer(&$BitFlags(*self), f)
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
                    __expr_safe_flags!(
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

            #[inline]
            pub const fn from_bits_retain(bits: $T) -> Self {
                Self { bits }
            }

            #[inline]
            pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> {
                $(
                    __expr_safe_flags!(
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

            #[inline]
            pub const fn iter(&self) -> $crate::iter::Iter<$BitFlags> {
                $crate::iter::Iter::__private_const_new(<$BitFlags as $crate::Flags>::FLAGS, $BitFlags::from_bits_retain(self.bits()), $BitFlags::from_bits_retain(self.bits()))
            }

            #[inline]
            pub const fn iter_names(&self) -> $crate::iter::IterNames<$BitFlags> {
                $crate::iter::IterNames::__private_const_new(<$BitFlags as $crate::Flags>::FLAGS, $BitFlags::from_bits_retain(self.bits()), $BitFlags::from_bits_retain(self.bits()))
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
    };
}

/// A macro that processed the input to `bitflags!` and shuffles attributes around
/// based on whether or not they're "expression-safe".
///
/// This macro is a token-tree muncher that works on 2 levels:
///
/// For each attribute, we explicitly match on its identifier, like `cfg` to determine
/// whether or not it should be considered expression-safe.
///
/// If you find yourself with an attribute that should be considered expression-safe
/// and isn't, it can be added here.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __expr_safe_flags {
    // Entrypoint: Move all flags and all attributes into `unprocessed` lists
    // where they'll be munched one-at-a-time
    (
        $(#[$inner:ident $($args:tt)*])*
        { $e:expr }
    ) => {
        __expr_safe_flags! {
            expr: { $e },
            attrs: {
                // All attributes start here
                unprocessed: [$(#[$inner $($args)*])*],
                processed: {
                    // Attributes that are safe on expressions go here
                    expr: [],
                },
            },
        }
    };
    // Process the next attribute on the current flag
    // `cfg`: The next flag should be propagated to expressions
    // NOTE: You can copy this rules block and replace `cfg` with
    // your attribute name that should be considered expression-safe
    (
        expr: { $e:expr },
            attrs: {
            unprocessed: [
                // cfg matched here
                #[cfg $($args:tt)*]
                $($attrs_rest:tt)*
            ],
            processed: {
                expr: [$($expr:tt)*],
            },
        },
    ) => {
        __expr_safe_flags! {
            expr: { $e },
            attrs: {
                unprocessed: [
                    $($attrs_rest)*
                ],
                processed: {
                    expr: [
                        $($expr)*
                        // cfg added here
                        #[cfg $($args)*]
                    ],
                },
            },
        }
    };
    // Process the next attribute on the current flag
    // `$other`: The next flag should not be propagated to expressions
    (
        expr: { $e:expr },
            attrs: {
            unprocessed: [
                // $other matched here
                #[$other:ident $($args:tt)*]
                $($attrs_rest:tt)*
            ],
            processed: {
                expr: [$($expr:tt)*],
            },
        },
    ) => {
        __expr_safe_flags! {
            expr: { $e },
                attrs: {
                unprocessed: [
                    $($attrs_rest)*
                ],
                processed: {
                    expr: [
                        // $other not added here
                        $($expr)*
                    ],
                },
            },
        }
    };
    // Once all attributes on all flags are processed, generate the actual code
    (
        expr: { $e:expr },
        attrs: {
            unprocessed: [],
            processed: {
                expr: [$(#[$expr:ident $($exprargs:tt)*])*],
            },
        },
    ) => {
        $(#[$expr $($exprargs)*])*
        { $e }
    }
}
