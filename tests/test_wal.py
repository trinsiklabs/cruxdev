"""Tests for write-ahead log."""

import os

from src.engine.wal import append, event_count, read


class TestWAL:
    def test_append_and_read(self, tmp_path):
        state_path = str(tmp_path / "state.json")
        append(state_path, "start", {"plan": "test.md"})

        events = read(state_path)
        assert len(events) == 1
        assert events[0]["event_type"] == "start"
        assert events[0]["plan"] == "test.md"
        assert "timestamp" in events[0]

    def test_multiple_appends(self, tmp_path):
        state_path = str(tmp_path / "state.json")
        append(state_path, "start")
        append(state_path, "submit", {"findings": 3})
        append(state_path, "phase_change", {"new_phase": "code_auditing"})

        events = read(state_path)
        assert len(events) == 3
        assert events[0]["event_type"] == "start"
        assert events[1]["event_type"] == "submit"
        assert events[2]["event_type"] == "phase_change"

    def test_event_count(self, tmp_path):
        state_path = str(tmp_path / "state.json")
        assert event_count(state_path) == 0

        append(state_path, "start")
        append(state_path, "submit")
        assert event_count(state_path) == 2

    def test_read_nonexistent(self, tmp_path):
        assert read(str(tmp_path / "nope.json")) == []

    def test_event_count_nonexistent(self, tmp_path):
        assert event_count(str(tmp_path / "nope.json")) == 0

    def test_no_details(self, tmp_path):
        state_path = str(tmp_path / "state.json")
        append(state_path, "simple_event")
        events = read(state_path)
        assert events[0]["event_type"] == "simple_event"
        assert "timestamp" in events[0]

    def test_wal_path_derivation(self, tmp_path):
        state_path = str(tmp_path / "abc123.json")
        append(state_path, "test")
        wal_file = str(tmp_path / "abc123.wal")
        assert os.path.exists(wal_file)

    def test_corrupt_line_skipped(self, tmp_path):
        state_path = str(tmp_path / "state.json")
        wal_path = str(tmp_path / "state.wal")
        with open(wal_path, "w") as f:
            f.write('{"event_type": "good"}\n')
            f.write('not json\n')
            f.write('{"event_type": "also_good"}\n')

        events = read(state_path)
        assert len(events) == 2
