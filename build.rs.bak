use std::fmt::Write;
use std::path::PathBuf;
use std::{env, fmt};

const LIBHDF5: Lib = Lib::prefixed("hdf5");
const LIBAEC: Lib = Lib::prefixed("aec");
const LIBSZAEC: Lib = Lib::prefixed("szaec");
const ZLIB: Lib = Lib {
    name: "zlib",
    lib_name: "z",
    win_lib_name: "zlib",
    prefixed: true,
};

// NOTE: these env vars should work on windows
// LIB="E:/rust/hdf5/lib;C:/Program Files (x86)/Windows Kits/10/Lib/10.0.22000.0/um/x64"
// CPATH="E:/rust/hdf5/include"

fn main() {
    let mut ctx = Context::new();
    ctx.link(LIBHDF5);
    ctx.link(LIBAEC);
    if ctx.is_win() {
        ctx.link(LIBSZAEC);
    }
    ctx.link(ZLIB);
    if ctx.is_win() {
        println!("cargo:rustc-link-lib=dylib=Shlwapi");
    }

    let bindings = ctx.build();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

struct Context {
    bindings: bindgen::Builder,
    is_win: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Lib {
    pub name: &'static str,
    pub lib_name: &'static str,
    pub win_lib_name: &'static str,
    pub prefixed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LinkType {
    Unknown,
    Static,
    Dynamic,
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            &match self {
                Self::Unknown => "",
                Self::Static => "static=",
                Self::Dynamic => "dylib=",
            },
            f,
        )
    }
}

impl Lib {
    #[allow(dead_code)]
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            lib_name: name,
            win_lib_name: name,
            prefixed: false,
        }
    }

    pub const fn prefixed(name: &'static str) -> Self {
        Self {
            name,
            lib_name: name,
            win_lib_name: name,
            prefixed: true,
        }
    }
}

fn rerun_env<T: fmt::Display>(name: T) {
    println!("cargo:rerun-if-env-changed={}", name);
}

impl Context {
    pub fn new() -> Self {
        rerun_env("PKG_CONFIG_ALL_STATIC");
        rerun_env("PKG_CONFIG_ALL_DINAMYC");

        let bindings = bindgen::Builder::default()
            .header_contents(
                "wrapper.h",
                "#include <hdf5.h>
#include <H5Fpublic.h>
#include <H5Tpublic.h>",
            )
            .clang_args(["-target", &env::var("TARGET").unwrap()])
            .trust_clang_mangling(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

        let mut is_win = false;
        if let Ok(triple) = env::var("TARGET") {
            if triple.contains("-windows-") {
                is_win = true;
            }
        }

        Self { bindings, is_win }
    }

    #[inline]
    pub fn is_win(&self) -> bool {
        self.is_win
    }

    pub fn link(&mut self, lib: Lib) {
        use cc_args::MergeCcArgs;

        let link_type = Self::link_type(lib.name);

        if let Ok(lib) = pkg_config::Config::new()
            .cargo_metadata(true)
            .env_metadata(true)
            .statik(link_type == LinkType::Static)
            .probe(lib.name)
        {
            unsafe {
                core::ptr::write_volatile(
                    &mut self.bindings,
                    core::ptr::read_volatile(&self.bindings).merge_cc_args(&lib),
                );
            }
        } else {
            println!(
                "cargo:rustc-link-lib={}{}{}",
                link_type,
                if self.is_win && lib.prefixed {
                    "lib"
                } else {
                    ""
                },
                if self.is_win {
                    lib.win_lib_name
                } else {
                    lib.lib_name
                }
            );
        }
    }

    fn link_type(name: &str) -> LinkType {
        #[derive(Clone)]
        struct Suffix<'a, T: fmt::Display>(T, &'a str);
        impl<'a, T: fmt::Display> fmt::Display for Suffix<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)?;
                fmt::Display::fmt(&self.1, f)
            }
        }
        let uc = UpperCase(name.chars());
        let lib_sta = Suffix(uc.clone(), "_STATIC");
        let lib_dyn = Suffix(uc.clone(), "_DYNAMIC");

        rerun_env(lib_sta.clone());
        rerun_env(lib_dyn.clone());
        rerun_env(Suffix(uc, "_NO_PKG_CONFIG"));

        if env::var_os(lib_sta.to_string()).is_some() {
            LinkType::Static
        } else if env::var_os(lib_dyn.to_string()).is_some() {
            LinkType::Dynamic
        } else if env::var_os("PKG_CONFIG_ALL_STATIC").is_some() {
            LinkType::Static
        } else if env::var_os("PKG_CONFIG_ALL_DINAMYC").is_some() {
            LinkType::Dynamic
        } else {
            LinkType::Unknown
        }
    }

    pub fn build(self) -> bindgen::Bindings {
        self.bindings
            .generate()
            .expect("Unable to generate bindings")
    }
}

#[derive(Clone)]
struct UpperCase<T: IntoIterator<Item = char> + Clone>(T);

impl<T: IntoIterator<Item = char> + Clone> fmt::Display for UpperCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.clone().into_iter() {
            for c in c.to_uppercase() {
                f.write_char(c)?;
            }
        }
        Ok(())
    }
}

