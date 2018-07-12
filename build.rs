extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    Command::new("git").arg("clone")
                       .arg("--recursive")
                       .arg("git://git.ghostscript.com/mupdf.git")
                       .spawn()
                       .expect("Failed to clone MuPDF")
                       .wait()
                       .expect("Failed to wait for git process");
    Command::new("make").arg("-C")
                        .arg("mupdf")
                        .arg("HAVE_X11=no")
                        .arg("HAVE_GLUT=no")
                        .spawn()
                        .expect("Failed to compile MuPDF")
                        .wait()
                        .expect("Failed to wait for make process");

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
