"""Build plan template evolution — improve plan templates from convergence data.

Tracks which plan attributes correlate with better convergence outcomes,
evolves the template to reduce convergence rounds.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class PlanAttribute:
    """A measurable attribute of a build plan."""
    name: str
    present: bool
    description: str = ""


@dataclass
class PlanOutcome:
    """Convergence outcome for a plan."""
    plan_name: str
    attributes: list[PlanAttribute] = field(default_factory=list)
    convergence_rounds: int = 0
    escalated: bool = False


@dataclass
class AttributeCorrelation:
    """Correlation between a plan attribute and convergence speed."""
    attribute_name: str
    plans_with: int = 0
    plans_without: int = 0
    avg_rounds_with: float = 0.0
    avg_rounds_without: float = 0.0

    @property
    def impact(self) -> float:
        """Negative = attribute reduces rounds (good), positive = increases."""
        if self.avg_rounds_without == 0:
            return 0.0
        return self.avg_rounds_with - self.avg_rounds_without


def correlate_attributes(outcomes: list[PlanOutcome]) -> list[AttributeCorrelation]:
    """Correlate plan attributes with convergence outcomes.

    For each attribute, compare average convergence rounds
    for plans that have it vs plans that don't.
    """
    # Collect all attribute names
    all_attrs: set[str] = set()
    for outcome in outcomes:
        for attr in outcome.attributes:
            all_attrs.add(attr.name)

    correlations = []
    for attr_name in sorted(all_attrs):
        with_attr = [o for o in outcomes if any(a.name == attr_name and a.present for a in o.attributes)]
        without_attr = [o for o in outcomes if not any(a.name == attr_name and a.present for a in o.attributes)]

        avg_with = sum(o.convergence_rounds for o in with_attr) / len(with_attr) if with_attr else 0
        avg_without = sum(o.convergence_rounds for o in without_attr) / len(without_attr) if without_attr else 0

        correlations.append(AttributeCorrelation(
            attribute_name=attr_name,
            plans_with=len(with_attr),
            plans_without=len(without_attr),
            avg_rounds_with=round(avg_with, 1),
            avg_rounds_without=round(avg_without, 1),
        ))

    return correlations


def suggest_template_improvements(
    correlations: list[AttributeCorrelation],
) -> list[str]:
    """Suggest template improvements based on attribute correlations.

    Recommends making attributes with negative impact (reduces rounds) mandatory,
    and removing attributes with strong positive impact (increases rounds).
    """
    suggestions = []

    for corr in correlations:
        if corr.impact < -1 and corr.plans_with >= 2:
            suggestions.append(
                f"Make '{corr.attribute_name}' mandatory — reduces rounds by {abs(corr.impact):.1f} on average"
            )
        elif corr.impact > 2 and corr.plans_without >= 2:
            suggestions.append(
                f"Reconsider '{corr.attribute_name}' — increases rounds by {corr.impact:.1f} on average"
            )

    return suggestions
