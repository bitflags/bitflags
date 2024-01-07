//! Generate the user-facing atomic flags type.
//!
//! The code here belongs to the end-user, so new trait implementations and methods can't be
//! added without potentially breaking users.

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __declare_atomic_bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $AtomicBitFlags:ident : $Flags:ident
    ) => {
        $(#[$outer])*
        $vis struct $AtomicBitFlags(<<$Flags as $crate::Flags>::Bits as $crate::HasAtomic>::Atomic);

        impl $crate::AtomicFlags for $AtomicBitFlags {
            type Flags = $Flags;
            type AtomicBits = <<$Flags as $crate::Flags>::Bits as $crate::HasAtomic>::Atomic;
            
            fn atomic_bits(&self) -> &Self::AtomicBits {
                &self.0
            }
            fn from_bits_retain(bits: <Self::Flags as $crate::Flags>::Bits) -> Self {
                Self(Self::AtomicBits::from(bits))
            }
        }
    }
}
