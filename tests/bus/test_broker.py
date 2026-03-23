"""Tests for session bus broker."""

import os
import time

import pytest

from src.bus.broker import Broker, MessageSeverity, MessageType


@pytest.fixture
def broker(tmp_path):
    db_path = str(tmp_path / "bus.db")
    return Broker(db_path=db_path)


# --- Session management ---


def test_register_session(broker):
    sid = broker.register_session("cruxdev", "/path/to/cruxdev")
    assert len(sid) == 8
    sessions = broker.list_sessions()
    assert len(sessions) == 1
    assert sessions[0].project == "cruxdev"


def test_list_sessions(broker):
    broker.register_session("crux", "/path/to/crux")
    broker.register_session("cruxcli", "/path/to/cruxcli")
    broker.register_session("cruxdev", "/path/to/cruxdev")
    sessions = broker.list_sessions()
    assert len(sessions) == 3
    projects = [s.project for s in sessions]
    assert "crux" in projects
    assert "cruxcli" in projects
    assert "cruxdev" in projects


def test_heartbeat(broker):
    sid = broker.register_session("crux", "/p")
    old = broker.list_sessions()[0].last_heartbeat
    time.sleep(0.01)
    broker.heartbeat(sid)
    new = broker.list_sessions()[0].last_heartbeat
    assert new > old


def test_unregister_session(broker):
    sid = broker.register_session("crux", "/p")
    broker.unregister_session(sid)
    assert broker.list_sessions() == []


def test_cleanup_stale(broker):
    sid = broker.register_session("crux", "/p")
    # Manually set heartbeat to old time
    with broker._conn() as conn:
        conn.execute("UPDATE sessions SET last_heartbeat = ?", (time.time() - 7200,))
    removed = broker.cleanup_stale_sessions(max_age_seconds=3600)
    assert removed == 1
    assert broker.list_sessions() == []


def test_cleanup_keeps_fresh(broker):
    broker.register_session("crux", "/p")
    removed = broker.cleanup_stale_sessions(max_age_seconds=3600)
    assert removed == 0
    assert len(broker.list_sessions()) == 1


# --- Messaging ---


def test_send_and_receive(broker):
    msg_id = broker.send_message(
        type="issue",
        source_project="crux",
        target_project="cruxdev",
        title="Bug found",
        body="The planning phase gets stuck",
        severity="high",
    )
    assert len(msg_id) == 8

    inbox = broker.check_inbox("cruxdev")
    assert len(inbox) == 1
    assert inbox[0].title == "Bug found"
    assert inbox[0].severity == "high"
    assert inbox[0].acknowledged is False


def test_inbox_filters_by_project(broker):
    broker.send_message("issue", "crux", "cruxdev", "For cruxdev", "body", "medium")
    broker.send_message("issue", "crux", "cruxcli", "For cruxcli", "body", "medium")

    cruxdev_inbox = broker.check_inbox("cruxdev")
    assert len(cruxdev_inbox) == 1
    assert cruxdev_inbox[0].title == "For cruxdev"

    cruxcli_inbox = broker.check_inbox("cruxcli")
    assert len(cruxcli_inbox) == 1
    assert cruxcli_inbox[0].title == "For cruxcli"


def test_broadcast(broker):
    broker.send_message("pattern", "cruxdev", "*", "New pattern", "Use atomic writes", "low")

    crux_inbox = broker.check_inbox("crux")
    cruxcli_inbox = broker.check_inbox("cruxcli")
    assert len(crux_inbox) == 1
    assert len(cruxcli_inbox) == 1
    assert crux_inbox[0].title == "New pattern"


def test_acknowledge(broker):
    msg_id = broker.send_message("issue", "crux", "cruxdev", "Bug", "details", "high")

    assert broker.acknowledge(msg_id) is True

    inbox = broker.check_inbox("cruxdev")
    assert len(inbox) == 0  # Acknowledged messages filtered out

    inbox_all = broker.check_inbox("cruxdev", include_acknowledged=True)
    assert len(inbox_all) == 1
    assert inbox_all[0].acknowledged is True
    assert inbox_all[0].acknowledged_at is not None


