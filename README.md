# alloc-shim

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![rustc-badge]][rustc-url]
![maintenance-badge]

[crates-badge]: https://img.shields.io/crates/v/alloc-shim.svg
[crates-url]: https://crates.io/crates/alloc-shim
[docs-badge]: https://docs.rs/alloc-shim/badge.svg
[docs-url]: https://docs.rs/alloc-shim
[license-badge]: https://img.shields.io/crates/l/alloc-shim.svg
[license]: #license
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[maintenance-badge]: https://img.shields.io/badge/maintenance-deprecated-red.svg

A shim crate for to import items of alloc crate ergonomically.

**This crate is deprecated.** You can now write:

```rust
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
