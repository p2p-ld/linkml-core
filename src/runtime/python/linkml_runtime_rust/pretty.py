"""Utilities for pretty-printing LinkMLValue trees.

This module provides a pure Python helper to render a :class:`LinkMLValue`
as a human readable tree. It is intended for debugging and diagnostic use.
"""

from __future__ import annotations

from typing import Any, List

try:  # pragma: no cover - runtime optional during type checking
    from . import LinkMLValue
except Exception:  # pragma: no cover - fallback when extension missing
    LinkMLValue = Any  # type: ignore[misc]

__all__ = ["pretty_linkml_value"]


def pretty_linkml_value(value: "LinkMLValue", indent: int = 0) -> str:
    """Return a tree-style string representation of ``value``.

    Parameters
    ----------
    value: LinkMLValue
        The value to render.
    indent: int, optional
        Starting indentation (number of spaces).
    """
    prefix = " " * indent
    slot = value.slot_name()
    class_name = value.class_name()
    parts: List[str] = []
    if slot:
        parts.append(str(slot))
    if class_name:
        parts.append(f"<{class_name}>")

    pyval = value.as_python()
    if not isinstance(pyval, (dict, list)):
        parts.append(repr(pyval))
        return f"{prefix}{' '.join(parts)}"

    lines = [f"{prefix}{' '.join(parts)}"]
    if isinstance(pyval, dict):
        for key in value.keys():
            lines.append(f"{prefix}  {key}:")
            lines.append(pretty_linkml_value(value[key], indent + 4))
    else:
        for idx in range(len(value)):
            lines.append(f"{prefix}  -")
            lines.append(pretty_linkml_value(value[idx], indent + 4))
    return "\n".join(lines)
