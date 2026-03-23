"""Tests for audit dimension prioritization."""

from src.improvement.dimension_priority import get_dimension_stats, rank_by_findings
from src.engine.state import ConvergencePhase, Finding, FindingSeverity, RoundResult


def _make_round(findings_dims):
    findings = [
        Finding(id=f"f{i}", file="a.py", dimension=dim,
                severity=FindingSeverity.HIGH, description="d", suggested_fix="f")
        for i, dim in enumerate(findings_dims)
    ]
    return RoundResult(round=0, phase=ConvergencePhase.CODE_AUDITING,
                       findings=findings, findings_fixed=0, timestamp=1.0)


def test_rank_by_findings():
    history = [
        _make_round(["security", "correctness", "correctness"]),
        _make_round(["correctness", "security"]),
    ]
    default = ["correctness", "completeness", "edge_cases", "security"]
    result = rank_by_findings(history, default)

    assert result[0] == "correctness"  # 3 findings
    assert result[1] == "security"  # 2 findings
    # completeness and edge_cases at the end (no findings)
    assert "completeness" in result
    assert "edge_cases" in result


def test_rank_empty_history():
    result = rank_by_findings([], ["a", "b", "c"])
    assert result == ["a", "b", "c"]


def test_rank_preserves_all():
    history = [_make_round(["a"])]
    result = rank_by_findings(history, ["a", "b", "c"])
    assert set(result) == {"a", "b", "c"}
    assert result[0] == "a"


def test_get_dimension_stats():
    history = [
        _make_round(["correctness", "security", "correctness"]),
    ]
    stats = get_dimension_stats(history)
    assert stats["correctness"] == 2
    assert stats["security"] == 1


def test_get_dimension_stats_empty():
    assert get_dimension_stats([]) == {}
