"""Evolution state — tracks the autonomous evolution loop.

Two-layer memory:
- Layer 1: Append-only JSONL archive (immutable history)
- Layer 2: Synthesized active context (compressed working memory)
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import asdict, dataclass, field
from typing import Optional


@dataclass
class EvolutionCycle:
    cycle_id: int
    started_at: float
    completed_at: Optional[float] = None
    beat: str = "gather"  # gather, evaluate, integrate, post, engage
    gathered: list[str] = field(default_factory=list)
    evaluated: list[dict] = field(default_factory=list)
    integrated: list[str] = field(default_factory=list)
    posted: list[str] = field(default_factory=list)
    engaged: list[str] = field(default_factory=list)
    error: Optional[str] = None


@dataclass
class EvolutionState:
    project: str
    cycle_count: int = 0
    current_cycle: Optional[EvolutionCycle] = None
    last_completed_at: Optional[float] = None
    protected_files: list[str] = field(default_factory=lambda: [
        "CLAUDE.md",
        ".claude/CLAUDE.md",
        "LICENSE",
        "README.md",
        "src/engine/convergence.py",
        "src/engine/state.py",
        "src/bus/broker.py",
    ])
    north_star_goals: list[str] = field(default_factory=list)


# --- Layer 1: Append-only archive ---


def append_to_archive(archive_path: str, entry: dict) -> None:
    """Append an entry to the JSONL archive. Immutable — never modified."""
    os.makedirs(os.path.dirname(archive_path), exist_ok=True)
    entry["timestamp"] = time.time()
    with open(archive_path, "a") as f:
        f.write(json.dumps(entry) + "\n")


def read_archive(archive_path: str, max_entries: int = 1000) -> list[dict]:
    """Read entries from the archive."""
    if not os.path.exists(archive_path):
        return []
    entries = []
    with open(archive_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                entries.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    return entries[-max_entries:]


# --- Layer 2: Synthesized active context ---


def save_active_context(context_path: str, state: EvolutionState) -> None:
    """Save the active context (compressed working memory)."""
    os.makedirs(os.path.dirname(context_path), exist_ok=True)
    data = asdict(state)
    with open(context_path, "w") as f:
        json.dump(data, f, indent=2)


def load_active_context(context_path: str) -> EvolutionState:
    """Load the active context."""
    if not os.path.exists(context_path):
        return EvolutionState(project="unknown")
    with open(context_path) as f:
        data = json.load(f)

    cycle_data = data.pop("current_cycle", None)
    current_cycle = None
    if cycle_data:
        current_cycle = EvolutionCycle(**cycle_data)

    return EvolutionState(
        project=data.get("project", "unknown"),
        cycle_count=data.get("cycle_count", 0),
        current_cycle=current_cycle,
        last_completed_at=data.get("last_completed_at"),
        protected_files=data.get("protected_files", []),
        north_star_goals=data.get("north_star_goals", []),
    )


# --- Protected files ---


def check_protected_files(
    state: EvolutionState,
    changed_files: list[str],
) -> list[str]:
    """Check if any changed files are protected. Returns list of violations."""
    violations = []
    for f in changed_files:
        for protected in state.protected_files:
            if f == protected or f.endswith("/" + protected):
                violations.append(f)
    return violations
