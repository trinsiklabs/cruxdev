"""Project classification — determine type(s), maturity, and required templates.

Classifies a project by analyzing its contents: code, docs, configs, etc.
Supports composite types (most projects are multi-type).
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from typing import Literal


ProjectType = Literal[
    "software-existing",
    "software-greenfield",
    "business-existing",
    "business-new",
    "product-saas",
    "website",
    "infrastructure",
    "consulting-client",
    "research",
    "campaign",
]

Maturity = Literal["idea", "minimal", "growing", "production", "mature"]


@dataclass
class Classification:
    """Project classification result."""
    primary_type: ProjectType
    secondary_types: list[ProjectType] = field(default_factory=list)
    maturity: Maturity = "minimal"
    confidence: float = 0.0
    signals: dict[str, list[str]] = field(default_factory=dict)

    @property
    def all_types(self) -> list[ProjectType]:
        return [self.primary_type] + self.secondary_types


# File/dir patterns that indicate project type
TYPE_SIGNALS: dict[ProjectType, list[str]] = {
    "software-existing": ["src/", "lib/", "tests/", "setup.py", "pyproject.toml", "package.json", "Cargo.toml", "go.mod"],
    "software-greenfield": [],  # Detected by absence of code
    "website": ["index.html", "astro.config.*", "next.config.*", "public/", "pages/", "src/pages/"],
    "product-saas": ["Dockerfile", "docker-compose.*", ".env.example", "api/", "app/"],
    "infrastructure": ["terraform/", "*.tf", "ansible/", "k8s/", "Makefile", ".github/workflows/"],
    "research": ["papers/", "experiments/", "data/", "notebooks/", "*.ipynb"],
    "business-existing": ["docs/BUSINESS_PLAN.md", "docs/BUDGET.md", "docs/OPERATIONS.md"],
    "business-new": [],  # Detected by absence + business intent
    "consulting-client": ["clients/", "proposals/", "deliverables/"],
    "campaign": ["campaigns/", "marketing/", "ads/", "content/"],
}

MATURITY_SIGNALS: dict[Maturity, list[str]] = {
    "production": ["CI/CD", ".github/workflows/", "Dockerfile", "DEPLOYMENT.md"],
    "growing": ["tests/", "docs/", "CHANGELOG.md"],
    "minimal": ["README.md", "src/"],
    "mature": ["CONTRIBUTING.md", "LICENSE", "SECURITY.md", "CODE_OF_CONDUCT.md"],
}


def classify_project(project_dir: str) -> Classification:
    """Classify a project by analyzing its directory structure.

    Scans for known file patterns and returns the best-fit classification.
    """
    found_signals: dict[str, list[str]] = {}
    type_scores: dict[str, int] = {}

    # Scan directory
    entries = _scan_entries(project_dir)

    for proj_type, patterns in TYPE_SIGNALS.items():
        matches = []
        for pattern in patterns:
            for entry in entries:
                if _matches_pattern(entry, pattern):
                    matches.append(entry)
        if matches:
            found_signals[proj_type] = matches
            type_scores[proj_type] = len(matches)

    # Determine primary type
    if not type_scores:
        return Classification(
            primary_type="software-greenfield",
            maturity="idea",
            confidence=0.3,
            signals=found_signals,
        )

    sorted_types = sorted(type_scores.items(), key=lambda x: x[1], reverse=True)
    primary = sorted_types[0][0]
    secondary = [t for t, _ in sorted_types[1:] if _ > 0]

    # Assess maturity
    maturity = _assess_maturity(entries)

    # Confidence based on signal count
    max_score = sorted_types[0][1]
    confidence = min(1.0, max_score / 5)

    return Classification(
        primary_type=primary,
        secondary_types=secondary,
        maturity=maturity,
        confidence=round(confidence, 2),
        signals=found_signals,
    )


def _scan_entries(project_dir: str) -> list[str]:
    """Scan project directory for entries (files and dirs)."""
    entries = []
    if not os.path.isdir(project_dir):
        return entries
    for root, dirs, files in os.walk(project_dir):
        # Skip hidden dirs and node_modules
        dirs[:] = [d for d in dirs if not d.startswith(".") and d != "node_modules" and d != "__pycache__"]
        rel_root = os.path.relpath(root, project_dir)
        for d in dirs:
            path = os.path.join(rel_root, d) + "/" if rel_root != "." else d + "/"
            entries.append(path)
        for f in files:
            path = os.path.join(rel_root, f) if rel_root != "." else f
            entries.append(path)
    return entries


def _matches_pattern(entry: str, pattern: str) -> bool:
    """Check if an entry matches a pattern (simple glob-like matching)."""
    if pattern.endswith("/"):
        # Directory pattern
        return entry.endswith("/") and entry.rstrip("/").endswith(pattern.rstrip("/"))
    if pattern.startswith("*."):
        # Extension pattern
        return entry.endswith(pattern[1:])
    if "*" in pattern:
        # Simple wildcard — match prefix
        prefix = pattern.split("*")[0]
        return entry.startswith(prefix) or ("/" + prefix) in entry
    # Exact match
    return entry == pattern or entry.endswith("/" + pattern)


def _assess_maturity(entries: list[str]) -> Maturity:
    """Assess project maturity from directory entries."""
    entry_str = " ".join(entries).lower()

    # Check from most mature to least
    mature_indicators = sum(1 for s in ["contributing", "license", "security.md", "code_of_conduct"]
                           if s in entry_str)
    if mature_indicators >= 3:
        return "mature"

    production_indicators = sum(1 for s in [".github/workflows/", "dockerfile", "deployment"]
                               if s in entry_str)
    if production_indicators >= 2:
        return "production"

    growing_indicators = sum(1 for s in ["tests/", "docs/", "changelog"]
                           if s in entry_str)
    if growing_indicators >= 2:
        return "growing"

    if any(s in entry_str for s in ["readme", "src/"]):
        return "minimal"

    return "idea"
