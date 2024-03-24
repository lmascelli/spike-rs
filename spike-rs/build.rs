use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(hdf5_lib_dir) = env::var("HDF5_LIB_DIR") {
	println!("cargo:rustc-link-search={}", hdf5_lib_dir);
    } 
    println!("cargo:rustc-link-lib=hdf5");

    let bindings_builder =  bindgen::Builder::default()
        .header("wrapper.h");
    let bindings_builder = if let Ok(hdf5_include_dir) = env::var("HDF5_INCLUDE_DIR") {
	bindings_builder.clang_arg(format!("-I{}", hdf5_include_dir))
    } else {
	bindings_builder
    };
    let bindings = bindings_builder
	.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
	.generate()
	.expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.
        write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
