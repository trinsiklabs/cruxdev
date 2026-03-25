"""Tests for prompt A/B testing module."""

from src.improvement.prompt_ab import (
    ABTestResult,
    PromptVariant,
    compare_variants,
)


class TestCompareVariants:
    def test_b_wins(self):
        a = PromptVariant("a", "baseline", "prompt a", {"accuracy": 0.7, "speed": 0.8})
        b = PromptVariant("b", "challenger", "prompt b", {"accuracy": 0.9, "speed": 0.9})
        result = compare_variants(a, b)
        assert result.winner == "b"
        assert result.safe_to_deploy is True

    def test_a_wins(self):
        a = PromptVariant("a", "baseline", "prompt a", {"accuracy": 0.9, "speed": 0.9})
        b = PromptVariant("b", "challenger", "prompt b", {"accuracy": 0.7, "speed": 0.7})
        result = compare_variants(a, b)
        assert result.winner == "a"
        assert result.safe_to_deploy is False

    def test_tie(self):
        a = PromptVariant("a", "A", "p", {"x": 0.5, "y": 0.5})
        b = PromptVariant("b", "B", "p", {"x": 0.5, "y": 0.5})
        result = compare_variants(a, b)
        assert result.winner == "tie"

    def test_mixed_results(self):
        a = PromptVariant("a", "A", "p", {"accuracy": 0.9, "speed": 0.5})
        b = PromptVariant("b", "B", "p", {"accuracy": 0.7, "speed": 0.9})
        result = compare_variants(a, b)
        assert result.winner == "tie"  # 1 win each

    def test_critical_dimension_blocks_deploy(self):
        a = PromptVariant("a", "A", "p", {"accuracy": 0.9, "speed": 0.3, "quality": 0.3})
        b = PromptVariant("b", "B", "p", {"accuracy": 0.7, "speed": 0.9, "quality": 0.9})
        result = compare_variants(a, b, critical_dimensions=["accuracy"])
        assert result.winner == "b"
        assert result.safe_to_deploy is False  # b loses on critical "accuracy"

    def test_safe_deploy_no_critical_regression(self):
        a = PromptVariant("a", "A", "p", {"accuracy": 0.8, "speed": 0.5})
        b = PromptVariant("b", "B", "p", {"accuracy": 0.9, "speed": 0.7})
        result = compare_variants(a, b, critical_dimensions=["accuracy"])
        assert result.safe_to_deploy is True

    def test_summary(self):
        a = PromptVariant("a", "Baseline", "p", {"x": 0.5})
        b = PromptVariant("b", "New", "p", {"x": 0.9})
        result = compare_variants(a, b)
        assert "New wins" in result.summary

    def test_summary_tie(self):
        a = PromptVariant("a", "A", "p", {"x": 0.5})
        b = PromptVariant("b", "B", "p", {"x": 0.5})
        result = compare_variants(a, b)
        assert "Tie" in result.summary
