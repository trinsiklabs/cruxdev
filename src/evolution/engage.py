"""Beat 5: ENGAGE — process community input, respond to issues.

Social sessions are ISOLATED from code evolution sessions.
This beat only processes input — it does NOT modify code.
"""

from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass, field


@dataclass
class EngageAction:
    action_type: str  # "label", "comment", "close", "triage"
    issue_number: int
    detail: str


def triage_issue(issue: dict) -> EngageAction:
    """Triage a GitHub issue — suggest labels and priority."""
    title = issue.get("title", "").lower()
    labels = [l.get("name", "") for l in issue.get("labels", [])]

    if not labels:
        # Auto-suggest label based on title keywords
        if any(w in title for w in ["bug", "error", "crash", "fail"]):
            suggested = "bug"
        elif any(w in title for w in ["feature", "add", "support", "request"]):
            suggested = "enhancement"
        elif any(w in title for w in ["doc", "readme", "typo"]):
            suggested = "documentation"
        else:
            suggested = "triage"

        return EngageAction(
            action_type="label",
            issue_number=issue.get("number", 0),
            detail=f"Suggested label: {suggested}",
        )

    return EngageAction(
        action_type="triage",
        issue_number=issue.get("number", 0),
        detail="Already labeled",
    )


def add_label(repo: str, issue_number: int, label: str) -> bool:  # pragma: no cover — gh CLI
    """Add a label to a GitHub issue via gh CLI."""
    try:
        result = subprocess.run(
            ["gh", "issue", "edit", str(issue_number), "--repo", repo,
             "--add-label", label],
            capture_output=True, timeout=15,
        )
        return result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return False


def add_comment(repo: str, issue_number: int, body: str) -> bool:  # pragma: no cover — gh CLI
    """Add a comment to a GitHub issue via gh CLI."""
    try:
        result = subprocess.run(
            ["gh", "issue", "comment", str(issue_number), "--repo", repo,
             "--body", body],
            capture_output=True, timeout=15,
        )
        return result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return False


def process_issues(
    issues: list[dict],
    repo: str = "",
    dry_run: bool = True,
) -> list[EngageAction]:
    """Process GitHub issues — triage, label, comment.

    dry_run=True (default) only returns actions, doesn't execute.
    Social isolation: this function NEVER modifies code.
    """
    actions = []
    for issue in issues:
        action = triage_issue(issue)
        actions.append(action)

        if not dry_run and repo and action.action_type == "label":  # pragma: no cover — live mode
            label = action.detail.replace("Suggested label: ", "")
            add_label(repo, action.issue_number, label)

    return actions
