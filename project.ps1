# change the following variables with the correct directories for your system

if ($IsWindows) {
    $env:HDF5_LIB_DIR = "D:/msys64/mingw64/lib";
    $env:HDF5_INCLUDE_DIR = "D:/msys64/mingw64/include";
    $Script:pycode_input = "pycode_rs.dll"
    $Script:pycode_output = "pycode_rs.pyd"
} elseif ($IsLinux) {
    $Script:pycode_input = "libpycode_rs.so"
    $Script:pycode_output = "pycode_rs.so"
}

Clear-Host

switch ($args[0]) {
  "build" {
    cargo build;
    copy -Force ./target/debug/$Script:pycode_input ./pycode/$Script:pycode_output
  }
  "run" {
    cargo run
  }
  "doc" {
    cargo doc --open
  }
  "release" {
    cargo build --release;
    copy -Force ./target/release/$Script:pycode_input ./pycode/$Script:pycode_output
  }
  default {
    python ./pycode/main.py
  }
}
