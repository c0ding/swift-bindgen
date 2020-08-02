//! Swift objects.

use crate::sys::heap::{self, HeapObject, Metadata, WeakReference};
use std::{fmt, os::raw::c_int, ptr::NonNull};

/// An object that may be owned or unowned.
#[repr(transparent)]
pub struct Object(HeapObject);

// Safe because `Object` is only ever used behind a reference
unsafe impl Send for Object {}
unsafe impl Sync for Object {}

impl Object {
    /// Returns `self` as a `*mut` pointer suitable for FFI.
    #[inline]
    pub const fn as_ptr(&self) -> *mut HeapObject {
        self as *const Self as *mut HeapObject
    }

    /// Returns the strong retain count of `self`.
    #[inline]
    pub fn strong_retain_count(&self) -> usize {
        unsafe { heap::swift_retainCount(self.as_ptr()) }
    }

    /// Returns the unowned retain count of `self`.
    #[inline]
    pub fn unowned_retain_count(&self) -> usize {
        unsafe { heap::swift_unownedRetainCount(self.as_ptr()) }
    }

    /// Returns the weak retain count of `self`.
    #[inline]
    pub fn weak_retain_count(&self) -> usize {
        unsafe { heap::swift_weakRetainCount(self.as_ptr()) }
    }
}

/// An owned object.
#[derive(Debug)]
#[repr(transparent)]
pub struct Owned(NonNull<HeapObject>);

impl Drop for Owned {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.release() };
    }
}

impl Clone for Owned {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { self.retain() };
        Self(self.0)
    }
}

impl AsRef<Object> for Owned {
    #[inline]
    fn as_ref(&self) -> &Object {
        self.as_obj()
    }
}

impl AsMut<Object> for Owned {
    #[inline]
    fn as_mut(&mut self) -> &mut Object {
        self.as_obj_mut()
    }
}

unsafe impl Send for Owned {}
// TODO: Find out if owned object references can be shared safely
// unsafe impl Sync for Owned {}

impl fmt::Pointer for Owned {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Owned {
    /// Creates a new instance from `ptr`.
    ///
    /// # Safety
    ///
    /// `ptr` must reference a valid Swift object.
    #[inline]
    pub const unsafe fn from_ptr<T>(ptr: *mut T) -> Self {
        Self(NonNull::new_unchecked(ptr).cast())
    }

    /// Returns `self` as a `*mut` pointer suitable for FFI.
    #[inline]
    pub const fn as_ptr(&self) -> *mut HeapObject {
        self.0.as_ptr()
    }

    /// Returns `self` as a `NonNull` pointer suitable for FFI.
    #[inline]
    pub const fn as_non_null(&self) -> NonNull<HeapObject> {
        self.0
    }

    /// Returns a shared reference to the underlying object.
    #[inline]
    pub fn as_obj(&self) -> &Object {
        unsafe { &*self.as_ptr().cast() }
    }

    /// Returns a mutable reference to the underlying object.
    #[inline]
    pub fn as_obj_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.as_ptr().cast() }
    }

    /// Returns the strong retain count of `self`.
    #[inline]
    pub fn retain_count(&self) -> usize {
        self.as_obj().strong_retain_count()
    }

    /// Increments the strong retain count of `self`.
    #[inline]
    pub unsafe fn retain(&self) {
        heap::swift_retain(self.as_ptr());
    }

    /// Increments the strong retain count of `self` by `n`.
    #[inline]
    pub unsafe fn retain_n(&self, n: u32) {
        heap::swift_retain_n(self.as_ptr(), n);
    }

    /// Increments the strong retain count of `self` in a non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_retain(&self) {
        heap::swift_nonatomic_retain(self.as_ptr());
    }

    /// Increments the unowned retain count of `self` by `n` in a
    /// non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_retain_n(&self, n: u32) {
        heap::swift_nonatomic_retain_n(self.as_ptr(), n);
    }

    /// Clones `self` in a non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_clone(&self) -> Self {
        self.nonatomic_retain();
        Self(self.0)
    }

    /// Decrements the strong retain count of `self`.
    #[inline]
    pub unsafe fn release(&self) {
        heap::swift_release(self.as_ptr());
    }

    /// Sets the `RC_DEALLOCATING_FLAG` flag non-atomically.
    ///
    /// # Safety
    ///
    /// The strong reference count of `self` must be 1 and no other thread may
    /// retain the object during executing this function.
    #[inline]
    pub unsafe fn set_deallocating(&self) {
        heap::swift_setDeallocating(self.as_ptr());
    }
}

/// An unowned object.
#[derive(Debug)]
#[repr(transparent)]
pub struct Unowned(NonNull<HeapObject>);

