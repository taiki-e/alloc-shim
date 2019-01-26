#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(any(feature = "alloc", feature = "std"))]
mod alloc_shim {
    #[cfg(not(feature = "std"))]
    pub use crate::alloc::*;
    #[cfg(feature = "std")]
    pub use std::{prelude::v1 as prelude, *};
}

#[test]
fn core_fn() {
    pub use core::sync::atomic::AtomicPtr;
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn alloc_fn() {
    pub use crate::alloc_shim::sync::Arc;
    pub use core::sync::atomic::AtomicPtr;
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::sync::{atomic::AtomicPtr, Arc};
}
