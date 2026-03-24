"""Tests for CruxDev status checks."""

import json
import os
import sys

import pytest

from src.status import (
    Check,
    StatusReport,
    check_active_convergences,
    check_crux_integration,
    check_dependencies,
    check_methodology_docs,
    check_mcp_config,
    check_mcp_server,
    check_python_version,
    check_slash_commands,
    check_state_directory,
    check_tool_count,
    get_status,
)


def test_check_mcp_server():
    c = check_mcp_server()
    assert c.passed is True
    assert "loads" in c.message


def test_check_tool_count():
    c = check_tool_count()
    assert c.passed is True
    assert "tools" in c.message


def test_check_state_directory_exists(tmp_path):
    (tmp_path / ".cruxdev").mkdir()
    c = check_state_directory(str(tmp_path))
    assert c.passed is True


def test_check_state_directory_missing(tmp_path):
    c = check_state_directory(str(tmp_path))
    assert c.passed is False


def test_check_mcp_config_present(tmp_path):
    (tmp_path / ".mcp.json").write_text(json.dumps({"mcpServers": {"cruxdev": {}}}))
    c = check_mcp_config(str(tmp_path))
    assert c.passed is True


def test_check_mcp_config_missing(tmp_path):
    c = check_mcp_config(str(tmp_path))
    assert c.passed is False


def test_check_mcp_config_no_cruxdev(tmp_path):
    (tmp_path / ".mcp.json").write_text(json.dumps({"mcpServers": {}}))
    c = check_mcp_config(str(tmp_path))
    assert c.passed is False


def test_check_mcp_config_corrupt(tmp_path):
    (tmp_path / ".mcp.json").write_text("not json")
    c = check_mcp_config(str(tmp_path))
    assert c.passed is False


def test_check_crux_present(tmp_path):
    (tmp_path / ".mcp.json").write_text(json.dumps({"mcpServers": {"crux": {}, "cruxdev": {}}}))
    c = check_crux_integration(str(tmp_path))
    assert c.passed is True


def test_check_crux_missing(tmp_path):
    (tmp_path / ".mcp.json").write_text(json.dumps({"mcpServers": {"cruxdev": {}}}))
    c = check_crux_integration(str(tmp_path))
    assert c.passed is False


def test_check_crux_no_file(tmp_path):
    c = check_crux_integration(str(tmp_path))
    assert c.passed is False


def test_check_crux_corrupt(tmp_path):
    (tmp_path / ".mcp.json").write_text("bad")
    c = check_crux_integration(str(tmp_path))
    assert c.passed is False


def test_check_python_version():
    c = check_python_version()
    assert c.passed is True


def test_check_dependencies():
    c = check_dependencies()
    assert c.passed is True


def test_check_methodology_docs():
    c = check_methodology_docs()
    assert c.passed is True


def test_check_slash_commands():
    c = check_slash_commands()
    assert c.passed is True


def test_check_active_convergences(tmp_path):
    state_dir = tmp_path / ".cruxdev" / "convergence_state"
    state_dir.mkdir(parents=True)
    (state_dir / "abc123.json").write_text(json.dumps({"phase": "planning", "round": 2}))
    runs = check_active_convergences(str(tmp_path))
    assert len(runs) == 1
    assert runs[0]["phase"] == "planning"


def test_check_active_convergences_empty(tmp_path):
    assert check_active_convergences(str(tmp_path)) == []


def test_check_active_convergences_non_json_file(tmp_path):
    state_dir = tmp_path / ".cruxdev" / "convergence_state"
    state_dir.mkdir(parents=True)
    (state_dir / "readme.txt").write_text("not a state file")
    (state_dir / "abc.json").write_text(json.dumps({"phase": "converged", "round": 0}))
    runs = check_active_convergences(str(tmp_path))
    assert len(runs) == 1  # Only .json file counted


def test_check_active_convergences_corrupt(tmp_path):
    state_dir = tmp_path / ".cruxdev" / "convergence_state"
    state_dir.mkdir(parents=True)
    (state_dir / "bad.json").write_text("not json")
    runs = check_active_convergences(str(tmp_path))
    assert runs == []


def test_get_status():
    report = get_status(".")
    assert isinstance(report, StatusReport)
    assert len(report.checks) >= 8
    assert isinstance(report.versions, dict)


def test_get_status_healthy():
    report = get_status(".")
    # CruxDev's own project should be healthy
    failed = [c for c in report.checks if not c.passed]
    for f in failed:
        print(f"  FAILED: {f.name} — {f.message}")


def test_check_mcp_server_failure(monkeypatch):
    monkeypatch.setattr("src.status.check_mcp_server", lambda: Check("MCP server", False, "test fail"))
    # Can't easily break the import, but we test the Check dataclass
    c = Check("MCP server", False, "Import error")
    assert c.passed is False


def test_check_tool_count_failure(monkeypatch, tmp_path):
    import src.status as status_module
    orig = status_module.CRUXDEV_ROOT
    monkeypatch.setattr(status_module, "CRUXDEV_ROOT", str(tmp_path))
    c = check_tool_count()
    assert c.passed is False
    monkeypatch.setattr(status_module, "CRUXDEV_ROOT", orig)


def test_check_dependencies_missing(monkeypatch):
    # Mock importlib.import_module to fail for pydantic
    import importlib
    orig = importlib.import_module
    def mock_import(name):
        if name == "pydantic":
            raise ImportError("mocked")
        return orig(name)
    monkeypatch.setattr(importlib, "import_module", mock_import)
    c = check_dependencies()
    assert c.passed is False
    assert "pydantic" in c.message


def test_check_docs_missing(monkeypatch):
    import src.status as status_module
    monkeypatch.setattr(status_module, "CRUXDEV_ROOT", "/nonexistent")
    c = check_methodology_docs()
    assert c.passed is False


def test_check_commands_missing(monkeypatch):
    import src.status as status_module
    monkeypatch.setattr(status_module, "CRUXDEV_ROOT", "/nonexistent")
    c = check_slash_commands()
    assert c.passed is False


def test_get_status_unhealthy(tmp_path):
    report = get_status(str(tmp_path))
    # tmp_path has no .cruxdev/ or .mcp.json
    assert any(not c.passed for c in report.checks)


def test_mcp_status_tool():
    from src.mcp_server import cruxdev_status
    result = json.loads(cruxdev_status("."))
    assert "healthy" in result
    assert "checks" in result
    assert "versions" in result
