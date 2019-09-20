# Unreleased

# 0.3.4 - 2019-09-20

* Fix urls in readme.

# 0.3.3 - 2019-09-20

* Removed some package metadata fields.

# 0.3.2 - 2019-03-19

* **This crate is deprecated.** You can now write:

  ```rust
  #![cfg_attr(feature = "alloc", feature(alloc))]

  #[cfg(all(feature = "alloc", not(feature = "std")))]
  extern crate alloc;
  #[cfg(feature = "std")]
  extern crate std as alloc;
  ```

* Updated to new nightly.

# 0.3.1 - 2019-02-18

* Removed "futures" feature (Update to new nightly).

# 0.3.0 - 2019-01-29

* Rewrited to the same layout as the `alloc` crate.

# 0.2.1 - 2019-01-28

* Fixed a bug that can be compiled with the stable compiler when both `std` and `alloc` are specified.

* Fixed documentation.

# 0.2.0 - 2019-01-27

* Changed APIs. You can also use it like [this example](https://github.com/taiki-e/alloc-shim/tree/v0.2.0/examples/std-shim).

* Changed lib name to `alloc`.

* Improved documentation.

# 0.1.0 - 2019-01-26

Initial release
