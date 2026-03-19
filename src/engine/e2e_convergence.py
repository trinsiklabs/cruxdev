"""E2E test convergence — run tests, verify coverage, iterate.

The engine runs tests via subprocess (code, not LLM).
"""

from .convergence import (
    check_convergence,
    check_max_rounds,
    check_timeout,
    escalate,
    record_round,
    run_tests,
)
from .persistence import save_state
from .state import ConvergencePhase, ConvergenceState


def run_e2e_convergence(
    state: ConvergenceState,
    state_path: str,
    project_dir: str = ".",
    test_command: list[str] | None = None,
) -> None:
    """Run E2E test convergence loop.

    Runs tests repeatedly until they pass for two consecutive rounds
    or escalates on failure.
    """
    state.phase = ConvergencePhase.E2E_TESTING
    state.round = 0
    state.consecutive_clean = 0

    if test_command is None:
        # No test command configured — skip E2E
        record_round(state, [])
        record_round(state, [])
        save_state(state, state_path)
        return

    while not check_convergence(state):
        if check_timeout(state):
            escalate(state, "e2e_convergence_timeout")
            save_state(state, state_path)
            return

        if check_max_rounds(state):
            escalate(state, "e2e_convergence_max_rounds")
            save_state(state, state_path)
            return

        test_result = run_tests(project_dir, test_command)

        if test_result.passed:
            record_round(state, [])
        else:
            from .state import Finding, FindingSeverity

            finding = Finding(
                id=f"e2e_fail_r{state.round}",
                file="test_suite",
                dimension="e2e",
                severity=FindingSeverity.HIGH,
                description=f"Tests failed: {test_result.output[:200]}",
                suggested_fix="Fix failing tests",
            )
            record_round(state, [finding])

        save_state(state, state_path)
