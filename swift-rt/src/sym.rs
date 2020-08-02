//! Symbol name operations.

use crate::sys::sym::swift_demangle;
use std::{error::Error, fmt, os::raw::c_char};

/// An error returned when attempting to demangle a non-mangled Swift symbol.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DemangleError(());

impl fmt::Display for DemangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to demangle symbol")
    }
}

impl Error for DemangleError {}

/// Attempts to demangle `symbol`, returning an error if it isn't mangled.
#[inline]
pub fn demangle(symbol: &[u8]) -> Result<String, DemangleError> {
    let mut buf = String::new();
    unsafe {
        let buf = buf.as_mut_vec();
        demangle_into(symbol, buf)?;
    }
    Ok(buf)
}

/// Attempts to demangle `symbol` into `buffer`, returning the demangled slice
/// or an error if `symbol` isn't mangled.
///
/// This allows for reusing the same buffer while still having a safe UTF-8
/// string for the demangled name.
pub fn demangle_into<'b>(
    symbol: &[u8],
    buffer: &'b mut Vec<u8>,
) -> Result<&'b mut str, DemangleError> {
    let old_len = buffer.len();
    let rem_capacity = buffer.capacity() - old_len;

    // Get the demangled length
    let mut demangled_len = 0;
    unsafe {
        let fake_buffer_ptr = 1 as *mut c_char;
        let ptr = swift_demangle(
            symbol.as_ptr() as *const c_char,
            symbol.len(),
            fake_buffer_ptr,
            &mut demangled_len,
            0, // Only 0 is currently supported
        );
        if ptr.is_null() {
            return Err(DemangleError(()));
        }
    }

    if rem_capacity < demangled_len {
        buffer.reserve(demangled_len - rem_capacity);
    }

    unsafe {
        // Get `buffer` start *after* a potential reallocation
        let demangled_start = buffer.as_mut_ptr().offset(old_len as isize);

        let total_len = old_len + demangled_len;
        buffer.set_len(total_len);

        swift_demangle(
            symbol.as_ptr() as *const c_char,
            symbol.len(),
            demangled_start as *mut c_char,
            &mut demangled_len,
            0, // Only 0 is currently supported
        );

        let demangled = buffer.get_unchecked_mut(old_len..);
        Ok(std::str::from_utf8_unchecked_mut(demangled))
    }
}
