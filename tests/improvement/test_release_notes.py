"""Tests for release notes generation."""

from src.improvement.release_notes import (
    CommitInfo,
    categorize_commit,
    generate_release_notes,
    parse_git_log,
)


def test_categorize_feature():
    assert categorize_commit("feat: add new thing") == "feature"


def test_categorize_fix():
    assert categorize_commit("fix: broken stuff") == "fix"


def test_categorize_docs():
    assert categorize_commit("docs: update readme") == "docs"


def test_categorize_refactor():
    assert categorize_commit("refactor: clean up code") == "refactor"


def test_categorize_test():
    assert categorize_commit("test: add more tests") == "test"


def test_categorize_chore():
    assert categorize_commit("chore: bump deps") == "chore"


def test_categorize_ci():
    assert categorize_commit("ci: fix pipeline") == "chore"


def test_categorize_breaking():
    assert categorize_commit("BREAKING: removed API") == "breaking"


def test_categorize_unknown():
    assert categorize_commit("random commit message") == "chore"


def test_generate_empty():
    notes = generate_release_notes("v1.0.0", [])
    assert "v1.0.0" in notes
    assert "No changes" in notes


def test_generate_with_commits():
    commits = [
        CommitInfo(hash="abc1234", message="feat: add auth", category="feature"),
        CommitInfo(hash="def5678", message="fix: login bug", category="fix"),
        CommitInfo(hash="ghi9012", message="docs: update readme", category="docs"),
    ]
    notes = generate_release_notes("v2.0.0", commits)
    assert "v2.0.0" in notes
    assert "Features" in notes
    assert "Bug Fixes" in notes
    assert "Documentation" in notes
    assert "abc1234"[:7] in notes


def test_generate_breaking():
    commits = [
        CommitInfo(hash="xyz1234", message="BREAKING: remove old API", category="breaking"),
    ]
    notes = generate_release_notes("v3.0.0", commits)
    assert "Breaking Changes" in notes


def test_parse_git_log():
    commits = parse_git_log(".")
    assert isinstance(commits, list)
    assert len(commits) > 0
    assert commits[0].hash
    assert commits[0].message


def test_parse_git_log_with_tag():
    commits = parse_git_log(".", since_tag="nonexistent_tag_xyz")
    assert commits == []


def test_parse_git_log_not_a_repo(tmp_path):
    commits = parse_git_log(str(tmp_path))
    assert commits == []
