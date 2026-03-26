"""Tests for plan status line management."""

from src.engine.plan_status import read_plan_status, update_plan_status


class TestUpdatePlanStatus:
    def test_updates_status(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** NOT STARTED\n")
        assert update_plan_status(str(plan), "IN PROGRESS") is True
        assert "IN PROGRESS" in plan.read_text()

    def test_updates_to_converged(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** IN PROGRESS\n")
        assert update_plan_status(str(plan), "CONVERGED") is True
        assert "CONVERGED" in plan.read_text()

    def test_updates_to_escalated(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** IN PROGRESS\n")
        assert update_plan_status(str(plan), "ESCALATED") is True
        assert "ESCALATED" in plan.read_text()

    def test_no_status_line(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\nNo status here\n")
        assert update_plan_status(str(plan), "IN PROGRESS") is False

    def test_file_not_found(self):
        assert update_plan_status("/nonexistent/plan.md", "IN PROGRESS") is False

    def test_invalid_status(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** NOT STARTED\n")
        assert update_plan_status(str(plan), "INVALID") is False

    def test_already_at_status(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** IN PROGRESS\n")
        assert update_plan_status(str(plan), "IN PROGRESS") is False

    def test_preserves_rest_of_file(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** NOT STARTED\n\n## Phase 1\n- [ ] task\n")
        update_plan_status(str(plan), "IN PROGRESS")
        content = plan.read_text()
        assert "## Phase 1" in content
        assert "- [ ] task" in content


class TestReadPlanStatus:
    def test_reads_status(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n**Status:** IN PROGRESS\n")
        assert read_plan_status(str(plan)) == "IN PROGRESS"

    def test_no_status_line(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("# Plan\n")
        assert read_plan_status(str(plan)) is None

    def test_file_not_found(self):
        assert read_plan_status("/nonexistent/plan.md") is None

    def test_unreadable_file(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("**Status:** X")
        plan.chmod(0o000)
        assert read_plan_status(str(plan)) is None
        plan.chmod(0o644)


class TestOSErrors:
    def test_update_unreadable(self, tmp_path):
        plan = tmp_path / "plan.md"
        plan.write_text("**Status:** NOT STARTED")
        plan.chmod(0o000)
        assert update_plan_status(str(plan), "IN PROGRESS") is False
        plan.chmod(0o644)

    def test_update_unwritable_dir(self, tmp_path):
        sub = tmp_path / "readonly"
        sub.mkdir()
        plan = sub / "plan.md"
        plan.write_text("**Status:** NOT STARTED")
        plan.chmod(0o444)
        result = update_plan_status(str(plan), "IN PROGRESS")
        plan.chmod(0o644)
        assert isinstance(result, bool)
