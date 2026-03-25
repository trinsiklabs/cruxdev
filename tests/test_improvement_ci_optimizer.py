"""Tests for CI optimizer module."""

from src.improvement.ci_optimizer import (
    CIAnalysis,
    OptimizationSuggestion,
    WorkflowStep,
    analyze_workflow,
)


class TestAnalyzeWorkflow:
    def test_detects_cacheable_steps(self):
        steps = [
            WorkflowStep("Install dependencies", 45.0, cacheable=False),
            WorkflowStep("Run tests", 30.0),
        ]
        analysis = analyze_workflow(steps)
        cache_suggestions = [s for s in analysis.suggestions if s.suggestion_type == "cache"]
        assert len(cache_suggestions) == 1
        assert "Install" in cache_suggestions[0].step_name

    def test_detects_parallelizable_steps(self):
        steps = [
            WorkflowStep("Unit tests", 60.0, parallelizable=True),
        ]
        analysis = analyze_workflow(steps)
        par_suggestions = [s for s in analysis.suggestions if s.suggestion_type == "parallelize"]
        assert len(par_suggestions) == 1

    def test_detects_slow_steps(self):
        steps = [
            WorkflowStep("Build", 180.0),
        ]
        analysis = analyze_workflow(steps)
        slow_suggestions = [s for s in analysis.suggestions if s.suggestion_type == "optimize"]
        assert len(slow_suggestions) == 1
        assert "180s" in slow_suggestions[0].description

    def test_no_suggestions_for_fast_cached_steps(self):
        steps = [
            WorkflowStep("Quick step", 5.0, cacheable=True),
        ]
        analysis = analyze_workflow(steps)
        assert len(analysis.suggestions) == 0

    def test_total_duration(self):
        steps = [
            WorkflowStep("A", 10.0),
            WorkflowStep("B", 20.0),
        ]
        analysis = analyze_workflow(steps)
        assert analysis.total_duration == 30.0

    def test_potential_savings(self):
        steps = [
            WorkflowStep("Install deps", 100.0, cacheable=False),
        ]
        analysis = analyze_workflow(steps)
        assert analysis.potential_savings > 0

    def test_empty_workflow(self):
        analysis = analyze_workflow([])
        assert analysis.total_duration == 0.0
        assert len(analysis.suggestions) == 0

    def test_short_parallelizable_ignored(self):
        steps = [
            WorkflowStep("Quick", 10.0, parallelizable=True),
        ]
        analysis = analyze_workflow(steps)
        par_suggestions = [s for s in analysis.suggestions if s.suggestion_type == "parallelize"]
        assert len(par_suggestions) == 0  # Too short to parallelize

    def test_setup_keyword_detected(self):
        steps = [
            WorkflowStep("Setup environment", 50.0),
        ]
        analysis = analyze_workflow(steps)
        cache_suggestions = [s for s in analysis.suggestions if s.suggestion_type == "cache"]
        assert len(cache_suggestions) == 1
