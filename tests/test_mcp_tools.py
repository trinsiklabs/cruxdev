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
    plan.write_text("# Build Plan for Feature X\n\n## Phase 1: Implementation\n\n- [ ] Write the code\n- [ ] Run pytest to verify\n\n## Convergence\n\nTwo consecutive clean passes required.\n")
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
    plan.write_text("# Plan\n- [ ] do stuff\n")
    result = json.loads(start_convergence(str(plan)))
    assert "convergence_id" in result
    assert result["status"] == "started"
    assert "task" in result


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

    # Invalid JSON should be handled gracefully
    result = json.loads(convergence_submit_result(cid, "not json"))
    assert result["status"] == "result_accepted"


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
    plan.write_text("# Plan\n- [ ] item\npytest\nconvergence\n")

    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    # Drive through planning phase
    for _ in range(10):
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


def test_convergence_next_task_with_files(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n")
    start = json.loads(start_convergence(str(plan)))
    cid = start["convergence_id"]

    result = json.loads(convergence_next_task(
        cid, source_files="a.py,b.py", doc_files="README.md", test_command="pytest",
    ))
    assert "task" in result
