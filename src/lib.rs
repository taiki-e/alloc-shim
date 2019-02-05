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
//! alloc-shim = "0.3.0"
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
//! Add this to your crate root (lib.rs or main.rs):
//!
//! ```rust,ignore
//! #![cfg_attr(feature = "alloc", feature(alloc))]
//! ```
//!
//! Now, you can use alloc-shim:
//!
//! ```rust
//! # #![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]
//! #[cfg(any(feature = "alloc", feature = "std"))]
//! use alloc::prelude::*; // And more...
//! ```
//!
//! The current version of alloc-shim requires Rust 1.31 or later.
//!
//! ## Crate Features
//!
//! If not either `std` or `alloc` is specified, this crate does nothing.
//!
//! * `std`
//!   * Disabled by default.
//!   * Enable to use `std` crate.
//!
//! * `alloc`
//!   * Disabled by default.
//!   * Enable to use `alloc` crate.
//!   * Note that `std` crate is used if both `std` and `alloc` are specified at the same time.
//!   * This requires Rust Nightly.
//!
//! * `futures`
//!   * Disabled by default.
//!   * Enable to use `alloc::task`.
//!   * This requires Rust Nightly.
//!

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.3.0")]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]
#![cfg_attr(feature = "futures", feature(futures_api))]
#![deny(rust_2018_idioms)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as liballoc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod shim {
    pub use liballoc::{
        alloc, borrow, boxed, collections, fmt, format, rc, slice, str, string, vec,
    };

    /// Synchronization primitives
    pub mod sync {
        pub use liballoc::sync::*;

        // `alloc::sync` does not include `atomic` module
        // pub use core::sync::atomic;
    }

    #[cfg(feature = "futures")]
    pub use liballoc::task;

    // FIXME(taiki-e):
    // `alloc::prelude` is required to use `#![feature(alloc)]` now.
    // Should we rewrite it so that it can be used without specifying `#![feature(alloc)]`?
    //
    // /// The alloc Prelude
    // pub mod prelude {
    //    pub use liballoc::prelude::*;
    // }
    pub use liballoc::prelude;
}

#[cfg(feature = "std")]
mod shim {
    pub use std::{alloc, borrow, boxed, collections, fmt, format, rc, slice, str, string, vec};

    /// Synchronization primitives
    pub mod sync {
        pub use std::sync::{Arc, Weak};

        // `alloc::sync` does not include `atomic` module
        // pub use std::sync::atomic;
    }

    #[cfg(feature = "futures")]
    pub use std::task;

    // The layout in the prelude module is different for `std` and `alloc`.
    /// The alloc Prelude
    pub use std::prelude::v1 as prelude;
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub use self::shim::*;
