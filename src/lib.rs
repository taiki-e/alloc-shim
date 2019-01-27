//! A shim crate for to import items of alloc crate ergonomically.
//!
//! [Examples](https://github.com/taiki-e/alloc-shim/tree/master/examples)
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! alloc-shim = { version = "0.1" }
//! ```
//!
//! Set the features so that `std` depends on `alloc-shim/std`, and `alloc` depends on `alloc-shim/alloc`:
//!
//! ```toml
//! [features]
//! std = ["alloc-shim/std"]
//! alloc = ["alloc-shim/alloc"]
//! ```
//!
//! Add this to your crate root (`lib.rs` or `main.rs`):
//!
//! ```rust
//! #![cfg_attr(feature = "alloc", feature(alloc))]
//! ```
//!
//! Now, you can use alloc-shim:
//!
//! ```rust
//! use alloc::prelude::v1::*; // And more...
//! ```
//!
//! The current version of alloc-shim requires Rust 1.31 or later.
//!

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.1.0")]
#![cfg_attr(
    all(feature = "alloc", not(feature = "std")),
    feature(alloc, futures_api)
)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as liballoc;

#[cfg(not(feature = "std"))]
mod shim {
    // str: https://doc.rust-lang.org/nightly/core/str/index.html vs https://doc.rust-lang.org/nightly/alloc/str/index.html
    pub use core::{str, *};
    #[cfg(feature = "alloc")]
    pub use liballoc::{alloc, borrow, fmt, slice, task, *};

    /// The alloc Prelude
    pub mod prelude {
        /// The alloc Prelude
        pub mod v1 {
            pub use core::prelude::v1::*;
            #[cfg(feature = "alloc")]
            pub use liballoc::prelude::*;
        }
    }

    /// Synchronization primitives
    pub mod sync {
        pub use core::sync::atomic;
        #[cfg(feature = "alloc")]
        pub use liballoc::sync::*;
    }
}

#[cfg(not(feature = "std"))]
pub use self::shim::*;

#[cfg(feature = "std")]
pub use std::*;
