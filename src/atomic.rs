//! Generate the user-facing atomic flags type.
//!
//! The code here belongs to the end-user, so new trait implementations and methods can't be
//! added without potentially breaking users.

use core::sync::atomic::*;
use crate::{Bits, Flags};

/// Bits type that has an atomic variant
pub trait HasAtomic : Bits {
    type Atomic: AtomicBits<Bits=Self>;
}

/// A type that can be used as atomic storage for a flags type
pub trait AtomicBits :
    From<Self::Bits>
    + Sized
    + 'static
{
    type Bits: HasAtomic<Atomic=Self>;

    fn fetch_and(&self, val: Self::Bits, order: Ordering) -> Self::Bits;
    fn fetch_or(&self, val: Self::Bits, order: Ordering) -> Self::Bits;
    fn fetch_xor(&self, val: Self::Bits, order: Ordering) -> Self::Bits;
    fn load(&self, order: Ordering) -> Self::Bits;
    fn store(&self, val: Self::Bits, order: Ordering);
    fn swap(&self, val: Self::Bits, order: Ordering) -> Self::Bits;
}


macro_rules! impl_atomic {
    ($a1:ident $i1:ident, $a2:ident $i2:ident) => {
        impl_atomic!($a1 $i1);
        impl_atomic!($a2 $i2);
    };
    ($atomic:ident $i:ident) => {
        impl HasAtomic for $i {
            type Atomic = $atomic;
        }

        impl AtomicBits for $atomic {
            type Bits = $i;
            fn fetch_and(&self, val: Self::Bits, order: Ordering) -> Self::Bits {
                self.fetch_and(val, order)
            }
            fn fetch_or(&self, val: Self::Bits, order: Ordering) -> Self::Bits{
                self.fetch_or(val, order)
            }
            fn fetch_xor(&self, val: Self::Bits, order: Ordering) -> Self::Bits{
                self.fetch_xor(val, order)
            }
            fn load(&self, order: Ordering) -> Self::Bits{
                self.load(order)
            }
            fn store(&self, val: Self::Bits, order: Ordering){
                self.store(val, order)
            }
            fn swap(&self, val: Self::Bits, order: Ordering) -> Self::Bits{
                self.swap(val, order)
            }
        }
    };
}

impl_atomic!(AtomicU8 u8, AtomicI8 i8);
impl_atomic!(AtomicU16 u16, AtomicI16 i16);
impl_atomic!(AtomicU32 u32, AtomicI32 i32);
impl_atomic!(AtomicU64 u64, AtomicI64 i64);
impl_atomic!(AtomicUsize usize, AtomicIsize isize);

/// Wrapper to enable atomic bitflag operations
/// 
/// 
#[repr(transparent)]
pub struct Atomic<F>(<<F as Flags>::Bits as HasAtomic>::Atomic)
    where F: Flags, <F as Flags>::Bits: HasAtomic;

impl<F> From<F> for Atomic<F>
    where F: Flags, <F as Flags>::Bits: HasAtomic
{
    fn from(value: F) -> Self {
        Self(value.bits().into())
    }
}

impl<F> core::fmt::Debug for Atomic<F>
where
    F: Flags + core::fmt::Debug,
    <F as Flags>::Bits: HasAtomic
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let flags = self.load(Ordering::Relaxed);
        core::fmt::Debug::fmt(&flags, f)
    }
}

impl<F> Default for Atomic<F>
    where
        F: Flags + Default,
        <F as Flags>::Bits: HasAtomic
{
    fn default() -> Self {
        F::default().into()
    }
}

impl<F> Atomic<F>
    where F: Flags, <F as Flags>::Bits: HasAtomic
{
    pub fn empty() -> Self {
        F::empty().into()
    }
    
    pub fn all() -> Self {
        F::all().into()
    }
    
    pub fn load(&self, ordering: Ordering) -> F {
        F::from_bits_retain(self.0.load(ordering))
    }
    
    pub fn store(&self, value: F, ordering: Ordering) {
        self.0.store(value.bits(), ordering);
    }
    
    pub fn swap(&self, value: F, ordering: Ordering) -> F {
        let bits = self.0.swap(value.bits(), ordering);
        F::from_bits_retain(bits)
    }
    
    pub fn fetch_insert(&self, value: F, ordering: Ordering) -> F {
        let bits = self.0.fetch_or(value.bits(), ordering);
        F::from_bits_retain(bits)
    }
    
    pub fn fetch_remove(&self, value: F, ordering: Ordering) -> F {
        let bits = self.0.fetch_and(!value.bits(), ordering);
        F::from_bits_retain(bits)
    }
    
    pub fn fetch_toggle(&self, value: F, ordering: Ordering) -> F {
        let bits = self.0.fetch_xor(value.bits(), ordering);
        F::from_bits_retain(bits)
    }

    pub fn fetch_set(&self, val: F, set: bool, ordering: Ordering) -> F {
        if set {
            self.fetch_insert(val, ordering)
        } else {
            self.fetch_remove(val, ordering)
        }
    }
}
