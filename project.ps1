# $env:LIB = "E:/rust/hdf5/lib;C:/Program Files (x86)/Windows Kits/10/Lib/10.0.22000.0/um/x64"
# $env:CPATH = "E:/rust/hdf5/include"

$Script:OldPath = $env:Path;
$env:Path += ";E:/rust/hdf5/bin";
$env:HDF5_DIR = "E:/rust/hdf5";
Write-Output $env:Path;


clear && cargo build

$env:Path = $Script:OldPath;
