#!/usr/bin/env python3
"""Standalone hook runner — called by Claude Code's PostToolUse hook.

Usage: python3 hook_runner.py PROJECT_NAME
"""

import os
import sys

# Add cruxdev to path
cruxdev_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, cruxdev_root)

from src.bus.hook import check_and_notify, format_notification


def main():  # pragma: no cover — entry point
    project = sys.argv[1] if len(sys.argv) > 1 else os.path.basename(os.getcwd())
    messages = check_and_notify(project)
    if messages:
        notification = format_notification(messages)
        print(notification, file=sys.stderr)


if __name__ == "__main__":  # pragma: no cover
    main()
