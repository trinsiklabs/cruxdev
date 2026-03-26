"""Convergence index — maps plan files to active convergence runs.

Enables deterministic resume: given a plan file, find the active run
instead of accidentally starting a new one.

Index stored at <project_dir>/.cruxdev/convergence_index.json
"""

import json
import os
import time
from dataclasses import dataclass


@dataclass
class IndexEntry:
    plan_file: str
    convergence_id: str
    status: str  # active, converged, escalated
    started_at: float
    state_path: str


def _index_path(project_dir: str) -> str:
    return os.path.join(project_dir, ".cruxdev", "convergence_index.json")


def load_index(project_dir: str) -> list[IndexEntry]:
    """Load the convergence index for a project."""
    path = _index_path(project_dir)
    if not os.path.exists(path):
        return []
    try:
        with open(path) as f:
            data = json.load(f)
        return [
            IndexEntry(
                plan_file=e["plan_file"],
                convergence_id=e["convergence_id"],
                status=e["status"],
                started_at=e["started_at"],
                state_path=e["state_path"],
            )
            for e in data
        ]
    except (json.JSONDecodeError, KeyError):
        return []


def save_index(project_dir: str, entries: list[IndexEntry]) -> None:
    """Save the convergence index."""
    path = _index_path(project_dir)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    data = [
        {
            "plan_file": e.plan_file,
            "convergence_id": e.convergence_id,
            "status": e.status,
            "started_at": e.started_at,
            "state_path": e.state_path,
        }
        for e in entries
    ]
    tmp = path + ".tmp"
    with open(tmp, "w") as f:
        json.dump(data, f, indent=2)
    os.replace(tmp, path)


def find_active_run(project_dir: str, plan_file: str) -> IndexEntry | None:
    """Find an active convergence run for a plan file."""
    entries = load_index(project_dir)
    # Normalize plan_file for comparison
    norm = os.path.abspath(plan_file)
    for entry in entries:
        if os.path.abspath(entry.plan_file) == norm and entry.status == "active":
            return entry
    return None


def register_run(
    project_dir: str,
    plan_file: str,
    convergence_id: str,
    state_path: str,
) -> IndexEntry:
    """Register a new convergence run in the index."""
    entries = load_index(project_dir)
    entry = IndexEntry(
        plan_file=plan_file,
        convergence_id=convergence_id,
        status="active",
        started_at=time.time(),
        state_path=state_path,
    )
    entries.append(entry)
    save_index(project_dir, entries)
    return entry


def update_run_status(
    project_dir: str,
    convergence_id: str,
    new_status: str,
) -> bool:
    """Update a run's status in the index. Returns True if found."""
    entries = load_index(project_dir)
    for entry in entries:
        if entry.convergence_id == convergence_id:
            entry.status = new_status
            save_index(project_dir, entries)
            return True
    return False
