use crate::BitFlags;

pub struct Iter<B: BitFlags> {
    inner: IterNames<B>,
    done: bool,
}

impl<B: BitFlags> Iter<B> {
    pub fn new(flags: &B) -> Self {
        Iter {
            inner: IterNames::new(flags),
            done: false,
        }
    }
    
    #[doc(hidden)]
    pub const fn __private_const_new(flags: &'static [(&'static str, B)], source: B, state: B) -> Self {
        Iter {
            inner: IterNames::__private_const_new(flags, source, state),
            done: false,
        }
    }
}

impl<B: BitFlags> Iterator for Iter<B> {
    type Item = B;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((_, value)) => Some(value),
            None if !self.done => {
                self.done = true;
                
                // After iterating through valid names, if there are any bits left over
                // then return one final value that includes them. This makes `into_iter`
                // and `from_iter` roundtrip
                if !self.inner.remaining().is_empty() {
                    Some(B::from_bits_retain(self.inner.state.bits()))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

pub struct IterNames<B: BitFlags> {
    flags: &'static [(&'static str, B)],
    idx: usize,
    source: B,
    state: B,
}

impl<B: BitFlags> IterNames<B> {
    pub fn new(flags: &B) -> Self {
        IterNames {
            flags: B::FLAGS,
            idx: 0,
            state: B::from_bits_retain(flags.bits()),
            source: B::from_bits_retain(flags.bits()),
        }
    }
    
    #[doc(hidden)]
    pub const fn __private_const_new(flags: &'static [(&'static str, B)], source: B, state: B) -> Self {
        IterNames {
            flags,
            idx: 0,
            state,
            source,
        }
    }
    
    pub fn remaining(&self) -> &B {
        &self.state
    }
}

impl<B: BitFlags> Iterator for IterNames<B> {
    type Item = (&'static str, B);
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((name, flag)) = self.flags.get(self.idx) {
            self.idx += 1;

            let bits = flag.bits();

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
            if self.source.contains(B::from_bits_retain(bits)) {
                self.state.remove(B::from_bits_retain(bits));

                return Some((name, B::from_bits_retain(bits)));
            }
        }
        
        None
    }
}
