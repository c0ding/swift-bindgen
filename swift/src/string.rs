use crate::Bool;
use std::{char, cmp, ffi::CStr, fmt, hash, slice, str};

/// A string type designed to represent text that is known at compile time.
///
/// See [documentation](https://developer.apple.com/documentation/swift/staticstring).
#[repr(C)]
#[derive(Copy, Clone, Eq)]
pub struct StaticString {
    // Swift defines this as a `Builtin.Word` (`Int`), but `*const u8` is
    // required in order for `StaticString::from_str_unchecked` to be `const`
    // since a `*const u8` cannot be casted to a `usize` within a `const fn`.
    //
    // According to internal comments:
    // > We don't go through UnsafePointer here to make things simpler for alias
    // > analysis. A higher-level algorithm may be trying to make sure an
    // > unrelated buffer is not accessed or freed.
    //
    // Because this is an immutable pointer, there shouldn't be any aliasing
    // issues.
    start_ptr_or_data: *const u8,
    utf8_code_unit_count: usize,
    flags: i8,
}

// SAFETY: Always references static data.
unsafe impl Send for StaticString {}
unsafe impl Sync for StaticString {}

impl fmt::Debug for StaticString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.with_str(|s| s.fmt(f))
    }
}

impl fmt::Display for StaticString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.with_str(|s| s.fmt(f))
    }
}

impl PartialEq for StaticString {
    fn eq(&self, other: &Self) -> bool {
        // This implementation attempts to perform as little work as possible
        // by only calling `char::encode_utf8` in the case where one uses a
        // pointer representation but not the other.
        //
        // The alternative of using `with_str` would call `char::encode_utf8`
        // twice in the case where neither uses a pointer representation. This
        // instead will do a direct comparison without wasting time on encoding.

        let mut ch_buf = [0u8; 4];

        let (a, b) = match (self.to_str(), other.to_str()) {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => {
                let b = unsafe {
                    char::from_u32_unchecked(other.utf8_code_unit_count as u32)
                };
                let b = &*b.encode_utf8(&mut ch_buf);

                (a, b)
            }
            (None, Some(b)) => {
                let a = unsafe {
                    char::from_u32_unchecked(self.utf8_code_unit_count as u32)
                };
                let a = &*a.encode_utf8(&mut ch_buf);

                (a, b)
            }
            (None, None) => {
                // Cast to `u32` to get close to `char` without converting.
                let a = self.start_ptr_or_data as u32;
                let b = other.start_ptr_or_data as u32;

                // Compare directly if neither uses a pointer representation.
                return a == b;
            }
        };

        a == b
    }
}

impl PartialOrd for StaticString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StaticString {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // This implementation attempts to perform as little work as possible
        // by only calling `char::encode_utf8` in the case where one uses a
        // pointer representation but not the other.
        //
        // The alternative of using `with_str` would call `char::encode_utf8`
        // twice in the case where neither uses a pointer representation. This
        // instead will do a direct comparison without wasting time on encoding.

        let mut ch_buf = [0u8; 4];

        let (a, b) = match (self.to_str(), other.to_str()) {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => {
                let b = unsafe {
                    char::from_u32_unchecked(other.utf8_code_unit_count as u32)
                };
                let b = &*b.encode_utf8(&mut ch_buf);

                (a, b)
            }
            (None, Some(b)) => {
                let a = unsafe {
                    char::from_u32_unchecked(self.utf8_code_unit_count as u32)
                };
                let a = &*a.encode_utf8(&mut ch_buf);

                (a, b)
            }
            (None, None) => {
                // Compare directly if neither uses a pointer representation.
                //
                // Cast to `u32` to get close to `char` without converting.
                let a = self.start_ptr_or_data as u32;
                let b = other.start_ptr_or_data as u32;

                return a.cmp(&b);
            }
        };

        a.cmp(b)
    }
}

impl hash::Hash for StaticString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // Always hash `&str` to ensure the same hash regardless of
        // representation.
        self.with_str(|s| s.hash(state));
    }
}

impl From<StaticString> for std::string::String {
    fn from(s: StaticString) -> Self {
        s.with_str(|s| s.into())
    }
}

/// Functions for Rust ergonomics.
impl StaticString {
    /// Creates an instance from a static null-terminated Rust string slice.
    #[inline]
    pub fn from_str(s: &'static str) -> Option<Self> {
        // TODO: Return `Result` with custom error type.
        CStr::from_bytes_with_nul(s.as_bytes()).ok()?;

        if s.is_ascii() {
            Some(unsafe { Self::from_ascii_unchecked(s.as_bytes()) })
        } else {
            Some(unsafe { Self::from_str_unchecked(s) })
        }
    }

    /// Creates an instance from a static Rust string slice without checking for
    /// a null terminator.
    ///
    /// # Safety
    ///
    /// - The string must end with a null byte and not have any null bytes
    ///   within.
    ///
    /// - The string must contain more than just ASCII characters. This is to
    ///   ensure that [`is_ascii`](#method.is_ascii) returns correct results.
    ///   Use [`from_ascii_unchecked`](#method.from_ascii_unchecked) if using
    ///   an ASCII-only string.
    #[inline]
    pub const unsafe fn from_str_unchecked(s: &'static str) -> Self {
        Self {
            start_ptr_or_data: s.as_ptr(),
            utf8_code_unit_count: s.len(),
            flags: 0,
        }
    }

