extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=mupdf");
    println!("cargo:rustc-link-lib=static=mupdf-third");
    println!("cargo:rustc-link-lib=static=mupdf-threads");
    println!("cargo:rustc-link-search=native=mupdf/build/release");

    let bindings = bindgen::Builder::default()
                                    .header("wrapper.h")
                                    .generate()
                                    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("mupdf_bindings.rs"))
            .expect("Couldn't write bindings!");
}
