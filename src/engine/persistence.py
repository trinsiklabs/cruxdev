"""Atomic state persistence.

Write-then-rename for crash safety. All serialization is deterministic.
"""

import json
import os
import tempfile
import time
from dataclasses import asdict
from typing import Any

from .state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
    RoundResult,
    TestRunResult,
)


def serialize(state: ConvergenceState) -> dict[str, Any]:
    """Convert ConvergenceState to a JSON-serializable dict."""
    data = asdict(state)
    # Convert enums to their values
    data["phase"] = state.phase.value
    data["history"] = [
        {
            **asdict(r),
            "phase": r.phase.value,
            "findings": [
                {
                    **asdict(f),
                    "severity": f.severity.value,
                }
                for f in r.findings
            ],
        }
        for r in state.history
    ]
    return data


def deserialize(data: dict[str, Any]) -> ConvergenceState:
    """Convert a dict back to ConvergenceState."""
    history = []
    for r in data.get("history", []):
        findings = [
            Finding(
                id=f["id"],
                file=f["file"],
                dimension=f["dimension"],
                severity=FindingSeverity(f["severity"]),
                description=f["description"],
                suggested_fix=f["suggested_fix"],
                fixed=f.get("fixed", False),
            )
            for f in r.get("findings", [])
        ]
        history.append(
            RoundResult(
                round=r["round"],
                phase=ConvergencePhase(r["phase"]),
                findings=findings,
                findings_fixed=r["findings_fixed"],
                timestamp=r["timestamp"],
            )
        )

    return ConvergenceState(
        plan_file=data["plan_file"],
        phase=ConvergencePhase(data["phase"]),
        round=data.get("round", 0),
        max_rounds=data.get("max_rounds", 5),
        consecutive_clean=data.get("consecutive_clean", 0),
        convergence_threshold=data.get("convergence_threshold", 2),
        failures=data.get("failures", {}),
        max_failures=data.get("max_failures", 3),
        deadline=data.get("deadline"),
        timeout_per_task=data.get("timeout_per_task", 900.0),
        history=history,
        escalation_reason=data.get("escalation_reason"),
        created_at=data.get("created_at", 0.0),
        updated_at=data.get("updated_at", 0.0),
        project_dir=data.get("project_dir", ""),
    )


def save_state(state: ConvergenceState, path: str) -> None:
    """Atomic write of convergence state to disk."""
    state.updated_at = time.time()
    data = serialize(state)
    parent = os.path.dirname(os.path.abspath(path))
    os.makedirs(parent, exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=parent, prefix=".convergence_")
    try:
        os.write(fd, json.dumps(data, indent=2).encode())
        os.close(fd)
        fd = -1
        os.rename(tmp, os.path.abspath(path))  # atomic on POSIX
    except Exception:
        if fd >= 0:
            os.close(fd)
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def load_state(path: str) -> ConvergenceState:
    """Load convergence state from disk."""
    with open(path) as f:
        data = json.load(f)
    return deserialize(data)
