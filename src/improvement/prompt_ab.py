"""Prompt A/B testing — test mode prompt variations against scenarios.

Compares prompt variants on historical scenarios and selects winners.
Never deploys a variant that performs worse on any critical dimension.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class PromptVariant:
    """A prompt variant to test."""
    id: str
    name: str
    content: str
    scores: dict[str, float] = field(default_factory=dict)  # dimension → score


@dataclass
class ABTestResult:
    """Result of an A/B test between two prompt variants."""
    variant_a: PromptVariant
    variant_b: PromptVariant
    winner: str  # "a", "b", or "tie"
    dimensions: dict[str, str] = field(default_factory=dict)  # dimension → winner
    safe_to_deploy: bool = False

    @property
    def summary(self) -> str:
        if self.winner == "tie":
            return "Tie — no significant difference"
        winner_name = self.variant_a.name if self.winner == "a" else self.variant_b.name
        return f"{winner_name} wins"


def compare_variants(
    variant_a: PromptVariant,
    variant_b: PromptVariant,
    critical_dimensions: list[str] | None = None,
) -> ABTestResult:
    """Compare two prompt variants across all scored dimensions.

    Rules:
    - Variant wins a dimension if its score is higher
    - Overall winner is the variant that wins more dimensions
    - A variant is NOT safe to deploy if it loses on any critical dimension

    Args:
        variant_a: First variant (typically the current/baseline)
        variant_b: Second variant (the challenger)
        critical_dimensions: Dimensions where regression is not allowed
    """
    critical = set(critical_dimensions or [])
    all_dims = set(variant_a.scores.keys()) | set(variant_b.scores.keys())

    dimension_results = {}
    a_wins = 0
    b_wins = 0
    b_regresses_critical = False

    for dim in all_dims:
        score_a = variant_a.scores.get(dim, 0.0)
        score_b = variant_b.scores.get(dim, 0.0)

        if score_b > score_a:
            dimension_results[dim] = "b"
            b_wins += 1
        elif score_a > score_b:
            dimension_results[dim] = "a"
            a_wins += 1
            if dim in critical:
                b_regresses_critical = True
        else:
            dimension_results[dim] = "tie"

    if a_wins > b_wins:
        winner = "a"
    elif b_wins > a_wins:
        winner = "b"
    else:
        winner = "tie"

    return ABTestResult(
        variant_a=variant_a,
        variant_b=variant_b,
        winner=winner,
        dimensions=dimension_results,
        safe_to_deploy=(winner == "b" and not b_regresses_critical),
    )
