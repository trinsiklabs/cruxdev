"""Tests for MCP tool functions (the FastMCP @mcp.tool() wrappers)."""

import json
import os

import pytest

from src.mcp_server import (
    convergence_cancel,
    convergence_next_task,
    convergence_status,
    convergence_submit_result,
    create_plan_template,
    get_adoption_process,
    get_methodology,
    init,
    start_convergence,
    validate_plan_structure,
)


@pytest.fixture(autouse=True)
def setup(tmp_path):
    init(str(tmp_path / "state"))
    yield


# --- Bootstrap tools ---


def test_get_methodology():
    result = get_methodology()
    assert isinstance(result, str)
    assert len(result) > 100  # Should have real content


def test_get_adoption_process():
    result = get_adoption_process()
    assert isinstance(result, str)
    assert len(result) > 100


# --- Planning tools ---


def test_create_plan_template():
    result = create_plan_template("Build a REST API")
    assert "Build a REST API" in result
    assert "Phase 1" in result
    assert "- [ ]" in result


def test_validate_plan_structure_valid(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Build Plan for Feature X\n\n## Phase 1: Implementation\n\n- [ ] Write the code\n- [ ] Run pytest to verify\n\n## Document Alignment\n\n- docs/DESIGN.md — design spec\n\n## Convergence\n\nTwo consecutive clean passes required.\n")
    result = validate_plan_structure(str(plan))
    data = json.loads(result)
    assert data["valid"] is True


def test_validate_plan_structure_invalid(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("no heading no checklist")
    result = validate_plan_structure(str(plan))
    data = json.loads(result)
    assert data["valid"] is False


# --- Convergence tools ---


def test_start_convergence(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n**Status:** NOT STARTED\n- [ ] do stuff\n")
    result = json.loads(start_convergence(str(plan)))
    assert "convergence_id" in result
    assert result["status"] == "started"
    # Plan status should be updated to IN PROGRESS
    assert "IN PROGRESS" in plan.read_text()
    assert "task" in result


def test_convergence_escalated_updates_plan_status(tmp_path):
    from src.engine.persistence import load_state, save_state
    from src.mcp_server import state_path as get_state_path
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n**Status:** IN PROGRESS\n- [ ] task\n")
    start = json.loads(start_convergence(str(plan), max_rounds=1))
    cid = start["convergence_id"]

    sp = get_state_path(cid)
    state = load_state(sp)
    state.round = 10
    save_state(state, sp)

    result = json.loads(convergence_next_task(cid))
    assert "ESCALATED" in plan.read_text()


def test_submit_escalated_updates_plan_status(tmp_path):
    from src.engine.persistence import load_state, save_state
    from src.mcp_server import state_path as get_state_path
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n**Status:** IN PROGRESS\n- [ ] task\n")
    start = json.loads(start_convergence(str(plan), max_rounds=1))
    cid = start["convergence_id"]

    sp = get_state_path(cid)
    state = load_state(sp)
    state.round = 10
    save_state(state, sp)

    result = json.loads(convergence_submit_result(cid, "[]"))
    assert "ESCALATED" in plan.read_text()


def test_start_convergence_resumes_active_run(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n**Status:** NOT STARTED\n- [ ] task\n")
    start1 = json.loads(start_convergence(str(plan), project_dir=str(tmp_path)))
    cid1 = start1["convergence_id"]
    assert start1["status"] == "started"

    # Calling start again with same plan should RESUME, not create new
    start2 = json.loads(start_convergence(str(plan), project_dir=str(tmp_path)))
    assert start2["status"] == "resumed"
    assert start2["convergence_id"] == cid1


def test_start_convergence_with_files(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    result = json.loads(start_convergence(
        str(plan),
        source_files="src/a.py,src/b.py",
        doc_files="README.md",
        test_command="pytest tests/",
    ))
    assert "convergence_id" in result


def test_convergence_next_task(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_next_task(cid))
    assert "task" in result
    assert result["convergence_id"] == cid


def test_convergence_submit_result(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    # Submit through planning phase
    result = json.loads(convergence_submit_result(cid, "[]"))
    assert result["status"] == "result_accepted"
    assert result["round"] == 1


def test_convergence_submit_with_findings(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    findings = json.dumps([{
        "id": "f1", "file": "a.py", "dimension": "correctness",
        "severity": "high", "description": "bug", "suggested_fix": "fix",
    }])
    result = json.loads(convergence_submit_result(cid, findings))
    assert result["consecutive_clean"] == 0


def test_convergence_submit_invalid_json(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    # Invalid JSON must be REJECTED — fail closed, not silent clean pass
    result = json.loads(convergence_submit_result(cid, "not json"))
    assert result["status"] == "rejected"
    assert "Invalid JSON" in result["error"]


def test_convergence_submit_missing_fields(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    # Missing required fields must be rejected
    result = json.loads(convergence_submit_result(cid, '[{"id": "f1"}]'))
    assert result["status"] == "rejected"
    assert "missing required fields" in result["error"]


def test_convergence_submit_not_array(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_submit_result(cid, '{"not": "array"}'))
    assert result["status"] == "rejected"


def test_convergence_submit_returns_next_task(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n- [ ] task\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_submit_result(cid, "[]"))
    assert result["status"] == "result_accepted"
    assert "next_task" in result
    assert "continue" in result


def test_convergence_submit_empty_string(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n- [ ] task\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_submit_result(cid, ""))
    assert result["status"] == "result_accepted"
    assert result["findings_count"] == 0


def test_convergence_submit_non_dict_skipped(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n- [ ] task\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    findings = json.dumps(["not a dict", {
        "id": "f1", "file": "a.py", "dimension": "correctness",
        "severity": "high", "description": "bug", "suggested_fix": "fix",
    }])
    result = json.loads(convergence_submit_result(cid, findings))
    assert result["status"] == "result_accepted"
    assert result["findings_count"] == 1


def test_convergence_status(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_status(cid))
    assert result["convergence_id"] == cid
    assert "phase" in result
    assert "terminal" in result


def test_convergence_cancel(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_cancel(cid))
    assert result["status"] == "cancelled"

    status = json.loads(convergence_status(cid))
    assert status["phase"] == "escalated"


def test_full_convergence_loop(tmp_path):
    """Simulate a full convergence loop: start → submit clean passes → done."""
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n- [x] 1.1 Already done\npytest\nconvergence\n")

    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    # Drive through all phases — each needs 2 clean passes
    for _ in range(30):
        task = json.loads(convergence_next_task(cid))
        if task["task"]["task_type"] in ("done", "escalated"):
            break
        convergence_submit_result(cid, "[]")

    status = json.loads(convergence_status(cid))
    assert status["terminal"] is True


def test_get_methodology_missing_file():
    from unittest.mock import patch
    with patch("src.mcp_server.CRUXDEV_ROOT", "/nonexistent"):
        result = get_methodology()
        assert "not found" in result


def test_get_adoption_process_missing_file():
    from unittest.mock import patch
    with patch("src.mcp_server.CRUXDEV_ROOT", "/nonexistent"):
        result = get_adoption_process()
        assert "not found" in result


def test_list_convergences_empty_state_dir():
    from src.mcp_server import list_convergences
    from unittest.mock import patch
    with patch("src.mcp_server.STATE_DIR", "/nonexistent"):
        result = list_convergences()
        assert result == []


def test_install_cruxdev_tool(tmp_path):
    from src.mcp_server import install_cruxdev
    result = json.loads(install_cruxdev(str(tmp_path)))
    assert result["status"] == "installed"
    assert os.path.exists(os.path.join(str(tmp_path), ".mcp.json"))


def test_convergence_next_task_with_files(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_next_task(
        cid, source_files="a.py,b.py", doc_files="README.md", test_command="pytest",
    ))
    assert "task" in result
