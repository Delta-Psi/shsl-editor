use std::env;
use std::path::Path;

fn main() {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let output_dir = env::var("CMAKECARGO_BUILD_DIR").unwrap();
        let output_path = Path::new(&output_dir).join("shsl-library.h");

        cbindgen::Builder::new()
                .with_crate(crate_dir)
                .with_language(cbindgen::Language::Cxx)
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file(output_path);
}
