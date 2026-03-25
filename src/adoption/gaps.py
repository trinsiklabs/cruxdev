"""Gap analysis — compare project state against template requirements.

Detects missing documents, stub documents, and known deficiencies.
Generates GAPS.md from analysis.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from typing import Literal

from .inventory import Inventory
from .templates import Template, TemplateSet


GapSeverity = Literal["critical", "high", "medium", "low"]
GapStatus = Literal["open", "in-progress", "closed", "not-applicable"]


@dataclass
class Gap:
    """A single gap between project state and template requirements."""
    template_name: str
    template_file: str
    severity: GapSeverity
    status: GapStatus = "open"
    reason: str = ""
    justification: str = ""  # For not-applicable items


@dataclass
class GapAnalysis:
    """Complete gap analysis result."""
    project_dir: str
    gaps: list[Gap] = field(default_factory=list)

    @property
    def critical(self) -> list[Gap]:
        return [g for g in self.gaps if g.severity == "critical" and g.status == "open"]

    @property
    def open_gaps(self) -> list[Gap]:
        return [g for g in self.gaps if g.status == "open"]

    @property
    def by_severity(self) -> dict[str, list[Gap]]:
        result: dict[str, list[Gap]] = {}
        for g in self.gaps:
            if g.status != "open":
                continue
            result.setdefault(g.severity, []).append(g)
        return result

    def to_markdown(self) -> str:
        """Generate GAPS.md content."""
        lines = ["# GAPS — Gap Analysis", ""]
        lines.append(f"**Total gaps:** {len(self.open_gaps)} open")
        lines.append(f"**Critical:** {len(self.critical)}")
        lines.append("")

        for severity in ("critical", "high", "medium", "low"):
            gaps_at_level = [g for g in self.gaps if g.severity == severity and g.status == "open"]
            if gaps_at_level:
                lines.append(f"## {severity.title()}")
                lines.append("")
                for gap in gaps_at_level:
                    lines.append(f"- [ ] **{gap.template_name}** (`{gap.template_file}`)")
                    if gap.reason:
                        lines.append(f"  - {gap.reason}")
                lines.append("")

        # Not-applicable items
        na_items = [g for g in self.gaps if g.status == "not-applicable"]
        if na_items:
            lines.append("## Not Applicable")
            lines.append("")
            for gap in na_items:
                lines.append(f"- **{gap.template_name}**: {gap.justification}")
            lines.append("")

        return "\n".join(lines)


TODO_MARKERS = ["TODO", "FIXME", "TBD", "PLACEHOLDER", "STUB"]


def _is_stub(filepath: str) -> bool:
    """Check if a file is a stub (contains TODO markers, very short)."""
    try:
        with open(filepath) as f:
            content = f.read()
        if len(content.strip()) < 50:
            return True
        upper = content.upper()
        return any(marker in upper for marker in TODO_MARKERS)
    except (OSError, UnicodeDecodeError):
        return False


def analyze_gaps(
    project_dir: str,
    inventory: Inventory,
    template_set: TemplateSet,
) -> GapAnalysis:
    """Compare project inventory against required templates.

    Checks:
    1. Missing documents (template required, file doesn't exist)
    2. Stub documents (file exists but has TODO markers)
    3. Severity mapping: R=critical, P=high, M=medium, O=low
    """
    analysis = GapAnalysis(project_dir=project_dir)
    existing_files = {item.path.lower() for item in inventory.items}

    severity_map = {
        "R": "critical",
        "P": "high",
        "M": "medium",
        "O": "low",
    }

    for template in template_set.templates:
        template_lower = template.filename.lower()

        # Check if file exists
        found = template_lower in existing_files
        if not found:
            # Also check without docs/ prefix
            alt = template_lower.replace("docs/", "")
            found = alt in existing_files

        if not found:
            analysis.gaps.append(Gap(
                template_name=template.name,
                template_file=template.filename,
                severity=severity_map.get(template.requirement, "low"),
                reason="Document does not exist",
            ))
            continue

        # Check if it's a stub
        full_path = os.path.join(project_dir, template.filename)
        if os.path.exists(full_path) and _is_stub(full_path):
            analysis.gaps.append(Gap(
                template_name=template.name,
                template_file=template.filename,
                severity=severity_map.get(template.requirement, "low"),
                reason="Document is a stub (contains TODO markers or is too short)",
            ))

    return analysis
