"""Tests for competitive analysis runner — single-call orchestration."""

import json
import os

from src.competitors.runner import (
    AnalysisResult,
    CompetitorInput,
    parse_competitor_inputs,
    setup,
    write_results,
)


class TestSetup:
    def test_full_analysis(self):
        result = setup(
            our_name="CruxDev",
            our_description="Autonomous convergence framework",
            our_category="AI coding tools",
            our_features=["convergence engine", "TDD gates", "session bus"],
            competitors=[
                CompetitorInput(
                    name="Superpowers",
                    url="https://github.com/obra/superpowers",
                    category="official",
                    features=["context skills", "autonomous loops"],
                    strengths=["Lightweight"],
                    weaknesses=["No convergence detection"],
                ),
                CompetitorInput(
                    name="DeepSource",
                    url="https://deepsource.io",
                    category="official",
                    features=["static analysis", "auto-fix"],
                    pricing="Freemium",
                ),
                CompetitorInput(
                    name="WatchTool",
                    url="https://watchtool.dev",
                    category="watch",
                    features=["monitoring"],
                ),
            ],
        )
        assert result.our_name == "CruxDev"
        assert result.summary["total_competitors"] == 3
        assert result.summary["official"] == 2
        assert result.summary["watch"] == 1
        assert result.summary["comparison_pages_generated"] >= 2  # official + watch
        assert len(result.competitors_doc) > 100
        assert len(result.gap_analysis) > 10
        assert len(result.discovery_queries) > 0
        assert "Superpowers" in result.competitors_doc

    def test_with_overview(self):
        result = setup(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features=["A"],
            competitors=[],
            overview="Custom overview text.",
        )
        assert "Custom overview text." in result.competitors_doc

    def test_gap_detection(self):
        result = setup(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features=["feature_a"],
            competitors=[
                CompetitorInput("CompA", "u", "official", features=["feature_a", "feature_b"]),
                CompetitorInput("CompB", "u", "official", features=["feature_b", "feature_c"]),
            ],
        )
        gap_features = [g for g in result.summary if "gap" in g]
        assert result.summary["total_gaps"] > 0
        assert result.summary["must_close"] >= 1  # feature_b in 2 official

    def test_no_competitors(self):
        result = setup("Us", "desc", "cat", ["A"], [])
        assert result.summary["total_competitors"] == 0
        assert result.summary["total_gaps"] == 0

    def test_noted_competitors_no_comparison_page(self):
        result = setup(
            our_name="Us",
            our_description="tool",
            our_category="tools",
            our_features=["A"],
            competitors=[
                CompetitorInput("Noted", "u", "noted", features=["B"]),
            ],
        )
        assert result.summary["comparison_pages_generated"] == 0


class TestWriteResults:
    def test_writes_competitors_md(self, tmp_path):
        result = setup("Us", "desc", "cat", ["A"], [
            CompetitorInput("Comp", "u", "official", features=["B"]),
        ])
        files = write_results(result, str(tmp_path))
        assert len(files) >= 1
        assert os.path.exists(os.path.join(str(tmp_path), "docs", "COMPETITORS.md"))

    def test_writes_comparison_pages(self, tmp_path):
        result = setup("Us", "desc", "cat", ["A"], [
            CompetitorInput("Comp", "https://comp.com", "official", features=["B"]),
        ])
        files = write_results(result, str(tmp_path), vs_dir="vs")
        vs_files = [f for f in files if "/vs/" in f]
        assert len(vs_files) == 1

    def test_no_vs_dir_skips_pages(self, tmp_path):
        result = setup("Us", "desc", "cat", ["A"], [
            CompetitorInput("Comp", "u", "official", features=["B"]),
        ])
        files = write_results(result, str(tmp_path))
        assert len(files) == 1  # Only COMPETITORS.md


class TestParseCompetitorInputs:
    def test_json_string(self):
        raw = json.dumps([{"name": "A", "url": "u", "features": ["x"]}])
        result = parse_competitor_inputs(raw)
        assert len(result) == 1
        assert result[0].name == "A"
        assert result[0].features == ["x"]

    def test_dict_list(self):
        result = parse_competitor_inputs([{"name": "A", "url": "u"}])
        assert len(result) == 1

    def test_string_features(self):
        result = parse_competitor_inputs([{"name": "A", "features": "x, y, z"}])
        assert result[0].features == ["x", "y", "z"]

    def test_dict_features(self):
        result = parse_competitor_inputs([{"name": "A", "features": [{"name": "x"}, {"name": "y"}]}])
        assert result[0].features == ["x", "y"]

    def test_string_strengths(self):
        result = parse_competitor_inputs([{"name": "A", "strengths": "fast, reliable"}])
        assert result[0].strengths == ["fast", "reliable"]

    def test_string_weaknesses(self):
        result = parse_competitor_inputs([{"name": "A", "weaknesses": "slow, buggy"}])
        assert result[0].weaknesses == ["slow", "buggy"]

    def test_none(self):
        assert parse_competitor_inputs(None) == []

    def test_invalid_json(self):
        assert parse_competitor_inputs("not json") == []

    def test_non_list(self):
        assert parse_competitor_inputs(42) == []

    def test_non_dict_entries(self):
        result = parse_competitor_inputs(["not a dict", {"name": "A"}])
        assert len(result) == 1

    def test_defaults(self):
        result = parse_competitor_inputs([{}])
        assert result[0].name == "unknown"
        assert result[0].url == ""
        assert result[0].category == "noted"
        assert result[0].features == []
