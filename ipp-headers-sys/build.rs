extern crate bindgen;

use std::env;
use std::path::{Path,PathBuf};

fn main() {
    let ipproot = match env::var("IPPROOT") {
        Ok(dir) => dir,
        Err(e) => {
            panic!("Environment variable IPPROOT could not be read: {}", e);
        }
    };

    let ipp_path = Path::new(&ipproot);
    let includedir = ipp_path.join("include");

    let header = includedir.join("ipp.h");

    if !header.is_file() {
        panic!("no header found at {:?}, cannot proceed.", header);
    }

    let arg = format!("-I{}",includedir.to_str().unwrap());

    let builder = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .clang_arg(arg)
            .raw_line("extern crate ipp_ctypes;")
            .ctypes_prefix("ipp_ctypes")
            .constified_enum_module("Ipp.*")
            // .rustfmt_bindings(false)
            .derive_partialeq(true);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
