"""Counter-research — adversarial verification of claims.

Generates negation queries, finds alternative explanations,
and checks for replication failures. Mandatory for all research.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class CounterResult:
    """Result of counter-research on a claim."""
    original_claim: str
    negation_queries: list[str] = field(default_factory=list)
    counter_evidence: list[str] = field(default_factory=list)
    alternative_explanations: list[str] = field(default_factory=list)
    robustness: str = "moderate"  # robust, moderate, fragile, contested

    @property
    def is_contested(self) -> bool:
        return self.robustness == "contested"

    @property
    def has_counter_evidence(self) -> bool:
        return len(self.counter_evidence) > 0


def generate_negation_queries(claim: str) -> list[str]:
    """Generate search queries to find evidence against a claim.

    Strategies:
    1. Direct negation ("X is not true")
    2. Failure reports ("X failed", "X problems")
    3. Alternative framing ("instead of X", "better than X")
    4. Criticism ("X criticism", "X limitations")
    """
    queries = []

    # Direct negation
    queries.append(f"{claim} not true")
    queries.append(f"{claim} wrong")

    # Failure/problems
    queries.append(f"{claim} problems")
    queries.append(f"{claim} limitations")

    # Criticism
    queries.append(f"{claim} criticism")

    return queries


def assess_robustness(
    counter_count: int,
    alternative_count: int,
    supporting_count: int,
) -> str:
    """Assess claim robustness based on evidence balance.

    Returns: robust, moderate, fragile, or contested.
    """
    if supporting_count == 0:
        return "fragile"

    total_against = counter_count + alternative_count
    ratio = total_against / (total_against + supporting_count) if (total_against + supporting_count) > 0 else 0

    if ratio >= 0.5:
        return "contested"
    if ratio >= 0.3:
        return "fragile"
    if ratio >= 0.1:
        return "moderate"
    return "robust"


def run_counter_research(
    claim: str,
    counter_evidence: list[str] | None = None,
    alternative_explanations: list[str] | None = None,
    supporting_count: int = 1,
) -> CounterResult:
    """Run counter-research on a claim.

    Args:
        claim: The claim to verify adversarially
        counter_evidence: Evidence found against the claim
        alternative_explanations: Alternative explanations
        supporting_count: Number of supporting sources
    """
    counter = counter_evidence or []
    alternatives = alternative_explanations or []

    queries = generate_negation_queries(claim)
    robustness = assess_robustness(len(counter), len(alternatives), supporting_count)

    return CounterResult(
        original_claim=claim,
        negation_queries=queries,
        counter_evidence=counter,
        alternative_explanations=alternatives,
        robustness=robustness,
    )
