"""Tests for document normalization module."""

import os

from src.adoption.normalize import (
    NormalizeResult,
    generate_stub,
    inject_frontmatter,
    normalize_document,
)


class TestInjectFrontmatter:
    def test_adds_frontmatter(self):
        content = "# My Document\n\nSome content."
        result = inject_frontmatter(content, "My Document", "2026-03-24")
        assert result.startswith("---")
        assert 'title: "My Document"' in result
        assert "# My Document" in result

    def test_preserves_existing_frontmatter(self):
        content = "---\ntitle: Existing\n---\n# Doc"
        result = inject_frontmatter(content, "New Title")
        assert result == content  # Unchanged

    def test_default_date(self):
        result = inject_frontmatter("# Doc", "Doc")
        assert "last_updated:" in result


class TestGenerateStub:
    def test_generates_stub(self):
        stub = generate_stub("Business Plan", "docs/BUSINESS_PLAN.md", "high")
        assert "---" in stub
        assert "Business Plan" in stub
        assert "TODO" in stub
        assert "high" in stub
        assert 'migration_status: "stub"' in stub

    def test_default_priority(self):
        stub = generate_stub("Doc", "doc.md")
        assert "medium" in stub


class TestNormalizeDocument:
    def test_normalizes_and_moves(self, tmp_path):
        source = tmp_path / "old" / "readme.md"
        source.parent.mkdir()
        source.write_text("# My Project\n\nContent here.")

        dest = tmp_path / "docs" / "README.md"
        result = normalize_document(str(source), str(dest), "My Project")

        assert result.frontmatter_added is True
        assert result.content_preserved is True
        assert os.path.exists(str(dest))

        with open(str(dest)) as f:
            content = f.read()
        assert content.startswith("---")
        assert "My Project" in content

    def test_archives_original(self, tmp_path):
        source = tmp_path / "original.md"
        source.write_text("# Original\n\nContent.")

        dest = tmp_path / "normalized" / "doc.md"
        archive = tmp_path / "archive"
        result = normalize_document(str(source), str(dest), "Doc", str(archive))

        assert os.path.exists(str(archive / "original.md"))
        with open(str(archive / "original.md")) as f:
            assert f.read() == "# Original\n\nContent."

    def test_handles_unreadable_file(self, tmp_path):
        dest = tmp_path / "dest.md"
        result = normalize_document("/nonexistent/file.md", str(dest), "Title")
        assert result.content_preserved is False
        assert "Failed to read" in result.notes

    def test_preserves_existing_frontmatter(self, tmp_path):
        source = tmp_path / "with_fm.md"
        source.write_text("---\ntitle: Already There\n---\n# Doc\n\nContent.")

        dest = tmp_path / "out.md"
        result = normalize_document(str(source), str(dest), "Doc")
        assert result.frontmatter_added is False
