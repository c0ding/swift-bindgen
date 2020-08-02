//! Symbol name operations.

// Based on:
// include/swift/Demangling/Demangle.h

use std::os::raw::c_char;

extern "C" {
    /// Demangles a Swift symbol name. Returns null if the input string is not a
    /// Swift mangled name.
    ///
    /// # Parameters
    ///
    /// - `mangledName`: the symbol name that needs to be demangled.
    ///
    /// - `mangledNameLength`: the length of the string that should be
    ///   demangled.
    ///
    /// - `outputBuffer`: the user provided buffer where the demangled name will
    ///   be placed. If null, a new buffer will be `malloc`ed. In that case, the
    ///   user of this API is responsible for freeing the returned buffer.
    ///
    /// - `outputBufferSize`: the size of the output buffer. If the demangled
    ///   name does not fit into `outputBuffer`, the output will be truncated
    ///   and the size will be updated, indicating how large the buffer should
    ///   be.
    ///
    /// - `flags`: can be used to select the demangling style.
    // char *swift_demangle(const char *mangledName,
    //                      size_t mangledNameLength,
    //                      char *outputBuffer,
    //                      size_t *outputBufferSize,
    //                      uint32_t flags);
    pub fn swift_demangle(
        mangledName: *const c_char,
        mangledNameLength: usize,
        outputBuffer: *mut c_char,
        outputBufferSize: *mut usize,
        flags: u32,
    ) -> *mut c_char;
}
