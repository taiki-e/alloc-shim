# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.3.4] - 2019-09-20

* Fix urls in readme.

## [0.3.3] - 2019-09-20

* Removed some package metadata fields.

## [0.3.2] - 2019-03-19

* **This crate is deprecated.** You can now write:

  ```rust
  #![cfg_attr(feature = "alloc", feature(alloc))]

  #[cfg(all(feature = "alloc", not(feature = "std")))]
  extern crate alloc;
  #[cfg(feature = "std")]
  extern crate std as alloc;
  ```

* Updated to new nightly.

## [0.3.1] - 2019-02-18

* Removed "futures" feature (Update to new nightly).

## [0.3.0] - 2019-01-29

* Rewrited to the same layout as the `alloc` crate.

## [0.2.1] - 2019-01-28

* Fixed a bug that can be compiled with the stable compiler when both `std` and `alloc` are specified.

* Fixed documentation.

## [0.2.0] - 2019-01-27 - YANKED

* Changed APIs. You can also use it like [this example](https://github.com/taiki-e/alloc-shim/tree/v0.2.0/examples/std-shim).

* Changed lib name to `alloc`.

* Improved documentation.

## [0.1.0] - 2019-01-26

Initial release

[Unreleased]: https://github.com/taiki-e/alloc-shim/compare/v0.3.4...HEAD
[0.3.4]: https://github.com/taiki-e/alloc-shim/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/taiki-e/alloc-shim/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/taiki-e/alloc-shim/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taiki-e/alloc-shim/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taiki-e/alloc-shim/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/taiki-e/alloc-shim/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/alloc-shim/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/alloc-shim/releases/tag/v0.1.0
