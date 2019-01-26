#![deny(warnings)]
#![allow(unused_imports)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(
    not(feature = "std"),
    feature(futures_api, core_panic_info, core_intrinsics, raw)
)]

use alloc_shim::{
    alloc, any, arch, ascii, borrow, boxed, cell, char, clone, cmp, collections, convert, default,
    f32, f64, ffi, fmt, hash, hint, i128, i16, i32, i64, i8, isize, iter, marker, mem, num, ops,
    option, panic, prelude, prelude::v1, ptr, rc, result, slice, str, string, sync, sync::atomic,
    sync::Arc, time, u128, u16, u32, u64, u8, usize, vec,
};

#[cfg(not(feature = "std"))]
use alloc_shim::{future, intrinsics, pin, raw, task};
