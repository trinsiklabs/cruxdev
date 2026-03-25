"""Tests for template registry module."""

from src.adoption.templates import (
    Template,
    TemplateSet,
    TEMPLATES,
    TYPE_CATEGORIES,
    get_folder_structure,
    get_templates_for_type,
)


class TestTemplateSet:
    def test_required_filter(self):
        ts = TemplateSet(templates=[
            Template("code", "README", "desc", "README.md", "R"),
            Template("code", "CHANGELOG", "desc", "CHANGELOG.md", "P"),
            Template("code", "CONTRIBUTING", "desc", "CONTRIBUTING.md", "M"),
        ])
        assert len(ts.required) == 1
        assert ts.required[0].name == "README"

    def test_production_filter(self):
        ts = TemplateSet(templates=[
            Template("code", "README", "desc", "README.md", "R"),
            Template("code", "CHANGELOG", "desc", "CHANGELOG.md", "P"),
            Template("code", "CONTRIBUTING", "desc", "CONTRIBUTING.md", "M"),
        ])
        assert len(ts.production) == 2

    def test_by_category(self):
        ts = TemplateSet(templates=[
            Template("code", "README", "d", "README.md", "R"),
            Template("business", "Budget", "d", "docs/BUDGET.md", "P"),
        ])
        cats = ts.by_category
        assert "code" in cats
        assert "business" in cats


class TestGetTemplatesForType:
    def test_software_existing(self):
        ts = get_templates_for_type("software-existing", "production")
        names = [t.name for t in ts.templates]
        assert "README" in names
        assert "CLAUDE.md" in names
        assert "GAPS" in names

    def test_minimal_maturity_only_required(self):
        ts = get_templates_for_type("software-existing", "minimal")
        assert all(t.requirement == "R" for t in ts.templates)

    def test_growing_includes_production(self):
        ts = get_templates_for_type("software-existing", "growing")
        requirements = {t.requirement for t in ts.templates}
        assert "R" in requirements

    def test_production_includes_more(self):
        ts_min = get_templates_for_type("software-existing", "minimal")
        ts_prod = get_templates_for_type("software-existing", "production")
        assert len(ts_prod.templates) >= len(ts_min.templates)

    def test_website_type(self):
        ts = get_templates_for_type("website", "production")
        names = [t.name for t in ts.templates]
        assert "Deployment" in names

    def test_research_type(self):
        ts = get_templates_for_type("research", "production")
        names = [t.name for t in ts.templates]
        assert "Research Plan" in names

    def test_unknown_type(self):
        ts = get_templates_for_type("unknown-type", "minimal")
        # Should default to code + governance
        assert len(ts.templates) > 0


class TestGetFolderStructure:
    def test_software(self):
        dirs = get_folder_structure("software-existing")
        assert "src/" in dirs
        assert "tests/" in dirs
        assert "docs/" in dirs

    def test_website(self):
        dirs = get_folder_structure("website")
        assert "public/" in dirs

    def test_research(self):
        dirs = get_folder_structure("research")
        assert "data/" in dirs

    def test_unknown_gets_base(self):
        dirs = get_folder_structure("unknown")
        assert "docs/" in dirs
        assert "GAPS.md" in dirs


class TestTemplateRegistry:
    def test_all_templates_have_required_fields(self):
        for t in TEMPLATES:
            assert t.category
            assert t.name
            assert t.filename
            assert t.requirement in ("R", "P", "M", "O")

    def test_type_categories_cover_all_types(self):
        expected_types = [
            "software-existing", "software-greenfield", "business-existing",
            "business-new", "product-saas", "website", "infrastructure",
            "consulting-client", "research", "campaign",
        ]
        for t in expected_types:
            assert t in TYPE_CATEGORIES, f"Missing type: {t}"
