#![cfg_attr(
    all(feature = "alloc", not(feature = "std")),
    feature(alloc, alloc_prelude)
)]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![allow(unused_imports)]

#[cfg(any(feature = "alloc", feature = "std"))]
mod test {
    use alloc::{
        alloc as alloc_mod, borrow, boxed, collections, fmt, prelude::v1, prelude::v1::Box,
        prelude::v1::Vec, rc, slice, str, string, sync, sync::Arc, vec,
    };
}
