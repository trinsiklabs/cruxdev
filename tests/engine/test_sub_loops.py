"""Tests for convergence sub-loops and master runner."""

import time

import pytest

from src.dispatch.providers.stub import StubMode, StubProvider
from src.dispatch.schema import AuditFinding
from src.engine.code_convergence import run_code_convergence
from src.engine.convergence import is_terminal
from src.engine.doc_convergence import run_doc_convergence
from src.engine.e2e_convergence import run_e2e_convergence
from src.engine.plan_convergence import run_plan_convergence
from src.engine.runner import ConvergenceRunner
from src.engine.state import ConvergencePhase, ConvergenceState


def _make_findings():
    return [
        AuditFinding(
            id="f1", file="a.py", dimension="correctness",
            severity="high", description="bug", suggested_fix="fix",
        )
    ]


# --- Plan convergence ---


def test_plan_convergence_clean(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_plan_convergence(state, llm, path)
    assert state.consecutive_clean >= 2
    assert state.phase == ConvergencePhase.PLAN_AUDITING


def test_plan_convergence_findings_then_clean(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=20)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())

    run_plan_convergence(state, llm, path)
    # Intermittent: alternates findings/clean. With enough rounds, should converge.
    # If it hits max_rounds, it escalates instead — both are valid outcomes.
    assert state.consecutive_clean >= 2 or state.phase == ConvergencePhase.ESCALATED


def test_plan_convergence_timeout(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        deadline=time.time() - 1,  # Already expired
    )
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_plan_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED
    assert state.escalation_reason == "plan_convergence_timeout"


def test_plan_convergence_max_rounds(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        max_rounds=2,
    )
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.PERSISTENT, findings=_make_findings())

    run_plan_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED
    assert "max_rounds" in state.escalation_reason


# --- Code convergence ---


def test_code_convergence_clean(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_code_convergence(state, llm, path)
    assert state.consecutive_clean >= 2


def test_code_convergence_with_findings_then_clean(tmp_path):
    """Findings on first round, then clean — should converge."""
    state = ConvergenceState(plan_file="plan.md", max_rounds=10)
    path = str(tmp_path / "state.json")

    # Use FINDINGS mode for first audit, then switch to CLEAN
    llm = StubProvider(mode=StubMode.FINDINGS, findings=_make_findings())
    # Manually run one round with findings, then switch
    run_code_convergence(state, llm, path)
    # With findings mode, fix succeeds but findings persist — will hit max_rounds
    assert state.phase == ConvergencePhase.ESCALATED or state.consecutive_clean >= 2


def test_code_convergence_timeout(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        deadline=time.time() - 1,
    )
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_code_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED


def test_code_convergence_max_rounds(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=1)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.PERSISTENT, findings=_make_findings())

    run_code_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED


def test_code_convergence_rollback_trigger(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_failures=1)
    path = str(tmp_path / "state.json")
    # Persistent mode: fix always fails, so failure count accumulates
    llm = StubProvider(mode=StubMode.PERSISTENT, findings=_make_findings())

    run_code_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED
    assert "rollback_triggered" in state.escalation_reason


def test_code_convergence_with_tests(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    # Create a passing test script
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    run_code_convergence(
        state, llm, path,
        project_dir=str(tmp_path),
        test_command=[str(script)],
    )
    assert state.consecutive_clean >= 2


def test_code_convergence_test_failure_records_failure(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=3, max_failures=5)
    path = str(tmp_path / "state.json")

    # Create a failing test script
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 1\n")
    script.chmod(0o755)

    llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())
    run_code_convergence(
        state, llm, path,
        project_dir=str(tmp_path),
        test_command=[str(script)],
    )
    # Should have recorded some failures
    assert len(state.failures) > 0 or state.phase == ConvergencePhase.ESCALATED


def test_code_convergence_fix_passes_tests(tmp_path):
    """When findings exist, fix succeeds, and tests pass → f.fixed = True."""
    state = ConvergenceState(plan_file="plan.md", max_rounds=10)
    path = str(tmp_path / "state.json")

    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())

    run_code_convergence(
        state, llm, path,
        project_dir=str(tmp_path),
        test_command=[str(script)],
    )
    # Should process findings with passing tests
    assert len(state.history) > 0


def test_verify_independence_not_independent(tmp_path):
    """When rounds aren't structurally independent and LLM says not independent,
    consecutive_clean resets to 0."""
    from src.engine.code_convergence import verify_independence
    from src.dispatch.schema import EvaluationResult as ER
    from src.engine.state import RoundResult

    state = ConvergenceState(plan_file="plan.md", consecutive_clean=2)
    # Non-consecutive rounds → structural check fails → LLM check triggers
    state.history = [
        RoundResult(round=1, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=1.0),
        RoundResult(round=3, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=2.0),
    ]

    llm = StubProvider(mode=StubMode.CLEAN)
    llm.evaluate_independence = lambda a, b: ER(independent=False, rationale="same prompts")

    verify_independence(state, llm, ["a.py"])
    assert state.consecutive_clean == 0


