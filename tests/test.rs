#![deny(warnings)]
#![allow(unused_imports)]
#![cfg_attr(
    all(feature = "alloc", not(feature = "std")),
    feature(alloc, futures_api, core_panic_info, core_intrinsics, raw)
)]

use alloc::{
    alloc as alloc_mod, any, arch, ascii, borrow, cell, char, clone, cmp, convert, default, f32,
    f64, ffi, fmt, hash, hint, i128, i16, i32, i64, i8, isize, iter, marker, mem, num, ops, option,
    prelude, prelude::v1 as core_v1, ptr, result, slice, str, sync, sync::atomic as core_atomic,
    time, u128, u16, u32, u64, u8, usize,
};

#[cfg(feature = "alloc")]
use alloc::{boxed, collections, panic, prelude::v1, rc, string, sync::atomic, sync::Arc, vec};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{future, intrinsics, pin, raw, task};
