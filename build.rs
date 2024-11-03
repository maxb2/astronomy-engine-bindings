use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=astronomy/source/c/astronomy.c");
    println!("cargo::rerun-if-changed=astronomy/source/c/astronomy.h");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("astronomy/source/c/astronomy.c")
        .compile("astroeng");

    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("astronomy/source/c/")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("astronomy.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the static library file.
    // let lib_path = libdir_path.join("libastroeng.a");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
