//! Synchronization primitives.

#![allow(non_camel_case_types)]

use std::ffi::c_void;

/// Matches
/// [`dispatch_once_t`](https://developer.apple.com/documentation/dispatch/dispatch_once_t?language=objc)
/// on macOS and iOS.
#[cfg(target_vendor = "apple")]
pub type swift_once_t = std::os::raw::c_long;

/// Either a C++
/// [`std::once_flag`](https://en.cppreference.com/w/cpp/thread/once_flag) or
/// platform word if it is smaller.
#[cfg(not(target_vendor = "apple"))]
pub type swift_once_t = crate::OpaqueValue;

extern "C" {
    /// Runs the given function with the given context argument exactly once.
    ///
    /// # Parameters
    ///
    /// - `predicate`: must point to a global or static variable of static
    ///   extent of type [`swift_once_t`].
    ///
    /// - `function`: the function to be run exactly once.
    ///
    /// - `context`: data pointer passed into `func`.
    // void swift_once(swift_once_t *predicate,
    //                 void (*fn)(void *),
    //                 void *context);
    pub fn swift_once(
        predicate: *mut swift_once_t,
        function: Option<unsafe extern "C" fn(context: *mut c_void)>,
        context: *mut c_void,
    );
}
