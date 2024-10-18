use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hdf5_lib_dir = env::var("HDF5_LIB_DIR").expect("Please set the `HDF5_LIB_DIR` environment variable");

    println!("cargo:rustc-link-search=native={}", format!("{hdf5_lib_dir}"));
    println!("cargo:rustc-link-lib={}", "hdf5");

    Ok(())
}
