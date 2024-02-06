use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search={}", env::var("HDF5_LIB_DIR").unwrap());
    println!("cargo:rustc-link-lib=hdf5");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", env::var("HDF5_INCLUDE_DIR").unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.
        write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
