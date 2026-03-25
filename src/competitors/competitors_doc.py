"""Read/write COMPETITORS.md — structured competitor documentation.

Handles parsing existing COMPETITORS.md files and updating sections.
"""

import re
from dataclasses import dataclass, field

from .gap_analysis import GapAnalysisResult
from .research import CompetitorProfile


@dataclass
class CompetitorsDoc:
    """Parsed COMPETITORS.md structure."""
    title: str = "Competitors"
    overview: str = ""
    profiles: list[CompetitorProfile] = field(default_factory=list)
    gap_analysis: GapAnalysisResult | None = None
    raw_content: str = ""

    def find_profile(self, name: str) -> CompetitorProfile | None:
        """Find a competitor profile by name (case-insensitive)."""
        for p in self.profiles:
            if p.name.lower() == name.lower():
                return p
        return None


def parse_competitors_doc(content: str) -> CompetitorsDoc:
    """Parse a COMPETITORS.md file into structured data.

    Extracts competitor sections (### headers) and basic fields.
    """
    doc = CompetitorsDoc(raw_content=content)
    lines = content.split("\n")

    # Extract title
    for line in lines:
        if line.startswith("# ") and not line.startswith("## "):
            doc.title = line[2:].strip()
            break

    # Find competitor sections (### headers)
    current_profile: CompetitorProfile | None = None
    current_lines: list[str] = []

    for line in lines:
        if line.startswith("### ") and not line.startswith("#### "):
            # Save previous profile
            if current_profile:
                _apply_section_lines(current_profile, current_lines)
                doc.profiles.append(current_profile)

            name = line[4:].strip()
            current_profile = CompetitorProfile(name=name, url="")
            current_lines = []
        elif current_profile is not None:
            current_lines.append(line)

    # Save last profile
    if current_profile:
        _apply_section_lines(current_profile, current_lines)
        doc.profiles.append(current_profile)

    return doc


def _apply_section_lines(profile: CompetitorProfile, lines: list[str]) -> None:
    """Apply parsed lines to a CompetitorProfile."""
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("**URL:**"):
            profile.url = stripped.replace("**URL:**", "").strip()
        elif stripped.startswith("**Tagline:**"):
            profile.tagline = stripped.replace("**Tagline:**", "").strip()
        elif stripped.startswith("**Category:**"):
            val = stripped.replace("**Category:**", "").strip().lower()
            if val in ("official", "watch", "noted"):
                profile.category = val
        elif stripped.startswith("**Pricing:**"):
            profile.pricing = stripped.replace("**Pricing:**", "").strip()


def generate_competitors_doc(
    title: str,
    overview: str,
    profiles: list[CompetitorProfile],
    gap_result: GapAnalysisResult | None = None,
) -> str:
    """Generate a COMPETITORS.md from structured data.

    Args:
        title: Document title
        overview: Overview paragraph
        profiles: Competitor profiles to include
        gap_result: Optional gap analysis to include
    """
    lines = [f"# {title}", ""]

    if overview:
        lines.append(overview)
        lines.append("")

    # Group by category
    for category in ("official", "watch", "noted"):
        cat_profiles = [p for p in profiles if p.category == category]
        if cat_profiles:
            lines.append(f"## {category.title()} Competitors")
            lines.append("")
            for profile in cat_profiles:
                lines.append(profile.to_markdown())
                lines.append("")

    # Gap analysis
    if gap_result:
        lines.append(gap_result.to_markdown())

    return "\n".join(lines)


def update_competitor_section(
    doc_content: str,
    profile: CompetitorProfile,
) -> str:
    """Update a single competitor's section in an existing COMPETITORS.md.

    Replaces the content between ### Name and the next ### or ## header.
    If the competitor doesn't exist, appends to the appropriate category section.
    """
    new_section = profile.to_markdown()

    # Try to find and replace existing section
    pattern = rf"(### {re.escape(profile.name)}\n)(.*?)(?=\n### |\n## |\Z)"
    match = re.search(pattern, doc_content, re.DOTALL)

    if match:
        return doc_content[:match.start()] + new_section + "\n" + doc_content[match.end():]

    # Not found — append to appropriate category section
    category_header = f"## {profile.category.title()} Competitors"
    if category_header in doc_content:
        # Insert after the category header's content
        idx = doc_content.index(category_header)
        # Find next ## header
        next_header = doc_content.find("\n## ", idx + len(category_header))
        if next_header == -1:
            # Append at end
            return doc_content.rstrip() + "\n\n" + new_section + "\n"
        else:
            return doc_content[:next_header] + "\n" + new_section + "\n" + doc_content[next_header:]
    else:
        # No category section — append at end
        return doc_content.rstrip() + "\n\n" + new_section + "\n"
