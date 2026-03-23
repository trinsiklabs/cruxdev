"""Release notes generation from git history.

Parses git log, categorizes commits, generates markdown release notes.
"""

from __future__ import annotations

import re
import subprocess
from dataclasses import dataclass, field


@dataclass
class CommitInfo:
    hash: str
    message: str
    category: str  # "feature", "fix", "refactor", "docs", "test", "chore", "breaking"


@dataclass
class ReleaseNotes:
    version: str
    commits: list[CommitInfo] = field(default_factory=list)


CATEGORY_PATTERNS = [
    (r"^feat", "feature"),
    (r"^fix", "fix"),
    (r"^refactor", "refactor"),
    (r"^docs?", "docs"),
    (r"^test", "test"),
    (r"^chore|^ci|^build", "chore"),
    (r"BREAKING|breaking", "breaking"),
]


def categorize_commit(message: str) -> str:
    """Categorize a commit message."""
    first_line = message.split("\n")[0].lower().strip()
    for pattern, category in CATEGORY_PATTERNS:
        if re.search(pattern, first_line):
            return category
    return "chore"


def parse_git_log(
    project_dir: str,
    since_tag: str | None = None,
    max_commits: int = 100,
) -> list[CommitInfo]:
    """Parse git log into structured commit info."""
    cmd = ["git", "log", f"--max-count={max_commits}", "--pretty=format:%H|%s"]
    if since_tag:
        cmd.append(f"{since_tag}..HEAD")

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            timeout=30,
            cwd=project_dir,
        )
        if result.returncode != 0:
            return []

        commits = []
        for line in result.stdout.decode().strip().split("\n"):
            if not line or "|" not in line:  # pragma: no cover — git format edge case
                continue
            hash_val, message = line.split("|", 1)
            commits.append(CommitInfo(
                hash=hash_val.strip(),
                message=message.strip(),
                category=categorize_commit(message),
            ))
        return commits
    except (subprocess.TimeoutExpired, FileNotFoundError):  # pragma: no cover
        return []


def generate_release_notes(
    version: str,
    commits: list[CommitInfo],
) -> str:
    """Generate markdown release notes from categorized commits."""
    if not commits:
        return f"# {version}\n\nNo changes.\n"

    sections = {
        "breaking": ("Breaking Changes", []),
        "feature": ("Features", []),
        "fix": ("Bug Fixes", []),
        "refactor": ("Refactoring", []),
        "docs": ("Documentation", []),
        "test": ("Tests", []),
        "chore": ("Maintenance", []),
    }

    for commit in commits:
        cat = commit.category
        if cat in sections:
            sections[cat][1].append(commit)

    lines = [f"# {version}", ""]
    for key in ["breaking", "feature", "fix", "refactor", "docs", "test", "chore"]:
        title, section_commits = sections[key]
        if not section_commits:
            continue
        lines.append(f"## {title}")
        lines.append("")
        for c in section_commits:
            lines.append(f"- {c.message} (`{c.hash[:7]}`)")
        lines.append("")

    return "\n".join(lines)
