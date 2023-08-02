extern crate cbindgen;

use std::env;

fn main() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(cargo_dir)
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("headers/poc_test.h");
}