def test_acknowledge_nonexistent(broker):
    assert broker.acknowledge("nonexistent") is False


def test_get_message(broker):
    msg_id = broker.send_message("issue", "crux", "cruxdev", "Bug", "details", "high")
    msg = broker.get_message(msg_id)
    assert msg is not None
    assert msg.title == "Bug"


def test_get_message_nonexistent(broker):
    assert broker.get_message("nonexistent") is None


def test_get_all_messages(broker):
    broker.send_message("issue", "crux", "cruxdev", "Issue 1", "body", "high")
    broker.send_message("pattern", "cruxdev", "*", "Pattern 1", "body", "low")
    broker.send_message("issue", "cruxcli", "cruxdev", "Issue 2", "body", "medium")

    all_msgs = broker.get_all_messages()
    assert len(all_msgs) == 3

    issues_only = broker.get_all_messages(type_filter="issue")
    assert len(issues_only) == 2


# --- Convenience methods ---


def test_report_issue(broker):
    msg_id = broker.report_issue("crux", "cruxdev", "Engine bug", "Details here", "high")
    msg = broker.get_message(msg_id)
    assert msg.type == "issue"
    assert msg.severity == "high"


def test_report_improvement(broker):
    msg_id = broker.report_improvement("cruxcli", "cruxdev", "Add feature X", "Would help with...")
    msg = broker.get_message(msg_id)
    assert msg.type == "improvement"


def test_share_pattern(broker):
    msg_id = broker.share_pattern("crux", "atomic-writes", "Always use write-then-rename")
    msg = broker.get_message(msg_id)
    assert msg.type == "pattern"
    assert msg.target_project == "*"


def test_notify_breaking_change(broker):
    msg_ids = broker.notify_breaking_change(
        "cruxdev",
        ["crux", "cruxcli"],
        "Renamed ConvergencePhase.PLANNING",
    )
    assert len(msg_ids) == 2

    crux_inbox = broker.check_inbox("crux")
    assert len(crux_inbox) == 1
    assert crux_inbox[0].type == "breaking_change"
    assert crux_inbox[0].severity == "high"

    cruxcli_inbox = broker.check_inbox("cruxcli")
    assert len(cruxcli_inbox) == 1


# --- Enums ---


def test_message_types():
    assert MessageType.ISSUE.value == "issue"
    assert MessageType.IMPROVEMENT.value == "improvement"
    assert MessageType.PATTERN.value == "pattern"
    assert MessageType.BREAKING_CHANGE.value == "breaking_change"
    assert MessageType.CUSTOM.value == "custom"


def test_message_severity():
    assert MessageSeverity.HIGH.value == "high"
    assert MessageSeverity.MEDIUM.value == "medium"
    assert MessageSeverity.LOW.value == "low"


# --- Edge cases ---


def test_multiple_brokers_same_db(tmp_path):
    """Multiple broker instances can share the same database."""
    db_path = str(tmp_path / "shared.db")
    b1 = Broker(db_path=db_path)
    b2 = Broker(db_path=db_path)

    b1.register_session("crux", "/p")
    sessions = b2.list_sessions()
    assert len(sessions) == 1


def test_empty_inbox(broker):
    assert broker.check_inbox("nonexistent_project") == []


def test_message_ordering(broker):
    broker.send_message("issue", "a", "cruxdev", "First", "1", "low")
    time.sleep(0.01)
    broker.send_message("issue", "b", "cruxdev", "Second", "2", "low")

    inbox = broker.check_inbox("cruxdev")
    assert len(inbox) == 2
    # Most recent first
    assert inbox[0].title == "Second"
    assert inbox[1].title == "First"
