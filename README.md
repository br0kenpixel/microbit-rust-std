# Micro:Bit - Embedded Rust with std support template

This is a project template intended to be used with [`cargo-generate`](https://github.com/cargo-generate/cargo-generate).

✅ Restricted standard library support is pre-configured. I also have a [template for non-std projects](https://github.com/br0kenpixel/microbit-rust-nostd).

## Create project
```sh
cargo generate --git https://github.com/br0kenpixel/microbit-rust-std
```

### ⚠️ Notes
1. This project is pre-configured to use the nightly compiler. __You cannot use stable/beta, as [`#![feature(restricted_std)]`](https://doc.rust-lang.org/beta/unstable-book/library-features/restricted-std.html) requires a nightly
compiler!__
2. I am aware of the fact that [`alloc-cortex-m`](https://crates.io/crates/alloc-cortex-m) is deprecated, however I could not get [`embedded-alloc`](https://crates.io/crates/embedded-alloc) to compile.
3. This template is designed for projects using Micro:Bit __v2__ boards.