    /// Creates an instance from a static null-terminated ASCII byte slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let ascii = b"Hello Swift!\0";
    /// let string = swift::StaticString::from_ascii(ascii).unwrap();
    ///
    /// assert!(string.is_ascii());
    ///
    /// string.with_str(|string: &str| {
    ///     assert_eq!(string.as_bytes(), ascii);
    /// });
    /// ```
    #[inline]
    pub fn from_ascii(s: &'static [u8]) -> Option<Self> {
        // TODO: Return `Result` with custom error type.
        CStr::from_bytes_with_nul(s).ok()?;

        if s.is_ascii() {
            Some(unsafe { Self::from_ascii_unchecked(s) })
        } else {
            None
        }
    }

    /// Creates an instance from a static byte slice without checking for a null
    /// terminator or ASCII-only characters.
    ///
    /// # Safety
    ///
    /// - The string must end with a null byte and not have any null bytes
    ///   within.
    ///
    /// - The string must contain only ASCII (1-127) characters.
    #[inline]
    pub const unsafe fn from_ascii_unchecked(s: &'static [u8]) -> Self {
        Self {
            start_ptr_or_data: s.as_ptr(),
            utf8_code_unit_count: s.len(),
            flags: 2,
        }
    }

    /// Returns a Rust string slice if [`has_ptr_repr`](#method.has_ptr_repr) is
    /// `true`.
    #[inline]
    pub fn to_str(&self) -> Option<&'static str> {
        if self.has_ptr_repr() {
            Some(unsafe { self.to_str_unchecked() })
        } else {
            None
        }
    }

    /// Returns a Rust string slice without checking if
    /// [`has_ptr_repr`](#method.has_ptr_repr) is `true`.
    ///
    /// # Safety
    ///
    /// [`has_ptr_repr`](#method.has_ptr_repr) must be `true`.
    #[inline]
    pub unsafe fn to_str_unchecked(&self) -> &'static str {
        let ptr = self.start_ptr_or_data;
        let len = self.utf8_code_unit_count;
        str::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
    }

    /// Returns a null-terminated Rust string slice if
    /// [`has_ptr_repr`](#method.has_ptr_repr) is `true`.
    #[inline]
    pub fn to_str_with_nul(&self) -> Option<&'static str> {
        if self.has_ptr_repr() {
            Some(unsafe { self.to_str_with_nul_unchecked() })
        } else {
            None
        }
    }

    /// Returns a null-terminated Rust string slice without checking if
    /// [`has_ptr_repr`](#method.has_ptr_repr) is `true`.
    ///
    /// # Safety
    ///
    /// [`has_ptr_repr`](#method.has_ptr_repr) must be `true`.
    #[inline]
    pub unsafe fn to_str_with_nul_unchecked(&self) -> &'static str {
        let ptr = self.start_ptr_or_data as *const u8;
        let len = self.utf8_code_unit_count + 1;
        str::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
    }

    /// Invokes the given closure with a Rust string slice containing the static
    /// string’s UTF-8 code unit sequence (excluding the null terminator).
    ///
    /// This method works regardless of whether the static string stores a
    /// pointer or a single Unicode scalar value.
    #[inline]
    pub fn with_str<F, T>(&self, f: F) -> T
    where
        F: for<'a> FnOnce(&'a str) -> T,
    {
        if let Some(s) = self.to_str() {
            f(s)
        } else {
            let scalar = self.utf8_code_unit_count as u32;
            let ch = unsafe { char::from_u32_unchecked(scalar) };

            f(ch.encode_utf8(&mut [0; 4]))
        }
    }

    /// Invokes the given closure with a Rust string slice containing the static
    /// string’s UTF-8 code unit sequence (including the null terminator).
    ///
    /// This method works regardless of whether the static string stores a
    /// pointer or a single Unicode scalar value.
    #[inline]
    pub fn with_str_with_nul<F, T>(&self, f: F) -> T
    where
        F: for<'a> FnOnce(&'a str) -> T,
    {
        if let Some(s) = self.to_str_with_nul() {
            f(s)
        } else {
            let scalar = self.utf8_code_unit_count as u32;
            let ch = unsafe { char::from_u32_unchecked(scalar) };

            let mut buf = [0u8; 5];
            let len = ch.encode_utf8(&mut buf).len() + 1;
            let buf = &buf[..len];

            f(unsafe { str::from_utf8_unchecked(buf) })
        }
    }
}

/// Swift standard library functions.
impl StaticString {
    /// A Boolean value that indicates whether the static string stores a
    /// pointer to a null-terminated sequence of UTF-8 code units.
    ///
    /// If `false`, the static string stores a single Unicode scalar value.
    ///
    /// This is equivalent to [`hasPointerRepresentation`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/staticstring/1540244-haspointerrepresentation
    #[inline]
    pub fn has_ptr_repr(&self) -> Bool {
        self.flags & 1 == 0
    }

    /// A Boolean value that indicates whether the static string represents only
    /// ASCII code units (or an ASCII scalar value).
    #[inline]
    pub fn is_ascii(&self) -> Bool {
        self.flags & 2 != 0
    }

    // TODO: Add fatalError-ing functions:
    // - isASCII (https://developer.apple.com/documentation/swift/staticstring/1539231-isascii)
    // - unicodeScalar (https://developer.apple.com/documentation/swift/staticstring/2905209-unicodescalar)
    // - utf8CodeUnitCount (https://developer.apple.com/documentation/swift/staticstring/1641534-utf8codeunitcount)
    // - utf8Start (https://developer.apple.com/documentation/swift/staticstring/1541181-utf8start)

    // TODO: Add withUTF8Buffer (https://developer.apple.com/documentation/swift/staticstring/1541188-withutf8buffer)
}
