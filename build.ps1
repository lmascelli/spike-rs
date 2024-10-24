# TODO remove the $IsWindows check
$env:HDF5_INCLUDE_DIR = "C:/Users/Leonardo/Documents/unige/hdf5/1.14.5/include"
$env:HDF5_LIB_DIR = "C:/Users/Leonardo/Documents/unige/hdf5/1.14.5/lib"

$Script:HelpText = @"
================================================================================
spike-rs build.ps1

USAGE:
./spike-rs.ps1 COMMAND

COMMANDS:

install-python        install python
install-llvm          install llvm
install-rust          install the rust toolchain
install-cmake         install cmake

================================================================================
"@


if ($IsWindows) {
    switch ($args[0]) {
        "run" {
            cargo run --release
        }

        "build" {
            cargo build --release 
        }

        "install-python" {
            winget install -e --id Python.Python.3.12
        }

        "install-llvm" {
            winget install -e --id LLVM.LLVM
        }

        "install-rust" {
            Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
            ./rustup-init.exe
        }

        "install-cmake" {
            winget install -e --id Kitware.CMake
        }

        default {
            Write-Host $Script:HelpText
        }
    }
}

if ($IsLinux) {
    switch ($args[0]) {
        "run" {
            cargo run
        }

        "build" {
            cargo build
        }

        "clippy" {
            cargo clippy
        }

        default {
            Write-Host $Script:HelpText
        }
    }
}
