bitflags
========

A Rust macro to generate structures which behave like a set of bitflags

[![Build Status](https://travis-ci.org/rust-lang-nursery/bitflags.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/bitflags)

[Documentation](https://doc.rust-lang.org/bitflags)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitflags = "0.8"
```

and this to your crate root:

```rust
#[macro_use]
extern crate bitflags;
```

## 128-bit integer bitflags (nightly only)

`u128` and `i128` are supported just like any other integer type.

```rust
#![feature(i128_type)]

#[macro_use]
extern crate bitflags;

bitflags! {
    flags Flags128: u128 {
        const A   = 0x0000_0000_0000_0000_0000_0000_0000_0001,
        const B   = 0x0000_0000_0000_1000_0000_0000_0000_0000,
        const C   = 0x8000_0000_0000_0000_0000_0000_0000_0000,
        const ABC = A.bits | B.bits | C.bits,
    }
}
```
