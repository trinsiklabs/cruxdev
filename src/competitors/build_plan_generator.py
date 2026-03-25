"""Auto-generate build plans to close competitive gaps.

When gap analysis identifies a "must-close" gap, this module generates
a build plan following the standard CruxDev template.
"""

from dataclasses import dataclass

from .gap_analysis import FeatureGap


@dataclass
class GeneratedPlan:
    """A generated build plan for closing a gap."""
    filename: str
    content: str
    gap: FeatureGap


def generate_plan_filename(
    plan_number: int,
    gap: FeatureGap,
) -> str:
    """Generate a build plan filename from a gap.

    Format: BUILD_PLAN_NNN_GAP_COMPETITOR_FEATURE.md
    """
    # Use first competitor name and feature name
    competitor = gap.competitors_with_feature[0] if gap.competitors_with_feature else "unknown"
    feature = gap.feature_name

    # Sanitize for filename
    comp_slug = competitor.upper().replace(" ", "_").replace("-", "_")
    feat_slug = feature.upper().replace(" ", "_").replace("-", "_")

    return f"BUILD_PLAN_{plan_number:03d}_GAP_{comp_slug}_{feat_slug}.md"


def generate_gap_plan(
    plan_number: int,
    gap: FeatureGap,
    our_name: str,
    context: str = "",
) -> GeneratedPlan:
    """Generate a build plan to close a competitive gap.

    Args:
        plan_number: Build plan number (for filename)
        gap: The gap to close
        our_name: Our product name
        context: Optional additional context about how competitors implement this
    """
    filename = generate_plan_filename(plan_number, gap)
    comps = ", ".join(gap.competitors_with_feature)

    lines = [
        f"# BUILD_PLAN_{plan_number:03d}: Close Gap — {gap.feature_name}",
        "",
        f"**Status:** NOT STARTED",
        f"**Goal:** Implement {gap.feature_name} to close competitive gap with {comps}.",
        f"**Priority:** {gap.priority}",
        f"**Gap Source:** Competitive analysis — {comps} {'have' if len(gap.competitors_with_feature) > 1 else 'has'} this feature, {our_name} does not.",
        "",
        "**Rule:** TDD. Tests before code. 100% coverage.",
        "",
        "---",
        "",
        "## Document Alignment",
        "",
        "### Product Docs:",
        "- docs/COMPETITORS.md — competitive analysis showing this gap",
        "- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — convergence methodology",
        "",
        "---",
        "",
        "## Competitive Context",
        "",
        f"### Who has {gap.feature_name}?",
        "",
    ]

    for comp in gap.competitors_with_feature:
        lines.append(f"- **{comp}**")

    if gap.rationale:
        lines.append("")
        lines.append(f"### Why close this gap?")
        lines.append("")
        lines.append(gap.rationale)

    if context:
        lines.append("")
        lines.append("### Implementation notes")
        lines.append("")
        lines.append(context)

    lines.extend([
        "",
        "---",
        "",
        "## Phase 1: Implementation",
        "",
        f"- [ ] 1.1 Research how {comps} implement{'' if len(gap.competitors_with_feature) == 1 else 's'} {gap.feature_name}",
        f"- [ ] 1.2 Design our approach",
        f"- [ ] 1.3 Write tests",
        f"- [ ] 1.4 Implement feature",
        f"- [ ] 1.5 Coverage ≥ 100%",
        "",
        "---",
        "",
        "## Phase 2: Convergence",
        "",
        f"- [ ] 2.1 All tests pass",
        f"- [ ] 2.2 Documentation updated",
        f"- [ ] 2.3 COMPETITORS.md gap marked as closed",
        f"- [ ] 2.4 Comparison pages updated",
        f"- [ ] 2.5 Two consecutive clean passes",
        "",
        "---",
        "",
        "## Post-Execution Convergence (Mandatory)",
        "",
        "- [ ] Documentation convergence: audit all docs against code, two clean passes",
        "- [ ] Website convergence: update metrics, audit content accuracy, two clean passes",
        "- [ ] Deployment: deploy per docs/DEPLOYMENT.md",
        "- [ ] Patterns update: capture learnings if novel",
        "- [ ] Inbox check: process messages from other sessions",
        "",
        "## Convergence Criteria",
        "",
        "- All checklist items complete",
        "- All tests pass, coverage ≥ 100%",
        "- Two consecutive clean audit passes",
        f"- {gap.feature_name} working in {our_name}",
        f"- COMPETITORS.md updated: gap marked closed",
        "",
        "---",
        "",
        "## Test Commands",
        "",
        "```bash",
        "python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100",
        "```",
        "",
        f"**Total: 15 checkboxes**",
    ])

    return GeneratedPlan(
        filename=filename,
        content="\n".join(lines),
        gap=gap,
    )
