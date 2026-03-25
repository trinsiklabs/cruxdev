"""Tests for session bus notification files and updated hook."""

import json
import os
import time

import pytest

from src.bus.broker import Broker
from src.bus.hook import check_and_notify, format_notification, mark_checked, should_check


@pytest.fixture
def broker(tmp_path):
    return Broker(db_path=str(tmp_path / "bus.db"))


@pytest.fixture
def last_check_file(tmp_path):
    return str(tmp_path / "last_check")


class TestNotificationFiles:
    def test_send_message_creates_notification(self, broker):
        broker.send_message("issue", "src", "target", "Bug", "details")
        notif = broker.read_notification("target")
        assert notif is not None
        assert notif["count"] == 1
        assert notif["latest_title"] == "Bug"

    def test_multiple_messages_increment_count(self, broker):
        broker.send_message("issue", "src", "target", "Bug 1", "d1")
        broker.send_message("issue", "src", "target", "Bug 2", "d2")
        notif = broker.read_notification("target")
        assert notif["count"] == 2
        assert notif["latest_title"] == "Bug 2"

    def test_broadcast_notifies_registered_sessions(self, broker):
        broker.register_session("project_a", "/a")
        broker.register_session("project_b", "/b")
        broker.send_message("pattern", "project_a", "*", "Pattern", "desc")
        # project_b should be notified, project_a (sender) should not
        assert broker.read_notification("project_b") is not None
        assert broker.read_notification("project_a") is None

    def test_clear_notification_removes_file(self, broker):
        broker.send_message("issue", "src", "target", "Bug", "d")
        assert broker.read_notification("target") is not None
        assert broker.clear_notification("target") is True
        assert broker.read_notification("target") is None

    def test_clear_notification_nonexistent(self, broker):
        assert broker.clear_notification("nonexistent") is False

    def test_read_notification_nonexistent(self, broker):
        assert broker.read_notification("nonexistent") is None

    def test_atomic_write(self, broker):
        # Send message and verify no .tmp file remains
        broker.send_message("issue", "src", "target", "Bug", "d")
        notify_dir = os.path.join(os.path.dirname(broker.db_path), "notifications")
        files = os.listdir(notify_dir)
        assert not any(f.endswith(".tmp") for f in files)


class TestHookWithNotifications:
    def test_check_and_notify_uses_notification_file(self, broker, last_check_file):
        broker.send_message("issue", "other", "myproject", "Bug", "details")

        messages = check_and_notify("myproject", last_check_file, broker_factory=lambda: broker)
        assert len(messages) == 1
        assert messages[0]["title"] == "Bug"

        # Notification file should be cleared after reading
        assert broker.read_notification("myproject") is None

    def test_check_and_notify_no_notification_skips_sqlite(self, broker, last_check_file):
        messages = check_and_notify("myproject", last_check_file, broker_factory=lambda: broker)
        assert messages == []

    def test_check_and_notify_respects_rate_limit(self, broker, last_check_file):
        broker.send_message("issue", "other", "myproject", "Bug", "d")

        # First call succeeds
        messages = check_and_notify("myproject", last_check_file, broker_factory=lambda: broker)
        assert len(messages) == 1

        # Send another message
        broker.send_message("issue", "other", "myproject", "Bug 2", "d")

        # Second call within rate limit returns empty
        messages = check_and_notify("myproject", last_check_file, broker_factory=lambda: broker)
        assert messages == []

    def test_check_and_notify_no_messages_after_notification(self, broker, last_check_file):
        # Create notification file manually but no actual messages
        broker._write_notification("myproject", "Ghost")
        messages = check_and_notify("myproject", last_check_file, broker_factory=lambda: broker)
        # If notification exists but no unacknowledged messages, returns empty
        assert messages == []


class TestHookBasics:
    def test_should_check_no_file(self, last_check_file):
        assert should_check(last_check_file) is True

    def test_should_check_recent(self, last_check_file):
        mark_checked(last_check_file)
        assert should_check(last_check_file) is False

    def test_should_check_stale(self, last_check_file):
        with open(last_check_file, "w") as f:
            f.write(str(time.time() - 120))  # 2 min ago
        assert should_check(last_check_file) is True

    def test_should_check_invalid_content(self, last_check_file):
        os.makedirs(os.path.dirname(last_check_file), exist_ok=True)
        with open(last_check_file, "w") as f:
            f.write("not a number")
        assert should_check(last_check_file) is True

    def test_format_notification_empty(self):
        assert format_notification([]) == ""

    def test_format_notification_messages(self):
        msgs = [
            {"type": "issue", "from": "crux", "title": "Bug", "severity": "high"},
            {"type": "pattern", "from": "cruxcli", "title": "Tip", "severity": "low"},
        ]
        output = format_notification(msgs)
        assert "[SESSION BUS] 2 new message(s)" in output
        assert "[!!]" in output  # high severity
        assert "[!]" in output   # non-high severity
        assert "/inbox" in output
