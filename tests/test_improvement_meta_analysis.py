"""Tests for meta-analysis module."""

from src.improvement.meta_analysis import (
    ConvergenceDataPoint,
    MethodologyInsight,
    analyze_convergence_history,
)


class TestAnalyzeConvergenceHistory:
    def test_detects_high_escalation_rate(self):
        data = [
            ConvergenceDataPoint("p1", 5, 10, 100, escalated=True),
            ConvergenceDataPoint("p2", 5, 8, 90, escalated=True),
            ConvergenceDataPoint("p3", 3, 5, 60, escalated=False),
        ]
        insights = analyze_convergence_history(data)
        assert any(i.element == "max_rounds" for i in insights)

    def test_detects_simple_plans(self):
        data = [
            ConvergenceDataPoint("p1", 1, 2, 30),
            ConvergenceDataPoint("p2", 1, 1, 20),
        ]
        insights = analyze_convergence_history(data)
        assert any(i.element == "plan_granularity" for i in insights)

    def test_detects_high_findings_rate(self):
        data = [
            ConvergenceDataPoint("p1", 3, 50, 100),
            ConvergenceDataPoint("p2", 2, 40, 80),
        ]
        insights = analyze_convergence_history(data)
        assert any(i.element == "audit_thoroughness" for i in insights)

    def test_empty_history(self):
        assert analyze_convergence_history([]) == []

    def test_normal_convergence(self):
        data = [
            ConvergenceDataPoint("p1", 3, 5, 60, phases_used=["plan", "code", "doc"]),
            ConvergenceDataPoint("p2", 4, 8, 80, phases_used=["plan", "code"]),
            ConvergenceDataPoint("p3", 3, 6, 70, phases_used=["plan", "code", "doc"]),
        ]
        insights = analyze_convergence_history(data)
        # No escalation, reasonable rounds, moderate findings
        assert not any(i.element == "max_rounds" for i in insights)
