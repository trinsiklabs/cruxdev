"""Tests for MCP competitor tool wrappers."""

import json
import os

import pytest

from src.mcp_server import (
    discover_competitors,
    generate_comparison_page,
    generate_gap_analysis,
    generate_gap_build_plan,
    init,
    research_competitor,
    verify_competitor_links,
)


@pytest.fixture(autouse=True)
def setup(tmp_path):
    init(str(tmp_path / "state"))
    yield


class TestDiscoverCompetitors:
    def test_returns_queries(self):
        result = json.loads(discover_competitors("AI coding", "AI tools"))
        assert "queries" in result
        assert len(result["queries"]) > 0
        assert "instructions" in result


class TestResearchCompetitor:
    def test_parses_research_text(self):
        research = """Tagline: Best in class
Pricing: $10/mo
Strengths:
- Fast
- Reliable
Features:
- Auto-fix: fixes bugs automatically"""
        result = json.loads(research_competitor("Acme", "https://acme.com", research))
        assert result["name"] == "Acme"
        assert result["url"] == "https://acme.com"
        assert result["tagline"] == "Best in class"
        assert result["pricing"] == "$10/mo"
        assert len(result["strengths"]) == 2
        assert len(result["features"]) == 1
        assert "markdown" in result


class TestVerifyCompetitorLinks:
    def test_verifies_links(self, monkeypatch):
        from src.competitors import verification
        def mock_check(url, timeout=10):
            from src.competitors.verification import LinkCheckResult
            return LinkCheckResult(url, "ok", 200)
        monkeypatch.setattr(verification, "check_url", mock_check)

        result = json.loads(verify_competitor_links(
            "Test", "Visit https://example.com for more"
        ))
        assert result["competitor"] == "Test"
        assert result["links_total"] == 1
        assert result["links_ok"] == 1


class TestGenerateGapAnalysis:
    def test_invalid_json(self):
        result = json.loads(generate_gap_analysis("Us", "testing", "not valid json"))
        assert "error" in result

    def test_string_features(self):
        competitors = json.dumps([
            {"name": "Comp", "url": "u", "category": "official",
             "features": ["auto-test", "deploy"]},
        ])
        result = json.loads(generate_gap_analysis("Us", "testing", competitors))
        assert result["total_gaps"] > 0

    def test_non_dict_entries_skipped(self):
        competitors = json.dumps(["not a dict", {"name": "Real", "url": "u", "features": []}])
        result = json.loads(generate_gap_analysis("Us", "testing", competitors))
        assert "our_name" in result

    def test_runs_analysis(self):
        competitors = json.dumps([
            {
                "name": "CompA",
                "url": "https://a.com",
                "category": "official",
                "features": [
                    {"name": "testing", "has": True},
                    {"name": "deployment", "has": True},
                ],
            },
            {
                "name": "CompB",
                "url": "https://b.com",
                "category": "official",
                "features": [
                    {"name": "testing", "has": True},
                    {"name": "monitoring", "has": True},
                ],
            },
        ])
        result = json.loads(generate_gap_analysis(
            "Us", "testing,security", competitors
        ))
        assert result["our_name"] == "Us"
        assert result["total_features"] > 0
        assert "gaps" in result
        assert "markdown" in result


class TestGenerateComparisonPage:
    def test_generates_page(self):
        research = """Tagline: Great tool
Features:
- Testing: automated tests
Strengths:
- Fast"""
        result = json.loads(generate_comparison_page(
            "Us", "testing,deployment", "Comp", "https://comp.com", research
        ))
        assert result["slug"] == "vs-comp"
        assert result["title"] == "Us vs Comp"
        assert "markdown" in result


class TestGenerateGapBuildPlan:
    def test_generates_plan(self):
        result = json.loads(generate_gap_build_plan(
            plan_number=20,
            feature_name="Auto Testing",
            competitors_with_feature="Acme,BetaCo",
            priority="must-close",
            our_name="CruxDev",
            context="They use webhooks",
        ))
        assert "BUILD_PLAN_020" in result["filename"]
        assert "Auto Testing" in result["content"]
        assert "webhooks" in result["content"]

    def test_without_context(self):
        result = json.loads(generate_gap_build_plan(
            plan_number=5,
            feature_name="Feat",
            competitors_with_feature="X",
            priority="should-close",
            our_name="Us",
        ))
        assert "BUILD_PLAN_005" in result["filename"]