impl Drop for Unowned {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.release() };
    }
}

impl Clone for Unowned {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { heap::swift_unownedRetain(self.as_ptr()) };
        Self(self.0)
    }
}

impl AsRef<Object> for Unowned {
    #[inline]
    fn as_ref(&self) -> &Object {
        self.as_obj()
    }
}

impl AsMut<Object> for Unowned {
    #[inline]
    fn as_mut(&mut self) -> &mut Object {
        self.as_obj_mut()
    }
}

unsafe impl Send for Unowned {}
// TODO: Find out if unowned object references can be shared safely
// unsafe impl Sync for Unowned {}

impl fmt::Pointer for Unowned {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Unowned {
    /// Creates a new instance from `ptr`.
    ///
    /// # Safety
    ///
    /// `ptr` must reference a valid Swift object.
    #[inline]
    pub const unsafe fn from_ptr<T>(ptr: *mut T) -> Self {
        Self(NonNull::new_unchecked(ptr).cast())
    }

    /// Returns `self` as a `*mut` pointer suitable for FFI.
    #[inline]
    pub const fn as_ptr(&self) -> *mut HeapObject {
        self as *const Self as *mut HeapObject
    }

    /// Returns a shared reference to the underlying object.
    #[inline]
    pub fn as_obj(&self) -> &Object {
        unsafe { &*self.as_ptr().cast() }
    }

    /// Returns a mutable reference to the underlying object.
    #[inline]
    pub fn as_obj_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.as_ptr().cast() }
    }

    /// Returns the unowned retain count of `self`.
    #[inline]
    pub fn retain_count(&self) -> usize {
        self.as_obj().unowned_retain_count()
    }

    /// Increments the unowned retain count of `self`.
    #[inline]
    pub unsafe fn retain(&self) {
        heap::swift_unownedRetain(self.as_ptr());
    }

    /// Increments the strong retain count of `self` by `n`.
    #[inline]
    pub unsafe fn retain_n(&self, n: u32) {
        heap::swift_unownedRetain_n(self.as_ptr(), n as c_int);
    }

    /// Increments the unowned retain count of `self` in a non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_retain(&self) {
        heap::swift_nonatomic_unownedRetain(self.as_ptr());
    }

    /// Increments the unowned retain count of `self` by `n` in a
    /// non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_retain_n(&self, n: u32) {
        heap::swift_nonatomic_unownedRetain_n(self.as_ptr(), n as c_int);
    }

    /// Clones `self` in a non-thread-safe way.
    #[inline]
    pub unsafe fn nonatomic_clone(&self) -> Self {
        self.nonatomic_retain();
        Self(self.0)
    }

    /// Decrements the unowned retain count of `self`.
    #[inline]
    pub unsafe fn release(&self) {
        heap::swift_unownedRelease(self.as_ptr());
    }
}

/// A weak object.
#[derive(Debug)]
#[repr(transparent)]
pub struct Weak(NonNull<WeakReference>);

impl Drop for Weak {
    #[inline]
    fn drop(&mut self) {
        unsafe { heap::swift_weakDestroy(self.0.as_ptr()) };
    }
}

impl Clone for Weak {
    #[inline]
    fn clone(&self) -> Self {
        unimplemented!("TODO: Figure out how to copy weak references")
    }
}

unsafe impl Send for Weak {}
// TODO: Find out if weak object references can be shared safely
// unsafe impl Sync for Weak {}

impl fmt::Pointer for Weak {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Weak {
    /// Loads the underlying object from `self`, or returns `None` if it has
    /// been deallocated.
    #[inline]
    pub fn load(&self) -> Option<Owned> {
        unsafe {
            let obj = heap::swift_weakLoadStrong(self.0.as_ptr());
            let ptr = NonNull::new(obj)?;
            Some(Owned(ptr))
        }
    }

    /// Consumes `self`, returning the underlying object or `None` if it has
    /// been deallocated.
    #[inline]
    pub fn take(self) -> Option<Owned> {
        unsafe {
            let obj = heap::swift_weakTakeStrong(self.0.as_ptr());
            let ptr = NonNull::new(obj)?;
            Some(Owned(ptr))
        }
    }
}

/// The metadata of some type; synonymous with `Any.Type`.
pub struct MetaType(Metadata);

impl fmt::Debug for MetaType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MetaType")
            .field(&NonNull::from(self))
            .finish()
    }
}

impl MetaType {
    /// Returns the name of the underlying type.
    #[inline]
    pub fn name(&self, qualified: bool) -> &str {
        unsafe {
            let name = heap::swift_getTypeName(&self.0, qualified);
            std::str::from_utf8_unchecked(name.into_bytes())
        }
    }
}
