"""Shared test fixtures for CruxDev."""

import pytest
from pathlib import Path


@pytest.fixture
def tmp_project(tmp_path):
    """Create a temporary project directory with CruxDev structure."""
    (tmp_path / "src").mkdir()
    (tmp_path / "tests").mkdir()
    (tmp_path / "engine").mkdir()
    (tmp_path / "skills").mkdir()
    (tmp_path / ".claude").mkdir()
    return tmp_path


@pytest.fixture
def tmp_state_dir(tmp_path):
    """Create a temporary directory for convergence state files."""
    state_dir = tmp_path / ".cruxdev"
    state_dir.mkdir()
    return state_dir
