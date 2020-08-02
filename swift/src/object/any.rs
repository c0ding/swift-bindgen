use super::Object;
use std::fmt;
use swift_rt::obj::Owned;

/// The protocol to which all classes implicitly conform.
///
/// See [documentation](https://developer.apple.com/documentation/swift/anyobject).
///
/// # Discussion
///
/// You use `AnyObject` when you need the flexibility of an untyped object or
/// when you use bridged Objective-C methods and properties that return an
/// untyped result. `AnyObject` can be used as the concrete type for an instance
/// of any class, class type, or class-only protocol.
///
/// For example:
///
/// ```swift
/// class FloatRef {
///     let value: Float
///     init(_ value: Float) {
///         self.value = value
///     }
/// }
///
/// let x = FloatRef(2.3)
/// let y: AnyObject = x
/// let z: AnyObject = FloatRef.self
/// ```
#[repr(transparent)]
#[derive(Clone)]
pub struct AnyObject(pub(super) Owned);

unsafe impl Object for AnyObject {}

impl From<Owned> for AnyObject {
    #[inline]
    fn from(owned: Owned) -> Self {
        Self(owned)
    }
}

impl From<AnyObject> for Owned {
    #[inline]
    fn from(object: AnyObject) -> Self {
        object.0
    }
}

impl AsRef<Owned> for AnyObject {
    #[inline]
    fn as_ref(&self) -> &Owned {
        &self.0
    }
}

impl fmt::Debug for AnyObject {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("AnyObject").field(&self.0.as_ptr()).finish()
    }
}

impl AnyObject {
    /// Creates an instance from some memory address.
    ///
    /// # Safety
    ///
    /// `ptr` _must_ point to a retained Swift object.
    #[inline]
    pub const unsafe fn from_ptr<T>(ptr: *mut T) -> Self {
        Self(Owned::from_ptr(ptr))
    }
}
