# alloc-shim

[![Build Status](https://travis-ci.com/taiki-e/alloc-shim.svg?branch=master)](https://travis-ci.com/taiki-e/alloc-shim)
[![version](https://img.shields.io/crates/v/alloc-shim.svg)](https://crates.io/crates/alloc-shim/)
[![documentation](https://docs.rs/alloc-shim/badge.svg)](https://docs.rs/alloc-shim/)
[![license](https://img.shields.io/crates/l/alloc-shim.svg)](https://crates.io/crates/alloc-shim/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

A shim crate for to import items of alloc crate ergonomically.

**This crate is deprecated.** You can now write:

```rust
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
