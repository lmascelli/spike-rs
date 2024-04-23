from sys import path
from os import getenv

PYCODE_PATH = getenv("PYCODE_PATH")
path.insert(0, PYCODE_PATH)

from pycode.gui.main import main

main()
