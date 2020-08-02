use std::{fmt, ptr::NonNull};
use swift_sys::OpaqueValue;

/// The metatype for `Any`, also known as `Any.Type`.
#[repr(transparent)]
#[derive(Debug)]
pub struct AnyType(
    // TODO: Replace `OpaqueValue` with a low-level `Metatype`.
    pub(crate) NonNull<OpaqueValue>,
);

// SAFETY: Types are global objects that can be referenced anywhere.
unsafe impl Send for AnyType {}
unsafe impl Sync for AnyType {}

impl From<AnyClass> for AnyType {
    #[inline]
    fn from(class: AnyClass) -> Self {
        class.0
    }
}

/// The protocol to which all class types implicitly conform.
///
/// See [documentation](https://developer.apple.com/documentation/swift/anyclass).
///
/// # Discussion
///
/// You can use the `AnyClass` protocol as the concrete type for an instance of
/// any class. When you do, all known `@objc` class methods and properties are
/// available as implicitly unwrapped optional methods and properties,
/// respectively.
///
/// For example:
///
/// ```swift
/// class IntegerRef {
///     @objc class func getDefaultValue() -> Int {
///         return 42
///     }
/// }
///
/// func getDefaultValue(_ c: AnyClass) -> Int? {
///     return c.getDefaultValue?()
/// }
/// ```
///
/// The `getDefaultValue(_:)` function uses optional chaining to safely call the
/// implicitly unwrapped class method on c. Calling the function with different
/// class types shows how the `getDefaultValue()` class method is only
/// conditionally available.
///
/// ```swift
/// print(getDefaultValue(IntegerRef.self))
/// // Prints "Optional(42)"
///
/// print(getDefaultValue(NSString.self))
/// // Prints "nil"
/// ```
#[repr(transparent)]
pub struct AnyClass(AnyType);

impl AsRef<AnyType> for AnyClass {
    #[inline]
    fn as_ref(&self) -> &AnyType {
        &self.0
    }
}

impl fmt::Debug for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("AnyClass").field(&(self.0).0).finish()
    }
}
