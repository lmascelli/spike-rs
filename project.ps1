$env:HDF5_LIB_DIR = "D:/msys64/mingw64/lib";
$env:HDF5_INCLUDE_DIR = "D:/msys64/mingw64/include";
Clear-Host
cargo build
copy -Force ./target/debug/spyke_rs.dll ./spyke_rs.pyd

if ($args[0] -eq "run") {
  cargo run
} elseif ($args[0] -eq "doc") {
  cargo doc --open
}
