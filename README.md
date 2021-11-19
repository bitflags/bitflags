# `bitflags`

[![Rust](https://github.com/bitflags/bitflags/workflows/Rust/badge.svg)](https://github.com/bitflags/bitflags/actions)
[![Latest version](https://img.shields.io/crates/v/bitflags.svg)](https://crates.io/crates/bitflags)
[![Documentation](https://docs.rs/bitflags/badge.svg)](https://docs.rs/bitflags)
![License](https://img.shields.io/crates/l/bitflags.svg)

A Rust macro to generate structures which behave like a set of bitflags.

- [Documentation](https://docs.rs/bitflags)
- [Release notes](https://github.com/bitflags/bitflags/releases)

## Getting stareted

Add this to your `Cargo.toml`:

```toml
[dependencies.bitflags]
version = "1.3"
```

Use the `bitflags!` macro to define a bitflags type:

```rust
use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq)]
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }
}

let e1 = Flags::A | Flags::C;
let e2 = Flags::B | Flags::C;

assert_eq!((e1 | e2), Flags::ABC);   // union
assert_eq!((e1 & e2), Flags::C);     // intersection
assert_eq!((e1 - e2), Flags::A);     // set difference
assert_eq!(!e2, Flags::A);           // set complement
```

## Minimum Supported Rust Version (MSRV)

The minimum supported Rust version is 1.46 due to use of associated constants and const functions.
This version may be bumped in minor releases.
