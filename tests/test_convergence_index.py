"""Tests for convergence index — plan→run mapping."""

import os

from src.engine.convergence_index import (
    IndexEntry,
    find_active_run,
    load_index,
    register_run,
    save_index,
    update_run_status,
)


class TestIndex:
    def test_empty_index(self, tmp_path):
        assert load_index(str(tmp_path)) == []

    def test_register_and_load(self, tmp_path):
        entry = register_run(str(tmp_path), "plan.md", "abc123", "/state/abc123.json")
        assert entry.plan_file == "plan.md"
        assert entry.convergence_id == "abc123"
        assert entry.status == "active"

        loaded = load_index(str(tmp_path))
        assert len(loaded) == 1
        assert loaded[0].convergence_id == "abc123"

    def test_find_active_run(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        register_run(str(tmp_path), str(plan), "abc123", "/state.json")

        found = find_active_run(str(tmp_path), str(plan))
        assert found is not None
        assert found.convergence_id == "abc123"

    def test_find_active_run_not_found(self, tmp_path):
        assert find_active_run(str(tmp_path), "nonexistent.md") is None

    def test_find_active_run_ignores_completed(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        register_run(str(tmp_path), str(plan), "abc123", "/state.json")
        update_run_status(str(tmp_path), "abc123", "converged")

        assert find_active_run(str(tmp_path), str(plan)) is None

    def test_update_run_status(self, tmp_path):
        register_run(str(tmp_path), "plan.md", "abc123", "/state.json")
        assert update_run_status(str(tmp_path), "abc123", "converged") is True

        loaded = load_index(str(tmp_path))
        assert loaded[0].status == "converged"

    def test_update_run_status_not_found(self, tmp_path):
        assert update_run_status(str(tmp_path), "nonexistent", "done") is False

    def test_multiple_runs(self, tmp_path):
        register_run(str(tmp_path), "plan1.md", "id1", "/s1.json")
        register_run(str(tmp_path), "plan2.md", "id2", "/s2.json")

        loaded = load_index(str(tmp_path))
        assert len(loaded) == 2

    def test_corrupt_index(self, tmp_path):
        idx = tmp_path / ".cruxdev" / "convergence_index.json"
        idx.parent.mkdir(parents=True)
        idx.write_text("not json")
        assert load_index(str(tmp_path)) == []
