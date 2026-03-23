"""Tests for task router — state machine driving Claude Code."""

import time

import pytest

from src.engine.persistence import save_state
from src.engine.state import ConvergencePhase, ConvergenceState
from src.engine.task_router import get_next_task, submit_result, _extract_alignment_docs


def test_planning_phase_returns_write_task(tmp_path):
    state = ConvergenceState(plan_file="plan.md")
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "write"
    assert "plan" in task.description.lower()


def test_plan_auditing_returns_audit_task(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.PLAN_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "audit"
    assert "plan" in task.description.lower()


def test_plan_auditing_converges_to_doc_alignment(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.PLAN_AUDITING,
        consecutive_clean=2,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    # Plan auditing now converges to doc_alignment (new gate)
    assert state.phase == ConvergencePhase.DOC_ALIGNMENT


def test_code_auditing_returns_audit_task(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CODE_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path, source_files=["src/main.py"])
    assert task.task_type == "audit"
    assert "code" in task.description.lower()
    assert "src/main.py" in task.files


def test_code_auditing_converges_to_doc(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.CODE_AUDITING,
        consecutive_clean=2,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert state.phase == ConvergencePhase.DOC_AUDITING


def test_doc_auditing_returns_audit_task(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.DOC_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path, doc_files=["README.md"])
    assert task.task_type == "audit"
    assert "doc" in task.description.lower()


def test_doc_auditing_converges_to_e2e(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.DOC_AUDITING,
        consecutive_clean=2,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert state.phase == ConvergencePhase.E2E_TESTING


def test_e2e_testing_returns_test_task(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.E2E_TESTING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path, test_command=["pytest"])
    assert task.task_type == "test"
    assert task.test_command == ["pytest"]


def test_e2e_converges_to_patterns_then_done(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.E2E_TESTING,
        consecutive_clean=2,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert state.phase == ConvergencePhase.CONVERGED
    assert task.task_type == "done"


def test_converged_returns_done(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CONVERGED)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "done"


def test_escalated_returns_escalated(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.ESCALATED,
        escalation_reason="timeout",
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "escalated"
    assert "timeout" in task.description


def test_timeout_escalates(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.CODE_AUDITING,
        deadline=time.time() - 1,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "escalated"
    assert "timeout" in task.description


def test_max_rounds_escalates(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.CODE_AUDITING,
        round=5,
        max_rounds=5,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "escalated"


def test_net_negative_escalates(tmp_path):
    from src.engine.state import Finding, FindingSeverity, RoundResult

    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.CODE_AUDITING,
    )
    # Create increasing findings pattern
    for i, count in enumerate([3, 5, 8]):
        findings = [
            Finding(id=f"f{j}", file="a.py", dimension="d",
                    severity=FindingSeverity.LOW, description=f"issue {j}",
                    suggested_fix="fix")
            for j in range(count)
        ]
        state.history.append(RoundResult(
            round=i, phase=ConvergencePhase.CODE_AUDITING,
            findings=findings, findings_fixed=0, timestamp=time.time(),
        ))
    state.round = 3

    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "escalated"
    assert "net_negative" in task.description


# --- submit_result ---


def test_submit_clean_result(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CODE_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    submit_result(state, path, {"findings": []})
    assert state.round == 1
    assert state.consecutive_clean == 1


def test_submit_findings_result(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CODE_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    submit_result(state, path, {
        "findings": [{
            "id": "f1", "file": "a.py", "dimension": "correctness",
            "severity": "high", "description": "bug", "suggested_fix": "fix",
            "fixed": False,
        }]
    })
    assert state.round == 1
    assert state.consecutive_clean == 0
    assert state.failures.get("f1", 0) == 1


def test_submit_fixed_findings(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CODE_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    submit_result(state, path, {
        "findings": [{
            "id": "f1", "file": "a.py", "dimension": "d",
            "severity": "low", "description": "issue", "suggested_fix": "fix",
            "fixed": True,
        }]
    })
    assert "f1" not in state.failures


def test_submit_execution_checklist(tmp_path):
    """Submitting result with checklist_item during execution phase marks it complete."""
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [ ] 1.1 Do thing\n## Document Alignment\n- d.md\n")

    state = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    submit_result(state, path, {
        "findings": [],
        "checklist_item": "1.1",
    })
    assert state.round == 1


def test_task_to_dict():
    from src.engine.task_router import Task
    t = Task(
        task_type="audit", description="audit code",
        files=["a.py"], dimensions=["correctness"],
        finding={"id": "f1"}, test_command=["pytest"],
        metadata={"round": 1},
    )
    d = t.to_dict()
    assert d["task_type"] == "audit"
    assert d["finding"] == {"id": "f1"}
    assert d["test_command"] == ["pytest"]
    assert d["metadata"] == {"round": 1}


def test_plan_audit_recommends_fast_tier(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.PLAN_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)
    task = get_next_task(state, path)
    assert task.recommended_tier == "fast"


def test_code_audit_recommends_standard_tier(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.CODE_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)
    task = get_next_task(state, path)
    assert task.recommended_tier == "standard"


def test_doc_audit_recommends_fast_tier(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.DOC_AUDITING)
    path = str(tmp_path / "state.json")
    save_state(state, path)
    task = get_next_task(state, path)
    assert task.recommended_tier == "fast"


def test_e2e_test_recommends_fast_tier(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.E2E_TESTING)
    path = str(tmp_path / "state.json")
    save_state(state, path)
    task = get_next_task(state, path)
    assert task.recommended_tier == "fast"


def test_planning_recommends_standard_tier(tmp_path):
    state = ConvergenceState(plan_file="plan.md", phase=ConvergencePhase.PLANNING)
    path = str(tmp_path / "state.json")
    save_state(state, path)
    task = get_next_task(state, path)
    assert task.recommended_tier == "standard"


def test_task_to_dict_with_tier():
    from src.engine.task_router import Task
    t = Task(task_type="audit", description="d", files=[], dimensions=[], recommended_tier="fast")
    d = t.to_dict()
    assert d["recommended_tier"] == "fast"


# --- Doc alignment ---


def test_doc_alignment_phase(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Document Alignment\n- docs/design.md — spec\n")
    state = ConvergenceState(plan_file=str(plan), phase=ConvergencePhase.DOC_ALIGNMENT)
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "doc_align"
    assert "alignment" in task.description.lower()
    assert task.recommended_tier == "standard"


def test_doc_alignment_converges_to_viability(tmp_path):
    state = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.DOC_ALIGNMENT,
        consecutive_clean=2,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    # Should advance past doc_alignment → viability → executing → code_auditing
    assert state.phase in (
        ConvergencePhase.VIABILITY,
        ConvergencePhase.EXECUTING,
        ConvergencePhase.CODE_AUDITING,
    )


# --- Alignment doc extraction ---


def test_extract_alignment_docs_list(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("""# Plan
## Document Alignment
- docs/DESIGN.md — design spec
- docs/PRICING.md — pricing rules
- memory/badges.md — badge requirements
## Next Section
""")
    docs = _extract_alignment_docs(str(plan))
    assert "docs/DESIGN.md" in docs
    assert "docs/PRICING.md" in docs
    assert "memory/badges.md" in docs


def test_extract_alignment_docs_table(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("""# Plan
## Document Alignment

| Doc | Purpose | Path |
|-----|---------|------|
| Design | spec | `docs/DESIGN.md` |
| Pricing | tiers | `docs/PRICING.md` |
""")
    docs = _extract_alignment_docs(str(plan))
    assert "docs/DESIGN.md" in docs
    assert "docs/PRICING.md" in docs


def test_extract_alignment_docs_missing_section(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [ ] do stuff\n")
    docs = _extract_alignment_docs(str(plan))
    assert docs == []


def test_extract_alignment_docs_missing_file():
    docs = _extract_alignment_docs("/nonexistent/plan.md")
    assert docs == []


def test_extract_alignment_docs_dedupes(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("""# Plan
## Document Alignment
- docs/DESIGN.md — first reference
- docs/DESIGN.md — duplicate reference
""")
    docs = _extract_alignment_docs(str(plan))
    assert docs.count("docs/DESIGN.md") == 1


def test_task_to_dict_minimal():
    from src.engine.task_router import Task
    t = Task(task_type="done", description="done", files=[], dimensions=[])
    d = t.to_dict()
    assert "finding" not in d
    assert "test_command" not in d
    assert "metadata" not in d
