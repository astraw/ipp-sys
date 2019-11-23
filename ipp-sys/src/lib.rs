/*! # What is this crate?

This crate provides wrappers of the Intel Integrated Performance Primitives
(Intel IPP) library made with rust-bindgen.

# Version support

Use the `2017`, `2018`, or `2019` cargo feature to use IPP 2017, 2018, or
2019 respectively.

# Download IPP

Get IPP from https://software.intel.com/en-us/intel-ipp

# Usage

Compile IPP statically into your app by setting environment variable
`IPP_STATIC=1`.

You must set the `IPPROOT` environment variable at compilation time. This is
used by the `build.rs` files to find your IPP installation. Typically, this is
set using a tool provided by Intel with IPP and run as follows.

On Linux:

```text
source /opt/intel/compilers_and_libraries_2019/linux/ipp/bin/ippvars.sh -arch intel64 -platform linux
```

On Mac:

```text
source /opt/intel/compilers_and_libraries_2019/mac/bin/compilervars.sh -arch intel64 -platform mac
```

On Windows:

```text
"C:\Program Files (x86)\IntelSWTools\compilers_and_libraries_2019\windows\ipp\bin\ippvars.bat" intel64
```

In `Cargo.toml`, include `ipp-sys` as a dependency with a feature to select
the IPP version used:

```text
[dependencies]
ipp-sys = { version = "0.4", features=["2019"] }
```

Now, you can use `ipp_sys` in your crate. You might like to check that you
compiled the correct version by doing something like:

```
// Get the version from IPP at runtime.
let linked_version_major = unsafe{ (*ipp_sys::ippGetLibVersion()).major };
let linked_version_minor = unsafe{ (*ipp_sys::ippGetLibVersion()).minor };

// Compare the runtime major version with the compile-time major version.
assert_eq!( linked_version_major as i32, ipp_sys::IPP_VERSION_MAJOR as i32);
// And compare the minor version, too.
assert_eq!( linked_version_minor as i32, ipp_sys::IPP_VERSION_MINOR as i32);

```

# API documentation

See the [ipp-headers-sys page on docs.rs](http://docs.rs/ipp-headers-sys).

*/

// The link* creates are used to link the library.
#![allow(unused_extern_crates)]
extern crate ipp_headers_sys;
#[cfg(feature = "do-linking")]
extern crate link_ippcore;
#[cfg(feature = "do-linking")]
extern crate link_ippcv;
#[cfg(feature = "do-linking")]
extern crate link_ippi;
#[cfg(feature = "do-linking")]
extern crate link_ipps;
#[cfg(feature = "do-linking")]
extern crate link_ippvm;

pub use ipp_headers_sys::*;
