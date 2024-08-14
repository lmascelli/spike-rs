use cmake::Config;
use git2::{build::RepoBuilder, FetchOptions};
use std::{
    env::var,
    path::{Path, PathBuf},
};

const BUILD: bool = true;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    if BUILD {
        //==============================================================================
        //
        //                            GIT CLONE
        //
        //==============================================================================

        let hdf5_dir = var("OUT_DIR")? + "/hdf5";
        let hdf5_path = Path::new(&hdf5_dir);
        let hdf5_install = hdf5_dir.clone() + "/install";
        if hdf5_path.exists() {
            let _repo = git2::Repository::open(hdf5_path)?;
        } else {
            let mut repo = RepoBuilder::new();
            let mut fetch_options = FetchOptions::default();
            fetch_options.depth(1);
            repo.fetch_options(fetch_options);
            // repo.branch("hdf5_1_14_3");
            repo.clone("https://github.com/HDFGroup/hdf5", hdf5_path)?;
        }

        let zlib_dir = var("OUT_DIR")? + "/zlib";
        let zlib_path = Path::new(&zlib_dir);
        let zlib_install = zlib_dir.clone() + "/install";
        if zlib_path.exists() {
            let _repo = git2::Repository::open(zlib_path)?;
        } else {
            let mut repo = RepoBuilder::new();
            let mut fetch_options = FetchOptions::default();
            fetch_options.depth(1);
            repo.fetch_options(fetch_options);
            // repo.branch("hdf5_1_14_3");
            repo.clone("https://github.com/madler/zlib", zlib_path)?;
        }

        //==============================================================================
        //
        //                            CMAKE
        //
        //==============================================================================

        let mut config = Config::new(zlib_path);
        let zlib_install_path = config
            .profile("Release")
            .define("BUILD_SHARED_LIBS", "OFF")
            .out_dir(zlib_install)
            .build();

        let mut config = Config::new(hdf5_path);
        config
            // .define("HDF5_ALLOW_EXTERNAL_SUPPORT", "GIT")
            // .define("HDF5_ENABLE_Z_LIB_SUPPORT", "ON")
            // .define("ZLIB_PACKAGE_NAME", "zlib")
            // .define("ZLIB_GIT_URL", "https://github.com/madler/zlib")
            // .define("ZLIB_GIT_BRANCH", "develop")
            // .define("ZLIB_USE_LOCALCONTENT", "ON")
            // .define("ZLIB_USE_EXTERNAL", "ON")
            .profile("Release");

        // config.define("HDF5_NO_PACKAGES", "ON");
        for option in &[
            "BUILD_SHARED_LIBS",
            "BUILD_TESTING",
            "HDF5_BUILD_TOOLS",
            "HDF5_BUILD_EXAMPLES",
            "HDF5_BUILD_JAVA",
            "HDF5_BUILD_FORTRAN",
            "HDF5_BUILD_CPP_LIB",
            "HDF5_BUILD_UTILS",
            "HDF5_ENABLE_PLUGIN_SUPPORT",
            "HDF5_ENABLE_SZIP_SUPPORT",
            "HDF5_ENABLE_PARALLEL",
            "HDF5_ENABLE_DEPRECATED_SYMBOLS",
            "HDF5_ENABLE_THREADSAFE",
            "ALLOW_UNSUPPORTED",
            "HDF5_BUILD_HL_LIB",
        ] {
            config.define(option, "OFF");
        }

        config.out_dir(hdf5_install);
        let hdf5_install_path = config.build();

        //==============================================================================
        //
        //                            LINK
        //
        //==============================================================================

        println!(
            "cargo:rustc-link-search=native={}",
            (hdf5_install_path.display().to_string() + "/lib")
        );
        println!(
            "cargo:rustc-link-search=native={}",
            (zlib_install_path.display().to_string() + "/lib")
        );
        match target_os.as_str() {
            "linux" => {
                println!("cargo:rustc-link-lib=static=hdf5");
                println!("cargo:rustc-link-lib=static=z");
            }
            "windows" => {
                println!("cargo:rustc-link-lib=static=libhdf5");
                println!("cargo:rustc-link-lib=static=zlibstatic");
                println!("cargo:rustc-link-lib=dylib=shlwapi");
            }
            _ => {
                todo!("this os is not yet supported");
            }
        }

        //==============================================================================
        //
        //                            BINDGEN
        //
        //==============================================================================

        let bindings_builder = bindgen::Builder::default().header("wrapper.h");
        let bindings_builder = bindings_builder.clang_arg(format!(
            "-I{}",
            hdf5_install_path.display().to_string() + "/include"
        ));
        let bindings = bindings_builder
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        Ok(())
    } else {
        if let Ok(hdf5_lib_dir) = var("HDF5_LIB_DIR") {
            println!("cargo:rustc-link-search={}", hdf5_lib_dir);
        }
        println!("cargo:rustc-link-lib=hdf5");

        let bindings_builder = bindgen::Builder::default().header("wrapper.h");
        let bindings_builder =
            if let Ok(hdf5_include_dir) = var("HDF5_INCLUDE_DIR") {
                bindings_builder.clang_arg(format!("-I{}", hdf5_include_dir))
            } else {
                bindings_builder
            };
        let bindings = bindings_builder
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
        Ok(())
    }
}

// use std::env;
// use std::path::PathBuf;
//
// fn main() {
//     if let Ok(hdf5_lib_dir) = env::var("HDF5_LIB_DIR") {
// 	println!("cargo:rustc-link-search={}", hdf5_lib_dir);
//     }
//     println!("cargo:rustc-link-lib=hdf5");
//
//     let bindings_builder =  bindgen::Builder::default()
//         .header("wrapper.h");
//     let bindings_builder = if let Ok(hdf5_include_dir) = env::var("HDF5_INCLUDE_DIR") {
// 	bindings_builder.clang_arg(format!("-I{}", hdf5_include_dir))
//     } else {
// 	bindings_builder
//     };
//     let bindings = bindings_builder
// 	.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
// 	.generate()
// 	.expect("Unable to generate bindings");
//
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings.
//         write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");
// }
