"""MCP input normalization — coerce tool inputs regardless of model quirks.

Different LLMs send tool arguments in different formats:
- Claude: well-structured JSON, dicts where expected
- MiMo V2 Pro: strings where dicts expected, raw objects where JSON strings expected
- Other models: missing fields, wrong types, extra whitespace

This module normalizes inputs so every MCP tool works with any model.
"""

from __future__ import annotations

import json


def to_json_string(value: object) -> str:
    """Ensure a value is a JSON string.

    Handles: already a string, dict/list (serialize), None (empty array).
    """
    if value is None:
        return "[]"
    if isinstance(value, str):
        return value
    return json.dumps(value)


def to_string_list(value: object, separator: str = ",") -> list[str]:
    """Ensure a value is a list of strings.

    Handles: comma-separated string, JSON array string, actual list, None.
    """
    if value is None or value == "":
        return []
    if isinstance(value, list):
        return [str(item).strip() for item in value if str(item).strip()]
    if isinstance(value, str):
        # Try JSON array first
        stripped = value.strip()
        if stripped.startswith("["):
            try:
                parsed = json.loads(stripped)
                if isinstance(parsed, list):
                    return [str(item).strip() for item in parsed if str(item).strip()]
            except json.JSONDecodeError:
                pass
        # Fall back to separator-split
        return [item.strip() for item in value.split(separator) if item.strip()]
    return [str(value)]


def to_dict_list(value: object) -> list[dict]:
    """Ensure a value is a list of dicts.

    Handles: JSON string, actual list, list of non-dicts (skip), None.
    """
    if value is None:
        return []
    if isinstance(value, str):
        try:
            parsed = json.loads(value)
            if isinstance(parsed, list):
                return [item for item in parsed if isinstance(item, dict)]
            if isinstance(parsed, dict):
                return [parsed]
        except json.JSONDecodeError:
            return []
    if isinstance(value, list):
        return [item for item in value if isinstance(item, dict)]
    if isinstance(value, dict):
        return [value]
    return []


def to_int(value: object, default: int = 0) -> int:
    """Ensure a value is an int.

    Handles: string numbers, floats, None.
    """
    if value is None:
        return default
    if isinstance(value, int):
        return value
    if isinstance(value, float):
        return int(value)
    if isinstance(value, str):
        try:
            return int(value)
        except ValueError:
            try:
                return int(float(value))
            except ValueError:
                return default
    return default


def to_pipe_list(value: object) -> list[str]:
    """Parse pipe-separated values (used for counter-evidence, alternatives).

    Handles: pipe-separated string, JSON array, actual list, None.
    """
    return to_string_list(value, separator="|")


def coerce_feature(item: object) -> dict | None:
    """Coerce a feature entry to a dict with name and has fields.

    Handles: dict with name/has, plain string, None.
    """
    if item is None:
        return None
    if isinstance(item, dict):
        if "name" not in item:
            return None
        return {"name": item["name"], "has": item.get("has", True)}
    if isinstance(item, str):
        return {"name": item, "has": True}
    return None


def normalize_competitors(raw: object) -> list[dict]:
    """Normalize a competitors list from any format a model might send.

    Handles: JSON string, list of dicts, list with non-dicts mixed in,
    dicts with features as strings or dicts.
    """
    items = to_dict_list(raw)
    result = []
    for item in items:
        features = []
        for f in item.get("features", []):
            coerced = coerce_feature(f)
            if coerced:
                features.append(coerced)
        result.append({
            "name": item.get("name", "unknown"),
            "url": item.get("url", ""),
            "category": item.get("category", "noted"),
            "features": features,
        })
    return result
