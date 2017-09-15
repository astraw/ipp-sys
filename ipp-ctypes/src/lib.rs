//! This crate exists to isolate the C types used by IPP.

#![allow(non_camel_case_types)]
pub use ::std::os::raw::{c_int, c_uint, c_ulonglong, c_longlong,
    c_char, c_uchar, c_ushort, c_schar, c_short, c_void};
