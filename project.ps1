$env:HDF5_LIB_DIR = "D:/msys64/mingw64/lib";
$env:HDF5_INCLUDE_DIR = "D:/msys64/mingw64/include";
Clear-Host

switch ($args[0]) {
  "build" {
    cargo build;
    copy -Force ./target/debug/spyke_rs.dll ./pycode/spyke_rs.pyd
  }
  "run" {
    cargo run
  }
  "doc" {
    cargo doc --open
  }
  "release" {
    cargo build --release;
    copy -Force ./target/release/spyke_rs.dll ./pycode/spyke_rs.pyd
  }
  default {
    python ./pycode/main.py
  }
}
