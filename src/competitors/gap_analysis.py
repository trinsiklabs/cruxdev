"""Gap analysis — compare our features against competitors.

Reads competitor profiles, builds a feature matrix, classifies gaps
by priority (must-close, should-close, nice-to-have, won't-do).
"""

from dataclasses import dataclass, field
from typing import Literal

from .research import CompetitorProfile


GapPriority = Literal["must-close", "should-close", "nice-to-have", "wont-do"]


@dataclass
class FeatureGap:
    """A gap where competitors have a feature we don't."""
    feature_name: str
    competitors_with_feature: list[str]
    priority: GapPriority = "should-close"
    rationale: str = ""
    status: Literal["open", "in-progress", "closed"] = "open"
    build_plan: str = ""  # path to build plan if one exists


@dataclass
class FeatureMatrixEntry:
    """One row in the feature comparison matrix."""
    feature: str
    our_status: bool = False
    competitor_status: dict[str, bool] = field(default_factory=dict)


@dataclass
class GapAnalysisResult:
    """Result of a gap analysis."""
    our_name: str
    feature_matrix: list[FeatureMatrixEntry] = field(default_factory=list)
    gaps: list[FeatureGap] = field(default_factory=list)

    @property
    def must_close(self) -> list[FeatureGap]:
        return [g for g in self.gaps if g.priority == "must-close"]

    @property
    def should_close(self) -> list[FeatureGap]:
        return [g for g in self.gaps if g.priority == "should-close"]

    @property
    def open_gaps(self) -> list[FeatureGap]:
        return [g for g in self.gaps if g.status == "open"]

    def to_markdown(self) -> str:
        """Render as markdown for COMPETITORS.md gap section."""
        lines = ["## Gap Analysis", ""]

        # Feature matrix
        if self.feature_matrix:
            competitors = set()
            for entry in self.feature_matrix:
                competitors.update(entry.competitor_status.keys())
            comp_list = sorted(competitors)

            header = f"| Feature | {self.our_name} | " + " | ".join(comp_list) + " |"
            sep = "|---|---|" + "|".join(["---"] * len(comp_list)) + "|"
            lines.append(header)
            lines.append(sep)

            for entry in self.feature_matrix:
                our = "Y" if entry.our_status else "N"
                comps = " | ".join(
                    "Y" if entry.competitor_status.get(c, False) else "N"
                    for c in comp_list
                )
                lines.append(f"| {entry.feature} | {our} | {comps} |")
            lines.append("")

        # Gaps by priority
        for priority in ["must-close", "should-close", "nice-to-have"]:
            gaps_at_priority = [g for g in self.gaps if g.priority == priority]
            if gaps_at_priority:
                lines.append(f"### {priority.title()}")
                for gap in gaps_at_priority:
                    status = f" [{gap.status}]" if gap.status != "open" else ""
                    comps = ", ".join(gap.competitors_with_feature)
                    lines.append(f"- **{gap.feature_name}**{status} — has: {comps}")
                    if gap.rationale:
                        lines.append(f"  - {gap.rationale}")
                lines.append("")

        return "\n".join(lines)


def build_feature_matrix(
    our_name: str,
    our_features: list[str],
    profiles: list[CompetitorProfile],
) -> list[FeatureMatrixEntry]:
    """Build a feature comparison matrix.

    Collects all features from us and all competitors,
    then marks which products have each feature.
    """
    all_features: dict[str, FeatureMatrixEntry] = {}

    # Our features
    for f in our_features:
        entry = FeatureMatrixEntry(feature=f, our_status=True)
        all_features[f.lower()] = entry

    # Competitor features
    for profile in profiles:
        for feat in profile.features:
            key = feat.name.lower()
            if key not in all_features:
                all_features[key] = FeatureMatrixEntry(feature=feat.name, our_status=False)
            all_features[key].competitor_status[profile.name] = feat.has_feature

    return sorted(all_features.values(), key=lambda e: e.feature)


def classify_gaps(
    feature_matrix: list[FeatureMatrixEntry],
    official_competitors: list[str],
) -> list[FeatureGap]:
    """Classify gaps from the feature matrix.

    Rules:
    - Feature we don't have + 2+ official competitors have → must-close
    - Feature we don't have + 1 official competitor has → should-close
    - Feature we don't have + only non-official competitors → nice-to-have
    """
    gaps = []
    for entry in feature_matrix:
        if entry.our_status:
            continue  # We have it, no gap

        comps_with = [
            name for name, has in entry.competitor_status.items()
            if has
        ]
        if not comps_with:
            continue  # Nobody has it

        official_with = [c for c in comps_with if c in official_competitors]

        if len(official_with) >= 2:
            priority: GapPriority = "must-close"
        elif len(official_with) == 1:
            priority = "should-close"
        else:
            priority = "nice-to-have"

        gaps.append(FeatureGap(
            feature_name=entry.feature,
            competitors_with_feature=comps_with,
            priority=priority,
        ))

    return gaps


def run_gap_analysis(
    our_name: str,
    our_features: list[str],
    profiles: list[CompetitorProfile],
    official_competitors: list[str] | None = None,
) -> GapAnalysisResult:
    """Run a complete gap analysis.

    Args:
        our_name: Our product name
        our_features: List of our feature names
        profiles: Competitor profiles to analyze
        official_competitors: Names of "official" competitors (affect priority).
                             Defaults to all profiles.
    """
    if official_competitors is None:
        official_competitors = [p.name for p in profiles]

    matrix = build_feature_matrix(our_name, our_features, profiles)
    gaps = classify_gaps(matrix, official_competitors)

    return GapAnalysisResult(
        our_name=our_name,
        feature_matrix=matrix,
        gaps=gaps,
    )
