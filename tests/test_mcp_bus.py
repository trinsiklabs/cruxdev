"""Tests for MCP session bus tools."""

import json
import os

import pytest

from src.mcp_server import (
    acknowledge_message,
    check_inbox,
    init,
    notify_breaking_change,
    report_improvement,
    report_issue,
    session_list,
    session_register,
    share_pattern,
)
from src.bus.broker import Broker


@pytest.fixture(autouse=True)
def setup(tmp_path, monkeypatch):
    # Use temp db for broker
    db_path = str(tmp_path / "bus.db")
    monkeypatch.setattr("src.mcp_server._get_broker", lambda: Broker(db_path=db_path))
    init(str(tmp_path / "state"))
    yield


def test_session_register():
    result = json.loads(session_register("cruxdev"))
    assert result["status"] == "registered"
    assert result["project"] == "cruxdev"
    assert len(result["session_id"]) == 8


def test_session_list():
    session_register("crux")
    session_register("cruxcli")
    result = json.loads(session_list())
    assert len(result) == 2
    projects = [s["project"] for s in result]
    assert "crux" in projects
    assert "cruxcli" in projects


def test_session_list_empty():
    result = json.loads(session_list())
    assert result == []


def test_report_issue():
    result = json.loads(report_issue("cruxdev", "Engine bug", "Details", "high"))
    assert result["status"] == "sent"
    assert result["to"] == "cruxdev"


def test_report_improvement():
    result = json.loads(report_improvement("cruxdev", "Add feature", "Would help"))
    assert result["status"] == "sent"


def test_share_pattern():
    result = json.loads(share_pattern("atomic-writes", "Use write-then-rename"))
    assert result["status"] == "broadcast"
    assert result["pattern"] == "atomic-writes"


def test_notify_breaking_change():
    result = json.loads(notify_breaking_change("crux,cruxcli", "Renamed phase"))
    assert result["status"] == "sent"
    assert len(result["message_ids"]) == 2
    assert result["to"] == ["crux", "cruxcli"]


def test_check_inbox():
    report_issue("cruxdev", "Bug", "Details", "high")
    result = json.loads(check_inbox("cruxdev"))
    assert len(result) == 1
    assert result[0]["title"] == "Bug"
    assert result[0]["type"] == "issue"


def test_check_inbox_empty():
    result = json.loads(check_inbox("cruxdev"))
    assert result == []


def test_check_inbox_auto_project():
    report_issue("cruxdev", "Bug", "Details", "medium")
    # check_inbox with empty string uses cwd basename
    result = json.loads(check_inbox(""))
    # May or may not match depending on cwd — at least shouldn't error
    assert isinstance(result, list)


def test_acknowledge_message():
    report_issue("cruxdev", "Bug", "Details", "high")
    inbox = json.loads(check_inbox("cruxdev"))
    msg_id = inbox[0]["id"]

    result = json.loads(acknowledge_message(msg_id))
    assert result["acknowledged"] is True

    # Should be gone from inbox
    inbox_after = json.loads(check_inbox("cruxdev"))
    assert len(inbox_after) == 0


def test_acknowledge_nonexistent():
    result = json.loads(acknowledge_message("nonexistent"))
    assert result["acknowledged"] is False


def test_full_workflow():
    """Full workflow: register, report, check, acknowledge."""
    session_register("crux")
    session_register("cruxdev")

    # Crux reports issue to CruxDev
    report_issue("cruxdev", "Planning phase stuck", "When plan has no items...", "high")

    # CruxDev checks inbox
    inbox = json.loads(check_inbox("cruxdev"))
    assert len(inbox) == 1
    assert inbox[0]["title"] == "Planning phase stuck"

    # CruxDev acknowledges after fixing
    acknowledge_message(inbox[0]["id"])

    # Inbox should be empty
    assert json.loads(check_inbox("cruxdev")) == []

    # Sessions still listed
    sessions = json.loads(session_list())
    assert len(sessions) == 2
