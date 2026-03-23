"""Tests for checklist parser."""

import pytest

from src.engine.checklist_parser import (
    ChecklistItem,
    all_complete,
    completion_summary,
    get_next_incomplete,
    mark_complete,
    parse_checklist,
)


SAMPLE_PLAN = """# BUILD_PLAN: Test

## Phase 1: Setup

- [ ] 1.1 Create project structure
- [ ] 1.2 Set up database
- [x] 1.3 Configure CI

## Phase 2: Build

- [ ] 2.1 Build the API
- [ ] 2.2 Build the frontend

## Document Alignment
- docs/DESIGN.md
"""


def test_parse_checklist(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text(SAMPLE_PLAN)

    items = parse_checklist(str(plan))
    assert len(items) == 5
    assert items[0].id == "1.1"
    assert items[0].phase == "Phase 1"
    assert items[0].description == "Create project structure"
    assert items[0].completed is False
    assert items[2].id == "1.3"
    assert items[2].completed is True
    assert items[3].id == "2.1"
    assert items[3].phase == "Phase 2"


def test_parse_checklist_missing_file():
    items = parse_checklist("/nonexistent.md")
    assert items == []


def test_parse_checklist_no_items(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\nNo checklist here.")
    items = parse_checklist(str(plan))
    assert items == []


def test_get_next_incomplete():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=True),
        ChecklistItem(id="1.2", phase="P1", description="d", completed=False),
        ChecklistItem(id="1.3", phase="P1", description="d", completed=False),
    ]
    next_item = get_next_incomplete(items)
    assert next_item is not None
    assert next_item.id == "1.2"


def test_get_next_incomplete_all_done():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=True),
    ]
    assert get_next_incomplete(items) is None


def test_get_next_incomplete_empty():
    assert get_next_incomplete([]) is None


def test_mark_complete():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=False),
        ChecklistItem(id="1.2", phase="P1", description="d", completed=False),
    ]
    assert mark_complete(items, "1.1") is True
    assert items[0].completed is True
    assert items[1].completed is False


def test_mark_complete_not_found():
    items = [ChecklistItem(id="1.1", phase="P1", description="d")]
    assert mark_complete(items, "9.9") is False


def test_all_complete_true():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=True),
        ChecklistItem(id="1.2", phase="P1", description="d", completed=True),
    ]
    assert all_complete(items) is True


def test_all_complete_false():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=True),
        ChecklistItem(id="1.2", phase="P1", description="d", completed=False),
    ]
    assert all_complete(items) is False


def test_all_complete_empty():
    assert all_complete([]) is True


def test_completion_summary():
    items = [
        ChecklistItem(id="1.1", phase="P1", description="d", completed=True),
        ChecklistItem(id="1.2", phase="P1", description="d", completed=False),
        ChecklistItem(id="1.3", phase="P1", description="d", completed=True),
    ]
    s = completion_summary(items)
    assert s["total"] == 3
    assert s["completed"] == 2
    assert s["remaining"] == 1
    assert s["percentage"] == 66.7


def test_completion_summary_empty():
    s = completion_summary([])
    assert s["total"] == 0
    assert s["percentage"] == 100.0


def test_execution_phase_returns_execute_task(tmp_path):
    """Integration test: task router returns execute tasks for green-field."""
    from src.engine.persistence import save_state
    from src.engine.state import ConvergencePhase, ConvergenceState
    from src.engine.task_router import get_next_task

    plan = tmp_path / "plan.md"
    plan.write_text(SAMPLE_PLAN)

    state = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert task.task_type == "execute"
    assert "1.1" in task.description
    assert task.metadata["checklist_item"] == "1.1"
    assert task.metadata["progress"]["total"] == 5


def test_execution_phase_all_complete_advances(tmp_path):
    """When all checklist items are complete, advance to code_audit."""
    from src.engine.persistence import save_state
    from src.engine.state import ConvergencePhase, ConvergenceState
    from src.engine.task_router import get_next_task

    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [x] 1.1 Done\n- [x] 1.2 Done\n## Document Alignment\n- d.md\n")

    state = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    # All items complete → should advance to code_auditing
    assert state.phase == ConvergencePhase.CODE_AUDITING


def test_execution_phase_no_checklist_advances(tmp_path):
    """When plan has no checklist items, advance to code_audit."""
    from src.engine.persistence import save_state
    from src.engine.state import ConvergencePhase, ConvergenceState
    from src.engine.task_router import get_next_task

    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\nNo checklist.\n## Document Alignment\n- d.md\n")

    state = ConvergenceState(
        plan_file=str(plan),
        phase=ConvergencePhase.EXECUTING,
    )
    path = str(tmp_path / "state.json")
    save_state(state, path)

    task = get_next_task(state, path)
    assert state.phase == ConvergencePhase.CODE_AUDITING
