"""Tests for research session."""

import os
from src.research.session import (
    ResearchFinding, ResearchSession, create_session,
    save_checkpoint, load_checkpoint, find_latest_checkpoint,
)


def test_create_session():
    s = create_session("attachment theory", ["What is attachment?", "Types?"])
    assert s.topic == "attachment theory"
    assert len(s.sub_questions) == 2
    assert len(s.session_id) == 8
    assert s.current_pass == 1
    assert not s.converged


def test_create_session_no_questions():
    s = create_session("topic")
    assert s.sub_questions == []


def test_save_and_load(tmp_path):
    s = create_session("test topic")
    s.findings.append(ResearchFinding(
        id="f1", content="Finding text", source_url="https://example.com",
        quality_score=0.8, tags=["tag1"],
    ))
    s.total_searches = 5
    s.novelty_scores = [0.8, 0.6, 0.4]

    path = save_checkpoint(s, str(tmp_path / "checkpoints"))
    loaded = load_checkpoint(path)

    assert loaded is not None
    assert loaded.topic == "test topic"
    assert loaded.total_searches == 5
    assert len(loaded.findings) == 1
    assert loaded.findings[0].content == "Finding text"


def test_load_missing():
    assert load_checkpoint("/nonexistent.json") is None


def test_load_corrupt(tmp_path):
    path = tmp_path / "bad.json"
    path.write_text("not json")
    assert load_checkpoint(str(path)) is None


def test_find_latest_checkpoint(tmp_path):
    cp_dir = str(tmp_path / "checkpoints")
    s1 = create_session("topic A")
    s1.started_at = 100.0
    save_checkpoint(s1, cp_dir)

    s2 = create_session("topic A")
    s2.started_at = 200.0
    save_checkpoint(s2, cp_dir)

    latest = find_latest_checkpoint(cp_dir, "topic A")
    assert latest is not None
    loaded = load_checkpoint(latest)
    assert loaded.started_at == 200.0


def test_find_no_checkpoint(tmp_path):
    assert find_latest_checkpoint(str(tmp_path), "missing") is None


def test_find_skips_converged(tmp_path):
    cp_dir = str(tmp_path / "cp")
    s = create_session("done topic")
    s.converged = True
    save_checkpoint(s, cp_dir)
    assert find_latest_checkpoint(cp_dir, "done topic") is None


def test_find_skips_non_json(tmp_path):
    cp_dir = str(tmp_path / "cp")
    os.makedirs(cp_dir)
    with open(os.path.join(cp_dir, "readme.txt"), "w") as f:
        f.write("not a checkpoint")
    s = create_session("topic")
    save_checkpoint(s, cp_dir)
    latest = find_latest_checkpoint(cp_dir, "topic")
    assert latest is not None  # Should find .json, skip .txt


def test_find_no_dir():
    assert find_latest_checkpoint("/nonexistent", "topic") is None
