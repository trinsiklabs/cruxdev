"""Tests for plan evolution module."""

from src.improvement.plan_evolution import (
    AttributeCorrelation,
    PlanAttribute,
    PlanOutcome,
    correlate_attributes,
    suggest_template_improvements,
)


class TestCorrelateAttributes:
    def test_basic_correlation(self):
        outcomes = [
            PlanOutcome("p1", [PlanAttribute("doc_alignment", True)], convergence_rounds=3),
            PlanOutcome("p2", [PlanAttribute("doc_alignment", True)], convergence_rounds=2),
            PlanOutcome("p3", [PlanAttribute("doc_alignment", False)], convergence_rounds=6),
        ]
        correlations = correlate_attributes(outcomes)
        assert len(correlations) == 1
        c = correlations[0]
        assert c.attribute_name == "doc_alignment"
        assert c.plans_with == 2
        assert c.plans_without == 1
        assert c.avg_rounds_with < c.avg_rounds_without

    def test_empty_outcomes(self):
        assert correlate_attributes([]) == []

    def test_multiple_attributes(self):
        outcomes = [
            PlanOutcome("p1", [
                PlanAttribute("checklist", True),
                PlanAttribute("tests_first", True),
            ], convergence_rounds=3),
            PlanOutcome("p2", [
                PlanAttribute("checklist", False),
                PlanAttribute("tests_first", True),
            ], convergence_rounds=5),
        ]
        correlations = correlate_attributes(outcomes)
        assert len(correlations) == 2


class TestAttributeCorrelation:
    def test_impact_negative(self):
        c = AttributeCorrelation("x", avg_rounds_with=3.0, avg_rounds_without=6.0)
        assert c.impact == -3.0  # reduces rounds

    def test_impact_positive(self):
        c = AttributeCorrelation("x", avg_rounds_with=8.0, avg_rounds_without=3.0)
        assert c.impact == 5.0  # increases rounds

    def test_impact_zero_without(self):
        c = AttributeCorrelation("x", avg_rounds_with=3.0, avg_rounds_without=0.0)
        assert c.impact == 0.0


class TestSuggestTemplateImprovements:
    def test_suggests_mandatory_for_beneficial(self):
        correlations = [
            AttributeCorrelation("doc_alignment", plans_with=3, plans_without=2,
                                avg_rounds_with=2.0, avg_rounds_without=6.0),
        ]
        suggestions = suggest_template_improvements(correlations)
        assert len(suggestions) == 1
        assert "mandatory" in suggestions[0].lower()

    def test_suggests_reconsider_for_harmful(self):
        correlations = [
            AttributeCorrelation("verbose_logging", plans_with=3, plans_without=2,
                                avg_rounds_with=8.0, avg_rounds_without=3.0),
        ]
        suggestions = suggest_template_improvements(correlations)
        assert len(suggestions) == 1
        assert "reconsider" in suggestions[0].lower()

    def test_no_suggestions_for_weak_correlation(self):
        correlations = [
            AttributeCorrelation("neutral", plans_with=3, plans_without=2,
                                avg_rounds_with=4.0, avg_rounds_without=4.5),
        ]
        suggestions = suggest_template_improvements(correlations)
        assert len(suggestions) == 0

    def test_requires_minimum_data(self):
        correlations = [
            AttributeCorrelation("low_data", plans_with=1, plans_without=1,
                                avg_rounds_with=2.0, avg_rounds_without=6.0),
        ]
        suggestions = suggest_template_improvements(correlations)
        assert len(suggestions) == 0  # Not enough data points
