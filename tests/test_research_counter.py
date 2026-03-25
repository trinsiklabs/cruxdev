"""Tests for counter-research module."""

from src.research.counter import (
    CounterResult,
    assess_robustness,
    generate_negation_queries,
    run_counter_research,
)


class TestGenerateNegationQueries:
    def test_generates_queries(self):
        queries = generate_negation_queries("TDD improves code quality")
        assert len(queries) >= 3
        assert any("not true" in q for q in queries)
        assert any("criticism" in q for q in queries)

    def test_includes_failure_queries(self):
        queries = generate_negation_queries("Rust is memory safe")
        assert any("problems" in q for q in queries)
        assert any("limitations" in q for q in queries)


class TestAssessRobustness:
    def test_robust_no_counter(self):
        assert assess_robustness(0, 0, 5) == "robust"

    def test_moderate(self):
        assert assess_robustness(1, 0, 5) == "moderate"

    def test_fragile_some_counter(self):
        assert assess_robustness(1, 1, 4) == "fragile"  # 2/6 = 0.33

    def test_contested_equal_evidence(self):
        assert assess_robustness(3, 2, 5) == "contested"

    def test_fragile_no_support(self):
        assert assess_robustness(0, 0, 0) == "fragile"

    def test_contested_strong_counter(self):
        assert assess_robustness(5, 0, 3) == "contested"


class TestCounterResult:
    def test_is_contested(self):
        r = CounterResult("claim", robustness="contested")
        assert r.is_contested is True

    def test_not_contested(self):
        r = CounterResult("claim", robustness="robust")
        assert r.is_contested is False

    def test_has_counter_evidence(self):
        r = CounterResult("claim", counter_evidence=["evidence1"])
        assert r.has_counter_evidence is True

    def test_no_counter_evidence(self):
        r = CounterResult("claim")
        assert r.has_counter_evidence is False


class TestRunCounterResearch:
    def test_basic(self):
        result = run_counter_research("Python is fast")
        assert result.original_claim == "Python is fast"
        assert len(result.negation_queries) >= 3
        assert result.robustness == "robust"  # no counter evidence provided

    def test_with_counter_evidence(self):
        result = run_counter_research(
            "Python is fast",
            counter_evidence=["Benchmark shows Python is slow"],
            supporting_count=1,
        )
        assert result.has_counter_evidence
        assert result.robustness in ("fragile", "contested")

    def test_with_alternatives(self):
        result = run_counter_research(
            "X is best",
            alternative_explanations=["Y is actually better", "Z is comparable"],
            supporting_count=1,
        )
        assert len(result.alternative_explanations) == 2

    def test_no_support_fragile(self):
        result = run_counter_research("Unverified claim", supporting_count=0)
        assert result.robustness == "fragile"
