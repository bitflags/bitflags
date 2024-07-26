#![allow(unknown_lints)]
#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::blanket_clippy_restriction_lints
)]
// deny all rustc's built-in lints
#![deny(
    warnings,
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]
// deny additional allow by default from rustc
#![deny(
    deprecated_in_future,
    ffi_unwind_calls,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

//! An example file for smoke tests

use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    /// Example Flags
    pub struct Flags: u32 {
        /// A
        const A = 0b0000_0001;
        /// B
        const B = 0b0000_0010;
        /// C
        const C = 0b0000_0100;
        /// ABC
        const ABC = Flags::A.bits() | Flags::B.bits() | Flags::C.bits();

        const _ = !0;
    }
}

#[allow(clippy::print_stdout, clippy::use_debug)]
fn main() {
    println!("{:?}", Flags::ABC);
}
