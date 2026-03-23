"""Tests for Beat 4: Post."""

import os
import time

from src.evolution.post import (
    generate_changelog_entry,
    generate_cycle_summary,
    generate_x_post,
    save_post,
    PostContent,
)
from src.evolution.state import EvolutionCycle


def _make_cycle():
    return EvolutionCycle(
        cycle_id=1,
        started_at=time.time(),
        gathered=["5 changes, 2 issues, 1 inbox"],
        evaluated=[{"title": "Bug fix"}],
        integrated=["Fixed auth bug"],
        posted=[],
        engaged=["label: issue #1"],
    )


def test_generate_summary():
    cycle = _make_cycle()
    summary = generate_cycle_summary(cycle)
    assert "#1" in summary
    assert "Gathered" in summary


def test_generate_summary_with_error():
    cycle = _make_cycle()
    cycle.error = "timeout"
    summary = generate_cycle_summary(cycle)
    assert "Error" in summary


def test_generate_x_post():
    cycle = _make_cycle()
    post = generate_x_post(cycle, "cruxdev")
    assert post.post_type == "x_post"
    assert len(post.body) <= 280
    assert "cruxdev" in post.body


def test_generate_x_post_truncation():
    cycle = _make_cycle()
    # Project name long enough to push total over 280
    post = generate_x_post(cycle, "x" * 300)
    assert len(post.body) <= 280
    assert post.body.endswith("...")


def test_generate_changelog():
    cycle = _make_cycle()
    entry = generate_changelog_entry(cycle)
    assert entry.post_type == "changelog_entry"
    assert "Fixed auth bug" in entry.body


def test_save_post(tmp_path):
    post = PostContent(title="Test", body="Content", post_type="x_post")
    path = save_post(post, str(tmp_path / "posts"))
    assert os.path.exists(path)
    with open(path) as f:
        content = f.read()
    assert "Test" in content
    assert "Content" in content
