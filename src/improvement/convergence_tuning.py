"""Convergence parameter tuning from historical data.

Analyzes past convergence runs to optimize:
- max_rounds per phase
- Audit dimension ordering (high-yield first)
- Convergence threshold validation
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass

from ..engine.state import ConvergenceState, RoundResult

# Safety floors — never go below these
MIN_CONVERGENCE_THRESHOLD = 2
MIN_MAX_ROUNDS = 3


@dataclass
class TuningRecommendation:
    parameter: str
    current_value: int | float
    recommended_value: int | float
    rationale: str


def analyze_rounds_to_convergence(
    history: list[RoundResult],
) -> int:
    """Calculate how many rounds it typically takes to converge.

    Returns the round number where consecutive clean passes started.
    """
    if not history:
        return 0

    consecutive_clean = 0
    for i, r in enumerate(history):
        if len(r.findings) == 0:
            consecutive_clean += 1
        else:
            consecutive_clean = 0

    return len(history)


def recommend_max_rounds(
    histories: list[list[RoundResult]],
    current_max: int = 5,
) -> TuningRecommendation:
    """Recommend max_rounds based on historical convergence data."""
    if not histories:
        return TuningRecommendation(
            parameter="max_rounds",
            current_value=current_max,
            recommended_value=current_max,
            rationale="No historical data — using default",
        )

    rounds_needed = [analyze_rounds_to_convergence(h) for h in histories]
    avg = sum(rounds_needed) / len(rounds_needed)
    max_needed = max(rounds_needed) if rounds_needed else current_max

    # Recommend max_needed + 1 buffer, but respect safety floor
    recommended = max(MIN_MAX_ROUNDS, max_needed + 1)

    return TuningRecommendation(
        parameter="max_rounds",
        current_value=current_max,
        recommended_value=recommended,
        rationale=f"Avg rounds: {avg:.1f}, max needed: {max_needed}, recommended: {recommended} (with buffer)",
    )


def rank_dimensions_by_yield(
    histories: list[list[RoundResult]],
) -> list[tuple[str, int]]:
    """Rank audit dimensions by how many issues they find.

    Returns list of (dimension, total_findings) sorted descending.
    """
    counts: dict[str, int] = defaultdict(int)

    for history in histories:
        for r in history:
            for f in r.findings:
                counts[f.dimension] += 1

    return sorted(counts.items(), key=lambda x: x[1], reverse=True)


def validate_convergence_threshold(
    threshold: int,
) -> TuningRecommendation:
    """Validate that convergence threshold meets the safety floor."""
    if threshold < MIN_CONVERGENCE_THRESHOLD:
        return TuningRecommendation(
            parameter="convergence_threshold",
            current_value=threshold,
            recommended_value=MIN_CONVERGENCE_THRESHOLD,
            rationale=f"Threshold {threshold} is below safety floor of {MIN_CONVERGENCE_THRESHOLD}",
        )
    return TuningRecommendation(
        parameter="convergence_threshold",
        current_value=threshold,
        recommended_value=threshold,
        rationale=f"Threshold {threshold} meets safety floor",
    )


def generate_tuning_report(
    histories: list[list[RoundResult]],
    current_max_rounds: int = 5,
    current_threshold: int = 2,
) -> dict:
    """Generate a complete tuning report from historical data."""
    max_rounds_rec = recommend_max_rounds(histories, current_max_rounds)
    threshold_rec = validate_convergence_threshold(current_threshold)
    dimension_ranking = rank_dimensions_by_yield(histories)

    return {
        "max_rounds": {
            "current": max_rounds_rec.current_value,
            "recommended": max_rounds_rec.recommended_value,
            "rationale": max_rounds_rec.rationale,
        },
        "convergence_threshold": {
            "current": threshold_rec.current_value,
            "recommended": threshold_rec.recommended_value,
            "rationale": threshold_rec.rationale,
        },
        "dimension_ranking": [
            {"dimension": dim, "findings": count}
            for dim, count in dimension_ranking
        ],
        "data_points": len(histories),
    }
