"""Tests for research archive."""

import json
import os
import time

from src.research.archive import archive_session, is_stale, load_archive
from src.research.session import ResearchFinding, create_session


def _make_session():
    s = create_session("Test Topic")
    s.findings.append(ResearchFinding(
        id="f1", content="Key finding about test topic",
        source_url="https://example.com/paper",
        quality_score=0.85, robustness="robust",
        counter_evidence=["https://counter.com"],
        tags=["research", "test"],
    ))
    s.seen_urls = ["https://example.com/paper", "https://other.com"]
    s.total_searches = 15
    s.quality_score = 78.5
    s.converged = True
    return s


def test_archive_creates_files(tmp_path):
    s = _make_session()
    archive_dir = str(tmp_path / "archives")
    result_dir = archive_session(s, archive_dir)

    assert os.path.exists(os.path.join(result_dir, "research.md"))
    assert os.path.exists(os.path.join(result_dir, "sources.json"))
    assert os.path.exists(os.path.join(result_dir, "meta.json"))


def test_archive_research_md(tmp_path):
    s = _make_session()
    result_dir = archive_session(s, str(tmp_path))

    with open(os.path.join(result_dir, "research.md")) as f:
        content = f.read()
    assert "Test Topic" in content
    assert "Key finding" in content
    assert "example.com" in content
    assert "Counter-evidence" in content


def test_archive_sources_json(tmp_path):
    s = _make_session()
    result_dir = archive_session(s, str(tmp_path))

    with open(os.path.join(result_dir, "sources.json")) as f:
        sources = json.load(f)
    assert len(sources) == 1
    assert sources[0]["url"] == "https://example.com/paper"
    assert sources[0]["quality_score"] == 0.85


def test_archive_meta_json(tmp_path):
    s = _make_session()
    result_dir = archive_session(s, str(tmp_path))

    with open(os.path.join(result_dir, "meta.json")) as f:
        meta = json.load(f)
    assert meta["topic"] == "Test Topic"
    assert meta["total_searches"] == 15
    assert meta["converged"] is True


def test_load_archive(tmp_path):
    s = _make_session()
    archive_session(s, str(tmp_path))

    result = load_archive(str(tmp_path), "test-topic")
    assert result is not None
    assert "research" in result
    assert "sources" in result
    assert "meta" in result


def test_load_archive_missing(tmp_path):
    assert load_archive(str(tmp_path), "nonexistent") is None


def test_is_stale_fresh(tmp_path):
    s = _make_session()
    archive_session(s, str(tmp_path))
    assert is_stale(str(tmp_path), "test-topic", max_age_days=30) is False


def test_is_stale_old(tmp_path):
    s = _make_session()
    result_dir = archive_session(s, str(tmp_path))
    # Manually set archived_at to 60 days ago
    meta_path = os.path.join(result_dir, "meta.json")
    with open(meta_path) as f:
        meta = json.load(f)
    meta["archived_at"] = time.time() - (60 * 86400)
    with open(meta_path, "w") as f:
        json.dump(meta, f)
    assert is_stale(str(tmp_path), "test-topic", max_age_days=30) is True


def test_is_stale_missing(tmp_path):
    assert is_stale(str(tmp_path), "nonexistent") is True


def test_is_stale_corrupt(tmp_path):
    slug_dir = tmp_path / "bad-topic"
    slug_dir.mkdir()
    (slug_dir / "meta.json").write_text("not json")
    assert is_stale(str(tmp_path), "bad-topic") is True


def test_archive_budget_exhausted(tmp_path):
    s = _make_session()
    s.converged = False
    s.budget_exhausted = True
    result_dir = archive_session(s, str(tmp_path))
    with open(os.path.join(result_dir, "research.md")) as f:
        content = f.read()
    assert "budget exhausted" in content


def test_archive_in_progress(tmp_path):
    s = _make_session()
    s.converged = False
    s.budget_exhausted = False
    result_dir = archive_session(s, str(tmp_path))
    with open(os.path.join(result_dir, "research.md")) as f:
        content = f.read()
    assert "in progress" in content
