"""Checklist parser — extract executable tasks from build plan markdown.

Reads plan files and converts checklist items into structured tasks
for the execution phase. Handles the standard format:
  - [ ] 1.1 Description of task
  - [x] 1.2 Already completed task
"""

from __future__ import annotations

import os
import re
from dataclasses import dataclass, field


@dataclass
class ChecklistItem:
    id: str  # e.g., "1.1", "2.3"
    phase: str  # e.g., "Phase 1"
    description: str
    completed: bool = False
    dependencies: list[str] = field(default_factory=list)


def parse_checklist(plan_file: str) -> list[ChecklistItem]:
    """Parse all checklist items from a build plan.

    Expects format:
        ## Phase N: Name
        - [ ] N.M Description
        - [x] N.M Already done
    """
    try:
        with open(plan_file) as f:
            content = f.read()
    except (FileNotFoundError, OSError):
        return []

    items = []
    current_phase = ""

    for line in content.split("\n"):
        # Detect phase headings
        phase_match = re.match(
            r"^##\s+(?:Phase\s+)?(\d+)[\s:.]+(.+)",
            line,
            re.IGNORECASE,
        )
        if phase_match:
            current_phase = f"Phase {phase_match.group(1)}"
            continue

        # Detect checklist items: - [ ] 1.1 Description or - [x] 1.2 Done
        item_match = re.match(
            r"^\s*-\s*\[\s*([xX ])?\s*\]\s*(\d+\.\d+)\s+(.*)",
            line,
        )
        if item_match:
            checked = item_match.group(1) in ("x", "X")
            item_id = item_match.group(2)
            description = item_match.group(3).strip()
            items.append(ChecklistItem(
                id=item_id,
                phase=current_phase,
                description=description,
                completed=checked,
            ))

    return items


def get_next_incomplete(items: list[ChecklistItem]) -> ChecklistItem | None:
    """Get the next incomplete checklist item."""
    for item in items:
        if not item.completed:
            return item
    return None


def mark_complete(items: list[ChecklistItem], item_id: str) -> bool:
    """Mark a checklist item as complete. Returns True if found."""
    for item in items:
        if item.id == item_id:
            item.completed = True
            return True
    return False


def all_complete(items: list[ChecklistItem]) -> bool:
    """Check if all checklist items are complete."""
    return all(item.completed for item in items) if items else True


def completion_summary(items: list[ChecklistItem]) -> dict:
    """Get completion statistics."""
    total = len(items)
    done = sum(1 for i in items if i.completed)
    return {
        "total": total,
        "completed": done,
        "remaining": total - done,
        "percentage": round(done / total * 100, 1) if total > 0 else 100.0,
    }


def mark_complete_in_file(plan_file: str, item_id: str) -> bool:
    """Durably mark a checklist item as complete in the plan file.

    Finds the line matching `- [ ] <item_id>` and replaces with `- [x] <item_id>`.
    Returns True if found and updated, False otherwise.
    """
    if not os.path.exists(plan_file):
        return False

    try:
        with open(plan_file) as f:
            content = f.read()
    except OSError:
        return False

    # Match the specific item: - [ ] N.M ...
    pattern = rf'(^\s*-\s*)\[\s*\](\s*{re.escape(item_id)}\s)'
    updated = re.sub(pattern, r'\1[x]\2', content, count=1, flags=re.MULTILINE)

    if updated == content:
        return False  # Item not found or already checked

    try:
        with open(plan_file, "w") as f:
            f.write(updated)
        return True
    except OSError:
        return False
