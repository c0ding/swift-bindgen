use std::{any::type_name, mem, slice};

mod any;
mod identifier;

pub use any::*;
pub use identifier::*;

/// Asserts that `$obj` has the same memory layout as `AnyObject`.
///
/// Although this is unconditional on debug/release, it will only be emitted if
/// it fails since `size_of` and `align_of` are evaluated at compile-time.
macro_rules! assert_layout {
    ($obj:ty) => {
        assert_eq!(
            mem::size_of::<$obj>(),
            mem::size_of::<AnyObject>(),
            "Incorrect size for {}",
            type_name::<$obj>(),
        );
        assert_eq!(
            mem::align_of::<$obj>(),
            mem::align_of::<AnyObject>(),
            "Incorrect alignment for {}",
            type_name::<$obj>(),
        );
    };
}

/// A Swift class type.
///
/// # Safety
///
/// Implementors of this trait _must_ have the same memory layout as
/// [`AnyObject`]:
///
/// - [`std::mem::size_of`] of 8
/// - [`std::mem::align_of`] of 8
/// - Zero cannot be represented (allows `Option<Self>` to be 8 bytes)
///
/// This should be done as such:
///
/// ```rust
/// #[repr(transparent)]
/// struct MyObject(swift::AnyObject);
///
/// unsafe impl swift::Object for MyObject {}
/// ```
///
/// [`std::mem::size_of`]: https://doc.rust-lang.org/std/mem/fn.size_of.html
/// [`std::mem::align_of`]: https://doc.rust-lang.org/std/mem/fn.align_of.html
/// [`AnyObject`]: struct.AnyObject.html
pub unsafe trait Object: Sized {
    /// Casts a shared slice of instances to a slice of `AnyObject`.
    #[inline]
    fn slice_as_any_object(slice: &[Self]) -> &[AnyObject] {
        assert_layout!(Self);

        let ptr = slice.as_ptr().cast::<AnyObject>();
        let len = slice.len();

        // SAFETY: `Self` is guaranteed by the implementor to have the same
        // memory layout as `AnyObject`. Otherwise the above assert fails.
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Returns a shared reference to `self` as an `AnyObject`.
    #[inline]
    fn as_any_object(&self) -> &AnyObject {
        assert_layout!(Self);

        // SAFETY: `Self` is guaranteed by the implementor to have the same
        // memory layout as `AnyObject`. Otherwise the above assert fails.
        unsafe { &*(self as *const Self).cast() }
    }

    /// Converts `self` into an `AnyObject` without incrementing the reference
    /// count.
    #[inline]
    fn into_any_object(self) -> AnyObject {
        assert_layout!(Self);

        // SAFETY: `Self` is guaranteed by the implementor to have the same
        // memory layout as `AnyObject`. Otherwise the above assert fails.
        let obj = unsafe { mem::transmute_copy(&self) };
        mem::forget(self);
        obj
    }
}
