"""Tests for core convergence loop logic."""

import os
import sys
import time
from unittest.mock import patch

import pytest

from src.engine.convergence import (
    PHASE_ORDER,
    advance_phase,
    check_convergence,
    check_max_rounds,
    check_net_negative,
    check_structural_independence,
    check_timeout,
    compact_history,
    escalate,
    is_terminal,
    record_failure,
    record_round,
    run_tests,
    should_rollback,
)
from src.engine.state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
    RoundResult,
)


# --- Phase advancement ---


def test_advance_phase_planning():
    assert advance_phase(ConvergencePhase.PLANNING) == ConvergencePhase.PLAN_AUDITING


def test_advance_phase_through_all():
    phase = ConvergencePhase.PLANNING
    visited = [phase]
    while phase != ConvergencePhase.CONVERGED:
        phase = advance_phase(phase)
        visited.append(phase)
    assert visited[-1] == ConvergencePhase.CONVERGED
    assert len(visited) == len(PHASE_ORDER)


def test_advance_phase_at_converged():
    assert advance_phase(ConvergencePhase.CONVERGED) == ConvergencePhase.CONVERGED


def test_phase_order_length():
    assert len(PHASE_ORDER) == 10
    assert PHASE_ORDER[0] == ConvergencePhase.PLANNING
    assert PHASE_ORDER[-1] == ConvergencePhase.CONVERGED


# --- Terminal check ---


def test_is_terminal_converged():
    assert is_terminal(ConvergencePhase.CONVERGED) is True


def test_is_terminal_escalated():
    assert is_terminal(ConvergencePhase.ESCALATED) is True


def test_is_terminal_planning():
    assert is_terminal(ConvergencePhase.PLANNING) is False


def test_is_terminal_executing():
    assert is_terminal(ConvergencePhase.EXECUTING) is False


# --- Timeout ---


def test_check_timeout_no_deadline():
    s = ConvergenceState(plan_file="p.md")
    assert check_timeout(s) is False


def test_check_timeout_not_expired():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() + 1000)
    assert check_timeout(s) is False


def test_check_timeout_expired():
    s = ConvergenceState(plan_file="p.md", deadline=time.time() - 1)
    assert check_timeout(s) is True


# --- Max rounds ---


def test_check_max_rounds_not_reached():
    s = ConvergenceState(plan_file="p.md", round=2, max_rounds=5)
    assert check_max_rounds(s) is False


def test_check_max_rounds_reached():
    s = ConvergenceState(plan_file="p.md", round=5, max_rounds=5)
    assert check_max_rounds(s) is True


def test_check_max_rounds_exceeded():
    s = ConvergenceState(plan_file="p.md", round=6, max_rounds=5)
    assert check_max_rounds(s) is True


def test_check_max_rounds_execution_phase_auto_scales(tmp_path):
    """Execution phase scales max_rounds to 2x checklist items."""
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n" + "".join(
        f"- [ ] 1.{i} Item {i}\n" for i in range(20)
    ))
    s = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
        round=10,
        max_rounds=5,
    )
    # 20 items * 2 = 40 limit. Round 10 < 40 → not exceeded
    assert check_max_rounds(s) is False


def test_check_max_rounds_execution_phase_escalates_at_2x(tmp_path):
    """Execution escalates at max(2x total items, max_rounds)."""
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [ ] 1.1 Item\n- [ ] 1.2 Item\n")
    s = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
        round=5,  # max(2*2, 5) = 5 → at limit
        max_rounds=5,
    )
    assert check_max_rounds(s) is True


def test_check_max_rounds_execution_no_plan_uses_default():
    """If plan file missing, execution uses max(max_rounds*2, max_rounds) = max_rounds*2."""
    s = ConvergenceState(
        plan_file="/nonexistent.md",
        phase=ConvergencePhase.EXECUTING,
        round=10,  # max(5*2, 5) = 10
        max_rounds=5,
    )
    assert check_max_rounds(s) is True


def test_check_max_rounds_viability_uses_3():
    """Viability phase caps at 3 rounds."""
    s = ConvergenceState(
        plan_file="p.md",
        phase=ConvergencePhase.VIABILITY,
        round=3,
        max_rounds=10,
    )
    assert check_max_rounds(s) is True


def test_check_max_rounds_patterns_update_uses_3():
    """Patterns update phase caps at 3 rounds."""
    s = ConvergenceState(
        plan_file="p.md",
        phase=ConvergencePhase.PATTERNS_UPDATE,
        round=3,
        max_rounds=10,
    )
    assert check_max_rounds(s) is True


# --- Net negative ---


def _make_round(round_num: int, num_findings: int) -> RoundResult:
    findings = [
        Finding(
            id=f"f{i}",
            file="x.py",
            dimension="d",
            severity=FindingSeverity.LOW,
            description="d",
            suggested_fix="f",
        )
        for i in range(num_findings)
    ]
    return RoundResult(
        round=round_num,
        phase=ConvergencePhase.CODE_AUDITING,
        findings=findings,
        findings_fixed=0,
        timestamp=time.time(),
    )


def test_check_net_negative_too_few_rounds():
    s = ConvergenceState(plan_file="p.md")
    s.history = [_make_round(0, 3), _make_round(1, 4)]
    assert check_net_negative(s) is False


def test_check_net_negative_increasing():
    s = ConvergenceState(plan_file="p.md")
    s.history = [_make_round(0, 3), _make_round(1, 5), _make_round(2, 8)]
    assert check_net_negative(s) is True


def test_check_net_negative_decreasing():
    s = ConvergenceState(plan_file="p.md")
    s.history = [_make_round(0, 8), _make_round(1, 5), _make_round(2, 3)]
    assert check_net_negative(s) is False


