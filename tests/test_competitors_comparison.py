"""Tests for comparison page generation."""

from src.competitors.comparison_page import (
    ComparisonPage,
    generate_comparison_content,
    generate_slug,
)
from src.competitors.research import CompetitorProfile, Feature


class TestGenerateSlug:
    def test_simple_name(self):
        assert generate_slug("Acme") == "vs-acme"

    def test_multi_word(self):
        assert generate_slug("Super Tool") == "vs-super-tool"

    def test_dots_replaced(self):
        assert generate_slug("tool.io") == "vs-tool-io"


class TestComparisonPage:
    def test_to_markdown_with_frontmatter(self):
        page = ComparisonPage(
            competitor_name="Acme",
            our_name="Us",
            slug="vs-acme",
            title="Us vs Acme",
            meta_description="Compare Us and Acme",
            content="# Us vs Acme\n\nContent here.",
            features_compared=5,
        )
        md = page.to_markdown()
        assert "---" in md
        assert 'title: "Us vs Acme"' in md
        assert 'description: "Compare Us and Acme"' in md
        assert 'slug: "vs-acme"' in md
        assert "Content here." in md


class TestGenerateComparisonContent:
    def test_generates_feature_table(self):
        profile = CompetitorProfile(
            name="Acme", url="https://acme.com",
            tagline="The best",
            features=[Feature("testing", "auto tests", True)],
        )
        page = generate_comparison_content("Us", ["testing", "deployment"], profile)
        assert "## Feature Comparison" in page.content
        assert "| Us | Acme |" in page.content
        assert page.features_compared >= 2
        assert page.slug == "vs-acme"
        assert page.title == "Us vs Acme"

    def test_includes_strengths_weaknesses(self):
        profile = CompetitorProfile(
            name="Comp", url="https://comp.com",
            strengths=["Fast"],
            weaknesses=["Expensive"],
        )
        page = generate_comparison_content("Us", [], profile)
        assert "Comp Strengths" in page.content
        assert "- Fast" in page.content
        assert "Comp Weaknesses" in page.content
        assert "- Expensive" in page.content

    def test_includes_pricing(self):
        profile = CompetitorProfile(
            name="Comp", url="https://comp.com",
            pricing="$99/month",
        )
        page = generate_comparison_content("Us", [], profile)
        assert "$99/month" in page.content

    def test_includes_tagline(self):
        profile = CompetitorProfile(
            name="Comp", url="https://comp.com",
            tagline="We are great",
        )
        page = generate_comparison_content("Us", [], profile)
        assert "We are great" in page.content

    def test_no_features_no_table(self):
        profile = CompetitorProfile(name="Empty", url="https://e.com")
        page = generate_comparison_content("Us", [], profile)
        assert page.features_compared == 0

    def test_meta_description(self):
        profile = CompetitorProfile(name="Comp", url="https://comp.com")
        page = generate_comparison_content("Us", [], profile)
        assert "Compare Us and Comp" in page.meta_description
