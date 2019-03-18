#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

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
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::sync::{atomic::AtomicPtr, Arc, Mutex};
}
