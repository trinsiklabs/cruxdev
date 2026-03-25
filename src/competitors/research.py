"""Structured competitor profiling — deep research on a single competitor.

Produces a CompetitorProfile with pricing, features, strengths, weaknesses,
tech stack, and differentiation.
"""

from dataclasses import dataclass, field
from typing import Literal


@dataclass
class Feature:
    """A single feature of a competitor."""
    name: str
    description: str
    has_feature: bool = True
    notes: str = ""


@dataclass
class CompetitorProfile:
    """Structured profile of a single competitor."""
    name: str
    url: str
    tagline: str = ""
    description: str = ""
    category: Literal["official", "watch", "noted"] = "noted"
    pricing: str = ""
    tech_stack: list[str] = field(default_factory=list)
    features: list[Feature] = field(default_factory=list)
    strengths: list[str] = field(default_factory=list)
    weaknesses: list[str] = field(default_factory=list)
    differentiation: str = ""
    last_researched: str = ""

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
