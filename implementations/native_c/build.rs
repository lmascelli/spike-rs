use cmake;

fn main() {
    let mut build = cmake::Config::new(".");
    build.build();
}
