use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = env::var("CMAKECARGO_BUILD_DIR").unwrap_or(env::var("OUT_DIR").unwrap());
    let output_path = Path::new(&output_dir).join("shsl-library_ffi.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::Cxx)
        .with_namespace("ffi")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_path);
}
