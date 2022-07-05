// build.rs

extern crate cbindgen;

use std::env;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let _package_name = env::var("CARGO_PKG_NAME").unwrap();
    let main_language = cbindgen::Language::C;
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(main_language)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}

