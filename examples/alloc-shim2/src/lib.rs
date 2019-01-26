#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]

#[cfg(feature = "alloc")]
#[allow(unused_extern_crates)]
extern crate alloc_shim as alloc;

#[test]
fn core_fn() {
    pub use core::sync::atomic::AtomicPtr;
}

#[cfg(feature = "alloc")]
#[test]
fn alloc_fn() {
    pub use alloc::sync::{atomic::AtomicPtr, Arc};
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::sync::{atomic::AtomicPtr, Arc};
}
