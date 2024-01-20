use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=E:/rust/hdf5/lib");
    println!("cargo:rustc-link-search=c:/Program Files (x86)/Windows Kits/10/Lib/10.0.22000.0/um/x64/");
    println!("cargo:rustc-link-lib=static=libhdf5");
    println!("cargo:rustc-link-lib=static=libaec");
    println!("cargo:rustc-link-lib=static=libszaec");
    println!("cargo:rustc-link-lib=static=libzlib");
    println!("cargo:rustc-link-lib=static=Shlwapi");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-IE:/rust/hdf5/include")
        .clang_arg(&format!("--target={}", env::var("TARGET").unwrap()))
        .trust_clang_mangling(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
