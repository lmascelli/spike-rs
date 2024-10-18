use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hdf5_include_dir = env::var("HDF5_INCLUDE_DIR").expect("Please set the `HDF5_INCLUDE_DIR` environment variable");
    let hdf5_lib_dir = env::var("HDF5_LIB_DIR").expect("Please set the `HDF5_LIB_DIR` environment variable");
    
    println!("cargo:rerun-if-changed=src/pycode_h5.c");
    println!("cargo:rerun-if-changed=src/pycode_h5.h");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    
    let mut build = cmake::Config::new(".");
    let native_c_location = build.profile("Release").build();

    let bindings = bindgen::Builder::default()
        .header("src/pycode_h5.h")
        .clang_arg(format!("-I{}", hdf5_include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native={}", format!("{}/lib", native_c_location.display()));
    println!("cargo:rustc-link-lib=static={}", "pycode_h5");
    println!("cargo:rustc-link-search=native={}", format!("{hdf5_lib_dir}"));
    println!("cargo:rustc-link-lib=dylib={}", "hdf5");
    Ok(())
}
