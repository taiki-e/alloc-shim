#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]

#[cfg(all(feature = "alloc", not(any(feature = "std", feature = "nightly"))))]
compile_error!("The `alloc` feature without `std` requires the `nightly` feature active to explicitly opt-in to unstable features");

#[test]
fn core_fn() {
    pub use core::sync::atomic::AtomicPtr;
}

#[cfg(feature = "alloc")]
#[test]
fn alloc_fn() {
    pub use alloc::prelude::*;
    pub use alloc::sync::Arc;

    let _ = alloc::vec![0u8];
    let _ = alloc::format!("");
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::sync::{atomic::AtomicPtr, Arc, Mutex};
}
