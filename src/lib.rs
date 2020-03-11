// Crate-level documentation (Overview) resides in the 'README.md' file.

// #![no_std] // Async will soon be available on non-std programs. https://github.com/rust-lang/rust/pull/69033

#![feature(concat_idents)]

pub mod core;

/// A module which is typically glob imported from:
///
/// ```
/// use rasm::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        core,
    };
}