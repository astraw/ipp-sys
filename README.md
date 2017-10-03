# ipp-sys - bindings to Intel IPP

This directory contains several crates:

 - `ipp-sys`: toplevel convenience crate that depends on `ipp-headers-sys` and `link-*` crates
 - `ipp-ctypes`: C types used by IPP
 - `ipp-headers-sys`: provide FFI headers from `ipp.h` (made with `bindgen`)
 - `ipp-sys-build-help`: helper for build.rs in the `link-*` crates
 - `link-ippcore`: link ippcore library
 - `link-ippcv`: link ippcv library
 - `link-ippi`: link ippi library
 - `link-ipps`: link ipps library

Typically, you can just depend on `ipp-sys`. The `link-*` crates do not provide any rust code, but only serve
to link the relevant IPP library.

## Download IPP

Get IPP from https://software.intel.com/en-us/intel-ipp

## Usage

Compile IPP statically into your app by setting environment variable `IPP_STATIC=1`.

You must set the `IPPROOT` environment variable. This is used by the `build.rs` files in the crates here to find your IPP installation. Typically, this is set using a tool provided by Intel with IPP and run as follows.

On Linux:

    source /opt/intel/bin/compilervars.sh -arch intel64 -platform linux

On Mac:

    source /opt/intel/compilers_and_libraries/mac/bin/compilervars.sh -arch intel64 -platform mac

On Windows:

    "C:\Program Files (x86)\IntelSWTools\compilers_and_libraries_2017\windows\ipp\bin\ippvars.bat" intel64

Now you can build your crate that depends on IPP with a standard cargo command, such as:

    # Inside the path with your crate's Cargo.toml
    cargo build

## License

The `ipp-sys` Rust crates are licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Please consult documentation for the [Intel
IPP](https://software.intel.com/en-us/intel-ipp) library for its license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

## Code of conduct

Anyone who interacts with ipp-sys in any space including but not
limited to this GitHub repository is expected to follow our [code of
conduct](https://github.com/astraw/ipp-sys/blob/master/code_of_conduct.md).
