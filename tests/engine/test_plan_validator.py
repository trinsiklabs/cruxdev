"""Tests for plan validator."""

import pytest

from src.engine.plan_validator import get_plan_template, validate_plan


def test_validate_missing_file():
    result = validate_plan("/nonexistent/plan.md")
    assert not result.valid
    assert "not found" in result.errors[0]


def test_validate_good_plan(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("""# BUILD_PLAN: Test
## Phase 1: Setup
- [ ] 1.1 Do the thing
- [ ] 1.2 Run pytest tests
## Document Alignment
- docs/DESIGN.md — design spec
## Convergence
Two consecutive clean passes.
""")
    result = validate_plan(str(plan))
    assert result.valid
    assert result.errors == []


def test_validate_no_title(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("Just some text without a heading\n- [ ] item\n")
    result = validate_plan(str(plan))
    assert "title" in result.errors[0].lower()


def test_validate_no_checklist(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\nSome text without checklists\n")
    result = validate_plan(str(plan))
    assert not result.valid
    assert any("checklist" in e.lower() for e in result.errors)


def test_validate_no_phases_warning(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Random Section\n- [ ] item\npytest\nconvergence\n")
    result = validate_plan(str(plan))
    assert any("phase" in w.lower() for w in result.warnings)


def test_validate_no_tests_warning(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [ ] item\nconvergence\n")
    result = validate_plan(str(plan))
    assert any("test" in w.lower() for w in result.warnings)


def test_validate_no_convergence_warning(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [ ] item\npytest\n")
    result = validate_plan(str(plan))
    assert any("convergence" in w.lower() for w in result.warnings)


def test_validate_too_short(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# P\n")
    result = validate_plan(str(plan))
    assert not result.valid
    assert any("short" in e.lower() for e in result.errors)


def test_validate_to_dict(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n- [ ] item\npytest\nconvergence\n")
    result = validate_plan(str(plan))
    d = result.to_dict()
    assert "valid" in d
    assert "errors" in d
    assert "warnings" in d


def test_get_plan_template():
    t = get_plan_template("Migrate to Python 3.14")
    assert "Migrate to Python 3.14" in t
    assert "- [ ]" in t
    assert "Phase 1" in t
    assert "convergence" in t.lower()
    assert "coverage" in t.lower()
    assert "Document Alignment" in t


def test_validate_checked_items(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Plan\n## Phase 1\n- [x] done item\n- [ ] todo item\npytest\nconvergence\n## Document Alignment\n- doc.md\n")
    result = validate_plan(str(plan))
    assert result.valid


def test_validate_missing_alignment_warns(tmp_path):
    plan = tmp_path / "plan.md"
    plan.write_text("# Build Plan for Testing Alignment\n\n## Phase 1: Setup\n\n- [ ] 1.1 Do the first task\n- [ ] 1.2 Run pytest to verify everything works\n\n## Convergence\n\nTwo consecutive clean passes required.\n")
    result = validate_plan(str(plan))
    assert result.valid  # Warning, not error — new projects may not have docs
    assert any("Document Alignment" in w for w in result.warnings)
