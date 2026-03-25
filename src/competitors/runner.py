"""Competitive analysis runner — single-call orchestration.

Replaces the multi-step LLM-orchestrated flow with a single engine-driven
pipeline. The LLM calls one tool, the engine does everything it can
deterministically, and returns structured results.

Two modes:
1. setup() — Initialize competitive analysis from known competitors (no LLM needed)
2. refresh() — Re-run analysis with existing data (no LLM needed)
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import asdict, dataclass, field

from .comparison_page import generate_comparison_content
from .competitors_doc import generate_competitors_doc
from .discovery import generate_discovery_queries
from .gap_analysis import GapAnalysisResult, run_gap_analysis
from .research import CompetitorProfile, Feature


@dataclass
class CompetitorInput:
    """Minimal input to define a competitor — no LLM needed."""
    name: str
    url: str
    category: str = "noted"  # official, watch, noted
    description: str = ""
    features: list[str] = field(default_factory=list)
    strengths: list[str] = field(default_factory=list)
    weaknesses: list[str] = field(default_factory=list)
    pricing: str = ""
    revenue_model: str = ""


@dataclass
class AnalysisResult:
    """Complete result of a competitive analysis run."""
    our_name: str
    competitors_doc: str  # Full COMPETITORS.md content
    gap_analysis: str  # Gap analysis markdown
    comparison_pages: dict[str, str]  # slug → markdown content
    discovery_queries: list[str]  # Queries for future research
    summary: dict  # Counts and highlights


def _input_to_profile(inp: CompetitorInput) -> CompetitorProfile:
    """Convert a CompetitorInput to a full CompetitorProfile."""
    return CompetitorProfile(
        name=inp.name,
        url=inp.url,
        category=inp.category,
        description=inp.description,
        features=[Feature(f, "") for f in inp.features],
        strengths=inp.strengths,
        weaknesses=inp.weaknesses,
        pricing=inp.pricing,
        revenue_model=inp.revenue_model,
        last_researched=time.strftime("%Y-%m-%d"),
    )


def setup(
    our_name: str,
    our_description: str,
    our_category: str,
    our_features: list[str],
    competitors: list[CompetitorInput],
    overview: str = "",
) -> AnalysisResult:
    """Run complete competitive analysis from known competitor data.

    This is the single-call entry point. No LLM orchestration needed.
    The engine:
    1. Converts inputs to profiles
    2. Builds feature matrix
    3. Classifies gaps
    4. Generates comparison pages
    5. Produces COMPETITORS.md
    6. Returns discovery queries for future research

    Args:
        our_name: Our product name
        our_description: What we do
        our_category: Market category
        our_features: Our feature list
        competitors: Known competitors with basic info
        overview: Overview paragraph for COMPETITORS.md
    """
    # Convert inputs to profiles
    profiles = [_input_to_profile(c) for c in competitors]

    # Run gap analysis
    official = [p.name for p in profiles if p.category == "official"]
    gap_result = run_gap_analysis(our_name, our_features, profiles, official)

    # Generate comparison pages
    comparison_pages = {}
    for profile in profiles:
        if profile.category in ("official", "watch"):
            page = generate_comparison_content(our_name, our_features, profile)
            comparison_pages[page.slug] = page.to_markdown()

    # Generate COMPETITORS.md
    doc = generate_competitors_doc(
        f"{our_name} — Competitive Analysis",
        overview or f"Competitive landscape for {our_name} in the {our_category} category.",
        profiles,
        gap_result,
    )

    # Generate discovery queries for future research
    queries = generate_discovery_queries(our_description, our_category)

    # Summary
    summary = {
        "total_competitors": len(profiles),
        "official": len([p for p in profiles if p.category == "official"]),
        "watch": len([p for p in profiles if p.category == "watch"]),
        "noted": len([p for p in profiles if p.category == "noted"]),
        "total_features_compared": len(gap_result.feature_matrix),
        "total_gaps": len(gap_result.gaps),
        "must_close": len(gap_result.must_close),
        "should_close": len(gap_result.should_close),
        "comparison_pages_generated": len(comparison_pages),
        "discovery_queries_for_research": len(queries),
    }

    return AnalysisResult(
        our_name=our_name,
        competitors_doc=doc,
        gap_analysis=gap_result.to_markdown(),
        comparison_pages=comparison_pages,
        discovery_queries=queries,
        summary=summary,
    )


def write_results(
    result: AnalysisResult,
    project_dir: str,
    docs_dir: str = "docs",
    vs_dir: str = "",
) -> list[str]:
    """Write analysis results to disk.

    Args:
        result: The analysis result to write
        project_dir: Project root directory
        docs_dir: Where to write COMPETITORS.md (relative to project_dir)
        vs_dir: Where to write comparison pages (empty = don't write)

    Returns list of files written.
    """
    written = []

    # Write COMPETITORS.md
    comp_path = os.path.join(project_dir, docs_dir, "COMPETITORS.md")
    os.makedirs(os.path.dirname(comp_path), exist_ok=True)
    with open(comp_path, "w") as f:
        f.write(result.competitors_doc)
    written.append(comp_path)

    # Write comparison pages
    if vs_dir and result.comparison_pages:
        vs_path = os.path.join(project_dir, vs_dir)
        os.makedirs(vs_path, exist_ok=True)
        for slug, content in result.comparison_pages.items():
            page_path = os.path.join(vs_path, f"{slug}.md")
            with open(page_path, "w") as f:
                f.write(content)
            written.append(page_path)

    return written


def parse_competitor_inputs(raw: object) -> list[CompetitorInput]:
    """Parse competitor inputs from any format a model might send.

    Handles: JSON string, list of dicts, dicts with string features.
    """
    if raw is None:
        return []
    if isinstance(raw, str):
        try:
            raw = json.loads(raw)
        except json.JSONDecodeError:
            return []
    if not isinstance(raw, list):
        return []

    results = []
    for item in raw:
        if not isinstance(item, dict):
            continue
        features = item.get("features", [])
        if isinstance(features, str):
            features = [f.strip() for f in features.split(",") if f.strip()]
        elif isinstance(features, list):
            features = [f if isinstance(f, str) else f.get("name", "") for f in features]

        strengths = item.get("strengths", [])
        if isinstance(strengths, str):
            strengths = [s.strip() for s in strengths.split(",") if s.strip()]

        weaknesses = item.get("weaknesses", [])
        if isinstance(weaknesses, str):
            weaknesses = [w.strip() for w in weaknesses.split(",") if w.strip()]

        results.append(CompetitorInput(
            name=item.get("name", "unknown"),
            url=item.get("url", ""),
            category=item.get("category", "noted"),
            description=item.get("description", ""),
            features=features,
            strengths=strengths,
            weaknesses=weaknesses,
            pricing=item.get("pricing", ""),
            revenue_model=item.get("revenue_model", ""),
        ))

    return results
