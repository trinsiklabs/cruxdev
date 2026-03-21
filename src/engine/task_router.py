"""Task router — determines what Claude Code should do next.

The engine doesn't call the LLM. It tells Claude Code what to do,
and processes the results. This module maps engine state to tasks.
"""

import json
import time
from dataclasses import dataclass
from typing import Optional

from .convergence import (
    check_convergence,
    check_max_rounds,
    check_net_negative,
    check_timeout,
    escalate,
    record_failure,
    record_round,
)
from .persistence import save_state
from .plan_validator import validate_plan
from .state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
)

# Dimension sets for each convergence type
PLAN_DIMENSIONS = ["completeness", "feasibility", "risk_assessment", "dependency_ordering", "testability"]
CODE_DIMENSIONS = ["correctness", "completeness", "edge_cases", "error_handling", "security", "performance", "maintainability", "test_coverage"]
DOC_DIMENSIONS = ["accuracy", "completeness", "consistency", "clarity", "currency"]


# Task type → recommended model tier
TASK_MODEL_TIERS: dict[str, str] = {
    "audit": "standard",
    "fix": "standard",
    "test": "fast",
    "write": "standard",
}

# Phase-specific overrides
PHASE_MODEL_TIERS: dict[str, str] = {
    "plan_auditing": "fast",
    "code_auditing": "standard",
    "doc_auditing": "fast",
    "e2e_testing": "fast",
}


@dataclass
class Task:
    """A task for Claude Code to execute."""
    task_type: str  # "audit", "fix", "test", "write", "done", "escalated"
    description: str
    files: list[str]
    dimensions: list[str]
    finding: Optional[dict] = None
    test_command: Optional[list[str]] = None
    metadata: Optional[dict] = None
    recommended_tier: Optional[str] = None  # e.g., "fast", "standard", "frontier"

    def to_dict(self) -> dict:
        d = {
            "task_type": self.task_type,
            "description": self.description,
            "files": self.files,
            "dimensions": self.dimensions,
        }
        if self.finding:
            d["finding"] = self.finding
        if self.test_command:
            d["test_command"] = self.test_command
        if self.metadata:
            d["metadata"] = self.metadata
        if self.recommended_tier:
            d["recommended_tier"] = self.recommended_tier
        return d


def get_next_task(
    state: ConvergenceState,
    state_path: str,
    source_files: list[str] | None = None,
    doc_files: list[str] | None = None,
    test_command: list[str] | None = None,
) -> Task:
    """Determine the next task for Claude Code based on engine state."""

    # Terminal states
    if state.phase == ConvergencePhase.CONVERGED:
        return Task(
            task_type="done",
            description="Convergence complete. Two consecutive clean passes achieved.",
            files=[], dimensions=[],
            metadata={"round": state.round, "history_length": len(state.history)},
        )

    if state.phase == ConvergencePhase.ESCALATED:
        return Task(
            task_type="escalated",
            description=f"Escalated: {state.escalation_reason}",
            files=[], dimensions=[],
            metadata={"reason": state.escalation_reason, "round": state.round},
        )

    # Safety checks
    if check_timeout(state):
        escalate(state, f"{state.phase.value}_timeout")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    if check_max_rounds(state):
        escalate(state, f"{state.phase.value}_max_rounds")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    if check_net_negative(state):
        escalate(state, f"{state.phase.value}_net_negative")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    # Phase-specific tasks
    # Determine model tier for this phase
    phase_tier = PHASE_MODEL_TIERS.get(state.phase.value)

    if state.phase == ConvergencePhase.PLANNING:
        return Task(
            task_type="write",
            description="Create or refine the build plan. Follow the methodology from get_methodology().",
            recommended_tier=TASK_MODEL_TIERS.get("write"),
            files=[state.plan_file],
            dimensions=[],
        )

    if state.phase == ConvergencePhase.PLAN_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.VIABILITY
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        return Task(
            task_type="audit",
            description=f"Audit the plan (round {state.round}). Check for gaps, risks, missing steps.",
            files=[state.plan_file],
            dimensions=PLAN_DIMENSIONS,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.VIABILITY:
        # Auto-advance — viability is a pass-through for now
        state.phase = ConvergencePhase.EXECUTING
        save_state(state, state_path)
        return get_next_task(state, state_path, source_files, doc_files, test_command)

    if state.phase == ConvergencePhase.EXECUTING:
        # Auto-advance — execution happens during code auditing
        state.phase = ConvergencePhase.CODE_AUDITING
        state.round = 0
        state.consecutive_clean = 0
        save_state(state, state_path)
        return get_next_task(state, state_path, source_files, doc_files, test_command)

    if state.phase == ConvergencePhase.CODE_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.DOC_AUDITING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        files = source_files or [state.plan_file]
        return Task(
            task_type="audit",
            description=f"Audit code (round {state.round}). Read the files, find issues, fix them, run tests.",
            files=files,
            dimensions=CODE_DIMENSIONS,
            test_command=test_command,
            recommended_tier=phase_tier or "standard",
        )

    if state.phase == ConvergencePhase.DOC_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.E2E_TESTING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        files = doc_files or [state.plan_file]
        return Task(
            task_type="audit",
            description=f"Audit documentation (round {state.round}). Check accuracy, completeness, consistency.",
            files=files,
            dimensions=DOC_DIMENSIONS,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.E2E_TESTING:
        if check_convergence(state):
            state.phase = ConvergencePhase.PATTERNS_UPDATE
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        return Task(
            task_type="test",
            description=f"Run the full test suite (round {state.round}).",
            files=[],
            dimensions=[],
            test_command=test_command,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.PATTERNS_UPDATE:
        # Auto-advance to converged
        state.phase = ConvergencePhase.CONVERGED
        save_state(state, state_path)
        return get_next_task(state, state_path)

    # Shouldn't reach here
    return Task(  # pragma: no cover
        task_type="escalated",
        description=f"Unknown phase: {state.phase.value}",
        files=[], dimensions=[],
    )


def submit_result(
    state: ConvergenceState,
    state_path: str,
    result: dict,
) -> None:
    """Process a result from Claude Code and update engine state."""
    task_type = result.get("task_type", "audit")
    findings_data = result.get("findings", [])

    findings = [
        Finding(
            id=f.get("id", f"f{i}"),
            file=f.get("file", "unknown"),
            dimension=f.get("dimension", "unknown"),
            severity=FindingSeverity(f.get("severity", "medium")),
            description=f.get("description", ""),
            suggested_fix=f.get("suggested_fix", ""),
            fixed=f.get("fixed", False),
        )
        for i, f in enumerate(findings_data)
    ]

    # Track failures for unfixed findings
    for f in findings:
        if not f.fixed:
            record_failure(state, f.id)

    record_round(state, findings)
    save_state(state, state_path)
