from typing import Any, Optional
import atexit
import builtins

from .pycode import PyPhase, init, close

init()
atexit.register(close)

class _PyCode:
    def __init__(self):
        self.variables: Dict[str, Any] = {}

    def set(self, variable: str, property_value: Any):
        self.variables[variable] = property_value

    def get(self, variable: str) -> Optional[Any]:
        if variable in self.variables:
            return self.variables[variable]
        else:
            return None

setattr(builtins, "PyCode", _PyCode())

from . import operations
from . import utils
from . import settings
