"""Tests for engine state data structures."""

import time

from src.engine.state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
    RoundResult,
    TestRunResult,
)


def test_convergence_phase_values():
    assert ConvergencePhase.PLANNING.value == "planning"
    assert ConvergencePhase.CONVERGED.value == "converged"
    assert ConvergencePhase.ESCALATED.value == "escalated"


def test_finding_severity_values():
    assert FindingSeverity.HIGH.value == "high"
    assert FindingSeverity.MEDIUM.value == "medium"
    assert FindingSeverity.LOW.value == "low"


def test_finding_defaults():
    f = Finding(
        id="f1",
        file="foo.py",
        dimension="correctness",
        severity=FindingSeverity.HIGH,
        description="bug",
        suggested_fix="fix it",
    )
    assert f.fixed is False
    assert f.id == "f1"


def test_finding_fixed():
    f = Finding(
        id="f1",
        file="foo.py",
        dimension="correctness",
        severity=FindingSeverity.LOW,
        description="style",
        suggested_fix="reformat",
        fixed=True,
    )
    assert f.fixed is True


def test_test_run_result_defaults():
    r = TestRunResult(passed=True, total=10, failures=0)
    assert r.coverage is None
    assert r.output == ""
    assert r.duration_seconds == 0.0


def test_test_run_result_full():
    r = TestRunResult(
        passed=False,
        total=10,
        failures=2,
        coverage=85.5,
        output="2 failed",
        duration_seconds=3.5,
    )
    assert r.failures == 2
    assert r.coverage == 85.5


def test_round_result():
    r = RoundResult(
        round=0,
        phase=ConvergencePhase.CODE_AUDITING,
        findings=[],
        findings_fixed=0,
        timestamp=1000.0,
    )
    assert r.round == 0
    assert r.phase == ConvergencePhase.CODE_AUDITING


def test_convergence_state_defaults():
    s = ConvergenceState(plan_file="plan.md")
    assert s.plan_file == "plan.md"
    assert s.phase == ConvergencePhase.PLANNING
    assert s.round == 0
    assert s.max_rounds == 5
    assert s.consecutive_clean == 0
    assert s.convergence_threshold == 2
    assert s.failures == {}
    assert s.max_failures == 3
    assert s.deadline is None
    assert s.timeout_per_task == 900.0
    assert s.history == []
    assert s.escalation_reason is None
    assert s.created_at > 0
    assert s.updated_at > 0


def test_convergence_state_custom():
    s = ConvergenceState(
        plan_file="big_plan.md",
        max_rounds=10,
        convergence_threshold=3,
        max_failures=5,
        timeout_per_task=600.0,
    )
    assert s.max_rounds == 10
    assert s.convergence_threshold == 3
    assert s.max_failures == 5
    assert s.timeout_per_task == 600.0


def test_all_phases_exist():
    phases = list(ConvergencePhase)
    assert len(phases) == 11
    assert ConvergencePhase.PLANNING in phases
    assert ConvergencePhase.ESCALATED in phases
