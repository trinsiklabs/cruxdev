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


def test_install_with_corrupt_settings(tmp_path):
    claude_dir = tmp_path / ".claude"
    claude_dir.mkdir()
    (claude_dir / "settings.local.json").write_text("not json")
    result = install(str(tmp_path))
    assert result["status"] == "installed"


def test_install_adds_hook(tmp_path):
    result = install(str(tmp_path))
    settings_path = tmp_path / ".claude" / "settings.local.json"
    assert settings_path.exists()
    import json
    with open(settings_path) as f:
        settings = json.load(f)
    assert "hooks" in settings
    assert "PostToolUse" in settings["hooks"]
    assert len(settings["hooks"]["PostToolUse"]) >= 1


def test_uninstall_preserves_cruxdev_state(tmp_path):
    install(str(tmp_path))
    state_dir = tmp_path / ".cruxdev"
    (state_dir / "some_state.json").write_text("{}")

    uninstall(str(tmp_path))
    assert (state_dir / "some_state.json").exists()


def test_install_adds_gitignore_security(tmp_path):
    install(str(tmp_path))
    gitignore = tmp_path / ".gitignore"
    assert gitignore.exists()
    content = gitignore.read_text()
    assert "CruxDev security" in content
    assert "**/tasks/*.output" in content
    assert "*.key" in content
    assert ".crux/" in content


def test_install_gitignore_idempotent(tmp_path):
    install(str(tmp_path))
    install(str(tmp_path))
    content = (tmp_path / ".gitignore").read_text()
    assert content.count("CruxDev security") == 1


def test_install_adds_secret_scanner(tmp_path):
    (tmp_path / ".git" / "hooks").mkdir(parents=True)
    install(str(tmp_path))
    hook = tmp_path / ".git" / "hooks" / "pre-commit"
    assert hook.exists()
    assert "secret" in hook.read_text().lower() or "BLOCKED" in hook.read_text()


def test_install_no_git_dir_skips_hook(tmp_path):
    # No .git dir — should not fail, just skip
    install(str(tmp_path))
    assert not (tmp_path / ".git" / "hooks" / "pre-commit").exists()


def test_install_missing_hook_script(tmp_path, monkeypatch):
    (tmp_path / ".git" / "hooks").mkdir(parents=True)
    monkeypatch.setattr("src.install.CRUXDEV_ROOT", str(tmp_path / "fake_root"))
    from src.install import _install_secret_scanner
    assert _install_secret_scanner(str(tmp_path)) is False


def test_install_preserves_existing_hook(tmp_path):
    hooks_dir = tmp_path / ".git" / "hooks"
    hooks_dir.mkdir(parents=True)
    existing = hooks_dir / "pre-commit"
    existing.write_text("#!/bin/bash\necho 'existing hook'")
    install(str(tmp_path))
    assert "existing hook" in existing.read_text()
