"""Code convergence — 8-dimension audit with independence checking.

All loop control is code. LLM is called only for audit/fix/evaluate tasks.
After every fix, code runs tests via subprocess to verify.
"""

from ..dispatch.llm import LLMDispatcher
from ..dispatch.schema import AuditResult
from .convergence import (
    check_convergence,
    check_max_rounds,
    check_structural_independence,
    check_timeout,
    escalate,
    record_failure,
    record_round,
    run_tests,
    should_rollback,
)
from .persistence import save_state
from .state import ConvergencePhase, ConvergenceState, Finding, FindingSeverity

CODE_DIMENSIONS = [
    "correctness",
    "completeness",
    "edge_cases",
    "error_handling",
    "security",
    "performance",
    "maintainability",
    "test_coverage",
]


def _audit_to_findings(result: AuditResult) -> list[Finding]:
    """Convert schema AuditFinding to engine Finding."""
    return [
        Finding(
            id=f.id,
            file=f.file,
            dimension=f.dimension,
            severity=FindingSeverity(f.severity),
            description=f.description,
            suggested_fix=f.suggested_fix,
        )
        for f in result.findings
    ]


def run_code_convergence(
    state: ConvergenceState,
    llm: LLMDispatcher,
    state_path: str,
    project_dir: str = ".",
    test_command: list[str] | None = None,
    files: list[str] | None = None,
) -> None:
    """Run code convergence loop.

    Audits code files, fixes issues, verifies with tests,
    checks independence of clean passes, repeats until convergence.
    """
    state.phase = ConvergencePhase.CODE_AUDITING
    state.round = 0
    state.consecutive_clean = 0
    audit_files = files or [state.plan_file]

    while not check_convergence(state):
        if check_timeout(state):
            escalate(state, "code_convergence_timeout")
            save_state(state, state_path)
            return

        if check_max_rounds(state):
            escalate(state, "code_convergence_max_rounds")
            save_state(state, state_path)
            return

        result = llm.audit(
            files=audit_files,
            dimensions=CODE_DIMENSIONS,
            skill_context="",
        )

        findings = _audit_to_findings(result)

        for f in findings:
            if should_rollback(state, f.id):
                escalate(state, f"rollback_triggered:{f.id}")
                save_state(state, state_path)
                return

            fix_result = llm.fix(
                finding_id=f.id,
                finding_description=f.description,
                file_path=f.file,
                file_content="",
                skill_context="",
            )

            if fix_result.success and test_command:
                test_result = run_tests(project_dir, test_command)
                if not test_result.passed:
                    f.fixed = False
                    record_failure(state, f.id)
                else:
                    f.fixed = True
            elif fix_result.success:
                f.fixed = True
            else:
                record_failure(state, f.id)

        record_round(state, findings)
        save_state(state, state_path)

    # Independence check on final two clean passes
    verify_independence(state, llm, audit_files)


def verify_independence(
    state: ConvergenceState,
    llm: LLMDispatcher,
    audit_files: list[str],
) -> None:
    """Check if the last two clean passes were independent.
    Code check first, LLM as secondary."""
    if len(state.history) < 2:
        return
    last_two = state.history[-2:]
    if check_structural_independence(last_two[0], last_two[1]):
        return
    # Not structurally independent — ask LLM as secondary check
    pass_a_result = AuditResult(
        findings=[], files_audited=audit_files, dimensions_checked=CODE_DIMENSIONS
    )
    pass_b_result = AuditResult(
        findings=[], files_audited=audit_files, dimensions_checked=CODE_DIMENSIONS
    )
    eval_result = llm.evaluate_independence(pass_a_result, pass_b_result)
    if not eval_result.independent:
        state.consecutive_clean = 0
