"""Structured competitor profiling — deep research on a single competitor.

Produces a CompetitorProfile with pricing, features, strengths, weaknesses,
tech stack, and differentiation.
"""

from dataclasses import dataclass, field
from typing import Literal


MoatType = Literal[
    "network_effects", "switching_costs", "brand", "data_flywheel",
    "regulatory", "execution_speed", "cost_advantage",
]


@dataclass
class Feature:
    """A single feature of a competitor."""
    name: str
    description: str
    has_feature: bool = True
    notes: str = ""


@dataclass
class MoatScore:
    """Moat assessment for a single moat type. Score 0-3."""
    moat_type: MoatType
    score: int = 0  # 0=none, 1=weak, 2=moderate, 3=strong
    evidence: str = ""


@dataclass
class ThreatAssessment:
    """Threat assessment for a competitor. Each dimension 1-5."""
    market_overlap: int = 1
    growth_velocity: int = 1
    resource_asymmetry: int = 1
    technical_proximity: int = 1
    time_to_relevance_months: int = 24

    @property
    def threat_score(self) -> float:
        """Average of the four scored dimensions."""
        return (self.market_overlap + self.growth_velocity +
                self.resource_asymmetry + self.technical_proximity) / 4

    @property
    def threat_level(self) -> str:
        s = self.threat_score
        if s >= 4:
            return "existential"
        if s >= 3:
            return "significant"
        if s >= 2:
            return "moderate"
        return "low"


@dataclass
class CompetitorProfile:
    """Structured profile of a single competitor."""
    name: str
    url: str
    tagline: str = ""
    description: str = ""
    category: Literal["official", "watch", "noted"] = "noted"
    pricing: str = ""
    revenue_model: str = ""
    tech_stack: list[str] = field(default_factory=list)
    features: list[Feature] = field(default_factory=list)
    strengths: list[str] = field(default_factory=list)
    weaknesses: list[str] = field(default_factory=list)
    differentiation: str = ""
    last_researched: str = ""
    moats: list[MoatScore] = field(default_factory=list)
    threat: ThreatAssessment = field(default_factory=ThreatAssessment)
    funding: str = ""
    growth_signals: list[str] = field(default_factory=list)

    def feature_names(self) -> list[str]:
        """Get list of feature names this competitor has."""
        return [f.name for f in self.features if f.has_feature]

    def to_markdown(self) -> str:
        """Render as markdown section for COMPETITORS.md."""
        lines = [f"### {self.name}"]
        lines.append(f"**URL:** {self.url}")
        if self.tagline:
            lines.append(f"**Tagline:** {self.tagline}")
        if self.category:
            lines.append(f"**Category:** {self.category}")
        if self.pricing:
            lines.append(f"**Pricing:** {self.pricing}")
        if self.description:
            lines.append("")
            lines.append(self.description)
        if self.tech_stack:
            lines.append("")
            lines.append("**Tech Stack:** " + ", ".join(self.tech_stack))
        if self.strengths:
            lines.append("")
            lines.append("**Strengths:**")
            for s in self.strengths:
                lines.append(f"- {s}")
        if self.weaknesses:
            lines.append("")
            lines.append("**Weaknesses:**")
            for w in self.weaknesses:
                lines.append(f"- {w}")
        if self.differentiation:
            lines.append("")
            lines.append(f"**Differentiation:** {self.differentiation}")
        if self.revenue_model:
            lines.append(f"**Revenue Model:** {self.revenue_model}")
        if self.funding:
            lines.append(f"**Funding:** {self.funding}")
        if self.moats:
            lines.append("")
            lines.append("**Moat Analysis:**")
            for m in self.moats:
                label = "none weak moderate strong".split()[m.score]
                lines.append(f"- {m.moat_type}: {label}")
                if m.evidence:
                    lines.append(f"  - {m.evidence}")
        if self.threat.threat_score > 1:
            lines.append("")
            lines.append(f"**Threat Level:** {self.threat.threat_level} ({self.threat.threat_score:.1f}/5)")
            if self.threat.time_to_relevance_months < 24:
                lines.append(f"**Time to Relevance:** {self.threat.time_to_relevance_months} months")
        if self.growth_signals:
            lines.append("")
            lines.append("**Growth Signals:**")
            for g in self.growth_signals:
                lines.append(f"- {g}")
        return "\n".join(lines)


def parse_profile_response(
    name: str,
    url: str,
    response_text: str,
) -> CompetitorProfile:
    """Parse an LLM research response into a structured CompetitorProfile.

    Extracts sections from the response text by looking for headers and
    known field patterns.
    """
    profile = CompetitorProfile(name=name, url=url)
    lines = response_text.strip().split("\n")

    current_section = ""
    for line in lines:
        stripped = line.strip()
        lower = stripped.lower()

        # Detect sections
        if lower.startswith("tagline:"):
            profile.tagline = stripped.split(":", 1)[1].strip()
            continue
        if lower.startswith("description:"):
            profile.description = stripped.split(":", 1)[1].strip()
            continue
        if lower.startswith("pricing:"):
            profile.pricing = stripped.split(":", 1)[1].strip()
            continue
        if lower.startswith("category:"):
            val = stripped.split(":", 1)[1].strip().lower()
            if val in ("official", "watch", "noted"):
                profile.category = val
            continue
        if lower.startswith("differentiation:"):
            profile.differentiation = stripped.split(":", 1)[1].strip()
            continue
        if lower.startswith("revenue model:"):
            profile.revenue_model = stripped.split(":", 1)[1].strip()
            continue
        if lower.startswith("funding:"):
            profile.funding = stripped.split(":", 1)[1].strip()
            continue

        # Section headers
        if lower in ("strengths:", "strengths"):
            current_section = "strengths"
            continue
        if lower in ("weaknesses:", "weaknesses"):
            current_section = "weaknesses"
            continue
        if lower.startswith("tech stack:") or lower == "tech stack":
            val = stripped.split(":", 1)[1].strip() if ":" in stripped else ""
            if val:
                profile.tech_stack = [t.strip() for t in val.split(",") if t.strip()]
            current_section = "tech_stack"
            continue
        if lower in ("features:", "features"):
            current_section = "features"
            continue

        # Parse list items under sections
        if stripped.startswith("- "):
            item = stripped[2:].strip()
            if current_section == "strengths":
                profile.strengths.append(item)
            elif current_section == "weaknesses":
                profile.weaknesses.append(item)
            elif current_section == "tech_stack":
                profile.tech_stack.append(item)
            elif current_section == "features":
                profile.features.append(Feature(
                    name=item.split(":")[0].strip() if ":" in item else item,
                    description=item.split(":", 1)[1].strip() if ":" in item else "",
                ))

    return profile
