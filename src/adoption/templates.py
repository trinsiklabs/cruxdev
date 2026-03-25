"""Template registry — all Key document types with requirement levels.

Maps project types and maturity levels to required document templates.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal


RequirementLevel = Literal["R", "P", "M", "O"]  # Required, Production, Mature, Optional


@dataclass
class Template:
    """A document template definition."""
    category: str
    name: str
    description: str
    filename: str
    requirement: RequirementLevel = "O"


@dataclass
class TemplateSet:
    """A set of templates applicable to a project."""
    templates: list[Template] = field(default_factory=list)

    @property
    def required(self) -> list[Template]:
        return [t for t in self.templates if t.requirement == "R"]

    @property
    def production(self) -> list[Template]:
        return [t for t in self.templates if t.requirement in ("R", "P")]

    @property
    def by_category(self) -> dict[str, list[Template]]:
        result: dict[str, list[Template]] = {}
        for t in self.templates:
            result.setdefault(t.category, []).append(t)
        return result


# Master template registry
TEMPLATES: list[Template] = [
    # Code templates
    Template("code", "README", "Project overview and setup", "README.md", "R"),
    Template("code", "CLAUDE.md", "AI assistant configuration", "CLAUDE.md", "R"),
    Template("code", "CHANGELOG", "Version history", "CHANGELOG.md", "P"),
    Template("code", "CONTRIBUTING", "Contribution guidelines", "CONTRIBUTING.md", "M"),
    Template("code", "LICENSE", "License file", "LICENSE", "P"),
    Template("code", "SECURITY", "Security policy", "SECURITY.md", "M"),

    # Business templates
    Template("business", "Business Plan", "Strategy and goals", "docs/BUSINESS_PLAN.md", "R"),
    Template("business", "Budget", "Financial planning", "docs/BUDGET.md", "P"),
    Template("business", "Operations", "Operational procedures", "docs/OPERATIONS.md", "P"),

    # Product templates
    Template("product", "Product Spec", "Feature specifications", "docs/PRODUCT_SPEC.md", "R"),
    Template("product", "User Stories", "User requirements", "docs/USER_STORIES.md", "R"),
    Template("product", "Roadmap", "Development roadmap", "docs/ROADMAP.md", "P"),

    # Website templates
    Template("website", "Deployment", "Deployment procedures", "docs/DEPLOYMENT.md", "R"),
    Template("website", "SEO Strategy", "Search engine optimization", "docs/SEO_STRATEGY.md", "P"),

    # Research templates
    Template("research", "Research Plan", "Research methodology", "docs/RESEARCH_PLAN.md", "R"),
    Template("research", "Findings", "Research results", "docs/FINDINGS.md", "R"),

    # Governance templates
    Template("governance", "GAPS", "Gap analysis tracking", "GAPS.md", "R"),
    Template("governance", "Competitors", "Competitive analysis", "docs/COMPETITORS.md", "P"),
    Template("governance", "Architecture", "System architecture", "docs/ARCHITECTURE.md", "P"),
]


# Which template categories apply to which project types
TYPE_CATEGORIES: dict[str, list[str]] = {
    "software-existing": ["code", "governance"],
    "software-greenfield": ["code", "governance"],
    "business-existing": ["business", "governance"],
    "business-new": ["business", "governance"],
    "product-saas": ["code", "product", "governance"],
    "website": ["code", "website", "governance"],
    "infrastructure": ["code", "governance"],
    "consulting-client": ["business", "governance"],
    "research": ["research", "governance"],
    "campaign": ["business", "governance"],
}


def get_templates_for_type(
    project_type: str,
    maturity: str = "minimal",
) -> TemplateSet:
    """Get applicable templates for a project type and maturity.

    Args:
        project_type: The project type (from classify)
        maturity: Current maturity level
    """
    categories = TYPE_CATEGORIES.get(project_type, ["code", "governance"])

    applicable = []
    for t in TEMPLATES:
        if t.category not in categories:
            continue

        # Filter by maturity level
        if maturity == "idea" and t.requirement not in ("R",):
            continue
        if maturity == "minimal" and t.requirement not in ("R",):
            continue
        if maturity == "growing" and t.requirement not in ("R", "P"):
            continue
        # production and mature get everything

        applicable.append(t)

    return TemplateSet(templates=applicable)


def get_folder_structure(project_type: str) -> list[str]:
    """Get recommended folder structure for a project type."""
    base = ["docs/", "GAPS.md"]

    type_dirs: dict[str, list[str]] = {
        "software-existing": ["src/", "tests/", "docs/", ".claude/"],
        "software-greenfield": ["src/", "tests/", "docs/", ".claude/"],
        "website": ["src/", "public/", "docs/"],
        "product-saas": ["src/", "tests/", "docs/", "api/"],
        "infrastructure": ["terraform/", "scripts/", "docs/"],
        "research": ["papers/", "data/", "experiments/", "docs/"],
        "business-existing": ["docs/", "reports/"],
        "business-new": ["docs/", "plans/"],
        "consulting-client": ["docs/", "deliverables/", "proposals/"],
        "campaign": ["content/", "assets/", "docs/"],
    }

    dirs = type_dirs.get(project_type, base)
    return sorted(set(dirs + base))
