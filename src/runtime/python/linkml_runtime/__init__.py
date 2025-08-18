"""Python package for :mod:`linkml_runtime` bindings."""

from .linkml_runtime import *  # noqa: F401,F403
from .linkml_runtime.linkml_schemaview import *  # noqa: F401,F403
from ._resolver import resolve_schemas as _resolve_schemas

# Attach the Python implementation of ``resolve_schemas`` to ``SchemaView``
SchemaView.resolve_schemas = _resolve_schemas  # type: ignore[attr-defined]

__all__ = [name for name in globals() if not name.startswith("_")]

