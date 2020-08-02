//! Runtime object management.

// Based on:
// include/swift/Runtime/RuntimeFunctions.def
// include/swift/Runtime/HeapObject.h
// stdlib/public/SwiftShims/HeapObject.h

use crate::OpaqueValue;
use std::os::raw::{c_char, c_int, c_void};

/// A metadata object.
pub type HeapMetadata = OpaqueValue;

/// Stores the reference counts for a [`HeapObject`].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InlineRefCounts(usize);

/// The Swift heap-object header.
///
/// Must match `RefCountedStructTy` in IRGen.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HeapObject {
    /// A valid pointer to a metadata object.
    pub metadata: *const HeapMetadata,

    /// The object's reference counts for ownership.
    pub ref_counts: InlineRefCounts,
}

// static_assert(sizeof(HeapObject) == 2*sizeof(void*),
//               "HeapObject must be two pointers long");
assert_eq_size!(HeapObject, [*const c_void; 2]);

// static_assert(alignof(HeapObject) == alignof(void*),
//               "HeapObject must be pointer-aligned");
assert_eq_align!(HeapObject, *const c_void);

/// An unowned reference in memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnownedReference {
    /// A pointer to the underling object.
    pub value: *mut HeapObject,
}

/// A weak reference in memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WeakReference(*mut c_void);

/// A boxed object.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct BoxPair {
    pub object: *mut HeapObject,
    pub buffer: *mut OpaqueValue,
}

/// Type metadata.
///
/// Refers to `TargetMetadata<InProcess>` in C++.
pub type Metadata = OpaqueValue;

/// The name of a type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TypeNamePair {
    /// The start of the name.
    pub data: *const c_char,
    /// The size of the name in bytes.
    pub length: usize, // technically `uintptr_t`
}

impl TypeNamePair {
    /// Returns `self` as a slice of `bytes`.
    #[inline]
    pub unsafe fn into_bytes<'a>(self) -> &'a [u8] {
        std::slice::from_raw_parts(self.data as *const u8, self.length)
    }
}

