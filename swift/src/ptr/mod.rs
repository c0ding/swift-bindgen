// This module defines newtypes around `NonNull<T>` instead of creating type
// aliases because Rust has no non-null pointer types with associated mutability
// like `*const T` and `*mut T`.
//
// Because these types are `repr(transparent)` around `NonNull<T>`, which itself
// is `repr(transparent)` around `*const T`, these can be safely used in FFI to
// represent the equivalent Swift types.

mod raw;
mod typed;

pub use raw::*;
pub use typed::*;
