"""Audit dimension prioritization — high-yield dimensions first.

Tracks which dimensions find the most issues and reorders audit
dimension lists accordingly. All dimensions still checked — just reordered.
"""

from __future__ import annotations

from collections import defaultdict

from ..engine.state import RoundResult


def rank_by_findings(
    history: list[RoundResult],
    default_order: list[str],
) -> list[str]:
    """Reorder dimensions by finding frequency (high-yield first).

    All dimensions in default_order are preserved. Dimensions not seen
    in history retain their original relative order.
    """
    counts: dict[str, int] = defaultdict(int)
    for r in history:
        for f in r.findings:
            counts[f.dimension] += 1

    # Partition into seen and unseen
    seen = [(dim, counts[dim]) for dim in default_order if counts.get(dim, 0) > 0]
    unseen = [dim for dim in default_order if counts.get(dim, 0) == 0]

    # Sort seen by count descending
    seen.sort(key=lambda x: x[1], reverse=True)

    return [dim for dim, _ in seen] + unseen


def get_dimension_stats(history: list[RoundResult]) -> dict[str, int]:
    """Get finding counts per dimension."""
    counts: dict[str, int] = defaultdict(int)
    for r in history:
        for f in r.findings:
            counts[f.dimension] += 1
    return dict(counts)
