use core::{sync::atomic::Ordering, fmt::Debug};

use super::*;

use crate::{Atomic, Flags, HasAtomic};

#[test]
fn cases() {
    case(
        TestFlags::ABC,
        &[
            (&|a| a.load(Ordering::Relaxed),TestFlags::ABC),
        ],
        TestFlags::ABC
        );
    case(
        TestFlags::A,
        &[
            (&|a| a.swap(TestFlags::B, Ordering::Relaxed),TestFlags::A),
            (&|a| a.swap(TestFlags::C, Ordering::Relaxed),TestFlags::B),
        ],
        TestFlags::C
        );
    case(
        TestFlags::empty(),
        &[
            (&|a| a.fetch_insert(TestFlags::A, Ordering::Relaxed), TestFlags::empty()),
            (&|a| a.fetch_toggle(TestFlags::ABC, Ordering::Relaxed), TestFlags::A),
            (&|a| a.fetch_remove(TestFlags::C, Ordering::Relaxed), TestFlags::B | TestFlags::C),
        ],
        TestFlags::B
        );
}

#[track_caller]
fn case<F: Flags>(
    init: F, 
    ops: &[(&dyn Fn(&Atomic<F>) -> F, F)],
    final_res: F
)
where
    F: Flags + Debug + Copy + PartialEq,
    <F as Flags>::Bits: HasAtomic,
{
    let atomic = Atomic::from(init);

    for (op, op_res) in ops {
        let op_val = op(&atomic);
        assert_eq!(&op_val, op_res, "expected={op_res:?} got={op_val:?}");
    }
    
    let final_val = atomic.load(Ordering::Relaxed);
    assert_eq!(final_val, final_res, "expected={final_res:?} got={final_val:?}");
}