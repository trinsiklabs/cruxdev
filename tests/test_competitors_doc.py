"""Tests for COMPETITORS.md read/write."""

from src.competitors.competitors_doc import (
    CompetitorsDoc,
    generate_competitors_doc,
    parse_competitors_doc,
    update_competitor_section,
)
from src.competitors.gap_analysis import (
    FeatureGap,
    FeatureMatrixEntry,
    GapAnalysisResult,
)
from src.competitors.research import CompetitorProfile


SAMPLE_DOC = """# Competitors Analysis

Overview of the competitive landscape.

## Official Competitors

### Acme
**URL:** https://acme.com
**Tagline:** The original
**Category:** official
**Pricing:** Free

### BetaCo
**URL:** https://betaco.io
**Category:** official

## Watch Competitors

### Gamma
**URL:** https://gamma.dev
**Category:** watch
"""


class TestParseCompetitorsDoc:
    def test_parse_title(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        assert doc.title == "Competitors Analysis"

    def test_parse_profiles(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        assert len(doc.profiles) == 3

    def test_parse_profile_fields(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        acme = doc.find_profile("Acme")
        assert acme is not None
        assert acme.url == "https://acme.com"
        assert acme.tagline == "The original"
        assert acme.category == "official"
        assert acme.pricing == "Free"

    def test_find_profile_case_insensitive(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        assert doc.find_profile("acme") is not None
        assert doc.find_profile("ACME") is not None

    def test_find_profile_not_found(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        assert doc.find_profile("NonExistent") is None

    def test_parse_empty_doc(self):
        doc = parse_competitors_doc("")
        assert len(doc.profiles) == 0

    def test_raw_content_preserved(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        assert doc.raw_content == SAMPLE_DOC


class TestGenerateCompetitorsDoc:
    def test_generates_title(self):
        doc = generate_competitors_doc("My Competitors", "", [])
        assert "# My Competitors" in doc

    def test_generates_overview(self):
        doc = generate_competitors_doc("Title", "This is the overview.", [])
        assert "This is the overview." in doc

    def test_groups_by_category(self):
        profiles = [
            CompetitorProfile(name="A", url="u", category="official"),
            CompetitorProfile(name="B", url="u", category="watch"),
            CompetitorProfile(name="C", url="u", category="noted"),
        ]
        doc = generate_competitors_doc("Title", "", profiles)
        assert "## Official Competitors" in doc
        assert "## Watch Competitors" in doc
        assert "## Noted Competitors" in doc

    def test_includes_gap_analysis(self):
        gap_result = GapAnalysisResult(
            our_name="Us",
            gaps=[FeatureGap("X", ["A"], "must-close")],
        )
        doc = generate_competitors_doc("Title", "", [], gap_result)
        assert "## Gap Analysis" in doc
        assert "Must-Close" in doc

    def test_no_empty_category_sections(self):
        profiles = [
            CompetitorProfile(name="A", url="u", category="official"),
        ]
        doc = generate_competitors_doc("Title", "", profiles)
        assert "## Official Competitors" in doc
        assert "## Watch Competitors" not in doc


class TestUpdateCompetitorSection:
    def test_update_existing(self):
        updated = CompetitorProfile(name="Acme", url="https://acme.com/new", tagline="Updated")
        result = update_competitor_section(SAMPLE_DOC, updated)
        assert "https://acme.com/new" in result
        assert "**Tagline:** Updated" in result
        # BetaCo should still be there
        assert "BetaCo" in result

    def test_add_new_to_existing_category(self):
        new_profile = CompetitorProfile(name="Delta", url="https://delta.com", category="official")
        result = update_competitor_section(SAMPLE_DOC, new_profile)
        assert "### Delta" in result
        assert "https://delta.com" in result

    def test_add_to_missing_category(self):
        new_profile = CompetitorProfile(name="New", url="https://new.com", category="noted")
        # SAMPLE_DOC doesn't have "## Noted Competitors"
        result = update_competitor_section(SAMPLE_DOC, new_profile)
        assert "### New" in result

    def test_add_to_last_category_section(self):
        # Doc where the matching category is the last ## section (no next ## after it)
        doc = """# Competitors

## Watch Competitors

### Existing
**URL:** https://existing.com
**Category:** watch
"""
        new_profile = CompetitorProfile(name="New", url="https://new.com", category="watch")
        result = update_competitor_section(doc, new_profile)
        assert "### New" in result
        assert "https://new.com" in result

    def test_roundtrip(self):
        doc = parse_competitors_doc(SAMPLE_DOC)
        regenerated = generate_competitors_doc(doc.title, "", doc.profiles)
        doc2 = parse_competitors_doc(regenerated)
        assert len(doc2.profiles) == len(doc.profiles)
        for p1, p2 in zip(doc.profiles, doc2.profiles):
            assert p1.name == p2.name
