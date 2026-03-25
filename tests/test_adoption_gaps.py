"""Tests for gap analysis module."""

import os

from src.adoption.gaps import (
    Gap,
    GapAnalysis,
    analyze_gaps,
    _is_stub,
)
from src.adoption.inventory import Inventory, InventoryItem
from src.adoption.templates import Template, TemplateSet


class TestGapAnalysis:
    def test_critical_filter(self):
        ga = GapAnalysis(".", gaps=[
            Gap("A", "a.md", "critical"),
            Gap("B", "b.md", "high"),
            Gap("C", "c.md", "critical", status="closed"),
        ])
        assert len(ga.critical) == 1

    def test_open_gaps(self):
        ga = GapAnalysis(".", gaps=[
            Gap("A", "a.md", "critical"),
            Gap("B", "b.md", "high", status="closed"),
        ])
        assert len(ga.open_gaps) == 1

    def test_by_severity(self):
        ga = GapAnalysis(".", gaps=[
            Gap("A", "a.md", "critical"),
            Gap("B", "b.md", "high"),
            Gap("C", "c.md", "critical"),
            Gap("D", "d.md", "critical", status="closed"),  # should be excluded
        ])
        by_sev = ga.by_severity
        assert len(by_sev["critical"]) == 2
        assert len(by_sev["high"]) == 1

    def test_to_markdown(self):
        ga = GapAnalysis(".", gaps=[
            Gap("README", "README.md", "critical", reason="Missing"),
            Gap("License", "LICENSE", "high", status="not-applicable", justification="Not needed for internal project"),
        ])
        md = ga.to_markdown()
        assert "# GAPS" in md
        assert "**README**" in md
        assert "Missing" in md
        assert "Not Applicable" in md
        assert "Not needed for internal project" in md

    def test_to_markdown_empty(self):
        ga = GapAnalysis(".")
        md = ga.to_markdown()
        assert "# GAPS" in md


class TestIsStub:
    def test_stub_with_todo(self, tmp_path):
        f = tmp_path / "stub.md"
        f.write_text("# Title\n\nTODO: Fill this in\n\nSome content here that makes it long enough to pass length check but has TODO.")
        assert _is_stub(str(f)) is True

    def test_stub_too_short(self, tmp_path):
        f = tmp_path / "short.md"
        f.write_text("# Title")
        assert _is_stub(str(f)) is True

    def test_not_stub(self, tmp_path):
        f = tmp_path / "real.md"
        f.write_text("# Real Document\n\nThis is a comprehensive document with actual content that spans multiple paragraphs and contains real information.")
        assert _is_stub(str(f)) is False

    def test_nonexistent(self):
        assert _is_stub("/nonexistent/file.md") is False


class TestAnalyzeGaps:
    def test_detects_missing_document(self, tmp_path):
        inv = Inventory(str(tmp_path), items=[])
        ts = TemplateSet(templates=[
            Template("code", "README", "desc", "README.md", "R"),
        ])
        result = analyze_gaps(str(tmp_path), inv, ts)
        assert len(result.gaps) == 1
        assert result.gaps[0].severity == "critical"
        assert "does not exist" in result.gaps[0].reason

    def test_detects_stub_document(self, tmp_path):
        readme = tmp_path / "README.md"
        readme.write_text("# Title\n\nTODO: Write this document with actual content and make it comprehensive enough.")

        inv = Inventory(str(tmp_path), items=[
            InventoryItem("README.md", "markdown", 100),
        ])
        ts = TemplateSet(templates=[
            Template("code", "README", "desc", "README.md", "R"),
        ])
        result = analyze_gaps(str(tmp_path), inv, ts)
        assert len(result.gaps) == 1
        assert "stub" in result.gaps[0].reason

    def test_no_gaps_when_complete(self, tmp_path):
        readme = tmp_path / "README.md"
        readme.write_text("# My Project\n\nThis is a complete README with comprehensive documentation about the project goals, setup instructions, and usage guidelines.")

        inv = Inventory(str(tmp_path), items=[
            InventoryItem("README.md", "markdown", 500),
        ])
        ts = TemplateSet(templates=[
            Template("code", "README", "desc", "README.md", "R"),
        ])
        result = analyze_gaps(str(tmp_path), inv, ts)
        assert len(result.gaps) == 0

    def test_severity_mapping(self, tmp_path):
        inv = Inventory(str(tmp_path), items=[])
        ts = TemplateSet(templates=[
            Template("code", "A", "d", "a.md", "R"),
            Template("code", "B", "d", "b.md", "P"),
            Template("code", "C", "d", "c.md", "M"),
            Template("code", "D", "d", "d.md", "O"),
        ])
        result = analyze_gaps(str(tmp_path), inv, ts)
        severities = {g.template_name: g.severity for g in result.gaps}
        assert severities["A"] == "critical"
        assert severities["B"] == "high"
        assert severities["C"] == "medium"
        assert severities["D"] == "low"

    def test_finds_file_in_docs_subdir(self, tmp_path):
        (tmp_path / "docs").mkdir()
        readme = tmp_path / "docs" / "DEPLOYMENT.md"
        readme.write_text("# Deployment\n\nThis document describes the complete deployment process for production environments with comprehensive details.")

        inv = Inventory(str(tmp_path), items=[
            InventoryItem("docs/DEPLOYMENT.md", "markdown", 500),
        ])
        ts = TemplateSet(templates=[
            Template("website", "Deployment", "d", "docs/DEPLOYMENT.md", "R"),
        ])
        result = analyze_gaps(str(tmp_path), inv, ts)
        assert len(result.gaps) == 0
