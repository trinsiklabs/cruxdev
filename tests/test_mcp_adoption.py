"""Tests for MCP adoption tool wrappers."""

import json
import os

import pytest

from src.mcp_server import (
    analyze_gaps,
    classify_project,
    gap_status,
    get_templates,
    init,
    inventory_project,
)


@pytest.fixture(autouse=True)
def setup(tmp_path):
    init(str(tmp_path / "state"))
    yield


class TestClassifyProject:
    def test_classifies_software(self, tmp_path):
        (tmp_path / "src").mkdir()
        (tmp_path / "tests").mkdir()
        (tmp_path / "pyproject.toml").write_text("[build]")

        result = json.loads(classify_project(str(tmp_path)))
        assert result["primary_type"] == "software-existing"
        assert "maturity" in result
        assert "confidence" in result

    def test_classifies_empty(self, tmp_path):
        result = json.loads(classify_project(str(tmp_path)))
        assert result["primary_type"] == "software-greenfield"


class TestInventoryProject:
    def test_inventories(self, tmp_path):
        (tmp_path / "README.md").write_text("# Hi")
        (tmp_path / "main.py").write_text("print(1)")

        result = json.loads(inventory_project(str(tmp_path)))
        assert result["total_items"] == 2
        assert result["total_size"] > 0
        assert "markdown" in result


class TestGetTemplates:
    def test_returns_templates(self):
        result = json.loads(get_templates("software-existing", "production"))
        assert result["total"] > 0
        assert "templates" in result

    def test_minimal_has_fewer(self):
        minimal = json.loads(get_templates("software-existing", "minimal"))
        production = json.loads(get_templates("software-existing", "production"))
        assert minimal["total"] <= production["total"]


class TestAnalyzeGaps:
    def test_finds_gaps(self, tmp_path):
        result = json.loads(analyze_gaps(str(tmp_path)))
        assert result["total_gaps"] > 0
        assert "gaps" in result
        assert "markdown" in result

    def test_with_readme(self, tmp_path):
        (tmp_path / "README.md").write_text("# Project\n\nThis is a comprehensive README with lots of actual content describing the project.")
        result = json.loads(analyze_gaps(str(tmp_path)))
        gap_names = [g["name"] for g in result["gaps"]]
        assert "README" not in gap_names


class TestGapStatus:
    def test_returns_counts(self, tmp_path):
        result = json.loads(gap_status(str(tmp_path)))
        assert "critical" in result
        assert "high" in result
        assert "total_open" in result
