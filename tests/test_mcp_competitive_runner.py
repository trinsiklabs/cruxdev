"""Tests for setup_competitive_analysis MCP tool."""

import json
import os

import pytest

from src.mcp_server import init, setup_competitive_analysis


@pytest.fixture(autouse=True)
def setup_state(tmp_path):
    init(str(tmp_path / "state"))
    yield


class TestSetupCompetitiveAnalysis:
    def test_full_pipeline(self, tmp_path):
        competitors = json.dumps([
            {
                "name": "Rival",
                "url": "https://rival.com",
                "category": "official",
                "features": ["auto-fix", "static analysis"],
                "strengths": ["Fast"],
                "weaknesses": ["No convergence"],
                "pricing": "Free",
            },
            {
                "name": "Watcher",
                "url": "https://watcher.dev",
                "category": "watch",
                "features": ["monitoring"],
            },
        ])
        result = json.loads(setup_competitive_analysis(
            our_name="CruxDev",
            our_description="Autonomous convergence framework",
            our_category="AI coding tools",
            our_features="convergence engine,TDD gates,session bus",
            competitors_json=competitors,
            project_dir=str(tmp_path),
        ))
        assert result["total_competitors"] == 2
        assert result["official"] == 1
        assert result["total_gaps"] > 0
        assert result["comparison_pages_generated"] >= 1
        assert len(result["files_written"]) >= 1
        assert os.path.exists(os.path.join(str(tmp_path), "docs", "COMPETITORS.md"))

    def test_no_write(self, tmp_path):
        competitors = json.dumps([{"name": "A", "url": "u", "features": ["x"]}])
        result = json.loads(setup_competitive_analysis(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features="A",
            competitors_json=competitors,
            write_files=False,
        ))
        assert result["files_written"] == []

    def test_string_features_input(self):
        competitors = json.dumps([{"name": "A", "url": "u", "features": "x, y, z"}])
        result = json.loads(setup_competitive_analysis(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features="A, B",
            competitors_json=competitors,
            write_files=False,
        ))
        assert result["total_features_compared"] > 0

    def test_empty_competitors(self):
        result = json.loads(setup_competitive_analysis(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features="A",
            competitors_json="[]",
            write_files=False,
        ))
        assert result["total_competitors"] == 0
        assert result["total_gaps"] == 0

    def test_bad_json(self):
        result = json.loads(setup_competitive_analysis(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features="A",
            competitors_json="not json",
            write_files=False,
        ))
        assert result["total_competitors"] == 0
