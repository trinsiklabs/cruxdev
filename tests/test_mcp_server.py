"""Tests for MCP server interface."""

import time

import pytest

from src import mcp_server


@pytest.fixture(autouse=True)
def setup_state_dir(tmp_path):
    mcp_server.init(str(tmp_path / "convergence_state"))
    yield
    mcp_server._active_runs.clear()


def test_converge_returns_immediately():
    result = mcp_server.converge(plan_file="plan.md", timeout_minutes=1)
    assert "convergence_id" in result
    assert result["status"] == "started"
    assert len(result["convergence_id"]) == 8


def test_converge_creates_state_file():
    result = mcp_server.converge(plan_file="plan.md")
    cid = result["convergence_id"]
    import os
    assert os.path.exists(mcp_server.state_path(cid))


def test_check_status_after_converge():
    result = mcp_server.converge(plan_file="plan.md", timeout_minutes=1)
    cid = result["convergence_id"]

    # Wait for background thread to complete (stub is fast)
    time.sleep(0.5)

    status = mcp_server.check_convergence_status(cid)
    assert status["convergence_id"] == cid
    assert status["phase"] in ["converged", "escalated"]
    assert status["running"] is False


def test_cancel_convergence():
    result = mcp_server.converge(plan_file="plan.md", timeout_minutes=60)
    cid = result["convergence_id"]

    cancel = mcp_server.cancel_convergence(cid)
    assert cancel["status"] == "cancelled"

    status = mcp_server.check_convergence_status(cid)
    assert status["phase"] == "escalated"
    assert status["escalation_reason"] == "cancelled_by_user"


def test_concurrent_convergences():
    r1 = mcp_server.converge(plan_file="plan1.md", timeout_minutes=1)
    r2 = mcp_server.converge(plan_file="plan2.md", timeout_minutes=1)

    assert r1["convergence_id"] != r2["convergence_id"]

    time.sleep(0.5)

    s1 = mcp_server.check_convergence_status(r1["convergence_id"])
    s2 = mcp_server.check_convergence_status(r2["convergence_id"])

    assert s1["phase"] in ["converged", "escalated"]
    assert s2["phase"] in ["converged", "escalated"]


def test_list_convergences():
    mcp_server.converge(plan_file="plan1.md")
    mcp_server.converge(plan_file="plan2.md")
    time.sleep(0.5)

    listing = mcp_server.list_convergences()
    assert len(listing) >= 2


def test_list_convergences_empty(tmp_path):
    mcp_server.init(str(tmp_path / "empty_state"))
    listing = mcp_server.list_convergences()
    assert listing == []


def test_get_provider_stub():
    p = mcp_server.get_provider("stub")
    assert p is not None


def test_get_provider_anthropic():
    p = mcp_server.get_provider("anthropic", api_key="sk-test")
    from src.dispatch.providers.anthropic import AnthropicProvider
    assert isinstance(p, AnthropicProvider)


def test_get_provider_ollama():
    p = mcp_server.get_provider("ollama")
    from src.dispatch.providers.ollama import OllamaProvider
    assert isinstance(p, OllamaProvider)


def test_get_provider_unknown():
    with pytest.raises(ValueError, match="Unknown provider"):
        mcp_server.get_provider("nonexistent")


def test_converge_with_test_command(tmp_path):
    script = tmp_path / "test.sh"
    script.write_text("#!/bin/bash\nexit 0\n")
    script.chmod(0o755)

    result = mcp_server.converge(
        plan_file="plan.md",
        test_command=[str(script)],
        project_dir=str(tmp_path),
    )
    cid = result["convergence_id"]
    time.sleep(0.5)

    status = mcp_server.check_convergence_status(cid)
    assert status["phase"] in ["converged", "escalated"]


def test_list_convergences_with_corrupt_file(tmp_path):
    """Corrupt state files should be silently skipped."""
    mcp_server.init(str(tmp_path / "state"))
    state_dir = str(tmp_path / "state")
    import os
    os.makedirs(state_dir, exist_ok=True)
    with open(os.path.join(state_dir, "corrupt.json"), "w") as f:
        f.write("not valid json")
    listing = mcp_server.list_convergences()
    assert listing == []


def test_list_convergences_no_state_dir():
    """When state_dir doesn't exist, return empty list."""
    mcp_server._state_dir = "/nonexistent/path"
    listing = mcp_server.list_convergences()
    assert listing == []


def test_check_status_elapsed_time():
    result = mcp_server.converge(plan_file="plan.md")
    cid = result["convergence_id"]
    time.sleep(0.1)
    status = mcp_server.check_convergence_status(cid)
    assert status["elapsed_seconds"] >= 0.1
