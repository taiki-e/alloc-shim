//! A shim crate for to import items of alloc crate ergonomically.
//!
//! **This crate is deprecated.** You can now write:
//!
//! ```rust
//! #![cfg_attr(feature = "alloc", feature(alloc))]
//!
//! #[cfg(all(feature = "alloc", not(feature = "std")))]
//! extern crate alloc;
//! #[cfg(feature = "std")]
//! extern crate std as alloc;
//! ```
//!
//! [Examples](https://github.com/taiki-e/alloc-shim/tree/master/examples)
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

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.3.1")]
#![cfg_attr(
    all(feature = "alloc", not(feature = "std")),
    feature(alloc, alloc_prelude)
)]
#![deny(rust_2018_idioms)]
#![deprecated(since = "0.3.2", note = "this crate is deprecated without replacement")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as liballoc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use liballoc::*;

#[cfg(feature = "std")]
pub use std::{
    alloc, borrow, boxed, collections, fmt, format, prelude, rc, slice, str, string, sync, vec,
};
