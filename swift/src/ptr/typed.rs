use crate::Int;
use std::{fmt, hash, mem, ptr::NonNull, slice};

/// A pointer for accessing data of a specific type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafepointer).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnsafePointer<T>(NonNull<T>);

/// A pointer for accessing and manipulating data of a specific type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablepointer).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnsafeMutablePointer<T>(NonNull<T>);

/// A nonowning collection interface to a buffer of elements stored contiguously
/// in memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafebufferpointer).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnsafeBufferPointer<T> {
    // TODO: Replace `Option` with `Optional`.
    //
    // For now, this is fine since it gives us the memory representation we
    // want.
    start: Option<UnsafePointer<T>>,
    count: Int,
}

impl<T> fmt::Debug for UnsafeBufferPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeBufferPointer")
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

/// A nonowning collection interface to a buffer of mutable elements stored
/// contiguously in memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablebufferpointer).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnsafeMutableBufferPointer<T> {
    // TODO: Replace `Option` with `Optional`.
    //
    // For now, this is fine since it gives us the memory representation we
    // want.
    start: Option<UnsafeMutablePointer<T>>,
    count: Int,
}

impl<T> fmt::Debug for UnsafeMutableBufferPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeMutableBufferPointer")
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
    ($($pointer:ident,)+) => {$(
        impl<T> hash::Hash for $pointer<T> {
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
    UnsafePointer,
    UnsafeMutablePointer,
    UnsafeBufferPointer,
    UnsafeMutableBufferPointer,
}
