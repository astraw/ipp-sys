//! `ipp-sys` - Bindings to Intel Integrated Performance Primitives (Intel IPP).
//!
//! See the [ipp-sys github page](https://github.com/astraw/ipp-sys) for more
//! information, including usage notes.

// The link* creates are used to link the library.
#![allow(unused_extern_crates)]
extern crate ipp_headers_sys;
extern crate link_ippcore;
extern crate link_ippcv;
extern crate link_ippi;
extern crate link_ipps;

pub use ipp_headers_sys::*;
