//! A shim crate for to import items of alloc crate ergonomically.
//!

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.1.0")]
#![cfg_attr(not(feature = "std"), feature(alloc, futures_api))]

#[cfg(not(feature = "std"))]
extern crate alloc as liballoc;

#[cfg(not(feature = "std"))]
mod shim {
    // str: https://doc.rust-lang.org/nightly/core/str/index.html vs https://doc.rust-lang.org/nightly/alloc/str/index.html
    pub use core::{str, *};
    pub use liballoc::{alloc, borrow, fmt, slice, task, *};

    /// The alloc Prelude
    pub mod prelude {
        /// The alloc Prelude
        pub mod v1 {
            pub use core::prelude::v1::*;
            pub use liballoc::prelude::*;
        }
    }

    /// Synchronization primitives
    pub mod sync {
        pub use core::sync::atomic;
        pub use liballoc::sync::*;
    }
}

#[cfg(feature = "std")]
mod shim {
    pub use std::*;
}

pub use self::shim::*;
