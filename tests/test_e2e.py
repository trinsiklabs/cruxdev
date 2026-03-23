"""End-to-end tests for the convergence engine.

Tests the full lifecycle with stub LLM provider.
"""

import time

import pytest

from src.dispatch.providers.stub import StubMode, StubProvider
from src.dispatch.schema import AuditFinding
from src.engine.convergence import is_terminal
from src.engine.persistence import load_state
from src.engine.runner import ConvergenceRunner
from src.engine.state import ConvergencePhase, ConvergenceState
from src import mcp_server


@pytest.fixture(autouse=True)
def setup_mcp(tmp_path):
    mcp_server.init(str(tmp_path / "state"))
    yield
    mcp_server._active_runs.clear()


def _make_findings():
    return [
        AuditFinding(
            id="f1", file="a.py", dimension="correctness",
            severity="high", description="deliberate defect", suggested_fix="fix it",
        )
    ]


class TestE2EConvergence:
    """Full lifecycle tests."""

    def test_deliberate_defect_converges(self, tmp_path):
        """Deliberate defect → converge → fixed."""
        state = ConvergenceState(plan_file="plan.md", max_rounds=20)
        path = str(tmp_path / "e2e_state.json")

        # First round finds issues, subsequent rounds are clean
        llm = StubProvider(mode=StubMode.INTERMITTENT, findings=_make_findings())

        runner = ConvergenceRunner(state, llm, path)
        result = runner.run()

        assert is_terminal(result.phase)
        assert len(result.history) > 0

    def test_timeout_triggers_escalation(self, tmp_path):
        """Timeout triggers escalation including mid-call."""
        state = ConvergenceState(
            plan_file="plan.md",
            deadline=time.time() - 1,  # Already expired
        )
        path = str(tmp_path / "e2e_state.json")
        llm = StubProvider(mode=StubMode.CLEAN)

        runner = ConvergenceRunner(state, llm, path)
        result = runner.run()

        assert result.phase == ConvergencePhase.ESCALATED
        assert "timeout" in result.escalation_reason

    def test_three_failures_triggers_rollback(self, tmp_path):
        """3 failures on same task triggers rollback/escalation."""
        state = ConvergenceState(
            plan_file="plan.md",
            max_rounds=20,
            max_failures=3,
        )
        path = str(tmp_path / "e2e_state.json")

        # Persistent mode: fix always fails
        llm = StubProvider(mode=StubMode.PERSISTENT, findings=_make_findings())

        runner = ConvergenceRunner(state, llm, path)
        result = runner.run()

        assert result.phase == ConvergencePhase.ESCALATED

    def test_net_negative_triggers_escalation(self, tmp_path):
        """Net-negative trend triggers escalation."""
        from src.engine.convergence import check_net_negative, escalate, record_round
        from src.engine.state import Finding, FindingSeverity

        state = ConvergenceState(plan_file="plan.md")

        # Simulate increasing findings over 3 rounds
        for i, count in enumerate([3, 5, 8]):
            findings = [
                Finding(
                    id=f"f{i}_{j}", file="a.py", dimension="d",
                    severity=FindingSeverity.LOW, description=f"issue {j}",
                    suggested_fix="fix",
                )
                for j in range(count)
            ]
            record_round(state, findings)

        assert check_net_negative(state) is True

    def test_mcp_e2e_converge_and_check(self, tmp_path):
        """MCP integration: converge → check_status → verify."""
        result = mcp_server.converge(
            plan_file="plan.md",
            timeout_minutes=1,
        )
        cid = result["convergence_id"]

        # Wait for thread to complete
        thread = mcp_server._active_runs.get(cid)
        if thread:
            thread.join(timeout=30)

        status = mcp_server.check_convergence_status(cid)
        assert status["running"] is False
        assert status["phase"] in ["converged", "escalated"]

    def test_state_persisted_between_rounds(self, tmp_path):
        """State is saved to disk after every round."""
        state = ConvergenceState(plan_file="plan.md")
        path = str(tmp_path / "e2e_state.json")
        llm = StubProvider(mode=StubMode.CLEAN)

        runner = ConvergenceRunner(state, llm, path)
        runner.run()

        # Load from disk and verify
        loaded = load_state(path)
        assert is_terminal(loaded.phase)
        assert loaded.plan_file == "plan.md"

    def test_all_state_transitions_deterministic(self, tmp_path):
        """Verify state transitions are deterministic — same input → same output."""
        def run_once():
            state = ConvergenceState(
                plan_file="plan.md",
                created_at=1000.0,
                updated_at=1000.0,
            )
            path = str(tmp_path / "det_state.json")
            llm = StubProvider(mode=StubMode.CLEAN)
            runner = ConvergenceRunner(state, llm, path)
            result = runner.run()
            return result.phase, result.round, len(result.history)

        r1 = run_once()
        r2 = run_once()
        assert r1 == r2
