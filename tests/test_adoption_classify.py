"""Tests for project classification module."""

import os

from src.adoption.classify import (
    Classification,
    classify_project,
    _assess_maturity,
    _matches_pattern,
    _scan_entries,
)


class TestMatchesPattern:
    def test_directory_pattern(self):
        assert _matches_pattern("src/", "src/") is True
        assert _matches_pattern("lib/util/", "util/") is True

    def test_extension_pattern(self):
        assert _matches_pattern("main.py", "*.py") is True
        assert _matches_pattern("deep/module.py", "*.py") is True
        assert _matches_pattern("file.txt", "*.py") is False

    def test_exact_match(self):
        assert _matches_pattern("README.md", "README.md") is True
        assert _matches_pattern("docs/README.md", "README.md") is True

    def test_wildcard_prefix(self):
        assert _matches_pattern("astro.config.mjs", "astro.config.*") is True

    def test_no_match(self):
        assert _matches_pattern("random.txt", "setup.py") is False


class TestAssessMaturity:
    def test_mature(self):
        entries = ["contributing.md", "license", "security.md", "code_of_conduct.md"]
        assert _assess_maturity(entries) == "mature"

    def test_production(self):
        entries = [".github/workflows/", "dockerfile", "deployment.md"]
        assert _assess_maturity(entries) == "production"

    def test_growing(self):
        entries = ["tests/", "docs/", "changelog.md"]
        assert _assess_maturity(entries) == "growing"

    def test_minimal(self):
        entries = ["readme.md", "src/"]
        assert _assess_maturity(entries) == "minimal"

    def test_idea(self):
        entries = ["notes.txt"]
        assert _assess_maturity(entries) == "idea"


class TestClassifyProject:
    def test_software_project(self, tmp_path):
        (tmp_path / "src").mkdir()
        (tmp_path / "tests").mkdir()
        (tmp_path / "pyproject.toml").write_text("[build-system]")
        (tmp_path / "README.md").write_text("# My Project")

        result = classify_project(str(tmp_path))
        assert result.primary_type == "software-existing"
        assert result.maturity in ("minimal", "growing")

    def test_website_project(self, tmp_path):
        (tmp_path / "src" / "pages").mkdir(parents=True)
        (tmp_path / "public").mkdir()
        (tmp_path / "index.html").write_text("<html></html>")

        result = classify_project(str(tmp_path))
        assert "website" in result.all_types

    def test_greenfield_project(self, tmp_path):
        # Empty directory
        result = classify_project(str(tmp_path))
        assert result.primary_type == "software-greenfield"
        assert result.maturity == "idea"
        assert result.confidence == 0.3

    def test_composite_types(self, tmp_path):
        (tmp_path / "src").mkdir()
        (tmp_path / "tests").mkdir()
        (tmp_path / "Dockerfile").write_text("FROM python:3.12")
        (tmp_path / ".github" / "workflows").mkdir(parents=True)

        result = classify_project(str(tmp_path))
        assert len(result.all_types) >= 1

    def test_nonexistent_dir(self):
        result = classify_project("/nonexistent/path")
        assert result.primary_type == "software-greenfield"

    def test_all_types_property(self):
        c = Classification(
            primary_type="software-existing",
            secondary_types=["website", "infrastructure"],
        )
        assert c.all_types == ["software-existing", "website", "infrastructure"]


class TestScanEntries:
    def test_scans_files_and_dirs(self, tmp_path):
        (tmp_path / "src").mkdir()
        (tmp_path / "src" / "main.py").write_text("print('hello')")
        (tmp_path / "README.md").write_text("# Hello")

        entries = _scan_entries(str(tmp_path))
        assert any("src/" in e for e in entries)
        assert any("main.py" in e for e in entries)
        assert any("README.md" in e for e in entries)

    def test_skips_hidden(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / ".hidden_file").write_text("secret")

        entries = _scan_entries(str(tmp_path))
        assert not any(".git" in e for e in entries)
