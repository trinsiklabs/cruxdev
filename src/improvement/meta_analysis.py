"""Meta-analysis — analyze convergence data to improve methodology.

Correlates methodology elements with convergence speed, proposes
methodology changes with evidence.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class ConvergenceDataPoint:
    """Data from a single convergence run."""
    plan_name: str
    total_rounds: int
    total_findings: int
    duration_seconds: float
    phases_used: list[str] = field(default_factory=list)
    escalated: bool = False


@dataclass
class MethodologyInsight:
    """An insight from meta-analysis."""
    element: str
    correlation: str  # positive, negative, neutral
    evidence: str
    suggestion: str


def analyze_convergence_history(
    data_points: list[ConvergenceDataPoint],
) -> list[MethodologyInsight]:
    """Analyze convergence history for methodology improvement signals.

    Identifies:
    - Which phases correlate with faster convergence
    - Which patterns lead to escalation
    - Optimal round counts
    """
    insights = []

    if not data_points:
        return insights

    # Average rounds
    avg_rounds = sum(d.total_rounds for d in data_points) / len(data_points)

    # Escalation rate
    escalated = sum(1 for d in data_points if d.escalated)
    escalation_rate = escalated / len(data_points)

    if escalation_rate > 0.3:
        insights.append(MethodologyInsight(
            element="max_rounds",
            correlation="negative",
            evidence=f"{escalation_rate:.0%} of runs escalated",
            suggestion="Consider increasing max_rounds or breaking plans into smaller units",
        ))

    if avg_rounds < 2:
        insights.append(MethodologyInsight(
            element="plan_granularity",
            correlation="positive",
            evidence=f"Average {avg_rounds:.1f} rounds suggests plans are too simple",
            suggestion="Plans may not be challenging enough — add more audit dimensions",
        ))

    # Phase effectiveness
    phase_counts: dict[str, int] = {}
    for dp in data_points:
        for phase in dp.phases_used:
            phase_counts[phase] = phase_counts.get(phase, 0) + 1

    # Findings per round
    total_findings = sum(d.total_findings for d in data_points)
    total_rounds = sum(d.total_rounds for d in data_points)
    if total_rounds > 0:
        findings_per_round = total_findings / total_rounds
        if findings_per_round > 10:
            insights.append(MethodologyInsight(
                element="audit_thoroughness",
                correlation="positive",
                evidence=f"{findings_per_round:.1f} findings per round",
                suggestion="Audit dimensions are effective at finding issues",
            ))

    return insights
