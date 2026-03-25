"""Tests for build plan generation from competitive gaps."""

from src.competitors.build_plan_generator import (
    GeneratedPlan,
    generate_gap_plan,
    generate_plan_filename,
)
from src.competitors.gap_analysis import FeatureGap


class TestGeneratePlanFilename:
    def test_basic_filename(self):
        gap = FeatureGap("Auto Testing", ["Acme"], "must-close")
        filename = generate_plan_filename(12, gap)
        assert filename == "BUILD_PLAN_012_GAP_ACME_AUTO_TESTING.md"

    def test_with_hyphens(self):
        gap = FeatureGap("real-time sync", ["Tool-X"], "should-close")
        filename = generate_plan_filename(5, gap)
        assert "TOOL_X" in filename
        assert "REAL_TIME_SYNC" in filename

    def test_padding(self):
        gap = FeatureGap("feat", ["C"], "must-close")
        assert generate_plan_filename(1, gap).startswith("BUILD_PLAN_001_")
        assert generate_plan_filename(99, gap).startswith("BUILD_PLAN_099_")
        assert generate_plan_filename(100, gap).startswith("BUILD_PLAN_100_")

    def test_no_competitors_uses_unknown(self):
        gap = FeatureGap("feat", [], "must-close")
        filename = generate_plan_filename(1, gap)
        assert "UNKNOWN" in filename


class TestGenerateGapPlan:
    def test_generates_valid_plan(self):
        gap = FeatureGap("Auto Testing", ["Acme", "BetaCo"], "must-close",
                         rationale="Critical for adoption")
        plan = generate_gap_plan(12, gap, "CruxDev")
        assert plan.filename == "BUILD_PLAN_012_GAP_ACME_AUTO_TESTING.md"
        assert "# BUILD_PLAN_012" in plan.content
        assert "Auto Testing" in plan.content
        assert "Acme" in plan.content
        assert "BetaCo" in plan.content
        assert "CruxDev" in plan.content
        assert "must-close" in plan.content
        assert "Critical for adoption" in plan.content
        assert "- [ ]" in plan.content
        assert "100%" in plan.content

    def test_includes_document_alignment(self):
        gap = FeatureGap("feat", ["Comp"], "should-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert "## Document Alignment" in plan.content
        assert "COMPETITORS.md" in plan.content

    def test_includes_post_execution(self):
        gap = FeatureGap("feat", ["Comp"], "should-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert "Post-Execution Convergence" in plan.content
        assert "Documentation convergence" in plan.content
        assert "Website convergence" in plan.content

    def test_includes_context(self):
        gap = FeatureGap("feat", ["Comp"], "must-close")
        plan = generate_gap_plan(1, gap, "Us", context="They use webhooks for this")
        assert "They use webhooks for this" in plan.content
        assert "Implementation notes" in plan.content

    def test_no_context_no_section(self):
        gap = FeatureGap("feat", ["Comp"], "must-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert "Implementation notes" not in plan.content

    def test_single_competitor_grammar(self):
        gap = FeatureGap("feat", ["Comp"], "must-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert "Comp has this feature" in plan.content

    def test_multiple_competitors_grammar(self):
        gap = FeatureGap("feat", ["A", "B"], "must-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert "A, B have this feature" in plan.content

    def test_gap_reference_preserved(self):
        gap = FeatureGap("feat", ["Comp"], "must-close")
        plan = generate_gap_plan(1, gap, "Us")
        assert plan.gap is gap
