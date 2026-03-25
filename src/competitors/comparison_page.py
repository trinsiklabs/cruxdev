"""Generate /vs/<competitor> comparison pages for project websites.

Produces markdown content with SEO metadata, feature comparison tables,
and structured comparison sections.
"""

from dataclasses import dataclass, field

from .research import CompetitorProfile


@dataclass
class ComparisonPage:
    """Generated comparison page content."""
    competitor_name: str
    our_name: str
    slug: str  # URL slug: "vs-competitor"
    title: str = ""
    meta_description: str = ""
    content: str = ""
    features_compared: int = 0

    def to_markdown(self) -> str:
        """Render as markdown with frontmatter."""
        lines = [
            "---",
            f'title: "{self.title}"',
            f'description: "{self.meta_description}"',
            f'slug: "{self.slug}"',
            "---",
            "",
            self.content,
        ]
        return "\n".join(lines)


def generate_slug(competitor_name: str) -> str:
    """Generate URL slug from competitor name."""
    return "vs-" + competitor_name.lower().replace(" ", "-").replace(".", "-")


def generate_comparison_content(
    our_name: str,
    our_features: list[str],
    profile: CompetitorProfile,
) -> ComparisonPage:
    """Generate a comparison page for one competitor.

    Produces:
    - SEO-optimized title and meta description
    - Overview section
    - Feature comparison table
    - Strengths/weaknesses comparison
    - When to use which
    """
    slug = generate_slug(profile.name)
    title = f"{our_name} vs {profile.name}"
    meta_desc = f"Compare {our_name} and {profile.name}. See features, pricing, and which is right for you."

    lines: list[str] = []

    # Header
    lines.append(f"# {our_name} vs {profile.name}")
    lines.append("")
    if profile.tagline:
        lines.append(f"**{profile.name}:** {profile.tagline}")
        lines.append("")

    # Feature comparison table
    competitor_features = set(profile.feature_names())
    our_set = set(f.lower() for f in our_features)
    all_features = sorted(set(f.lower() for f in our_features) | set(f.lower() for f in competitor_features))

    if all_features:
        lines.append("## Feature Comparison")
        lines.append("")
        lines.append(f"| Feature | {our_name} | {profile.name} |")
        lines.append("|---|---|---|")
        features_counted = 0
        for feat in all_features:
            our_has = "Y" if feat in our_set else "N"
            comp_has = "Y" if feat in set(f.lower() for f in competitor_features) else "N"
            display_name = feat.title()
            lines.append(f"| {display_name} | {our_has} | {comp_has} |")
            features_counted += 1
        lines.append("")
    else:
        features_counted = 0

    # Strengths comparison
    if profile.strengths or profile.weaknesses:
        lines.append(f"## {profile.name} Strengths")
        lines.append("")
        for s in profile.strengths:
            lines.append(f"- {s}")
        lines.append("")

        if profile.weaknesses:
            lines.append(f"## {profile.name} Weaknesses")
            lines.append("")
            for w in profile.weaknesses:
                lines.append(f"- {w}")
            lines.append("")

    # Pricing
    if profile.pricing:
        lines.append("## Pricing")
        lines.append("")
        lines.append(f"**{profile.name}:** {profile.pricing}")
        lines.append("")

    content = "\n".join(lines)

    return ComparisonPage(
        competitor_name=profile.name,
        our_name=our_name,
        slug=slug,
        title=title,
        meta_description=meta_desc,
        content=content,
        features_compared=features_counted,
    )
