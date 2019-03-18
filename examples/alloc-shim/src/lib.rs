#![cfg_attr(feature = "alloc", feature(alloc))]

#[test]
fn core_fn() {
    pub use core::sync::atomic::AtomicPtr;
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn alloc_fn() {
    pub use alloc::prelude::v1::*;
    pub use alloc::sync::Arc;
    pub use core::sync::atomic::AtomicPtr;

    let _ = alloc::vec![0u8];
    let _ = alloc::format!("");
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::sync::{atomic::AtomicPtr, Arc, Mutex};
}
