use crate::{Bool, UInt32};

/// A Unicode scalar value.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unicode/scalar).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnicodeScalar(
    // TODO: Should this instead be `UInt32` to match Swift exactly?
    char,
);

impl UnicodeScalar {
    /// Creates an instance from a Rust `char`.
    #[inline]
    pub const fn from_char(ch: char) -> Self {
        Self(ch)
    }

    /// Converts `self` into a Rust `char`.
    #[inline]
    pub fn into_char(self) -> char {
        self.0
    }
}

/// Swift standard library functions.
impl UnicodeScalar {
    /// A numeric representation of the Unicode scalar.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unicode/scalar/2908540-value).
    #[inline]
    pub fn value(self) -> UInt32 {
        self.0 as UInt32
    }

    /// A Boolean value indicating whether the Unicode scalar is an ASCII
    /// character.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unicode/scalar/2905383-isascii).
    #[inline]
    pub fn is_ascii(&self) -> Bool {
        self.into_char().is_ascii()
    }
}
