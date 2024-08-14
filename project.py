from os import environ, curdir, system
from sys import path
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
PYSIDE6_UIC = "venv/bin/pyside6-uic"
PYSIDE6_RCC = "venv/bin/pyside6-rcc"

parser = argparse.ArgumentParser(
    prog="project.py",
    description="Script tool for PyCode",
    epilog="Mascelli Leonardo. Dibris. Università di Genova",
)

subparsers = parser.add_subparsers(
    title="Commands",
    description="Script commands for native pycode_rs library",
    help="build and run native library help",
    dest="command",
)
build_parser = subparsers.add_parser("build", help="Build the pycode_rs library")
build_parser.add_argument(
    "-r",
    "--release",
    action="store_true",
    dest="release",
    default=False,
    help="Build the release version",
)

build_parser.add_argument(
    "-win",
    "--windows",
    action="store_true",
    dest="windows",
    default=False,
    help="Cross build for windows",
)

run_parser = subparsers.add_parser("run", help="Run the native library test")
run_parser.add_argument(
    "-r",
    "--release",
    action="store_true",
    dest="release",
    default=False,
    help="Run the release version",
)

gui_parser = subparsers.add_parser("gui", help="Open the PyCode GUI")
gui_parser.add_argument(
    "-b", "--build", dest="build", action="store_true", help="Build Qt python bindings"
)

test_parser = subparsers.add_parser("test", help="Run the test.py file")


def build(release: bool, win: bool = False):
    if not win:
        print("============================================================")
        flags = "-r" if release else ""
        print("Building native pycode library")
        system(f"cargo build {flags}")
        lib_path = f"target/{'release' if release else 'debug'}/libpycode_rs.so"
        print("Coping to pycode package")
        copy(lib_path, "pycode/pycode_rs.so")
        print("============================================================")
    else:
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

    _ = test  # just to suppress linter complaining


def gui(build: bool):
    if build:
        gui_files = [
            "main",
            "project_view",
        ]

        for file in gui_files:
            system(f"{PYSIDE6_UIC} pycode/gui/{file}.ui -o pycode/gui/{file}.py")

        rc_files = []
        for file in rc_files:
            system(f"{PYSIDE6_RCC} pycode/res/{file}.rc -o pycode/res/{file}.py")
    else:
        from pycode.main import run as run_gui

        run_gui()


if __name__ == "__main__":
    system("clear")
    args = parser.parse_args()
    match args.command:
        case "build":
            build(args.release)
        case "run":
            run(args.release)
        case "test":
            test()
        case "gui":
            gui(args.build)
        case _:
            pass
