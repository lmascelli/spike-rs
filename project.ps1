$env:HDF5_LIB_DIR = "D:/msys64/mingw64/lib";
$env:HDF5_INCLUDE_DIR = "D:/msys64/mingw64/include";
Clear-Host
cargo build

if ($args[0] -eq "run") {
  cargo run
}
