"""Tests for convergence parameter tuning."""

import pytest
from src.improvement.convergence_tuning import (
    MIN_CONVERGENCE_THRESHOLD,
    MIN_MAX_ROUNDS,
    analyze_rounds_to_convergence,
    generate_tuning_report,
    rank_dimensions_by_yield,
    recommend_max_rounds,
    validate_convergence_threshold,
)
from src.engine.state import ConvergencePhase, Finding, FindingSeverity, RoundResult


def _make_round(round_num, num_findings=0, dimension="correctness"):
    findings = [
        Finding(id=f"f{i}", file="a.py", dimension=dimension,
                severity=FindingSeverity.HIGH, description="d", suggested_fix="f")
        for i in range(num_findings)
    ]
    return RoundResult(round=round_num, phase=ConvergencePhase.CODE_AUDITING,
                       findings=findings, findings_fixed=0, timestamp=1.0)


def test_analyze_empty():
    assert analyze_rounds_to_convergence([]) == 0


def test_analyze_immediate_convergence():
    history = [_make_round(0), _make_round(1)]
    assert analyze_rounds_to_convergence(history) == 2


def test_recommend_no_data():
    rec = recommend_max_rounds([], current_max=5)
    assert rec.recommended_value == 5
    assert "No historical data" in rec.rationale


def test_recommend_from_data():
    histories = [
        [_make_round(0, 3), _make_round(1, 1), _make_round(2), _make_round(3)],
        [_make_round(0, 2), _make_round(1), _make_round(2)],
    ]
    rec = recommend_max_rounds(histories)
    assert rec.recommended_value >= MIN_MAX_ROUNDS


def test_recommend_respects_floor():
    histories = [[_make_round(0), _make_round(1)]]
    rec = recommend_max_rounds(histories)
    assert rec.recommended_value >= MIN_MAX_ROUNDS


def test_rank_dimensions():
    histories = [
        [
            _make_round(0, 5, "correctness"),
            _make_round(1, 2, "security"),
            _make_round(2, 3, "correctness"),
        ],
    ]
    ranking = rank_dimensions_by_yield(histories)
    assert ranking[0][0] == "correctness"
    assert ranking[0][1] == 8


def test_rank_dimensions_empty():
    assert rank_dimensions_by_yield([]) == []


def test_validate_threshold_ok():
    rec = validate_convergence_threshold(2)
    assert rec.recommended_value == 2


def test_validate_threshold_too_low():
    rec = validate_convergence_threshold(1)
    assert rec.recommended_value == MIN_CONVERGENCE_THRESHOLD


def test_generate_report():
    histories = [
        [_make_round(0, 3, "correctness"), _make_round(1), _make_round(2)],
    ]
    report = generate_tuning_report(histories)
    assert "max_rounds" in report
    assert "convergence_threshold" in report
    assert "dimension_ranking" in report
    assert report["data_points"] == 1
