"""Tests for evolution state + two-layer memory."""

import json
import os
import time

import pytest

from src.evolution.state import (
    EvolutionCycle,
    EvolutionState,
    append_to_archive,
    check_protected_files,
    load_active_context,
    read_archive,
    save_active_context,
)


def test_evolution_state_defaults():
    s = EvolutionState(project="cruxdev")
    assert s.cycle_count == 0
    assert s.current_cycle is None
    assert len(s.protected_files) > 0


def test_evolution_cycle():
    c = EvolutionCycle(cycle_id=1, started_at=time.time())
    assert c.beat == "gather"
    assert c.error is None


# --- Archive (Layer 1) ---


def test_append_to_archive(tmp_path):
    path = str(tmp_path / "archive.jsonl")
    append_to_archive(path, {"type": "test", "data": "hello"})
    append_to_archive(path, {"type": "test", "data": "world"})

    entries = read_archive(path)
    assert len(entries) == 2
    assert entries[0]["data"] == "hello"
    assert "timestamp" in entries[0]


def test_read_archive_missing(tmp_path):
    assert read_archive(str(tmp_path / "missing.jsonl")) == []


def test_read_archive_corrupt(tmp_path):
    path = str(tmp_path / "archive.jsonl")
    with open(path, "w") as f:
        f.write("not json\n")
        f.write('{"valid": true}\n')
        f.write("\n")
    entries = read_archive(path)
    assert len(entries) == 1


def test_read_archive_max_entries(tmp_path):
    path = str(tmp_path / "archive.jsonl")
    for i in range(20):
        append_to_archive(path, {"i": i})
    entries = read_archive(path, max_entries=5)
    assert len(entries) == 5
    assert entries[0]["i"] == 15  # Last 5


# --- Active context (Layer 2) ---


def test_save_and_load_context(tmp_path):
    path = str(tmp_path / "context.json")
    state = EvolutionState(
        project="cruxdev",
        cycle_count=3,
        north_star_goals=["ship v1"],
    )
    save_active_context(path, state)
    loaded = load_active_context(path)
    assert loaded.project == "cruxdev"
    assert loaded.cycle_count == 3
    assert loaded.north_star_goals == ["ship v1"]


def test_save_with_cycle(tmp_path):
    path = str(tmp_path / "context.json")
    state = EvolutionState(project="crux")
    state.current_cycle = EvolutionCycle(cycle_id=1, started_at=1.0)
    save_active_context(path, state)
    loaded = load_active_context(path)
    assert loaded.current_cycle is not None
    assert loaded.current_cycle.cycle_id == 1


def test_load_missing_context(tmp_path):
    state = load_active_context(str(tmp_path / "missing.json"))
    assert state.project == "unknown"


# --- Protected files ---


def test_check_protected_no_violations():
    state = EvolutionState(project="x", protected_files=["README.md", "LICENSE"])
    violations = check_protected_files(state, ["src/main.py", "tests/test.py"])
    assert violations == []


def test_check_protected_with_violation():
    state = EvolutionState(project="x", protected_files=["README.md", "LICENSE"])
    violations = check_protected_files(state, ["src/main.py", "README.md"])
    assert violations == ["README.md"]


def test_check_protected_nested_path():
    state = EvolutionState(project="x", protected_files=[".claude/CLAUDE.md"])
    violations = check_protected_files(state, ["some/path/.claude/CLAUDE.md"])
    assert len(violations) == 1
