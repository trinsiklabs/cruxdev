"""Tests for competitor research/profiling module."""

from src.competitors.research import (
    CompetitorProfile,
    Feature,
    parse_profile_response,
)


class TestCompetitorProfile:
    def test_feature_names(self):
        profile = CompetitorProfile(
            name="Test", url="https://test.com",
            features=[
                Feature("A", "desc a", True),
                Feature("B", "desc b", False),
                Feature("C", "desc c", True),
            ]
        )
        assert profile.feature_names() == ["A", "C"]

    def test_to_markdown_basic(self):
        profile = CompetitorProfile(
            name="Acme", url="https://acme.com",
            tagline="Best tool ever",
            category="official",
            pricing="Free",
        )
        md = profile.to_markdown()
        assert "### Acme" in md
        assert "**URL:** https://acme.com" in md
        assert "**Tagline:** Best tool ever" in md
        assert "**Category:** official" in md
        assert "**Pricing:** Free" in md

    def test_to_markdown_full(self):
        profile = CompetitorProfile(
            name="Full",
            url="https://full.io",
            description="A comprehensive tool",
            tech_stack=["Python", "React"],
            strengths=["Fast", "Reliable"],
            weaknesses=["Expensive"],
            differentiation="Unique approach",
        )
        md = profile.to_markdown()
        assert "A comprehensive tool" in md
        assert "Python, React" in md
        assert "- Fast" in md
        assert "- Reliable" in md
        assert "- Expensive" in md
        assert "Unique approach" in md

    def test_to_markdown_empty(self):
        profile = CompetitorProfile(name="Min", url="https://min.com")
        md = profile.to_markdown()
        assert "### Min" in md
        assert "https://min.com" in md


class TestParseProfileResponse:
    def test_parse_full_response(self):
        response = """Tagline: The best tool
Description: A comprehensive solution
Category: official
Pricing: $10/month
Tech Stack: Python, TypeScript, React
Differentiation: Unique convergence approach

Strengths:
- Fast performance
- Easy to use

Weaknesses:
- Limited documentation

Features:
- Auto-testing: runs tests automatically
- Code review: AI-powered reviews"""

        profile = parse_profile_response("TestCo", "https://testco.com", response)
        assert profile.name == "TestCo"
        assert profile.url == "https://testco.com"
        assert profile.tagline == "The best tool"
        assert profile.description == "A comprehensive solution"
        assert profile.category == "official"
        assert profile.pricing == "$10/month"
        assert "Python" in profile.tech_stack
        assert "TypeScript" in profile.tech_stack
        assert profile.differentiation == "Unique convergence approach"
        assert "Fast performance" in profile.strengths
        assert "Easy to use" in profile.strengths
        assert "Limited documentation" in profile.weaknesses
        assert len(profile.features) == 2
        assert profile.features[0].name == "Auto-testing"
        assert profile.features[0].description == "runs tests automatically"

    def test_parse_minimal_response(self):
        response = "Tagline: Simple"
        profile = parse_profile_response("Min", "https://min.com", response)
        assert profile.tagline == "Simple"
        assert profile.strengths == []
        assert profile.features == []

    def test_parse_tech_stack_as_list(self):
        response = """Tech Stack:
- Python
- Go
- Rust"""
        profile = parse_profile_response("T", "https://t.com", response)
        assert "Python" in profile.tech_stack
        assert "Go" in profile.tech_stack
        assert "Rust" in profile.tech_stack

    def test_invalid_category_ignored(self):
        response = "Category: unknown_value"
        profile = parse_profile_response("T", "https://t.com", response)
        assert profile.category == "noted"  # default unchanged
