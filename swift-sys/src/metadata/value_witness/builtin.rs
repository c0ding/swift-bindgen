//! Value-witness tables of built-in types.
//!
//! Everything within this module is sourced from:
//! - [`BuiltinTypes.def`](https://github.com/apple/swift/blob/master/include/swift/Runtime/BuiltinTypes.def)
//! - [`ManglingMacros.h`](https://github.com/apple/swift/blob/master/include/swift/Demangling/ManglingMacros.h)

use super::ValueWitnessTable;

// Equivalent to `VALUE_WITNESS_SYM` in
// https://github.com/apple/swift/blob/master/include/swift/Demangling/ManglingMacros.h
macro_rules! VALUE_WITNESS_SYM {
    ($sym:expr) => {
        concat!("$s", $sym, "WV")
    };
}

/// Creates `extern "C"` link declarations for value-witness of built-in types.
macro_rules! builtins {
    ($($name:ident => $sym:expr, $builtin:expr;)+) => {
        builtins! {
            @processed $(
                $name =>
                    VALUE_WITNESS_SYM!($sym),
                    concat!("Table for `Builtin.", $builtin, "`.");
            )+
        }
    };
    (@processed $($name:ident => $link_name:expr, $doc:expr;)+) => {
        extern "C" {$(
            #[doc = $doc]
            #[link_name = $link_name]
            pub static $name: ValueWitnessTable;
        )+}
    };
}

/// Simplifies calling `builtins!` for SIMD types.
macro_rules! vectors {
    ($($name:ident => $size:expr, $sym:expr, $builtin:expr;)+) => {
        builtins! {$(
            $name => concat!($sym, "Bv", $size),
                     concat!("Vec", $size, "x", $builtin);
        )+}
    };
}

builtins! {
    I1   => "Bi1_",   "Int1";
    I7   => "Bi7_",   "Int7";
    I8   => "Bi8_",   "Int8";
    I16  => "Bi16_",  "Int16";
    I32  => "Bi32_",  "Int32";
    I63  => "Bi63_",  "Int63";
    I64  => "Bi64_",  "Int64";
    I128 => "Bi128_", "Int128";
    I256 => "Bi256_", "Int256";
    I512 => "Bi512_", "Int512";

    WORD => "Bw", "Word";

    F16  => "Bf16_",  "FPIEEE16";
    F32  => "Bf32_",  "FPIEEE32";
    F64  => "Bf64_",  "FPIEEE64";
    F80  => "Bf80_",  "FPIEEE80";
    F128 => "Bf128_", "FPIEEE128";

    NATIVE_OBJECT       => "Bo", "NativeObject";
    BRIDGE_OBJECT       => "Bb", "BridgeObject";
    RAW_POINTER         => "Bp", "RawPointer";
    UNSAFE_VALUE_BUFFER => "BB", "UnsafeValueBuffer";
    UNKNOWN_OBJECT      => "BO", "UnknownObject";
}

vectors! {
    VEC_I8_2  => "2",  "Bi8_", "Int8";
    VEC_I8_3  => "3",  "Bi8_", "Int8";
    VEC_I8_4  => "4",  "Bi8_", "Int8";
    VEC_I8_8  => "8",  "Bi8_", "Int8";
    VEC_I8_16 => "16", "Bi8_", "Int8";
    VEC_I8_32 => "32", "Bi8_", "Int8";
    VEC_I8_64 => "64", "Bi8_", "Int8";

    VEC_I16_2  => "2",  "Bi16_", "Int16";
    VEC_I16_3  => "3",  "Bi16_", "Int16";
    VEC_I16_4  => "4",  "Bi16_", "Int16";
    VEC_I16_8  => "8",  "Bi16_", "Int16";
    VEC_I16_16 => "16", "Bi16_", "Int16";
    VEC_I16_32 => "32", "Bi16_", "Int16";
    VEC_I16_64 => "64", "Bi16_", "Int16";

    VEC_I32_2  => "2",  "Bi32_", "Int32";
    VEC_I32_3  => "3",  "Bi32_", "Int32";
    VEC_I32_4  => "4",  "Bi32_", "Int32";
    VEC_I32_8  => "8",  "Bi32_", "Int32";
    VEC_I32_16 => "16", "Bi32_", "Int32";
    VEC_I32_32 => "32", "Bi32_", "Int32";
    VEC_I32_64 => "64", "Bi32_", "Int32";

    VEC_I64_2  => "2",  "Bi64_", "Int64";
    VEC_I64_3  => "3",  "Bi64_", "Int64";
    VEC_I64_4  => "4",  "Bi64_", "Int64";
    VEC_I64_8  => "8",  "Bi64_", "Int64";
    VEC_I64_16 => "16", "Bi64_", "Int64";
    VEC_I64_32 => "32", "Bi64_", "Int64";
    VEC_I64_64 => "64", "Bi64_", "Int64";

    VEC_F32_2  => "2",  "Bf32_", "FPIEEE32";
    VEC_F32_3  => "3",  "Bf32_", "FPIEEE32";
    VEC_F32_4  => "4",  "Bf32_", "FPIEEE32";
    VEC_F32_8  => "8",  "Bf32_", "FPIEEE32";
    VEC_F32_16 => "16", "Bf32_", "FPIEEE32";
    VEC_F32_32 => "32", "Bf32_", "FPIEEE32";
    VEC_F32_64 => "64", "Bf32_", "FPIEEE32";

    VEC_F64_2  => "2",  "Bf64_", "FPIEEE64";
    VEC_F64_3  => "3",  "Bf64_", "FPIEEE64";
    VEC_F64_4  => "4",  "Bf64_", "FPIEEE64";
    VEC_F64_8  => "8",  "Bf64_", "FPIEEE64";
    VEC_F64_16 => "16", "Bf64_", "FPIEEE64";
    VEC_F64_32 => "32", "Bf64_", "FPIEEE64";
    VEC_F64_64 => "64", "Bf64_", "FPIEEE64";
}
