"""Tests for evolution orchestrator — full cycle."""

import os
import time

import pytest

from src.evolution.orchestrator import EvolutionOrchestrator
from src.evolution.state import load_active_context, read_archive


def test_full_cycle_dry_run(tmp_path):
    """Complete evolution cycle in dry run mode."""
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test_project",
        dry_run=True,
    )

    cycle = orch.run_cycle()
    assert cycle.cycle_id == 1
    assert cycle.beat == "complete"
    assert cycle.error is None
    assert cycle.completed_at is not None


def test_cycle_increments_count(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )

    c1 = orch.run_cycle()
    c2 = orch.run_cycle()
    assert c1.cycle_id == 1
    assert c2.cycle_id == 2


def test_cycle_saves_context(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )
    orch.run_cycle()

    state = load_active_context(orch.context_path)
    assert state.cycle_count == 1
    assert state.last_completed_at is not None


def test_cycle_appends_archive(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )
    orch.run_cycle()

    archive = read_archive(orch.archive_path)
    assert len(archive) == 1
    assert archive[0]["type"] == "cycle_complete"
    assert archive[0]["cycle_id"] == 1


def test_cycle_generates_posts(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )
    cycle = orch.run_cycle()

    assert len(cycle.posted) >= 2  # changelog + x_post
    assert os.path.isdir(orch.posts_dir)
    posts = os.listdir(orch.posts_dir)
    assert len(posts) >= 2


def test_cycle_with_git_repo(tmp_path):
    """Cycle on actual git repo gathers real changes."""
    # Use the cruxdev repo itself
    orch = EvolutionOrchestrator(
        project_dir="/Users/user/personal/cruxdev",
        project_name="cruxdev",
        dry_run=True,
    )
    cycle = orch.run_cycle()
    assert cycle.beat == "complete"
    # Should have gathered git changes
    assert len(cycle.gathered) > 0


def test_cycle_creates_evolution_dir(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )
    orch.run_cycle()
    assert os.path.isdir(orch.evolution_dir)


def test_cycle_live_mode(tmp_path):
    """Live mode (non-dry-run) queues items for convergence."""
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=False,
    )
    cycle = orch.run_cycle()
    assert cycle.beat == "complete"


def test_cycle_error_handling(tmp_path):
    """Errors in beats are caught and stored."""
    from unittest.mock import patch

    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )

    with patch.object(orch, "_beat_gather", side_effect=RuntimeError("test error")):
        cycle = orch.run_cycle()
    assert cycle.beat == "error"
    assert cycle.error == "test error"


def test_integrate_skip_action(tmp_path):
    from src.evolution.evaluate import EvaluationItem, EvaluationResult
    from src.evolution.state import EvolutionState

    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path), project_name="test", dry_run=True,
    )
    evaluated = EvaluationResult(items=[
        EvaluationItem(source="t", title="Skip", description="d", priority=5, novel=True, action="skip"),
        EvaluationItem(source="t", title="Build", description="d", priority=1, novel=True, action="fix"),
    ])
    result = orch._beat_integrate(evaluated, EvolutionState(project="test"))
    assert len(result) == 1
    assert "Build" in result[0]


def test_integrate_live_mode(tmp_path):
    from src.evolution.evaluate import EvaluationItem, EvaluationResult
    from src.evolution.state import EvolutionState

    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path), project_name="test", dry_run=False,
    )
    evaluated = EvaluationResult(items=[
        EvaluationItem(source="t", title="Fix bug", description="d", priority=1, novel=True, action="fix"),
    ])
    result = orch._beat_integrate(evaluated, EvolutionState(project="test"))
    assert "Queued" in result[0]


def test_multiple_cycles_archive(tmp_path):
    orch = EvolutionOrchestrator(
        project_dir=str(tmp_path),
        project_name="test",
        dry_run=True,
    )
    orch.run_cycle()
    orch.run_cycle()
    orch.run_cycle()

    archive = read_archive(orch.archive_path)
    assert len(archive) == 3
    assert archive[0]["cycle_id"] == 1
    assert archive[2]["cycle_id"] == 3
