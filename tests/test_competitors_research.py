"""Tests for competitor research/profiling module."""

from src.competitors.research import (
    CompetitorProfile,
    Feature,
    MoatScore,
    ThreatAssessment,
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

    def test_to_markdown_with_moats_and_threat(self):
        profile = CompetitorProfile(
            name="Rival",
            url="https://rival.com",
            revenue_model="SaaS",
            funding="$10M Series A",
            moats=[
                MoatScore("network_effects", 3, "Large community"),
                MoatScore("switching_costs", 2),
            ],
            threat=ThreatAssessment(
                market_overlap=4, growth_velocity=3,
                resource_asymmetry=4, technical_proximity=3,
                time_to_relevance_months=6,
            ),
            growth_signals=["3x star growth last quarter", "Hired 5 engineers"],
        )
        md = profile.to_markdown()
        assert "Revenue Model:" in md
        assert "Funding:" in md
        assert "Moat Analysis:" in md
        assert "network_effects: strong" in md
        assert "switching_costs: moderate" in md
        assert "Large community" in md
        assert "significant" in md  # threat level
        assert "6 months" in md
        assert "Growth Signals:" in md
        assert "3x star growth" in md


class TestThreatAssessment:
    def test_threat_score(self):
        t = ThreatAssessment(market_overlap=4, growth_velocity=4,
                            resource_asymmetry=4, technical_proximity=4)
        assert t.threat_score == 4.0
        assert t.threat_level == "existential"

    def test_significant_threat(self):
        t = ThreatAssessment(market_overlap=3, growth_velocity=3,
                            resource_asymmetry=3, technical_proximity=3)
        assert t.threat_level == "significant"

    def test_moderate_threat(self):
        t = ThreatAssessment(market_overlap=2, growth_velocity=2,
                            resource_asymmetry=2, technical_proximity=2)
        assert t.threat_level == "moderate"

    def test_low_threat(self):
        t = ThreatAssessment(market_overlap=1, growth_velocity=1,
                            resource_asymmetry=1, technical_proximity=1)
        assert t.threat_level == "low"

    def test_default(self):
        t = ThreatAssessment()
        assert t.threat_score == 1.0
        assert t.threat_level == "low"


class TestMoatScore:
    def test_basic(self):
        m = MoatScore("brand", 3, "Category leader")
        assert m.moat_type == "brand"
        assert m.score == 3


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

    def test_parse_revenue_model_and_funding(self):
        response = """Revenue Model: Open-core SaaS
Funding: $50M Series B"""
        profile = parse_profile_response("T", "https://t.com", response)
        assert profile.revenue_model == "Open-core SaaS"
        assert profile.funding == "$50M Series B"
