"""Research session — state tracking, checkpointing, recovery.

Each research session tracks: topic, sub-questions, convergence state,
findings, sources fetched, quality scores.
"""

from __future__ import annotations

import json
import os
import time
import uuid
from dataclasses import asdict, dataclass, field
from typing import Optional


@dataclass
class ResearchFinding:
    id: str
    content: str
    source_url: str
    quality_score: float = 0.0
    robustness: str = "moderate"  # robust, moderate, fragile, contested
    pass_found: int = 1  # Which pass found this (1-5)
    counter_evidence: list[str] = field(default_factory=list)
    tags: list[str] = field(default_factory=list)


@dataclass
class ResearchSession:
    session_id: str
    topic: str
    sub_questions: list[str] = field(default_factory=list)
    findings: list[ResearchFinding] = field(default_factory=list)
    seen_urls: list[str] = field(default_factory=list)
    total_searches: int = 0
    novelty_scores: list[float] = field(default_factory=list)
    current_pass: int = 1
    converged: bool = False
    budget_exhausted: bool = False
    quality_score: float = 0.0
    started_at: float = field(default_factory=time.time)
    completed_at: Optional[float] = None


def create_session(topic: str, sub_questions: list[str] | None = None) -> ResearchSession:
    """Create a new research session."""
    return ResearchSession(
        session_id=str(uuid.uuid4())[:8],
        topic=topic,
        sub_questions=sub_questions or [],
    )


def save_checkpoint(session: ResearchSession, checkpoint_dir: str) -> str:
    """Save session state to disk for crash recovery."""
    os.makedirs(checkpoint_dir, exist_ok=True)
    path = os.path.join(checkpoint_dir, f"{session.session_id}.json")

    data = asdict(session)
    # Convert findings to dicts
    data["findings"] = [asdict(f) for f in session.findings]

    with open(path, "w") as f:
        json.dump(data, f, indent=2)
    return path


def load_checkpoint(checkpoint_path: str) -> ResearchSession | None:
    """Load a session from checkpoint."""
    if not os.path.exists(checkpoint_path):
        return None
    try:
        with open(checkpoint_path) as f:
            data = json.load(f)
        findings = [ResearchFinding(**fd) for fd in data.pop("findings", [])]
        session = ResearchSession(**data)
        session.findings = findings
        return session
    except (json.JSONDecodeError, TypeError, KeyError):
        return None


def find_latest_checkpoint(checkpoint_dir: str, topic: str) -> str | None:
    """Find the most recent checkpoint for a topic."""
    if not os.path.isdir(checkpoint_dir):
        return None
    candidates = []
    for f in os.listdir(checkpoint_dir):
        if not f.endswith(".json"):
            continue
        path = os.path.join(checkpoint_dir, f)
        session = load_checkpoint(path)
        if session and session.topic == topic and not session.converged:
            candidates.append((session.started_at, path))
    if not candidates:
        return None
    candidates.sort(reverse=True)
    return candidates[0][1]
