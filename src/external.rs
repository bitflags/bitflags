//! Conditional trait implementations for external libraries.

#[cfg(feature = "serde")]
pub mod serde_support;

#[cfg(feature = "arbitrary")]
pub mod arbitrary_support;

/// Implements traits from external libraries for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_external_bitflags {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {
        // Any new library traits impls should be added here
        // Use `serde` as an example: generate code when the feature is available,
        // and a no-op when it isn't

        __impl_external_bitflags_serde! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }

        __impl_external_bitflags_arbitrary! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }
    };
}

/// Implement `Serialize` and `Deserialize` for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "serde")]
macro_rules! __impl_external_bitflags_serde {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {
        impl $crate::__private::serde::Serialize for $InternalBitFlags {
            fn serialize<S: $crate::__private::serde::Serializer>(
                &self,
                serializer: S,
            ) -> $crate::__private::core::result::Result<S::Ok, S::Error> {
                $crate::__private::serde_support::serialize_bits_default::<$InternalBitFlags, $T, S>(
                    &self,
                    serializer,
                )
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for $InternalBitFlags {
            fn deserialize<D: $crate::__private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> $crate::__private::core::result::Result<Self, D::Error> {
                $crate::__private::serde_support::deserialize_bits_default::<$InternalBitFlags, $T, D>(
                    deserializer,
                )
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
macro_rules! __impl_external_bitflags_serde {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {};
}

/// Implement `Arbitrary` for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "arbitrary")]
macro_rules! __impl_external_bitflags_arbitrary {
    (
            $InternalBitFlags:ident: $T:ty {
                $(
                    $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
            }
    ) => {
        impl<'a> $crate::__private::arbitrary::Arbitrary<'a> for $InternalBitFlags {
            fn arbitrary(
                u: &mut $crate::__private::arbitrary::Unstructured<'a>,
            ) -> $crate::__private::arbitrary::Result<Self> {
                Self::from_bits(u.arbitrary()?).ok_or_else(|| $crate::__private::arbitrary::Error::IncorrectFormat)
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "arbitrary"))]
macro_rules! __impl_external_bitflags_arbitrary {
    (
            $InternalBitFlags:ident: $T:ty {
                $(
                    $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
            }
    ) => {};
}
