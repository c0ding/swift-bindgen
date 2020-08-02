use crate::Int;
use std::{ffi::c_void, fmt, hash, mem, ptr::NonNull, slice};

/// A raw pointer for accessing untyped data.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsaferawpointer).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnsafeRawPointer(NonNull<c_void>);

/// A raw pointer for accessing and manipulating untyped data.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablerawpointer).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnsafeMutableRawPointer(NonNull<c_void>);

/// A nonowning collection interface to the bytes in a region of memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsaferawbufferpointer).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnsafeRawBufferPointer {
    // TODO: Replace `Option` with `Optional`.
    //
    // For now, this is fine since it gives us the memory representation we
    // want.
    start: Option<UnsafeRawPointer>,
    count: Int,
}

impl fmt::Debug for UnsafeRawBufferPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeRawBufferPointer")
            .field(
                "start",
                match &self.start {
                    Some(start) => &start.0,
                    None => &"nil",
                },
            )
            .field("count", &self.count)
            .finish()
    }
}

/// A mutable nonowning collection interface to the bytes in a region of memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablerawbufferpointer).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnsafeMutableRawBufferPointer {
    // TODO: Replace `Option` with `Optional`.
    //
    // For now, this is fine since it gives us the memory representation we
    // want.
    start: Option<UnsafeMutableRawPointer>,
    count: Int,
}

impl fmt::Debug for UnsafeMutableRawBufferPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeMutableRawBufferPointer")
            .field(
                "start",
                match &self.start {
                    Some(start) => &start.0,
                    None => &"nil",
                },
            )
            .field("count", &self.count)
            .finish()
    }
}

macro_rules! impl_hash {
    ($($pointer:ty,)+) => {$(
        impl hash::Hash for $pointer {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                let ptr = (self as *const Self).cast::<u8>();
                let len = mem::size_of::<Self>();

                let bytes = unsafe { slice::from_raw_parts(ptr, len) };

                bytes.hash(state);
            }

            #[inline]
            fn hash_slice<H: hash::Hasher>(data: &[Self], state: &mut H) {
                let ptr = data.as_ptr().cast::<u8>();
                let len = data.len() * mem::size_of::<Self>();

                let bytes = unsafe { slice::from_raw_parts(ptr, len) };

                bytes.hash(state);
            }
        }
    )+};
}

impl_hash! {
    UnsafeRawPointer,
    UnsafeMutableRawPointer,
    UnsafeRawBufferPointer,
    UnsafeMutableRawBufferPointer,
}