extern "C" {
    /// Allocates a new heap object.
    ///
    /// The returned memory is uninitialized outside of the heap-object header.
    /// The object has an initial retain count of 1, and its metadata is set to
    /// the given value.
    ///
    /// At some point "soon after return", it will become an invariant that
    /// `metadata->getSize(returnValue)` will equal `requiredSize`.
    ///
    /// Either aborts or throws a swift exception if the allocation fails.
    ///
    /// # Parameters
    ///
    /// - `requiredSize`: the required size of the allocation, including the
    ///   header.
    ///
    /// - `requiredAlignmentMask`: the required alignment of the allocation;
    ///   always one less than a power of 2 that's at least `alignof(void*)`.
    // HeapObject *swift_allocObject(HeapMetadata const *metadata,
    //                               size_t requiredSize,
    //                               size_t requiredAlignmentMask);
    pub fn swift_allocObject(
        metadata: *const HeapMetadata,
        requiredSize: usize,
        requiredAlignmentMask: usize,
    ) -> *mut HeapObject;

    /// Deallocate the given memory.
    ///
    /// It must have been returned by [`swift_allocObject`] and the strong
    /// reference must have the `RC_DEALLOCATING_FLAG` flag set, but otherwise
    /// the object is in an unknown state.
    ///
    /// # Parameters
    ///
    /// - `object`: never null.
    ///
    /// - `allocatedSize`: the allocated size of the object from the program's
    ///   perspective, i.e. the value.
    ///
    /// - `allocatedAlignMask`: the alignment requirement that was passed to
    ///   `allocObject`.
    // void swift_deallocObject(HeapObject *object,
    //                          size_t allocatedSize,
    //                          size_t allocatedAlignMask);
    pub fn swift_deallocObject(
        object: *mut HeapObject,
        allocatedSize: usize,
        allocatedAlignMask: usize,
    );

    /// Deallocate an uninitialized object with a strong reference count of +1.
    ///
    /// It must have been returned by [`swift_allocObject`], but otherwise the
    /// object is in an unknown state.
    ///
    /// # Parameters
    ///
    /// - `object`: never null.
    ///
    /// - `allocatedSize`: the allocated size of the object from the program's
    ///   perspective, i.e. the value.
    ///
    /// - `allocatedAlignMask`: the alignment requirement that was passed to
    ///   `allocObject`.
    // void swift_deallocUninitializedObject(HeapObject *object,
    //                                       size_t allocatedSize,
    //                                       size_t allocatedAlignMask);
    pub fn swift_deallocUninitializedObject(
        object: *mut HeapObject,
        allocatedSize: usize,
        allocatedAlignMask: usize,
    );

    /// Deallocate the given memory after destroying instance variables.
    ///
    /// Destroys instance variables in classes more derived than the given
    /// metatype.
    ///
    /// It must have been returned by [`swift_allocObject`], possibly used as an
    /// Objective-C class instance, and the strong reference must be equal to 1.
    ///
    /// # Parameters
    ///
    /// - `object`: never null.
    ///
    /// - `type`: most derived class whose instance variables do not need to be
    ///   destroyed.
    ///
    /// - `allocatedSize`: the allocated size of the object from the program's
    ///   perspective, i.e. the value.
    ///
    /// - `allocatedAlignMask`: the alignment requirement that was passed to
    ///   `allocObject`.
    // void swift_deallocPartialClassInstance(HeapObject *object,
    //                                        const HeapMetadata *type,
    //                                        size_t allocatedSize,
    //                                        size_t allocatedAlignMask);
    pub fn swift_deallocPartialClassInstance(
        object: *mut HeapObject,
        type_: *const HeapMetadata,
        allocatedSize: usize,
        allocatedAlignMask: usize,
    );

    /// Initializes the object header of a stack allocated object.
    ///
    /// Returns the passed `object` pointer.
    ///
    /// # Parameters
    ///
    /// - `metadata`: the object's metadata which is stored in the header.
    ///
    /// - `object`: the pointer to the object's memory on the stack.
    // HeapObject *swift_initStackObject(HeapMetadata const *metadata,
    //                                   HeapObject *object);
    pub fn swift_initStackObject(
        metadata: *const HeapMetadata,
        object: *mut HeapObject,
    ) -> *mut HeapObject;

    /// Initializes the object header of a static object which is statically
    /// allocated in the data section.
    ///
    /// Returns the passed `object` pointer.
    ///
    /// # Parameters
    ///
    /// - `metadata`: the object's metadata which is stored in the header.
    ///
    /// - `object`: the pointer to the object's memory on the stack. It is
    ///   assumed that at offset -1 there is a `swift_once` token allocated.
    // HeapObject *swift_initStaticObject(HeapMetadata const *metadata,
    //                                    HeapObject *object);
    pub fn swift_initStaticObject(
        metadata: *const HeapMetadata,
        object: *mut HeapObject,
    ) -> *mut HeapObject;

    /// Deallocate the given memory allocated by [`swift_allocBox`].
    ///
    /// The memory was returned by [`swift_allocBox`] but is otherwise in an
    /// unknown state.
    ///
    /// The given [`Metadata`] pointer must be the same metadata pointer that was
    /// passed to [`swift_allocBox`] when the memory was allocated.
    // void swift_deallocBox(HeapObject *object);
    pub fn swift_deallocBox(object: *mut HeapObject);

    /// Project the value out of a box. `object` must have been allocated
    /// using `swift_allocBox`, or by the compiler using a statically-emitted
    /// box metadata object.
    // OpaqueValue *swift_projectBox(HeapObject *object);
    pub fn swift_projectBox(object: *mut HeapObject) -> *mut OpaqueValue;

    ////////////////////////////////////////////////////////////////////////////
    // Owned References
    ////////////////////////////////////////////////////////////////////////////

    /// Atomically increments the retain count of an object.
    ///
    /// Returns the object because this enables tail call optimization and the
    /// argument register to be live through the call on architectures whose
    /// argument and return register is the same register.
    ///
    /// # Parameters
    ///
    /// - `object`: may be null, in which case this is a no-op.
    // HeapObject *swift_retain(HeapObject *object);
    pub fn swift_retain(object: *mut HeapObject) -> *mut HeapObject;

    /// Performs [`swift_retain`](fn.swift_retain.html) `n` times.
    // HeapObject *swift_retain_n(HeapObject *object, uint32_t n);
    pub fn swift_retain_n(object: *mut HeapObject, n: u32) -> *mut HeapObject;

    // HeapObject *swift_retain(HeapObject *object);
    pub fn swift_nonatomic_retain(object: *mut HeapObject) -> *mut HeapObject;

    /// Performs [`swift_nonatomic_retain`](fn.swift_nonatomic_retain.html) `n` times.
    // HeapObject *swift_retain_n(HeapObject *object, uint32_t n);
    pub fn swift_nonatomic_retain_n(
        object: *mut HeapObject,
        n: u32,
    ) -> *mut HeapObject;

    // size_t swift_retainCount(HeapObject *object);
    pub fn swift_retainCount(object: *mut HeapObject) -> usize;

    /// Atomically decrements the retain count of `object`.
    ///
    /// If the retain count reaches zero, `object` is destroyed as follows:
    ///
    /// ```cpp
    /// size_t allocSize = object->metadata->destroy(object);
    /// if (allocSize) swift_deallocObject(object, allocSize);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `object`: may be null, in which case this is a no-op.
    // void swift_release(HeapObject *object);
    pub fn swift_release(object: *mut HeapObject);

    /// Atomically decrements the retain count of `object` by `n`.
    // void swift_release_n(HeapObject *object, uint32_t n);
    pub fn swift_release_n(object: *mut HeapObject, n: u32);

    /// Non-atomically decrements the retain count of `object`.
    // void swift_nonatomic_release(HeapObject *object);
    pub fn swift_nonatomic_release(object: *mut HeapObject);

    /// Non-atomically decrements the retain count of an object by `n`.
    // void swift_nonatomic_release_n(HeapObject *object, uint32_t n);
    pub fn swift_nonatomic_release_n(object: *mut HeapObject, n: u32);

    /// Sets the `RC_DEALLOCATING_FLAG` flag.
    ///
    /// This is done non-atomically.
    ///
    /// The strong reference count of `object` must be 1 and no other thread may
    /// retain the object during executing this function.
    // void swift_setDeallocating(HeapObject *object);
    pub fn swift_setDeallocating(object: *mut HeapObject);

    ////////////////////////////////////////////////////////////////////////////
    // Unowned References
    ////////////////////////////////////////////////////////////////////////////

    /// Atomically increments the unowned retain count.
    // HeapObject *swift_unownedRetain(HeapObject *value);
    pub fn swift_unownedRetain(object: *mut HeapObject) -> *mut HeapObject;

    /// Atomically increments the unowned retain count by `n`.
    // HeapObject *swift_unownedRetain_n(HeapObject *value, int n);
    pub fn swift_unownedRetain_n(
        object: *mut HeapObject,
        n: c_int,
    ) -> *mut HeapObject;

    /// Non-atomically increments the unowned retain count.
    // void *swift_nonatomic_unownedRetain(HeapObject *value);
    pub fn swift_nonatomic_unownedRetain(
        object: *mut HeapObject,
    ) -> *mut HeapObject;

    /// Non-atomically increments the unowned retain count by `n`.
    // HeapObject *swift_nonatomic_unownedRetain_n(HeapObject *value, int n);
    pub fn swift_nonatomic_unownedRetain_n(
        object: *mut HeapObject,
        n: c_int,
    ) -> *mut HeapObject;

    /// Returns the unowned retain count.
    // size_t swift_unownedRetainCount(HeapObject *object);
    pub fn swift_unownedRetainCount(object: *mut HeapObject) -> usize;

    /// Atomically decrements the unowned retain count.
    // void swift_unownedRelease(HeapObject *object);
    pub fn swift_unownedRelease(object: *mut HeapObject);

    /// Atomically decrements the unowned retain count by `n`.
    // void swift_unownedRelease_n(HeapObject *value, int n);
    pub fn swift_unownedRelease_n(object: *mut HeapObject, n: c_int);

    /// Non-atomically decrements the unowned retain count.
    // void swift_nonatomic_unownedRelease(HeapObject *value);
    pub fn swift_nonatomic_unownedRelease(object: *mut HeapObject);

    /// Non-atomically decrements the unowned retain count by `n`.
    // void swift_nonatomic_unownedRelease_n(HeapObject *value, int n);
    pub fn swift_nonatomic_unownedRelease_n(object: *mut HeapObject, n: c_int);

    ////////////////////////////////////////////////////////////////////////////
    // Weak References
    ////////////////////////////////////////////////////////////////////////////

    // size_t swift_weakRetainCount(HeapObject *object);
    pub fn swift_weakRetainCount(object: *mut HeapObject) -> usize;

    /// Initialize a weak reference.
    // WeakReference *swift_weakInit(WeakReference *ref, HeapObject *value);
    pub fn swift_weakInit(
        ref_: *mut WeakReference,
        value: *mut HeapObject,
    ) -> *mut WeakReference;

    /// Assign a new value to a weak reference.
    // WeakReference *swift_weakAssign(WeakReference *ref, HeapObject *value);
    pub fn swift_weakAssign(
        ref_: *mut WeakReference,
        value: *mut HeapObject,
    ) -> *mut WeakReference;

    /// Load a value from a weak reference.
    ///
    /// If the current value is a non-null object that has begun deallocation,
    /// returns null; otherwise, retains the object before returning.
    ///
    /// # Parameters
    ///
    /// - `ref_`: may never be null.
    // HeapObject *swift_weakLoadStrong(WeakReference *ref);
    pub fn swift_weakLoadStrong(ref_: *mut WeakReference) -> *mut HeapObject;

    /// Load a value from a weak reference as if by [`swift_weakLoadStrong`],
    /// but leaving the reference in an uninitialized state.
    // HeapObject *swift_weakTakeStrong(WeakReference *ref);
    pub fn swift_weakTakeStrong(ref_: *mut WeakReference) -> *mut HeapObject;

    /// Destroy a weak reference.
    // void swift_weakDestroy(WeakReference *ref);
    pub fn swift_weakDestroy(ref_: *mut WeakReference);

// TODO: Remaining functions for references
}

