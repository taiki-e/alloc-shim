extern crate alloc as std;

#[test]
fn core_fn() {
    pub use std::prelude::v1::*;
    pub use std::sync::atomic::AtomicPtr;
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn alloc_fn() {
    pub use std::prelude::v1::*;
    pub use std::sync::{atomic::AtomicPtr, Arc};
}

#[cfg(feature = "std")]
#[test]
fn std_fn() {
    pub use std::prelude::v1::*;
    pub use std::sync::{atomic::AtomicPtr, Arc, Mutex};
}
