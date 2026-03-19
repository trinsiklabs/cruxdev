"""Tests for state persistence — serialization, deserialization, atomic writes."""

import json
import os
import time

import pytest

from src.engine.persistence import deserialize, load_state, save_state, serialize
from src.engine.state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
    RoundResult,
)


def _make_state_with_history() -> ConvergenceState:
    s = ConvergenceState(
        plan_file="plan.md",
        phase=ConvergencePhase.CODE_AUDITING,
        round=3,
        consecutive_clean=1,
        failures={"task1": 2},
    )
    s.history.append(
        RoundResult(
            round=0,
            phase=ConvergencePhase.CODE_AUDITING,
            findings=[
                Finding(
                    id="f1",
                    file="foo.py",
                    dimension="correctness",
                    severity=FindingSeverity.HIGH,
                    description="bug",
                    suggested_fix="fix it",
                    fixed=True,
                )
            ],
            findings_fixed=1,
            timestamp=1000.0,
        )
    )
    s.history.append(
        RoundResult(
            round=1,
            phase=ConvergencePhase.CODE_AUDITING,
            findings=[],
            findings_fixed=0,
            timestamp=2000.0,
        )
    )
    return s


# --- Serialization roundtrip ---


def test_serialize_empty_state():
    s = ConvergenceState(plan_file="plan.md")
    data = serialize(s)
    assert data["plan_file"] == "plan.md"
    assert data["phase"] == "planning"
    assert data["history"] == []


def test_serialize_with_history():
    s = _make_state_with_history()
    data = serialize(s)
    assert data["phase"] == "code_auditing"
    assert len(data["history"]) == 2
    assert data["history"][0]["findings"][0]["severity"] == "high"
    assert data["history"][0]["findings"][0]["fixed"] is True


def test_deserialize_roundtrip():
    original = _make_state_with_history()
    data = serialize(original)
    restored = deserialize(data)

    assert restored.plan_file == original.plan_file
    assert restored.phase == original.phase
    assert restored.round == original.round
    assert restored.consecutive_clean == original.consecutive_clean
    assert restored.failures == original.failures
    assert len(restored.history) == len(original.history)
    assert restored.history[0].findings[0].id == "f1"
    assert restored.history[0].findings[0].severity == FindingSeverity.HIGH
    assert restored.history[0].findings[0].fixed is True
    assert restored.history[1].findings == []


def test_deserialize_defaults():
    data = {"plan_file": "p.md", "phase": "planning"}
    s = deserialize(data)
    assert s.round == 0
    assert s.max_rounds == 5
    assert s.failures == {}
    assert s.deadline is None


def test_serialize_all_phases():
    for phase in ConvergencePhase:
        s = ConvergenceState(plan_file="p.md", phase=phase)
        data = serialize(s)
        restored = deserialize(data)
        assert restored.phase == phase


def test_serialize_all_severities():
    for sev in FindingSeverity:
        f = Finding(
            id="f1", file="x.py", dimension="d",
            severity=sev, description="d", suggested_fix="f",
        )
        s = ConvergenceState(plan_file="p.md")
        s.history.append(
            RoundResult(
                round=0, phase=ConvergencePhase.PLANNING,
                findings=[f], findings_fixed=0, timestamp=1.0,
            )
        )
        data = serialize(s)
        restored = deserialize(data)
        assert restored.history[0].findings[0].severity == sev


# --- File persistence ---


def test_save_and_load(tmp_path):
    path = str(tmp_path / "state.json")
    original = _make_state_with_history()
    save_state(original, path)

    assert os.path.exists(path)
    loaded = load_state(path)

    assert loaded.plan_file == original.plan_file
    assert loaded.phase == original.phase
    assert loaded.round == original.round
    assert len(loaded.history) == 2


def test_save_creates_parent_dir(tmp_path):
    path = str(tmp_path / "nested" / "deep" / "state.json")
    s = ConvergenceState(plan_file="p.md")
    save_state(s, path)
    assert os.path.exists(path)


def test_save_overwrites(tmp_path):
    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md", round=1)
    save_state(s, path)

    s.round = 5
    save_state(s, path)

    loaded = load_state(path)
    assert loaded.round == 5


def test_save_updates_timestamp(tmp_path):
    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md")
    old_time = s.updated_at
    time.sleep(0.01)
    save_state(s, path)
    assert s.updated_at > old_time


def test_load_nonexistent_raises():
    with pytest.raises(FileNotFoundError):
        load_state("/nonexistent/path/state.json")


def test_save_atomic_no_temp_files(tmp_path):
    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md")
    save_state(s, path)

    files = os.listdir(tmp_path)
    assert files == ["state.json"]


def test_save_error_cleans_up_temp_file(tmp_path):
    """When os.rename fails, temp file should be cleaned up and exception re-raised."""
    from unittest.mock import patch

    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md")

    with patch("src.engine.persistence.os.rename", side_effect=OSError("disk full")):
        with pytest.raises(OSError, match="disk full"):
            save_state(s, path)

    # Temp file should be cleaned up
    temp_files = [f for f in os.listdir(tmp_path) if f.startswith(".convergence_")]
    assert temp_files == []


def test_save_error_unlink_also_fails(tmp_path):
    """When both rename and unlink fail, exception still propagates."""
    from unittest.mock import patch

    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md")

    with patch("src.engine.persistence.os.rename", side_effect=OSError("rename failed")):
        with patch("src.engine.persistence.os.unlink", side_effect=OSError("unlink failed")):
            with pytest.raises(OSError, match="rename failed"):
                save_state(s, path)


def test_save_error_closes_fd_if_write_fails(tmp_path):
    """When os.write fails (fd still open), fd should be closed and temp cleaned up."""
    from unittest.mock import patch

    path = str(tmp_path / "state.json")
    s = ConvergenceState(plan_file="p.md")

    with patch("src.engine.persistence.os.write", side_effect=IOError("write failed")):
        with pytest.raises(IOError, match="write failed"):
            save_state(s, path)

    # Temp file should be cleaned up
    temp_files = [f for f in os.listdir(tmp_path) if f.startswith(".convergence_")]
    assert temp_files == []


def test_saved_file_is_valid_json(tmp_path):
    path = str(tmp_path / "state.json")
    s = _make_state_with_history()
    save_state(s, path)

    with open(path) as f:
        data = json.load(f)
    assert isinstance(data, dict)
    assert data["plan_file"] == "plan.md"
