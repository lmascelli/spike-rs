from os import environ, curdir, system
from sys import argv, path
from shutil import copy
from pathlib import Path
import argparse

# change the following variables with the correct directories for your system
# for example:
#
# environ["HDF5_LIB_DIR"] = "D:/msys64/mingw64/lib"
# environ["HDF5_INCLUDE_DIR"] = "D:/msys64/mingw64/include"
environ["PYCODE_PATH"] = f"{Path(curdir).absolute()}/pycode"
path.append(environ["PYCODE_PATH"])

parser = argparse.ArgumentParser(
        prog="project.py",
        description="Script tool for PyCode",
        epilog="Mascelli Leonardo. Dibris. Università di Genova",
        )

subparsers = parser.add_subparsers(
        title="Commands",
        description="Script commands for native pycode_rs library",
        help="build and run native library help",
        dest="command"
        )
build_parser = subparsers.add_parser('build', help="Build the pycode_rs library")
build_parser.add_argument(
        '-r',
        '--release',
        action="store_true",
        dest="release",
        default=False,
        help="Build the release version"
        )

run_parser = subparsers.add_parser('run', help="Run the native library test")
run_parser.add_argument(
        '-r',
        '--release',
        action="store_true",
        dest="release",
        default=False,
        help="Run the release version"
        )

gui_parser = subparsers.add_parser('gui', help="Open the PyCode GUI")
test_parser = subparsers.add_parser('test', help="Run the test.py file")

def build(release: bool):
    print("============================================================")
    flags = "-r" if release else ""
    print("Building native pycode library")
    system(f"cargo build {flags}")
    lib_path = f"target/{'release' if release else 'debug'}/libpycode_rs.so"
    print("Coping to pycode package")
    copy(lib_path, "pycode/pycode_rs.so")
    print("============================================================")

def run(release: bool):
    flags = "-r" if release else ""
    system(f"cargo run {flags}")

def test():
    import test

def gui():
    import pycode

if __name__ == '__main__':
    args = parser.parse_args()
    match args.command:
        case "build":
            build(args.release)
        case "run":
            run(args.release)
        case "test":
            test()
        case "gui":
            gui()
        case _:
            pass
