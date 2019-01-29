#![deny(warnings)]
#![allow(unused_imports)]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]
#![cfg_attr(feature = "futures", feature(futures_api))]

#[cfg(any(feature = "alloc", feature = "std"))]
mod test {
    use alloc::{
        alloc as alloc_mod, borrow, boxed, collections, fmt, prelude, prelude::Box, prelude::Vec,
        rc, slice, str, string, sync, sync::Arc, vec,
    };

    // `alloc::sync` does not include `atomic` module
    // use alloc::sync::atomic;

    #[cfg(unstable)]
    use alloc::task;
}
