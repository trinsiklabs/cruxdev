"""Write-ahead log for convergence events.

Append-only JSONL log. Every mutation is logged BEFORE the state snapshot
is updated. Enables crash recovery and audit trails.

WAL file: <project_dir>/.cruxdev/convergence_state/<id>.wal
"""

import json
import os
import time


def _wal_path(state_path: str) -> str:
    """Derive WAL path from state path (.json → .wal)."""
    return state_path.rsplit(".", 1)[0] + ".wal"


def append(state_path: str, event_type: str, details: dict | None = None) -> None:
    """Append an event to the WAL. fsync for durability."""
    wal = _wal_path(state_path)
    os.makedirs(os.path.dirname(wal), exist_ok=True)

    entry = {
        "timestamp": time.time(),
        "event_type": event_type,
    }
    if details:
        entry.update(details)

    line = json.dumps(entry) + "\n"
    fd = os.open(wal, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    try:
        os.write(fd, line.encode())
        os.fsync(fd)
    finally:
        os.close(fd)


def read(state_path: str) -> list[dict]:
    """Read all WAL events."""
    wal = _wal_path(state_path)
    if not os.path.exists(wal):
        return []
    events = []
    with open(wal) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    events.append(json.loads(line))
                except json.JSONDecodeError:
                    continue
    return events


def event_count(state_path: str) -> int:
    """Count WAL events without loading them all."""
    wal = _wal_path(state_path)
    if not os.path.exists(wal):
        return 0
    count = 0
    with open(wal) as f:
        for line in f:
            if line.strip():
                count += 1
    return count
