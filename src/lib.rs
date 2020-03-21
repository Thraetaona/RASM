// Crate-level documentation (Overview) resides in the 'README.md' file.


#![no_std]

// This allows other crates to use RASM macros that depend on unstable features; without having to enable any nightly features.
#![feature(allow_internal_unstable)]
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