#![cfg_attr(not(feature = "std"), feature(alloc))]

#[test]
fn test() {
    pub use alloc::sync::{atomic::AtomicPtr, Arc};
}