// TODO: Support the Swift calling convention in rustc
// See https://github.com/rust-lang/rust/pull/64582
extern "C" {
    /// Allocates a heap object that can contain a value of the given type.
    ///
    /// Returns a Box structure containing a HeapObject* pointer to the
    /// allocated object, and a pointer to the value inside the heap object.
    ///
    /// The value pointer points to an uninitialized buffer of size and alignment
    /// appropriate to store a value of the given type.
    ///
    /// The heap object has an initial retain count of 1, and its metadata is set
    /// such that destroying the heap object destroys the contained value.
    // BoxPair swift_allocBox(Metadata const *type);
    pub fn swift_allocBox(type_: *const Metadata) -> BoxPair;

    /// Performs a uniqueness check on the pointer to a box structure. If the
    /// check fails, allocates a new box and stores the pointer in the buffer.
    // BoxPair swift_makeBoxUnique(OpaqueValue *buffer,
    //                             Metadata const *type,
    //                             size_t alignMask);
    pub fn swift_makeBoxUnique(
        buffer: *mut OpaqueValue,
        type_: *const Metadata,
        alignMask: usize,
    ) -> BoxPair;

    /// Return the name of a Swift type represented by a metadata object.
    ///
    /// This is defined in terms of Swift as:
    ///
    /// ```swift
    /// func _getTypeName(_ type: Any.Type, qualified: Bool) -> (UnsafePointer<UInt8>, Int)
    /// ```
    // swift_getTypeName(const Metadata *type, bool qualified);
    pub fn swift_getTypeName(
        buffer: *const Metadata,
        qualified: bool,
    ) -> TypeNamePair;
}
