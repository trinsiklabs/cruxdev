"""Plan convergence — focused audit, full-plan audit, viability.

All loop control is code. LLM is called only for audit/fix tasks.
"""

from ..dispatch.llm import LLMDispatcher
from ..dispatch.schema import AuditResult
from .convergence import (
    check_convergence,
    check_max_rounds,
    check_timeout,
    escalate,
    record_round,
)
from .persistence import save_state
from .state import ConvergencePhase, ConvergenceState, Finding, FindingSeverity

PLAN_DIMENSIONS = [
    "completeness",
    "feasibility",
    "risk_assessment",
    "dependency_ordering",
    "testability",
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


def run_plan_convergence(
    state: ConvergenceState,
    llm: LLMDispatcher,
    state_path: str,
) -> None:
    """Run plan convergence loop.

    Audits the plan file, fixes issues, repeats until
    two consecutive clean passes or escalation.
    """
    state.phase = ConvergencePhase.PLAN_AUDITING
    state.round = 0
    state.consecutive_clean = 0

    while not check_convergence(state):
        if check_timeout(state):
            escalate(state, "plan_convergence_timeout")
            save_state(state, state_path)
            return

        if check_max_rounds(state):
            escalate(state, "plan_convergence_max_rounds")
            save_state(state, state_path)
            return

        result = llm.audit(
            files=[state.plan_file],
            dimensions=PLAN_DIMENSIONS,
            skill_context="",
        )

        findings = _audit_to_findings(result)

        for f in findings:
            fix_result = llm.fix(
                finding_id=f.id,
                finding_description=f.description,
                file_path=f.file,
                file_content="",
                skill_context="",
            )
            if fix_result.success:
                f.fixed = True

        record_round(state, findings)
        save_state(state, state_path)
