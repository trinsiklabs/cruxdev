"""Tests for session bus hook — push notifications."""

import os
import time

import pytest

from src.bus.hook import (
    check_and_notify,
    format_notification,
    mark_checked,
    should_check,
)


def test_should_check_no_file(tmp_path):
    assert should_check(str(tmp_path / "nonexistent")) is True


def test_should_check_recent(tmp_path):
    check_file = str(tmp_path / "last_check")
    mark_checked(check_file)
    assert should_check(check_file) is False


def test_should_check_old(tmp_path):
    check_file = str(tmp_path / "last_check")
    with open(check_file, "w") as f:
        f.write(str(time.time() - 120))  # 2 minutes ago
    assert should_check(check_file) is True


def test_should_check_corrupt(tmp_path):
    check_file = str(tmp_path / "last_check")
    with open(check_file, "w") as f:
        f.write("not a number")
    assert should_check(check_file) is True


def test_mark_checked(tmp_path):
    check_file = str(tmp_path / "sub" / "last_check")
    mark_checked(check_file)
    assert os.path.exists(check_file)
    with open(check_file) as f:
        val = float(f.read().strip())
    assert val > 0


def test_check_and_notify_rate_limited(tmp_path):
    check_file = str(tmp_path / "last_check")
    mark_checked(check_file)  # Just checked
    result = check_and_notify("test", check_file)
    assert result == []


def test_check_and_notify_empty_inbox(tmp_path, monkeypatch):
    # Use a clean broker to avoid picking up real bus messages
    from src.bus.broker import Broker
    db_path = str(tmp_path / "clean_bus.db")
    import src.bus.broker as broker_module
    original = broker_module.Broker
    monkeypatch.setattr(broker_module, "Broker", lambda db_path=db_path: original(db_path=db_path))

    check_file = str(tmp_path / "last_check")
    result = check_and_notify("nonexistent_project", check_file)
    assert result == []


def test_check_and_notify_with_messages(tmp_path, monkeypatch):
    from src.bus.broker import Broker

    db_path = str(tmp_path / "bus.db")
    broker = Broker(db_path=db_path)
    broker.report_issue("crux", "cruxdev", "Bug found", "Details", "high")

    # Monkeypatch the Broker class in the broker module so hook imports it
    import src.bus.broker as broker_module
    original = broker_module.Broker
    monkeypatch.setattr(broker_module, "Broker", lambda db_path=db_path: original(db_path=db_path))

    check_file = str(tmp_path / "last_check")
    result = check_and_notify("cruxdev", check_file)
    assert len(result) == 1
    assert result[0]["title"] == "Bug found"
    assert result[0]["severity"] == "high"


def test_format_notification_empty():
    assert format_notification([]) == ""


def test_format_notification_with_messages():
    messages = [
        {"id": "1", "type": "issue", "from": "crux", "title": "Bug", "severity": "high"},
        {"id": "2", "type": "pattern", "from": "cruxcli", "title": "Pattern", "severity": "low"},
    ]
    output = format_notification(messages)
    assert "[SESSION BUS]" in output
    assert "2 new message" in output
    assert "Bug" in output
    assert "Pattern" in output
    assert "[!!]" in output  # high severity
    assert "/inbox" in output


def test_format_notification_single():
    messages = [
        {"id": "1", "type": "improvement", "from": "crux", "title": "Add feature", "severity": "medium"},
    ]
    output = format_notification(messages)
    assert "1 new message" in output
