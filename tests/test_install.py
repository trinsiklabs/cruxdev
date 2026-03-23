"""Tests for CruxDev installation."""

import json
import os

import pytest

from src.install import install, uninstall


def test_install_fresh_project(tmp_path):
    result = install(str(tmp_path))
    assert result["status"] == "installed"

    mcp_path = tmp_path / ".mcp.json"
    assert mcp_path.exists()

    with open(mcp_path) as f:
        config = json.load(f)
    assert "cruxdev" in config["mcpServers"]

    assert (tmp_path / ".cruxdev").is_dir()


def test_install_preserves_existing_servers(tmp_path):
    mcp_path = tmp_path / ".mcp.json"
    mcp_path.write_text(json.dumps({
        "mcpServers": {
            "crux": {"command": "python", "args": ["-m", "crux_server"]},
        }
    }))

    result = install(str(tmp_path))
    assert result["status"] == "installed"

    with open(mcp_path) as f:
        config = json.load(f)
    assert "crux" in config["mcpServers"]
    assert "cruxdev" in config["mcpServers"]
    assert any("full stack" in item for item in result["items"])


def test_install_without_crux(tmp_path):
    result = install(str(tmp_path))
    assert any("Crux MCP server not configured" in item for item in result["items"])


def test_install_idempotent(tmp_path):
    install(str(tmp_path))
    install(str(tmp_path))

    mcp_path = tmp_path / ".mcp.json"
    with open(mcp_path) as f:
        config = json.load(f)
    assert "cruxdev" in config["mcpServers"]


def test_install_has_correct_config(tmp_path):
    install(str(tmp_path))

    mcp_path = tmp_path / ".mcp.json"
    with open(mcp_path) as f:
        config = json.load(f)

    server = config["mcpServers"]["cruxdev"]
    assert "command" in server
    assert server["args"] == ["-m", "src.mcp_server"]
    assert "cwd" in server


def test_uninstall(tmp_path):
    install(str(tmp_path))
    result = uninstall(str(tmp_path))
    assert result["status"] == "uninstalled"

    mcp_path = tmp_path / ".mcp.json"
    with open(mcp_path) as f:
        config = json.load(f)
    assert "cruxdev" not in config["mcpServers"]


def test_uninstall_not_installed(tmp_path):
    result = uninstall(str(tmp_path))
    assert result["status"] == "not_installed"


def test_uninstall_not_in_config(tmp_path):
    (tmp_path / ".mcp.json").write_text(json.dumps({"mcpServers": {}}))

    result = uninstall(str(tmp_path))
    assert result["status"] == "not_installed"


def test_uninstall_preserves_cruxdev_state(tmp_path):
    install(str(tmp_path))
    state_dir = tmp_path / ".cruxdev"
    (state_dir / "some_state.json").write_text("{}")

    uninstall(str(tmp_path))
    assert (state_dir / "some_state.json").exists()
