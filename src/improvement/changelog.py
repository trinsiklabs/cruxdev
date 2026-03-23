"""Living changelog — auto-generated from git history.

Maintains CHANGELOG.md from conventional commits.
"""

from __future__ import annotations

import os

from .release_notes import CommitInfo, categorize_commit, parse_git_log


def generate_changelog(
    project_dir: str,
    version: str = "Unreleased",
    since_tag: str | None = None,
    max_commits: int = 200,
) -> str:
    """Generate a full CHANGELOG.md from git history."""
    commits = parse_git_log(project_dir, since_tag, max_commits)

    if not commits:
        return f"# Changelog\n\n## {version}\n\nNo changes recorded.\n"

    sections = {
        "breaking": ("Breaking Changes", []),
        "feature": ("Features", []),
        "fix": ("Bug Fixes", []),
        "refactor": ("Refactoring", []),
        "docs": ("Documentation", []),
        "test": ("Tests", []),
        "chore": ("Maintenance", []),
    }

    for c in commits:
        cat = c.category
        if cat in sections:
            sections[cat][1].append(c)

    lines = ["# Changelog", "", f"## {version}", ""]
    for key in ["breaking", "feature", "fix", "refactor", "docs", "test", "chore"]:
        title, section_commits = sections[key]
        if not section_commits:
            continue
        lines.append(f"### {title}")
        lines.append("")
        for c in section_commits:
            lines.append(f"- {c.message} (`{c.hash[:7]}`)")
        lines.append("")

    return "\n".join(lines)


def write_changelog(
    project_dir: str,
    version: str = "Unreleased",
    since_tag: str | None = None,
) -> str:
    """Generate and write CHANGELOG.md to the project root."""
    content = generate_changelog(project_dir, version, since_tag)
    path = os.path.join(project_dir, "CHANGELOG.md")
    with open(path, "w") as f:
        f.write(content)
    return path
