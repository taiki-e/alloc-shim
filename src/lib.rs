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
//! alloc-shim = { version = "0.2.0" }
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

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.2.0")]
#![cfg_attr(feature = "alloc", feature(alloc, futures_api))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as liballoc;

#[cfg(not(feature = "std"))]
mod shim {
    // str: https://doc.rust-lang.org/nightly/core/str/index.html vs https://doc.rust-lang.org/nightly/alloc/str/index.html
    pub use core::{str, *};

    #[cfg(feature = "alloc")]
    pub use liballoc::{alloc, borrow, fmt, slice, task, *};

    // The layout in the prelude module is different for `std` and `alloc`.
    /// The alloc Prelude
    #[cfg(feature = "alloc")]
    pub mod prelude {
        /// The alloc Prelude
        pub mod v1 {
            pub use core::prelude::v1::*;
            pub use liballoc::prelude::*;
        }
    }

    // `alloc::sync` does not include `atomic` module
    /// Synchronization primitives
    #[cfg(feature = "alloc")]
    pub mod sync {
        pub use core::sync::atomic;
        pub use liballoc::sync::*;
    }
}

#[cfg(not(feature = "std"))]
pub use self::shim::*;

#[cfg(feature = "std")]
pub use std::*;
