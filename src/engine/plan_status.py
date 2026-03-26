"""Plan status line management — update markdown status from engine state.

The **Status:** line in build plan markdown is derived output, updated by the
engine when convergence state changes. It is NEVER the source of truth.
"""

import os
import re


VALID_STATUSES = {"NOT STARTED", "IN PROGRESS", "CONVERGED", "ESCALATED"}


def update_plan_status(plan_file: str, new_status: str) -> bool:
    """Update the **Status:** line in a build plan markdown file.

    Returns True if the status was updated, False if no status line found
    or file doesn't exist. Never fails — if the line isn't there, we skip.
    """
    if new_status not in VALID_STATUSES:
        return False

    if not os.path.exists(plan_file):
        return False

    try:
        with open(plan_file) as f:
            content = f.read()
    except OSError:
        return False

    # Match **Status:** followed by any value
    pattern = r'(\*\*Status:\*\*\s*).+'
    match = re.search(pattern, content)
    if not match:
        return False

    updated = re.sub(pattern, rf'\g<1>{new_status}', content, count=1)
    if updated == content:
        return False  # Already at this status

    try:
        with open(plan_file, "w") as f:
            f.write(updated)
        return True
    except OSError:
        return False


def read_plan_status(plan_file: str) -> str | None:
    """Read the current **Status:** line from a plan file."""
    if not os.path.exists(plan_file):
        return None

    try:
        with open(plan_file) as f:
            content = f.read()
    except OSError:
        return None

    match = re.search(r'\*\*Status:\*\*\s*(.+)', content)
    if match:
        return match.group(1).strip()
    return None
