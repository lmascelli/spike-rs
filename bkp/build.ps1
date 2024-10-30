$env:HDF5_INCLUDE_DIR = "C:/Users/Leonardo/Documents/unige/hdf5/1.14.5/include"
$env:HDF5_LIB_DIR = "C:/Users/Leonardo/Documents/unige/hdf5/1.14.5/lib"

$Script:HelpText = @"
================================================================================
spike-rs build.ps1

USAGE:
./spike-rs.ps1 COMMAND

COMMANDS:

build              	build the release library
debug               build the debug library
clippy              run the clippy check analizer

================================================================================
"@

function Script:Install {
    Remove-Item install -Force -Recurse -ErrorAction Ignore
    Copy-Item pycode -Destination install/pycode -Recurse
    if ($IsLinux) {
        Copy-Item -Path target/release/libnative_c.so -Destination install/pycode/handlers/libnative_c.so -Force
    } else {
        Copy-Item -Path target/release/native_c.dll -Destination install/pycode/handlers/libnative_c.pyd -Force
    }
}

switch ($args[0]) {
    "run" {
        cargo run --release
    }

    "build" {
        cargo build --release 
    }

    "debug" {
        cargo build
    }

    "clippy" {
        cargo clippy
    }

    "install" {
       Script:Install 
    }

    "test" {
        Copy-Item test.py install

        # Get the path of the current script
        $scriptPath = $MyInvocation.MyCommand.Path

        # Get the directory of the script
        $scriptDir = Split-Path -Parent $scriptPath

        # Concatenate with the "install" folder
        $installPath = Join-Path $scriptDir "install"

        # Add the install path to the python modules path
        $env:PYTHONPATH += $installPath

        python install/test.py
    }

    default {
        Write-Host $Script:HelpText
    }
}
