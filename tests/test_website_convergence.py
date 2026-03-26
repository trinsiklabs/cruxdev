"""Tests for website convergence phase — auto-detect + webapp E2E/UAT."""

import os

from src.engine.convergence import PHASE_ORDER
from src.engine.state import ConvergencePhase, ConvergenceState
from src.engine.task_router import _detect_webapp, _detect_website, get_next_task
from src.engine.persistence import save_state


class TestDetectWebsite:
    def test_no_website(self, tmp_path):
        (tmp_path / ".git").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is False

    def test_has_deployment_md(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy\nSite: https://cruxdev.dev")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is True
        assert any("DEPLOYMENT.md" in f for f in result["files"])
        assert "cruxdev.dev" in result["site_url"]

    def test_has_website_md(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "WEBSITE.md").write_text("# Website\nURL: https://example.com")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is True

    def test_extracts_url(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("Deploy to https://runcrux.io here")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert "runcrux.io" in result["site_url"]

    def test_no_url_in_docs(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy\nNo URL here")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is True
        assert result["site_url"] == ""

    def test_nested_plan(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy")
        plans = tmp_path / "build_plans"
        plans.mkdir()
        plan = plans / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is True

    def test_root_fallback(self):
        result = _detect_website("/plan.md")
        assert result["has_website"] is False

    def test_unreadable_file(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        deploy = tmp_path / "docs" / "DEPLOYMENT.md"
        deploy.write_text("# Deploy")
        # Make unreadable
        deploy.chmod(0o000)
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")

        result = _detect_website(str(plan))
        assert result["has_website"] is True
        assert result["site_url"] == ""
        # Restore permissions for cleanup
        deploy.chmod(0o644)


class TestDetectWebapp:
    def test_has_app_dir(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "app").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is True

    def test_has_dockerfile(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "Dockerfile").write_text("FROM python:3.12")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is True

    def test_has_e2e_patterns(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "E2E_TEST_PATTERNS.md").write_text("# E2E")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is True

    def test_has_uat_patterns(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "UAT_TEST_PATTERNS.md").write_text("# UAT")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is True

    def test_static_site_not_webapp(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is False

    def test_root_fallback(self):
        assert _detect_webapp("/plan.md") is False

    def test_deep_path_no_markers(self, tmp_path):
        # Deeper than 5 levels, no project markers → walks up and stops
        deep = tmp_path / "a" / "b" / "c" / "d" / "e" / "f" / "g"
        deep.mkdir(parents=True)
        plan = deep / "plan.md"
        plan.write_text("# Plan")
        assert _detect_webapp(str(plan)) is False


class TestWebsiteConvergencePhase:
    def test_phase_in_order(self):
        assert ConvergencePhase.WEBSITE_CONVERGENCE in PHASE_ORDER
        # Must be after DOC_AUDITING and before E2E_TESTING
        idx = PHASE_ORDER.index(ConvergencePhase.WEBSITE_CONVERGENCE)
        assert PHASE_ORDER[idx - 1] == ConvergencePhase.DOC_AUDITING
        assert PHASE_ORDER[idx + 1] == ConvergencePhase.E2E_TESTING

    def test_skips_when_no_website(self, tmp_path):
        (tmp_path / ".git").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n- [ ] task")
        state = ConvergenceState(plan_file=str(plan))
        state.phase = ConvergencePhase.WEBSITE_CONVERGENCE
        state_path = str(tmp_path / "state.json")
        save_state(state, state_path)

        task = get_next_task(state, state_path)
        # Should have skipped to E2E_TESTING
        assert state.phase == ConvergencePhase.E2E_TESTING

    def test_audits_when_website_exists(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy to https://site.dev")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n- [ ] task")
        state = ConvergenceState(plan_file=str(plan))
        state.phase = ConvergencePhase.WEBSITE_CONVERGENCE
        state_path = str(tmp_path / "state.json")
        save_state(state, state_path)

        task = get_next_task(state, state_path)
        assert task.task_type == "audit"
        assert "Website convergence" in task.description
        assert "DEPLOYMENT.md" in task.description

    def test_webapp_includes_e2e_uat(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy")
        (tmp_path / "docs" / "E2E_TEST_PATTERNS.md").write_text("# E2E")
        (tmp_path / "app").mkdir()
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n- [ ] task")
        state = ConvergenceState(plan_file=str(plan))
        state.phase = ConvergencePhase.WEBSITE_CONVERGENCE
        state_path = str(tmp_path / "state.json")
        save_state(state, state_path)

        task = get_next_task(state, state_path)
        assert "WEBAPP" in task.description
        assert "E2E_TEST_PATTERNS" in task.description
        assert "UAT_TEST_PATTERNS" in task.description
        assert "e2e_testing" in task.dimensions
        assert "uat_testing" in task.dimensions

    def test_website_converges_then_advances(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / "docs").mkdir()
        (tmp_path / "docs" / "DEPLOYMENT.md").write_text("# Deploy")
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n- [ ] task")
        state = ConvergenceState(plan_file=str(plan))
        state.phase = ConvergencePhase.WEBSITE_CONVERGENCE
        state.consecutive_clean = 2  # Already converged
        state_path = str(tmp_path / "state.json")
        save_state(state, state_path)

        task = get_next_task(state, state_path)
        # Should advance to E2E_TESTING
        assert state.phase == ConvergencePhase.E2E_TESTING