def test_check_net_negative_mixed():
    s = ConvergenceState(plan_file="p.md")
    s.history = [_make_round(0, 3), _make_round(1, 5), _make_round(2, 4)]
    assert check_net_negative(s) is False


# --- Convergence ---


def test_check_convergence_zero_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=0)
    assert check_convergence(s) is False


def test_check_convergence_one_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=1)
    assert check_convergence(s) is False


def test_check_convergence_two_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=2)
    assert check_convergence(s) is True


def test_check_convergence_three_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=3)
    assert check_convergence(s) is True


def test_check_convergence_custom_threshold():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=2, convergence_threshold=3)
    assert check_convergence(s) is False
    s.consecutive_clean = 3
    assert check_convergence(s) is True


# --- Record round ---


def test_record_round_clean():
    s = ConvergenceState(plan_file="p.md")
    record_round(s, [])
    assert s.round == 1
    assert s.consecutive_clean == 1
    assert len(s.history) == 1
    assert s.history[0].findings_fixed == 0


def test_record_round_with_findings():
    s = ConvergenceState(plan_file="p.md")
    findings = [
        Finding(
            id="f1", file="a.py", dimension="d",
            severity=FindingSeverity.HIGH, description="d", suggested_fix="f",
        )
    ]
    record_round(s, findings)
    assert s.round == 1
    assert s.consecutive_clean == 0
    assert len(s.history) == 1


def test_record_round_resets_consecutive_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=1)
    findings = [
        Finding(
            id="f1", file="a.py", dimension="d",
            severity=FindingSeverity.LOW, description="d", suggested_fix="f",
        )
    ]
    record_round(s, findings)
    assert s.consecutive_clean == 0


def test_record_round_increments_consecutive_clean():
    s = ConvergenceState(plan_file="p.md", consecutive_clean=1)
    record_round(s, [])
    assert s.consecutive_clean == 2


def test_record_round_counts_fixed():
    findings = [
        Finding(
            id="f1", file="a.py", dimension="d",
            severity=FindingSeverity.LOW, description="d", suggested_fix="f", fixed=True,
        ),
        Finding(
            id="f2", file="b.py", dimension="d",
            severity=FindingSeverity.LOW, description="d", suggested_fix="f", fixed=False,
        ),
    ]
    s = ConvergenceState(plan_file="p.md")
    record_round(s, findings)
    assert s.history[0].findings_fixed == 1


# --- Structural independence ---


def test_structural_independence_same_round():
    a = _make_round(1, 0)
    b = _make_round(1, 0)
    assert check_structural_independence(a, b) is False


def test_structural_independence_consecutive():
    a = _make_round(1, 0)
    b = _make_round(2, 0)
    assert check_structural_independence(a, b) is True


def test_structural_independence_non_consecutive():
    a = _make_round(1, 0)
    b = _make_round(3, 0)
    assert check_structural_independence(a, b) is False


# --- Failure tracking ---


def test_record_failure():
    s = ConvergenceState(plan_file="p.md")
    record_failure(s, "task1")
    assert s.failures["task1"] == 1
    record_failure(s, "task1")
    assert s.failures["task1"] == 2


def test_should_rollback_below_threshold():
    s = ConvergenceState(plan_file="p.md")
    s.failures["task1"] = 2
    assert should_rollback(s, "task1") is False


def test_should_rollback_at_threshold():
    s = ConvergenceState(plan_file="p.md")
    s.failures["task1"] = 3
    assert should_rollback(s, "task1") is True


def test_should_rollback_unknown_task():
    s = ConvergenceState(plan_file="p.md")
    assert should_rollback(s, "unknown") is False


# --- History compaction ---


def test_compact_history_short():
    s = ConvergenceState(plan_file="p.md")
    for i in range(5):
        s.history.append(_make_round(i, 1))
    compact_history(s)
    assert len(s.history) == 5  # No compaction needed


def test_compact_history_long():
    s = ConvergenceState(plan_file="p.md")
    for i in range(8):
        r = _make_round(i, i % 3)
        r.findings_fixed = 1 if i % 2 == 0 else 0
        s.history.append(r)
    compact_history(s)
    assert len(s.history) == 4  # 1 summary + 3 recent
    assert s.history[0].round == -1  # sentinel
    assert s.history[1].round == 5
    assert s.history[2].round == 6
    assert s.history[3].round == 7


def test_compact_history_preserves_fixed_sum():
    s = ConvergenceState(plan_file="p.md")
    for i in range(8):
        r = _make_round(i, 2)
        r.findings_fixed = 1
        s.history.append(r)
    total_fixed_before = sum(r.findings_fixed for r in s.history)
    compact_history(s)
    total_fixed_after = sum(r.findings_fixed for r in s.history)
    assert total_fixed_after == total_fixed_before


# --- Escalation ---


def test_escalate():
    s = ConvergenceState(plan_file="p.md")
    escalate(s, "too many failures")
    assert s.phase == ConvergencePhase.ESCALATED
    assert s.escalation_reason == "too many failures"


# --- Test runner ---


def test_run_tests_success(tmp_path):
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\necho 'all passed'\nexit 0\n")
    script.chmod(0o755)
    result = run_tests(str(tmp_path), [str(script)], timeout=10)
    assert result.passed is True
    assert "all passed" in result.output


def test_run_tests_failure(tmp_path):
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\necho 'FAILED'\nexit 1\n")
    script.chmod(0o755)
    result = run_tests(str(tmp_path), [str(script)], timeout=10)
    assert result.passed is False


def test_run_tests_timeout(tmp_path):
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nsleep 30\n")
    script.chmod(0o755)
    result = run_tests(str(tmp_path), [str(script)], timeout=1)
    assert result.passed is False
    assert "timed out" in result.output.lower()
