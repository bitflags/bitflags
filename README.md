bitflags
========

[![Build Status](https://travis-ci.com/bitflags/bitflags.svg?branch=master)](https://travis-ci.com/bitflags/bitflags)
[![Join the chat at https://gitter.im/bitflags/Lobby](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/bitflags/Lobby?utm_source=badge&utm_medium=badge&utm_content=badge)
[![Latest version](https://img.shields.io/crates/v/bitflags.svg)](https://crates.io/crates/bitflags)
[![Documentation](https://docs.rs/bitflags/badge.svg)](https://docs.rs/bitflags)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-yellow.svg)
![License](https://img.shields.io/crates/l/bitflags.svg)

A Rust macro to generate structures which behave like a set of bitflags

- [Documentation](https://docs.rs/bitflags)
- [Release notes](https://github.com/bitflags/bitflags/releases)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitflags = "1.2"
```

and this to your source code wherever you use the `bitflags!{}` macro:

```rust
use bitflags::bitflags;
```

## Rust Version Support

The minimum supported Rust version is 1.33 due to use of `unsafe` in `const fn`.
