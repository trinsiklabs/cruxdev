"""Document normalization — convert and standardize to Key template format.

Handles frontmatter injection, folder reorganization, and stub generation.
"""

from __future__ import annotations

import os
import time
from dataclasses import dataclass, field


@dataclass
class NormalizeResult:
    """Result of normalizing a document."""
    source_path: str
    dest_path: str
    frontmatter_added: bool = False
    content_preserved: bool = True
    notes: str = ""


FRONTMATTER_TEMPLATE = """---
title: "{title}"
last_updated: "{date}"
migration_status: "migrated"
---

"""


def inject_frontmatter(
    content: str,
    title: str,
    date: str | None = None,
) -> str:
    """Add YAML frontmatter to a markdown document if not present.

    Args:
        content: Document content
        title: Document title
        date: Last updated date (ISO format), defaults to today
    """
    if content.lstrip().startswith("---"):
        return content  # Already has frontmatter

    if date is None:
        date = time.strftime("%Y-%m-%d")

    fm = FRONTMATTER_TEMPLATE.format(title=title, date=date)
    return fm + content


def generate_stub(
    template_name: str,
    template_file: str,
    priority: str = "medium",
) -> str:
    """Generate a stub document with TODO markers.

    Creates a minimal document with the template name as title
    and TODO markers for content.
    """
    date = time.strftime("%Y-%m-%d")
    lines = [
        "---",
        f'title: "{template_name}"',
        f'last_updated: "{date}"',
        'migration_status: "stub"',
        "---",
        "",
        f"# {template_name}",
        "",
        f"<!-- TODO: Complete this document. Priority: {priority} -->",
        "",
        "## Overview",
        "",
        "TODO: Add overview content.",
        "",
    ]
    return "\n".join(lines)


def normalize_document(
    source_path: str,
    dest_path: str,
    title: str,
    archive_dir: str | None = None,
) -> NormalizeResult:
    """Normalize a document: add frontmatter, move to correct location.

    Args:
        source_path: Path to the source document
        dest_path: Where the normalized document should go
        title: Document title for frontmatter
        archive_dir: If set, archive original to this directory
    """
    result = NormalizeResult(source_path=source_path, dest_path=dest_path)

    try:
        with open(source_path) as f:
            content = f.read()
    except (OSError, UnicodeDecodeError) as e:
        result.content_preserved = False
        result.notes = f"Failed to read source: {e}"
        return result

    # Inject frontmatter
    original_content = content
    content = inject_frontmatter(content, title)
    result.frontmatter_added = content != original_content

    # Archive original if requested
    if archive_dir:
        os.makedirs(archive_dir, exist_ok=True)
        archive_path = os.path.join(archive_dir, os.path.basename(source_path))
        with open(archive_path, "w") as f:
            f.write(original_content)

    # Write normalized document
    os.makedirs(os.path.dirname(dest_path), exist_ok=True)
    with open(dest_path, "w") as f:
        f.write(content)

    return result
