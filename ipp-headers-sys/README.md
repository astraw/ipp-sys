# ipp-headers-sys - rust-bindgen generated FFI declarations for IPP

This crate is part of the `ipp-sys` Rust Intel IPP bindings. See
[ipp-sys](https://github.com/astraw/ipp-sys) for more information.

## Regnerating the 2017 bindings:

    # set IPPROOT env var on windows
    "c:\Program Files (x86)\IntelSWTools\compilers_and_libraries_2017\windows\ipp\bin\ippvars.bat" intel64

    # set IPPROOT env var on mac
    source /opt/intel/compilers_and_libraries_2017/mac/bin/compilervars.sh -arch intel64 -platform mac

    # On all platforms (use %IPPROOT% on Windows)
    bindgen $IPPROOT/include/ipp.h --raw-line "pub use IppStatus::*;" --default-enum-style  "moduleconsts" --with-derive-partialeq --distrust-clang-mangling -o src/ipp_2017.rs

## Regnerating the 2018 bindings:

    # set IPPROOT env var on windows
    "c:\Program Files (x86)\IntelSWTools\compilers_and_libraries_2018\windows\ipp\bin\ippvars.bat" intel64

    # set IPPROOT env var on mac
    source /opt/intel/compilers_and_libraries_2018/mac/bin/compilervars.sh -arch intel64 -platform mac

    # On all platforms (use %IPPROOT% on Windows)
    bindgen $IPPROOT/include/ipp.h --default-enum-style  "moduleconsts" --with-derive-partialeq --distrust-clang-mangling -o src/ipp_2018.rs

## Regnerating the 2019 bindings:

    # set IPPROOT env var on windows
    "c:\Program Files (x86)\IntelSWTools\compilers_and_libraries_2019\windows\ipp\bin\ippvars.bat" intel64

    # set IPPROOT env var on mac
    source /opt/intel/compilers_and_libraries_2019/mac/bin/compilervars.sh -arch intel64 -platform mac

    # On all platforms (use %IPPROOT% on Windows)
    bindgen $IPPROOT/include/ipp.h --default-enum-style  "moduleconsts" --with-derive-partialeq --distrust-clang-mangling -o src/ipp_2019.rs
