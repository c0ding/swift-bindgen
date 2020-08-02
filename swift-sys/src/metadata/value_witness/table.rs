use std::os::raw::{c_uint, c_void};

// TODO: Verify that these sizes are correct.
assert_eq_size!(ValueWitnessTable, [u8; 88]);
assert_eq_size!(EnumValueWitnessTable, [u8; 112]);

/// A vtable of functions that implement value semantics of a type.
///
/// Provides fundamental operations such as allocating, copying, and destroying
/// values of the type. The value witness table also records the size,
/// alignment, stride, and other fundamental properties of the type.
///
/// Equivalent to fields in `#if WANT_REQUIRED_VALUE_WITNESSES` in
/// [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def).
/// Fields for `#if WANT_ENUM_VALUE_WITNESSES` are in
/// [`EnumValueWitnessTable`](struct.EnumValueWitnessTable.html).
#[repr(C)]
#[derive(Debug)]
pub struct ValueWitnessTable {
    /// Given an invalid buffer `dest`, initialize it as a copy of the object in
    /// the `src` buffer.
    ///
    /// Seen as:
    ///
    /// ```c
    /// T *(*initializeBufferWithCopyOfBuffer)(B *dest, B *src, M *self);
    /// ```
    pub initialize_buffer_with_copy_of_buffer:
        unsafe extern "C" fn(
            dest: *mut c_void,
            src: *mut c_void,
            self_: *mut c_void,
        ) -> *mut c_void,

    /// Given a valid object of this type, destroy it, leaving it as an invalid
    /// object. This is useful when generically destroying an object which has
    /// been allocated in-line, such as an array, struct, or tuple element.
    ///
    /// Seen as:
    ///
    /// ```c
    /// void (*destroy)(T *object, witness_t *self);
    /// ```
    pub destroy: unsafe extern "C" fn(object: *mut c_void, self_: *mut c_void),

    /// Given an invalid object of this type, initialize it as a copy of the
    /// `src` object. Returns the `dest` object.
    ///
    /// Seen as:
    ///
    /// ```c
    /// T *(*initializeWithCopy)(T *dest, T *src, M *self);
    /// ```
    pub initialize_with_copy: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *mut c_void,
    ) -> *mut c_void,

    /// Given a valid object of this type, change it to be a copy of the `src`
    /// object. Returns the `dest` object.
    ///
    /// Seen as:
    ///
    /// ```c
    /// T *(*assignWithCopy)(T *dest, T *src, M *self);
    /// ```
    pub assign_with_copy: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *mut c_void,
    ) -> *mut c_void,

    /// Given an invalid object of this type, initialize it by taking the value
    /// of the source object. The `src` object becomes invalid. Returns the
    /// `dest` object.
    ///
    /// Seen as:
    ///
    /// ```c
    /// T *(*initializeWithTake)(T *dest, T *src, M *self);
    /// ```
    pub initialize_with_take: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *mut c_void,
    ) -> *mut c_void,

    /// Given a valid object of this type, change it to be a copy of the `src`
    /// object. The source object becomes invalid. Returns the `dest` object.
    ///
    /// Seen as:
    ///
    /// ```c
    /// T *(*assignWithTake)(T *dest, T *src, M *self);
    /// ```
    pub assign_with_take: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *mut c_void,
    ) -> *mut c_void,

    /// Given an instance of valid single payload enum with a payload of this
    /// witness table's type (e.g `Optional<ThisType>`), get the tag of the
    /// enum.
    ///
    /// Seen as:
    ///
    /// ```c
    /// unsigned (*getEnumTagSinglePayload)(const T* enum, UINT_TYPE emptyCases);
    /// ```
    pub get_enum_tag_single_payload: unsafe extern "C" fn(
        enum_: *const c_void,
        empty_cases: c_uint,
        self_: *mut c_void,
    ) -> *mut c_uint,

    /// Given uninitialized memory for an instance of a single payload enum with
    /// a payload of this witness table's type (e.g `Optional<ThisType>`), store
    /// the tag.
    ///
    /// Seen as:
    ///
    /// ```c
    /// void (*storeEnumTagSinglePayload)(T* enum,
    ///                                   UINT_TYPE whichCase,
    ///                                   UINT_TYPE emptyCases);
    /// ```
    pub store_enum_tag_single_payload: unsafe extern "C" fn(
        enum_: *mut c_void,
        which_case: c_uint,
        empty_cases: c_uint,
        self_: *mut c_void,
    ) -> *mut c_uint,

    /// The required storage size of a single object of this type.
    pub size: usize,

    /// The required size per element of an array of this type. It is at least
    /// one, even for zero-sized types, like the empty tuple.
    pub stride: usize,

    /// The ValueWitnessAlignmentMask bits represent the required alignment of
    /// the first byte of an object of this type, expressed as a mask of the low
    /// bits that must not be set in the pointer. This representation can be
    /// easily converted to the 'alignof' result by merely adding 1, but it is
    /// more directly useful for performing dynamic structure layouts, and it
    /// grants an additional bit of precision in a compact field without needing
    /// to switch to an exponent representation.
    ///
    /// The ValueWitnessIsNonPOD bit is set if the type is not POD.
    ///
    /// The ValueWitnessIsNonInline bit is set if the type cannot be represented
    /// in a fixed-size buffer or if it is not bitwise takable.
    ///
    /// The ExtraInhabitantsMask bits represent the number of "extra
    /// inhabitants" of the bit representation of the value that do not form
    /// valid values of the type.
    ///
    /// The Enum_HasSpareBits bit is set if the type's binary representation has
    /// unused bits.
    ///
    /// The HasEnumWitnesses bit is set if the type is an enum type.
    pub flags: c_uint,

    /// The number of extra inhabitants in the type.
    pub extra_inhabitant_count: c_uint,
}

/// A value-witness table with enum entry points.
///
/// Equivalent to `EnumValueWitnessTable` in
/// [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/Runtime/Metadata.h).
///
/// This includes all fields within `#if WANT_ENUM_VALUE_WITNESSES` in
/// [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def).
#[repr(C)]
#[derive(Debug)]
pub struct EnumValueWitnessTable {
    /// The base value witness table.
    pub base: ValueWitnessTable,

    /// Given a valid object of this `enum` type, extracts the tag value
    /// indicating which case of the enum is inhabited. Returned values are in
    /// the range `[0..NumElements-1]`.
    ///
    /// Seen as:
    ///
    /// ```c
    /// unsigned (*getEnumTag)(T *obj, M *self);
    /// ```
    pub get_enum_tag:
        unsafe extern "C" fn(obj: *mut c_void, self_: *mut c_void) -> c_uint,

    /// Given a valid object of this enum type, destructively extracts the
    /// associated payload.
    ///
    /// Seen as:
    ///
    /// ```c
    /// void (*destructiveProjectEnumData)(T *obj, M *self);
    /// ```
    pub destructive_project_enum_data:
        unsafe extern "C" fn(obj: *mut c_void, self_: *mut c_void),

    /// Given an enum case tag and a valid object of case's payload type,
    /// destructively inserts the tag into the payload. The given tag value must
    /// be in the range `[-ElementsWithPayload..ElementsWithNoPayload-1]`.
    ///
    /// Seen as:
    ///
    /// ```c
    /// void (*destructiveInjectEnumTag)(T *obj, unsigned tag, M *self);
    /// ```
    pub destructive_inject_enum_data:
        unsafe extern "C" fn(obj: *mut c_void, tag: c_uint, self_: *mut c_void),
}
