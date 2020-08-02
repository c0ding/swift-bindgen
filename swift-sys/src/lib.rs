//! Raw bindings into Swift's runtime.
//!
//! This crate is under construction. Please contact
//! [Nikolai Vazquez](https://twitter.com/NikolaiVazquez) if you would like to
//! get involved.

#![deny(missing_docs)]

#[macro_use]
extern crate static_assertions;

pub mod heap;
pub mod metadata;
pub mod sym;
pub mod sync;

/// A value about which nothing is known.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OpaqueValue {
    _private: [u8; 0],
}
