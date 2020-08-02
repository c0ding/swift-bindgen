use super::AnyObject;
use crate::AnyType;
use std::{ffi::c_void, ptr::NonNull};

/// A unique identifier for a class instance or metatype.
///
/// See [documentation](https://developer.apple.com/documentation/swift/objectidentifier).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectIdentifier(
    // `Builtin.RawPointer` is non-nullable, so `NonNull` is appropriate here.
    NonNull<c_void>,
);

// SAFETY: The pointer is never dereferenced and can outlive its source.
unsafe impl Send for ObjectIdentifier {}
unsafe impl Sync for ObjectIdentifier {}

impl From<&AnyObject> for ObjectIdentifier {
    #[inline]
    fn from(object: &AnyObject) -> Self {
        Self::init_object(object)
    }
}

impl From<AnyObject> for ObjectIdentifier {
    #[inline]
    fn from(object: AnyObject) -> Self {
        Self::from(&object)
    }
}

impl ObjectIdentifier {
    /// Creates an instance that uniquely identifies the given class instance.
    ///
    /// This is equivalent to [`init(AnyObject)`][docs].
    ///
    /// Note that `object` is passed by-reference instead of directly by-value.
    /// This is done as a micro-optimization to prevent calling into the Swift
    /// runtime for (what should be) a cheap operation.
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/objectidentifier/1538294-init
    #[inline]
    pub const fn init_object(object: &AnyObject) -> Self {
        Self(object.0.as_non_null().cast())
    }

    /// Creates an instance that uniquely identifies the given metatype.
    ///
    /// This is equivalent to [`init(Any.Type)`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/objectidentifier/1539919-init
    #[inline]
    // TODO: Because `AnyType` is global, should it be passed by value?
    pub const fn init_type(ty: &AnyType) -> Self {
        Self(ty.0.cast())
    }
}
