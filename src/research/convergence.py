"""Research convergence detection — determines when research is "done enough."

Tracks novelty, coverage, contradictions, and budget to decide when
additional searching would produce diminishing returns.
"""

from __future__ import annotations

from dataclasses import dataclass

from .session import ResearchSession


DEFAULT_MAX_SEARCHES = 50
DEFAULT_MIN_SOURCES = 3
DEFAULT_NOVELTY_THRESHOLD = 0.10
DEFAULT_CONFIDENCE_FLOOR = 0.70


@dataclass
class ConvergenceCheck:
    converged: bool
    reason: str
    coverage_ok: bool
    novelty_ok: bool
    contradictions_resolved: bool
    budget_exhausted: bool
    novelty_avg: float


def calculate_novelty(new_facts: int, total_facts: int) -> float:
    """Calculate novelty score for a search result.

    novelty = proportion of facts that are genuinely new
    (Jaccard similarity < 0.3 to existing findings)
    """
    if total_facts == 0:
        return 0.0
    return new_facts / total_facts


def check_research_convergence(
    session: ResearchSession,
    max_searches: int = DEFAULT_MAX_SEARCHES,
    min_sources: int = DEFAULT_MIN_SOURCES,
    novelty_threshold: float = DEFAULT_NOVELTY_THRESHOLD,
) -> ConvergenceCheck:
    """Run convergence check on a research session.

    Returns ConvergenceCheck with detailed status.
    """
    # 1. COVERAGE: enough unique sources?
    unique_sources = len(set(session.seen_urls))
    coverage_ok = unique_sources >= min_sources

    # 2. NOVELTY: are we still finding new info?
    recent_novelty = session.novelty_scores[-5:] if session.novelty_scores else []
    novelty_avg = sum(recent_novelty) / len(recent_novelty) if recent_novelty else 1.0
    novelty_ok = len(recent_novelty) >= 3 and novelty_avg < novelty_threshold

    # 3. CONTRADICTIONS: any unresolved?
    contradictions = [
        f for f in session.findings
        if f.robustness == "contested" and not f.counter_evidence
    ]
    contradictions_resolved = len(contradictions) == 0

    # 4. BUDGET: hit the ceiling?
    budget_exhausted = session.total_searches >= max_searches

    # Determine convergence
    if budget_exhausted:
        return ConvergenceCheck(
            converged=True,
            reason="budget_exhausted",
            coverage_ok=coverage_ok,
            novelty_ok=novelty_ok,
            contradictions_resolved=contradictions_resolved,
            budget_exhausted=True,
            novelty_avg=novelty_avg,
        )

    natural = coverage_ok and novelty_ok and contradictions_resolved
    if natural:
        return ConvergenceCheck(
            converged=True,
            reason="natural_convergence",
            coverage_ok=True,
            novelty_ok=True,
            contradictions_resolved=True,
            budget_exhausted=False,
            novelty_avg=novelty_avg,
        )

    # Not converged — identify what's blocking
    reasons = []
    if not coverage_ok:
        reasons.append(f"coverage ({unique_sources}/{min_sources} sources)")
    if not novelty_ok:
        reasons.append(f"novelty ({novelty_avg:.2f} > {novelty_threshold})")
    if not contradictions_resolved:
        reasons.append(f"{len(contradictions)} unresolved contradictions")

    return ConvergenceCheck(
        converged=False,
        reason=f"not_converged: {', '.join(reasons)}",
        coverage_ok=coverage_ok,
        novelty_ok=novelty_ok,
        contradictions_resolved=contradictions_resolved,
        budget_exhausted=False,
        novelty_avg=novelty_avg,
    )
