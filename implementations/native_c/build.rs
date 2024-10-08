use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=src/pycode_h5.c");
    println!("cargo:rerun-if-changed=src/pycode_h5.h");
    
    let mut build = cmake::Config::new(".");
    let native_c_location = build.build();

    let bindings = bindgen::Builder::default()
        .header("src/pycode_h5.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native={}", format!("{}/lib", native_c_location.display()));
    println!("cargo:rustc-link-lib=static={}", "pycode_h5");
    println!("cargo:rustc-link-lib={}", "hdf5");
    Ok(())
}
