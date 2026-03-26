"""Tests for auto-discovery of doc files in convergence."""

import os

from src.engine.task_router import _auto_discover_docs


class TestAutoDiscoverDocs:
    def test_finds_docs_directory(self, tmp_path):
        # Create project structure
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "ARCHITECTURE.md").write_text("# Arch")
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy")
        plan = tmp_path / "build_plans" / "plan.md"
        plan.parent.mkdir()
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert len(result) >= 2
        assert any("ARCHITECTURE.md" in f for f in result)
        assert any("DEPLOYMENT.md" in f for f in result)

    def test_includes_readme(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DESIGN.md").write_text("# Design")
        (tmp_path / "README.md").write_text("# README")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert any("README.md" in f for f in result)

    def test_no_docs_dir_falls_back_to_plan(self, tmp_path):
        (tmp_path / ".git").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert result == [str(plan)]

    def test_empty_docs_dir_falls_back(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert result == [str(plan)]

    def test_nested_plan_finds_project_root(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "API.md").write_text("# API")
        plans = tmp_path / "build_plans"
        plans.mkdir()
        plan = plans / "BUILD_PLAN_001.md"
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert any("API.md" in f for f in result)

    def test_no_project_markers(self):
        # Use root filesystem where parent == project_root at /
        result = _auto_discover_docs("/plan.md")
        assert result == ["/plan.md"]

    def test_sorted_output(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "Z.md").write_text("z")
        (tmp_path / "docs" / "A.md").write_text("a")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _auto_discover_docs(str(plan))
        assert result == sorted(result)
