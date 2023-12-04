extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("lightnet/x86_64")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");


    let main_header_path = PathBuf::from("lightnet/include/lightnet.h")
        .canonicalize()
        .expect("cannot canonicalize path");


    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=lightnet");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header(main_header_path.to_str().unwrap())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}