"""Tests for changelog generation."""

import os
from src.improvement.changelog import generate_changelog, write_changelog


def test_generate_changelog():
    # Uses current repo's git history
    content = generate_changelog(".")
    assert "# Changelog" in content
    assert "Unreleased" in content


def test_generate_changelog_empty(tmp_path):
    content = generate_changelog(str(tmp_path))
    assert "No changes" in content


def test_generate_changelog_with_version():
    content = generate_changelog(".", version="v1.0.0")
    assert "v1.0.0" in content


def test_write_changelog(tmp_path):
    # Create a git repo in tmp_path
    import subprocess
    subprocess.run(["git", "init"], cwd=str(tmp_path), capture_output=True)
    subprocess.run(["git", "commit", "--allow-empty", "-m", "feat: initial"], cwd=str(tmp_path), capture_output=True)

    path = write_changelog(str(tmp_path))
    assert os.path.exists(path)
    with open(path) as f:
        assert "# Changelog" in f.read()
