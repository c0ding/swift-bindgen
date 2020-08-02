//! Functions for value type semantics.
//!
//! Everything within this module is sourced from:
//! - [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/Runtime/Metadata.h)
//! - [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def)

pub mod builtin;

mod table;
pub use table::*;
