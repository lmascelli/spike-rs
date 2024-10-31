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

test                          run the test.py script in the current venv

================================================================================
                                    
================================================================================
'@

Write-Host $PrintText
}

switch($args[0]) {
    "create-venv" {
        python -m venv .venv
        .venv/bin/pip install maturin matplotlib
    }

    "develop" {
        ./.venv/bin/maturin develop
    }

    "build" {
        ./.venv/bin/maturin build 
    }

    "test" {
        ./.venv/bin/python ./test.py
    }

    default {
        Script:PrintHelp
    }
}
