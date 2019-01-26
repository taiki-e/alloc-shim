#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod alloc_shim {
    #[cfg(not(feature = "std"))]
    pub use crate::alloc::*;
    #[cfg(feature = "std")]
    pub use std::{prelude::v1 as prelude, *};
}

#[test]
fn test() {
    pub use crate::alloc_shim::sync::Arc;
    pub use core::sync::atomic::AtomicPtr;
}
