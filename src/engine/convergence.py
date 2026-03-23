"""Core convergence loop logic.

All functions are deterministic — integer comparisons, list operations,
real-clock checks. No LLM calls.
"""

import subprocess
import time
from typing import Optional

from .state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    RoundResult,
    TestRunResult,
)

PHASE_ORDER = [
    ConvergencePhase.PLANNING,
    ConvergencePhase.PLAN_AUDITING,
    ConvergencePhase.DOC_ALIGNMENT,
    ConvergencePhase.VIABILITY,
    ConvergencePhase.EXECUTING,
    ConvergencePhase.CODE_AUDITING,
    ConvergencePhase.DOC_AUDITING,
    ConvergencePhase.E2E_TESTING,
    ConvergencePhase.PATTERNS_UPDATE,
    ConvergencePhase.CONVERGED,
]


def advance_phase(current: ConvergencePhase) -> ConvergencePhase:
    """Move to the next phase in the sequence."""
    idx = PHASE_ORDER.index(current)
    if idx + 1 >= len(PHASE_ORDER):
        return current
    return PHASE_ORDER[idx + 1]


def is_terminal(phase: ConvergencePhase) -> bool:
    """Check if phase is a terminal state."""
    return phase in (ConvergencePhase.CONVERGED, ConvergencePhase.ESCALATED)


def check_timeout(state: ConvergenceState) -> bool:
    """Real clock check. Code, not LLM."""
    if state.deadline is None:
        return False
    return time.time() > state.deadline


PHASE_MAX_ROUNDS: dict[str, int] = {
    "planning": 0,          # Uses state.max_rounds
    "plan_auditing": 0,     # Uses state.max_rounds
    "doc_alignment": 0,     # Uses state.max_rounds
    "viability": 3,
    "executing": -1,        # -1 = auto-scale from checklist items
    "code_auditing": 0,     # Uses state.max_rounds
    "doc_auditing": 0,      # Uses state.max_rounds
    "e2e_testing": 0,       # Uses state.max_rounds
    "patterns_update": 3,
}


def check_max_rounds(state: ConvergenceState) -> bool:
    """Phase-aware max rounds check. Code, not LLM.

    Execution phase uses auto-scaling: total_items * 2 as safety valve.
    Other phases use state.max_rounds or phase-specific limits.
    """
    phase_limit = PHASE_MAX_ROUNDS.get(state.phase.value, 0)

    if phase_limit == -1:
        # Auto-scale for execution: safety valve at 2x checklist items
        from .checklist_parser import parse_checklist
        items = parse_checklist(state.plan_file)
        total = len(items) if items else state.max_rounds
        limit = max(total * 2, state.max_rounds)
        return state.round >= limit

    limit = phase_limit if phase_limit > 0 else state.max_rounds
    return state.round >= limit


def check_net_negative(state: ConvergenceState) -> bool:
    """Compare last rounds' issue counts. Two consecutive increases = net negative."""
    if len(state.history) < 3:
        return False
    last = len(state.history[-1].findings)
    prev = len(state.history[-2].findings)
    prev_prev = len(state.history[-3].findings)
    return last > prev and prev > prev_prev


def check_convergence(state: ConvergenceState) -> bool:
    """Two consecutive clean passes. Code, not LLM."""
    return state.consecutive_clean >= state.convergence_threshold


def record_round(state: ConvergenceState, findings: list[Finding]) -> None:
    """Record a round's results. Update counters. Code, not LLM."""
    result = RoundResult(
        round=state.round,
        phase=state.phase,
        findings=findings,
        findings_fixed=sum(1 for f in findings if f.fixed),
        timestamp=time.time(),
    )
    state.history.append(result)

    if len(findings) == 0:
        state.consecutive_clean += 1
    else:
        state.consecutive_clean = 0

    state.round += 1


def check_structural_independence(
    round_a: RoundResult, round_b: RoundResult
) -> bool:
    """Code-level check: were two passes structurally independent?
    Different round numbers and consecutive."""
    if round_a.round == round_b.round:
        return False
    if abs(round_a.round - round_b.round) != 1:
        return False
    return True


def should_rollback(state: ConvergenceState, task_id: str) -> bool:
    """Three failures on same task. Code, not LLM."""
    return state.failures.get(task_id, 0) >= state.max_failures


def record_failure(state: ConvergenceState, task_id: str) -> None:
    """Increment failure counter. Code, not LLM."""
    state.failures[task_id] = state.failures.get(task_id, 0) + 1


def compact_history(state: ConvergenceState) -> None:
    """Summarize old rounds into aggregate stats.
    Prevents unbounded history growth."""
    if len(state.history) <= 5:
        return
    old = state.history[:-3]
    summary = RoundResult(
        round=-1,  # sentinel for "summary"
        phase=old[0].phase,
        findings=[],
        findings_fixed=sum(r.findings_fixed for r in old),
        timestamp=old[-1].timestamp,
    )
    state.history = [summary] + state.history[-3:]


def escalate(state: ConvergenceState, reason: str) -> None:
    """Move to escalated state with reason."""
    state.phase = ConvergencePhase.ESCALATED
    state.escalation_reason = reason
    state.updated_at = time.time()


def run_tests(
    project_dir: str,
    test_command: list[str],
    timeout: float = 300.0,
) -> TestRunResult:
    """Run project test suite via subprocess. Code runs tests, not LLM."""
    start = time.time()
    try:
        result = subprocess.run(
            test_command,
            capture_output=True,
            timeout=timeout,
            cwd=project_dir,
        )
        duration = time.time() - start
        output = result.stdout.decode(errors="replace")
        stderr = result.stderr.decode(errors="replace")
        passed = result.returncode == 0
        return TestRunResult(
            passed=passed,
            total=0,  # Parsed from output by caller
            failures=0 if passed else 1,
            output=output + stderr,
            duration_seconds=duration,
        )
    except subprocess.TimeoutExpired:
        return TestRunResult(
            passed=False,
            total=0,
            failures=1,
            output="Test run timed out",
            duration_seconds=timeout,
        )
