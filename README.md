bitflags
========

A Rust macro to generate structures which behave like a set of bitflags

[![Build Status](https://travis-ci.org/rust-lang-nursery/bitflags.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/bitflags)

- [Documentation](https://docs.rs/bitflags)
- [Release notes](https://github.com/rust-lang-nursery/bitflags/releases)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitflags = "0.9"
```

and this to your crate root:

```rust
#[macro_use]
extern crate bitflags;
```

## Release notes

bitflags 0.9.1

* Fix potential method conflicts in the implementation of the `fmt` traits
  (#105).
* Documentation updates.

bitflags 0.9.0

* [breaking change] Use `struct` keyword instead of `flags` to define bitflag
  types (#84).
* [breaking change] Terminate `const` items with semicolons instead of commas
  (#87).
* Implement the Hex, Octal, and Binary formatting traits (#86).
* Printing an empty flag value with the Debug trait now prints `"(empty)"`
  instead of nothing (#85).
* The `bitflags!` macro can now be used inside of a `fn` body, to define a
  type local to that function (#74).
* Improved documentation.

bitflags 0.8.2

* Update feature flag used when building bitflags as a dependency of the Rust
  toolchain.

bitflags 0.8.1

* Allow bitflags to be used as a dependency of the Rust toolchain.

bitflags 0.8.0

* Add support for the experimental `i128` and `u128` integer types (#57).
* [breaking change] Add `set` method: `flags.set(SOME_FLAG, true)` or
  `flags.set(SOME_FLAG, false)` (#55).  This may break code that defines its
  own `set` method.

bitflags 0.7.0

* Implement the `Extend` trait.
* Allow definitions inside the `bitflags!` macro to refer to items imported
  from other modules.
