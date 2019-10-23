//! A shim crate for to import items of alloc crate ergonomically.
//!
//! **This crate is deprecated.** You can now write:
//!
//! ```rust
//! #[cfg(all(feature = "alloc", not(feature = "std")))]
//! extern crate alloc;
//! #[cfg(feature = "std")]
//! extern crate std as alloc;
//! ```

#![doc(html_root_url = "https://docs.rs/alloc-shim/0.3.4")]
#![deny(rust_2018_idioms)]
#![deprecated(since = "0.3.4", note = "this crate is deprecated without replacement")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as alloc_crate;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc_crate::*;

#[cfg(feature = "std")]
pub use std::{alloc, borrow, boxed, collections, fmt, format, rc, slice, str, string, sync, vec};
