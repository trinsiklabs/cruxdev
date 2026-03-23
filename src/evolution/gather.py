"""Beat 1: GATHER — scan for changes, issues, and inspiration.

Reads: own source, GitHub issues, CI status, inspiration repos,
session bus inbox, competitor updates.
"""

from __future__ import annotations

import json
import os
import subprocess
from dataclasses import dataclass, field


@dataclass
class GatherResult:
    own_changes: list[str] = field(default_factory=list)
    github_issues: list[dict] = field(default_factory=list)
    ci_status: str = "unknown"
    inbox_messages: list[dict] = field(default_factory=list)
    inspiration: list[dict] = field(default_factory=list)


def gather_own_changes(project_dir: str, since_days: int = 1) -> list[str]:
    """Get recent git commits."""
    try:
        result = subprocess.run(
            ["git", "log", f"--since={since_days} days ago", "--pretty=format:%H|%s"],
            capture_output=True, timeout=10, cwd=project_dir,
        )
        if result.returncode != 0:
            return []
        commits = []
        for line in result.stdout.decode().strip().split("\n"):
            if line and "|" in line:
                commits.append(line)
        return commits
    except (subprocess.TimeoutExpired, FileNotFoundError):  # pragma: no cover
        return []


def gather_github_issues(repo: str, state: str = "open") -> list[dict]:  # pragma: no cover — gh CLI
    """Fetch open GitHub issues via gh CLI."""
    try:
        result = subprocess.run(
            ["gh", "issue", "list", "--repo", repo, "--state", state,
             "--json", "number,title,labels,createdAt", "--limit", "20"],
            capture_output=True, timeout=30,
        )
        if result.returncode != 0:
            return []
        return json.loads(result.stdout.decode())
    except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
        return []


def gather_ci_status(repo: str) -> str:  # pragma: no cover — gh CLI
    """Check latest CI run status via gh CLI."""
    try:
        result = subprocess.run(
            ["gh", "run", "list", "--repo", repo, "--limit", "1",
             "--json", "status,conclusion"],
            capture_output=True, timeout=15,
        )
        if result.returncode != 0:
            return "unknown"
        runs = json.loads(result.stdout.decode())
        if runs:
            return runs[0].get("conclusion", runs[0].get("status", "unknown"))
        return "no_runs"
    except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
        return "unknown"


def gather_inbox(project: str) -> list[dict]:
    """Read session bus inbox."""
    try:
        from ..bus.broker import Broker
        broker = Broker()
        messages = broker.check_inbox(project)
        return [
            {"id": m.id, "type": m.type, "from": m.source_project,
             "title": m.title, "body": m.body, "severity": m.severity}
            for m in messages
        ]
    except Exception:  # pragma: no cover — broker init may fail
        return []


def gather_all(
    project_dir: str,
    project_name: str,
    github_repo: str = "",
) -> GatherResult:
    """Run all gather operations."""
    result = GatherResult()
    result.own_changes = gather_own_changes(project_dir)
    result.inbox_messages = gather_inbox(project_name)
    if github_repo:  # pragma: no cover — gh CLI calls
        result.github_issues = gather_github_issues(github_repo)
        result.ci_status = gather_ci_status(github_repo)
    return result
