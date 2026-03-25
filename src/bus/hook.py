"""Session bus hook — auto-checks inbox with rate limiting.

Designed to run as a Claude Code PostToolUse hook. Checks the inbox
at most once per interval, prints new messages to stderr so the
session sees them.

Usage in .claude/settings.local.json:
{
  "hooks": {
    "PostToolUse": [{
      "command": "python3 /path/to/cruxdev/src/bus/hook_runner.py PROJECT_NAME"
    }]
  }
}
"""

from __future__ import annotations

import json
import os
import sys
import time


RATE_LIMIT_SECONDS = 60
LAST_CHECK_FILE = os.path.join(
    os.environ.get("HOME", ""),
    ".cruxdev",
    "bus_last_check",
)


def should_check(last_check_file: str = LAST_CHECK_FILE) -> bool:
    """Check if enough time has passed since last inbox check."""
    try:
        with open(last_check_file) as f:
            last = float(f.read().strip())
        return (time.time() - last) >= RATE_LIMIT_SECONDS
    except (FileNotFoundError, ValueError):
        return True


def mark_checked(last_check_file: str = LAST_CHECK_FILE) -> None:
    """Record that we just checked."""
    os.makedirs(os.path.dirname(last_check_file), exist_ok=True)
    with open(last_check_file, "w") as f:
        f.write(str(time.time()))


def check_and_notify(
    project: str,
    last_check_file: str = LAST_CHECK_FILE,
    broker_factory=None,
) -> list[dict]:
    """Check inbox if rate limit allows. Returns messages found.

    Uses notification files first (fast path), falls back to SQLite query.
    Clears the notification file after reading messages.

    Args:
        project: Project name to check inbox for
        last_check_file: Path to rate-limit tracking file
        broker_factory: Optional callable returning a Broker (for testing)
    """
    if not should_check(last_check_file):
        return []

    mark_checked(last_check_file)

    try:
        if broker_factory:
            broker = broker_factory()
        else:
            from .broker import Broker
            broker = Broker()

        # Fast path: check notification file first
        notification = broker.read_notification(project)
        if notification is None:
            return []

        # Notification exists — read actual messages from SQLite
        messages = broker.check_inbox(project)
        broker.clear_notification(project)
    except Exception:  # pragma: no cover — broker init may fail
        return []

    if not messages:
        return []

    result = []
    for m in messages:
        result.append({
            "id": m.id,
            "type": m.type,
            "from": m.source_project,
            "title": m.title,
            "severity": m.severity,
        })

    return result


def format_notification(messages: list[dict]) -> str:
    """Format messages for display."""
    if not messages:
        return ""

    lines = [f"\n[SESSION BUS] {len(messages)} new message(s):"]
    for m in messages:
        icon = "!!" if m["severity"] == "high" else "!"
        lines.append(f"  [{icon}] {m['type']} from {m['from']}: {m['title']}")
    lines.append("  Run /inbox to process them.\n")
    return "\n".join(lines)
