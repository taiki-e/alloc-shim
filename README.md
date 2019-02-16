# alloc-shim

[![Build Status](https://travis-ci.com/taiki-e/alloc-shim.svg?branch=master)](https://travis-ci.com/taiki-e/alloc-shim)
[![version](https://img.shields.io/crates/v/alloc-shim.svg)](https://crates.io/crates/alloc-shim/)
[![documentation](https://docs.rs/alloc-shim/badge.svg)](https://docs.rs/alloc-shim/)
[![license](https://img.shields.io/crates/l/alloc-shim.svg)](https://crates.io/crates/alloc-shim/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

A shim crate for to import items of alloc crate ergonomically.

[Examples](examples)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
alloc-shim = "0.3.0"
```

Set the features so that `std` depends on `alloc-shim/std`, and `alloc` depends on `alloc-shim/alloc`:

```toml
[features]
std = ["alloc-shim/std"]
alloc = ["alloc-shim/alloc"]
```

Add this to your crate root (lib.rs or main.rs):

```rust
#![cfg_attr(feature = "alloc", feature(alloc))]
```

Now, you can use alloc-shim:

```rust
#[cfg(any(feature = "alloc", feature = "std"))]
use alloc::prelude::*; // And more...
```

The current version of alloc-shim requires Rust 1.31 or later.

## Crate Features

If not either `std` or `alloc` is specified, this crate does nothing.

* `std`
  * Disabled by default.
  * Enable to use `std` crate.

* `alloc`
  * Disabled by default.
  * Enable to use `alloc` crate.
  * Note that `std` crate is used if both `std` and `alloc` are specified at the same time (and it can compile in the minimum required version of alloc-shim).
  * This requires Rust Nightly.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
