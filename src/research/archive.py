"""Research archive — permanent storage for research findings.

Research is permanent. Content derived from research is cheap and regenerable.
Archive structure: research_archives/{slug}/research.md, sources.json, meta.json
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import asdict

from .session import ResearchFinding, ResearchSession


def archive_session(
    session: ResearchSession,
    archive_dir: str,
) -> str:
    """Archive a completed research session permanently.

    Creates: research.md, sources.json, meta.json
    Returns path to archive directory.
    """
    slug = session.topic.lower().replace(" ", "-").replace("/", "-")
    session_dir = os.path.join(archive_dir, slug)
    os.makedirs(session_dir, exist_ok=True)

    # research.md — findings as markdown
    research_md = _generate_research_markdown(session)
    with open(os.path.join(session_dir, "research.md"), "w") as f:
        f.write(research_md)

    # sources.json — all source URLs with metadata
    sources = [
        {
            "url": finding.source_url,
            "quality_score": finding.quality_score,
            "robustness": finding.robustness,
            "pass": finding.pass_found,
            "tags": finding.tags,
        }
        for finding in session.findings
    ]
    with open(os.path.join(session_dir, "sources.json"), "w") as f:
        json.dump(sources, f, indent=2)

    # meta.json — session metadata
    meta = {
        "session_id": session.session_id,
        "topic": session.topic,
        "quality_score": session.quality_score,
        "total_searches": session.total_searches,
        "total_findings": len(session.findings),
        "total_sources": len(set(session.seen_urls)),
        "converged": session.converged,
        "budget_exhausted": session.budget_exhausted,
        "archived_at": time.time(),
    }
    with open(os.path.join(session_dir, "meta.json"), "w") as f:
        json.dump(meta, f, indent=2)

    return session_dir


def load_archive(archive_dir: str, slug: str) -> dict | None:
    """Load archived research for a topic."""
    session_dir = os.path.join(archive_dir, slug)
    if not os.path.isdir(session_dir):
        return None

    result = {}
    research_path = os.path.join(session_dir, "research.md")
    if os.path.exists(research_path):
        with open(research_path) as f:
            result["research"] = f.read()

    sources_path = os.path.join(session_dir, "sources.json")
    if os.path.exists(sources_path):
        with open(sources_path) as f:
            result["sources"] = json.load(f)

    meta_path = os.path.join(session_dir, "meta.json")
    if os.path.exists(meta_path):
        with open(meta_path) as f:
            result["meta"] = json.load(f)

    return result if result else None


def is_stale(archive_dir: str, slug: str, max_age_days: int = 30) -> bool:
    """Check if archived research is stale."""
    session_dir = os.path.join(archive_dir, slug)
    meta_path = os.path.join(session_dir, "meta.json")
    if not os.path.exists(meta_path):
        return True
    try:
        with open(meta_path) as f:
            meta = json.load(f)
        archived_at = meta.get("archived_at", 0)
        return (time.time() - archived_at) > (max_age_days * 86400)
    except (json.JSONDecodeError, KeyError):
        return True


def _generate_research_markdown(session: ResearchSession) -> str:
    """Generate markdown from research findings."""
    lines = [
        f"# Research: {session.topic}",
        "",
        f"**Session:** {session.session_id}",
        f"**Quality:** {session.quality_score:.1f}",
        f"**Sources:** {len(set(session.seen_urls))}",
        f"**Findings:** {len(session.findings)}",
        f"**Converged:** {'naturally' if session.converged and not session.budget_exhausted else 'budget exhausted' if session.budget_exhausted else 'in progress'}",
        "",
        "---",
        "",
    ]

    for i, finding in enumerate(session.findings, 1):
        lines.append(f"## Finding {i}")
        lines.append("")
        lines.append(finding.content)
        lines.append("")
        lines.append(f"**Source:** {finding.source_url}")
        lines.append(f"**Quality:** {finding.quality_score:.2f} | **Robustness:** {finding.robustness}")
        if finding.counter_evidence:
            lines.append(f"**Counter-evidence:** {', '.join(finding.counter_evidence)}")
        if finding.tags:
            lines.append(f"**Tags:** {', '.join(finding.tags)}")
        lines.append("")

    return "\n".join(lines)
