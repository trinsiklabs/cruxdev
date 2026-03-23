"""Tests for website metrics gathering."""

import os
from src.improvement.website_metrics import (
    count_mcp_tools,
    count_modes,
    count_tests,
    format_metrics_json,
    gather_metrics,
    ProjectMetrics,
)


def test_count_mcp_tools(tmp_path):
    f = tmp_path / "server.py"
    f.write_text("@mcp.tool()\ndef a(): pass\n@mcp.tool()\ndef b(): pass\n")
    assert count_mcp_tools(str(f)) == 2


def test_count_mcp_tools_missing():
    assert count_mcp_tools("/nonexistent.py") == 0


def test_count_modes(tmp_path):
    modes = tmp_path / "modes"
    modes.mkdir()
    (modes / "build-py.md").write_text("mode")
    (modes / "plan.md").write_text("mode")
    (modes / "not_a_mode.txt").write_text("skip")
    assert count_modes(str(modes)) == 2


def test_count_modes_missing():
    assert count_modes("/nonexistent") == 0


def test_gather_metrics(tmp_path):
    server = tmp_path / "server.py"
    server.write_text("@mcp.tool()\ndef t(): pass\n")
    modes = tmp_path / "modes"
    modes.mkdir()
    (modes / "a.md").write_text("m")

    metrics = gather_metrics(str(tmp_path), str(server), str(modes))
    assert metrics.tool_count == 1
    assert metrics.mode_count == 1


def test_gather_metrics_minimal(tmp_path):
    metrics = gather_metrics(str(tmp_path))
    assert metrics.tool_count == 0
    assert metrics.mode_count == 0


def test_format_metrics_json():
    m = ProjectMetrics(test_count=100, tool_count=10)
    result = format_metrics_json(m)
    import json
    data = json.loads(result)
    assert data["test_count"] == 100


def test_count_tests_no_command(tmp_path):
    result = count_tests(str(tmp_path), ["echo", "no tests"])
    assert result == 0


def test_count_tests_with_match(tmp_path):
    # Simulate pytest output
    result = count_tests(str(tmp_path), ["echo", "314 tests collected"])
    assert result == 314


def test_count_tests_missing_command(tmp_path):
    result = count_tests(str(tmp_path), ["/nonexistent/command"])
    assert result == 0
