"""Tests for Beat 1: Gather."""

from src.evolution.gather import (
    GatherResult,
    gather_all,
    gather_own_changes,
    gather_inbox,
)


def test_gather_own_changes():
    # Current repo has git history
    changes = gather_own_changes(".")
    assert isinstance(changes, list)
    assert len(changes) > 0


def test_gather_own_changes_no_repo(tmp_path):
    changes = gather_own_changes(str(tmp_path))
    assert changes == []


def test_gather_inbox():
    messages = gather_inbox("nonexistent_project")
    assert isinstance(messages, list)


def test_gather_all(tmp_path):
    result = gather_all(str(tmp_path), "test_project")
    assert isinstance(result, GatherResult)
    assert isinstance(result.own_changes, list)
    assert isinstance(result.inbox_messages, list)


def test_gather_all_with_repo():
    result = gather_all(".", "cruxdev", github_repo="")
    assert result.ci_status == "unknown"  # No repo → unknown