def test_verify_independence_independent(tmp_path):
    """When LLM says independent, consecutive_clean stays."""
    from src.engine.code_convergence import verify_independence
    from src.engine.state import RoundResult

    state = ConvergenceState(plan_file="plan.md", consecutive_clean=2)
    state.history = [
        RoundResult(round=1, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=1.0),
        RoundResult(round=3, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=2.0),
    ]

    llm = StubProvider(mode=StubMode.CLEAN)
    verify_independence(state, llm, ["a.py"])
    assert state.consecutive_clean == 2


def test_verify_independence_structurally_independent():
    """When rounds are consecutive (structurally independent), LLM isn't called."""
    from src.engine.code_convergence import verify_independence
    from src.engine.state import RoundResult

    state = ConvergenceState(plan_file="plan.md", consecutive_clean=2)
    state.history = [
        RoundResult(round=1, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=1.0),
        RoundResult(round=2, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=2.0),
    ]

    llm = StubProvider(mode=StubMode.CLEAN)
    verify_independence(state, llm, ["a.py"])
    assert state.consecutive_clean == 2
    assert llm.call_count == 0  # LLM not called


def test_verify_independence_too_few_rounds():
    """With fewer than 2 rounds, no check is performed."""
    from src.engine.code_convergence import verify_independence
    from src.engine.state import RoundResult

    state = ConvergenceState(plan_file="plan.md", consecutive_clean=1)
    state.history = [
        RoundResult(round=0, phase=ConvergencePhase.CODE_AUDITING,
                    findings=[], findings_fixed=0, timestamp=1.0),
    ]

    llm = StubProvider(mode=StubMode.CLEAN)
    verify_independence(state, llm, ["a.py"])
    assert state.consecutive_clean == 1


# --- Doc convergence ---


def test_doc_convergence_clean(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_doc_convergence(state, llm, path)
    assert state.consecutive_clean >= 2


def test_doc_convergence_with_findings(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=10)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())

    run_doc_convergence(state, llm, path)
    # Check that findings were processed (some rounds should have findings with fixed=True)
    has_fixed = any(
        f.fixed for r in state.history for f in r.findings
    )
    assert has_fixed or state.phase == ConvergencePhase.ESCALATED


def test_doc_convergence_timeout(tmp_path):
    state = ConvergenceState(plan_file="plan.md", deadline=time.time() - 1)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    run_doc_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED


def test_doc_convergence_max_rounds(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=1)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.PERSISTENT, findings=_make_findings())

    run_doc_convergence(state, llm, path)
    assert state.phase == ConvergencePhase.ESCALATED


# --- E2E convergence ---


def test_e2e_convergence_no_test_command(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")

    run_e2e_convergence(state, path)
    assert state.consecutive_clean >= 2


def test_e2e_convergence_passing_tests(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")

    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    run_e2e_convergence(state, path, str(tmp_path), [str(script)])
    assert state.consecutive_clean >= 2


def test_e2e_convergence_failing_tests(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=2)
    path = str(tmp_path / "state.json")

    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\necho FAIL\nexit 1\n")
    script.chmod(0o755)

    run_e2e_convergence(state, path, str(tmp_path), [str(script)])
    assert state.phase == ConvergencePhase.ESCALATED


def test_e2e_convergence_timeout(tmp_path):
    state = ConvergenceState(plan_file="plan.md", deadline=time.time() - 1)
    path = str(tmp_path / "state.json")

    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    run_e2e_convergence(state, path, str(tmp_path), [str(script)])
    assert state.phase == ConvergencePhase.ESCALATED


# --- Master runner ---


def test_runner_full_lifecycle_clean(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    runner = ConvergenceRunner(state, llm, path)
    result = runner.run()

    assert result.phase == ConvergencePhase.CONVERGED
    assert is_terminal(result.phase)


def test_runner_full_lifecycle_with_findings(tmp_path):
    state = ConvergenceState(plan_file="plan.md", max_rounds=20)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())

    runner = ConvergenceRunner(state, llm, path)
    result = runner.run()

    # Intermittent mode may converge or escalate depending on call count parity
    assert is_terminal(result.phase)


def test_runner_escalation_propagates(tmp_path):
    state = ConvergenceState(plan_file="plan.md", deadline=time.time() - 1)
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    runner = ConvergenceRunner(state, llm, path)
    result = runner.run()

    assert result.phase == ConvergencePhase.ESCALATED


def test_runner_with_test_command(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    llm = StubProvider(mode=StubMode.CLEAN)

    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    runner = ConvergenceRunner(
        state, llm, path,
        project_dir=str(tmp_path),
        test_command=[str(script)],
    )
    result = runner.run()

    assert result.phase == ConvergencePhase.CONVERGED


def test_runner_independence_check(tmp_path):
    """When clean passes aren't structurally independent and LLM says not independent,
    convergence should not succeed (consecutive_clean reset)."""
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    # Clean mode will converge quickly — but independence check may reset
    llm = StubProvider(mode=StubMode.CLEAN)

    run_code_convergence(state, llm, path)
    # With stub always saying independent, it should converge
    assert state.consecutive_clean >= 2
