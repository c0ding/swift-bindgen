use crate::OpaqueValue;
use std::ptr;

/// Kinds of Swift metadata records. Some of these are types, some aren't.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
// enum class MetadataKind : uint32_t
pub struct MetadataKind(pub u32);

const NON_TYPE_FLAG: u32 = 0x400;
const NON_HEAP_FLAG: u32 = 0x200;
const RUNTIME_PRIVATE_FLAG: u32 = 100;

// Documentation taken from `docs/ABI/TypeMetadata.rst`
impl MetadataKind {
    /// The largest possible non-isa-pointer metadata kind value.
    pub const LAST_ENUMERATED: Self = Self(0x7FF);

    /// Returns whether `self` is type metadata.
    #[inline]
    pub const fn is_type(&self) -> bool {
        self.0 & NON_TYPE_FLAG == 0
    }

    /// Returns whether `self` represents the native metadata kind for a Swift
    /// nominal type.
    #[inline]
    pub const fn is_nominal_type(&self) -> bool {
        self.is_class() | self.is_struct() | self.is_enum() | self.is_optional()
    }

    /// Returns whether `self` is runtime-private.
    ///
    /// External tools should not rely on the stability of these values or the
    /// precise binary layout of their associated data structures.
    #[inline]
    pub const fn is_runtime_private(&self) -> bool {
        self.0 & RUNTIME_PRIVATE_FLAG != 0
    }

    /// Returns whether `self` should be treated as an isa pointer.
    #[inline]
    pub const fn is_isa_ptr(&self) -> bool {
        self.0 > Self::LAST_ENUMERATED.0
    }

    /// Returns `self` as an isa pointer, or null if it isn't one.
    #[inline]
    pub fn to_isa_ptr(self) -> *const OpaqueValue {
        if self.is_isa_ptr() {
            self.0 as *const OpaqueValue
        } else {
            ptr::null()
        }
    }
}

macro_rules! kinds {
    ($(
        $(#[$kind_meta:meta])+
        $kind:ident = $value:expr;

        $(#[$is_kind_meta:meta])+
        $is_kind:ident;
    )+) => {
        /// Constants from [`include/swift/ABI/MetadataKind.def`][file].
        ///
        /// [file]: https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataKind.def
        impl MetadataKind {
            $(
                $(#[$kind_meta])+
                pub const $kind: Self = Self($value);
            )+

            $(
                $(#[$is_kind_meta])+
                pub const fn $is_kind(&self) -> bool {
                    self.0 == Self::$kind.0
                }
            )+
        }
    };
}

kinds! {
    /// A class type.
    CLASS = 0;

    /// Returns `true` if `self` is a class type.
    is_class;

    /// A struct type.
    STRUCT = 0 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a struct type.
    is_struct;

    /// An enum type.
    ENUM = 1 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an enum type.
    is_enum;

    /// An optional type.
    OPTIONAL = 2 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an optional type.
    is_optional;

    /// A foreign class, such as a Core Foundation class.
    FOREIGN_CLASS = 3 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a foreign class, such as a Core Foundation
    /// class.
    is_foreign_class;

    /// A type whose value is not exposed in the metadata system.
    OPAQUE = 0 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a type whose value is not exposed in the
    /// metadata system.
    is_opaque;

    /// A tuple.
    TUPLE = 1 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a tuple.
    is_tuple;

    /// A monomorphic function.
    FUNCTION = 2 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a monomorphic function.
    is_function;

    /// An existential type.
    EXISTENTIAL = 3 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an existential type.
    is_existential;

    /// A metatype.
    METATYPE = 4 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a metatype.
    is_metatype;

    /// An ObjC class wrapper.
    OBJC_CLASS_WRAPPER = 5 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an ObjC class wrapper.
    is_objc_class_wrapper;

    /// An existential metatype.
    EXISTENTIAL_METATYPE = 6 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an existential metatype.
    is_existential_metatype;

    /// A heap-allocated local variable using statically-generated metadata.
    HEAP_LOCAL_VARIABLE = 0 | NON_TYPE_FLAG;

    /// Returns `true` if `self` is a heap-allocated local variable using
    /// statically-generated metadata.
    is_heap_local_variable;

    /// A heap-allocated local variable using runtime-instantiated metadata.
    HEAP_GENERIC_LOCAL_VARIABLE = 0 | NON_TYPE_FLAG | RUNTIME_PRIVATE_FLAG;

    /// Returns `true` if `self` is a heap-allocated local variable using
    /// runtime-instantiated metadata.
    is_heap_generic_local_variable;

    /// A native error object.
    ERROR_OBJECT = 1 | NON_TYPE_FLAG | RUNTIME_PRIVATE_FLAG;

    /// Returns `true` if `self` is a native error object.
    is_error_object;
}
