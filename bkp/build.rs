use std::error::Error;

pub fn main() -> Result<(), dyn Box<Error>> {
    println!("cargo:rustc-link-search={}", todo!());
    println!("cargo:rustc-link-lib={}", "native_c");

    let bindings = bindgen::Builder::default();
    return Ok(());
}
