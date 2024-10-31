$env:HDF5_LIB_DIR = "~/Documents/unige/hdf5/1.14.5/lib"
$env:HDF5_INCLUDE_DIR = "~/Documents/unige/hdf5/1.14.5/include"

function Script:PrintHelp {
    $PrintText = @'
================================================================================
                                    PYCODE
================================================================================

USAGE:
./build.ps1 COMMAND

AVAILABLE COMMANDS:

create-venv                   create a new virtual environment with the necessary
                              packages installed

develop                       build and install the library in the current venv

build                         build the pycode library

================================================================================
                                    
================================================================================
'@

    Write-Host $PrintText
}

$bin_dir = "Scripts"
if ($IsLinux) {
    $bin_dir = "bin"
}

switch($args[0]) {
    "create-venv" {
        python -m venv .venv
        Invoke-Expression ".venv/$bin_dir/pip install maturin matplotlib"
    }

    "develop" {
        Invoke-Expression "./.venv/$bin_dir/maturin develop"
    }

    "build" {
        Invoke-Expression "./.venv/$bin_dir/maturin build"
    }

    default {
        Script:PrintHelp
    }
}
