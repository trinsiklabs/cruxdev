"""Document inventory — scan and catalog all project materials.

Produces a structured inventory of documents, code, assets, and configs
with format detection and quality classification.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from typing import Literal


Format = Literal["markdown", "code", "config", "data", "image", "pdf", "other"]
Quality = Literal["usable", "reference-only", "extract-info", "obsolete"]


EXTENSION_MAP: dict[str, Format] = {
    ".md": "markdown",
    ".mdx": "markdown",
    ".txt": "markdown",
    ".py": "code",
    ".js": "code",
    ".ts": "code",
    ".tsx": "code",
    ".jsx": "code",
    ".go": "code",
    ".rs": "code",
    ".rb": "code",
    ".ex": "code",
    ".exs": "code",
    ".sh": "code",
    ".json": "config",
    ".yaml": "config",
    ".yml": "config",
    ".toml": "config",
    ".ini": "config",
    ".cfg": "config",
    ".env": "config",
    ".csv": "data",
    ".tsv": "data",
    ".sql": "data",
    ".png": "image",
    ".jpg": "image",
    ".jpeg": "image",
    ".svg": "image",
    ".gif": "image",
    ".webp": "image",
    ".pdf": "pdf",
}


@dataclass
class InventoryItem:
    """A single item in the project inventory."""
    path: str
    format: Format
    size_bytes: int = 0
    last_modified: float = 0.0
    quality: Quality = "usable"
    notes: str = ""


@dataclass
class Inventory:
    """Complete project inventory."""
    project_dir: str
    items: list[InventoryItem] = field(default_factory=list)

    @property
    def by_format(self) -> dict[str, list[InventoryItem]]:
        result: dict[str, list[InventoryItem]] = {}
        for item in self.items:
            result.setdefault(item.format, []).append(item)
        return result

    @property
    def total_size(self) -> int:
        return sum(item.size_bytes for item in self.items)

    def to_markdown(self) -> str:
        """Generate intake-inventory.md content."""
        lines = ["# Project Inventory", ""]
        lines.append(f"**Total items:** {len(self.items)}")
        lines.append(f"**Total size:** {self.total_size:,} bytes")
        lines.append("")

        for fmt, items in sorted(self.by_format.items()):
            lines.append(f"## {fmt.title()} ({len(items)} items)")
            lines.append("")
            for item in sorted(items, key=lambda x: x.path):
                size = f"{item.size_bytes:,}b"
                lines.append(f"- `{item.path}` ({size}) [{item.quality}]")
            lines.append("")

        return "\n".join(lines)


def detect_format(path: str) -> Format:
    """Detect file format from extension."""
    _, ext = os.path.splitext(path)
    return EXTENSION_MAP.get(ext.lower(), "other")


def inventory_project(
    project_dir: str,
    skip_hidden: bool = True,
    skip_generated: bool = True,
) -> Inventory:
    """Scan a project directory and produce a complete inventory.

    Args:
        project_dir: Path to project root
        skip_hidden: Skip hidden files/dirs (default True)
        skip_generated: Skip node_modules, __pycache__, etc. (default True)
    """
    inventory = Inventory(project_dir=project_dir)
    skip_dirs = {"node_modules", "__pycache__", ".git", "dist", "build", "venv", ".venv"}

    if not os.path.isdir(project_dir):
        return inventory

    for root, dirs, files in os.walk(project_dir):
        if skip_hidden:
            dirs[:] = [d for d in dirs if not d.startswith(".")]
        if skip_generated:
            dirs[:] = [d for d in dirs if d not in skip_dirs]

        for fname in files:
            if skip_hidden and fname.startswith("."):
                continue

            full_path = os.path.join(root, fname)
            rel_path = os.path.relpath(full_path, project_dir)

            try:
                stat = os.stat(full_path)
                size = stat.st_size
                mtime = stat.st_mtime
            except OSError:
                size = 0
                mtime = 0.0

            item = InventoryItem(
                path=rel_path,
                format=detect_format(fname),
                size_bytes=size,
                last_modified=mtime,
            )
            inventory.items.append(item)

    return inventory
