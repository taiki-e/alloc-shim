#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![allow(unused_imports)]

#[cfg(any(feature = "alloc", feature = "std"))]
mod test {
    use alloc::{
        alloc as alloc_mod, borrow, boxed, collections, fmt, prelude, prelude::Box, prelude::Vec,
        rc, slice, str, string, sync, sync::Arc, vec,
    };

    // `alloc::sync` does not include `atomic` module
    // use alloc::sync::atomic;
}
