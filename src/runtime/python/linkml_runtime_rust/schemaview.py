# shim that re-exports the native submodule
from . import _native as _n
_mod = _n.linkml_schemaview

# Re-export public names
__all__ = getattr(_mod, "__all__", [])
for name in dir(_mod):
    if not name.startswith("_") or name in __all__:
        globals()[name] = getattr(_mod, name)