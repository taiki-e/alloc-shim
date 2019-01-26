# alloc-shim

[![Build Status](http://img.shields.io/travis/taiki-e/alloc-shim.svg)](https://travis-ci.org/taiki-e/alloc-shim)
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
alloc = { version = "0.1", package = "alloc-shim" }
```

Set the features so that `std` depends on `alloc-shim/std`:

```toml
[features]
std = ["alloc/std"]
```

Add this to your crate root (`lib.rs` or `main.rs`):

```rust
#![cfg_attr(not(feature = "std"), feature(alloc))]
```

Now, you can use alloc-shim:

```rust
use alloc::*;
```

The current version of alloc-shim requires Rust 1.31 or later.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
