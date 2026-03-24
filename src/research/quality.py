"""Research quality scoring — 3-tier system.

Tier 1: Fast gate (deterministic, no LLM)
Tier 2: Standard score (6 dimensions)
Tier 3: Deep score (full rubric, high-quality only)
"""

from __future__ import annotations

import re
from dataclasses import dataclass


@dataclass
class QualityResult:
    tier: int  # 1, 2, or 3
    score: float  # 0.0-1.0
    passed: bool
    details: dict


# --- Tier 1: Fast Gate (Deterministic) ---

BOILERPLATE_WORDS = {"subscribe", "click here", "newsletter", "cookie", "advertisement", "sign up", "unsubscribe"}


def fast_gate(content: str) -> QualityResult:
    """Tier 1: Deterministic quality check. No LLM needed.

    Checks: content length, real sentences, information density, boilerplate.
    Score < 0.3 → REJECT.
    """
    if not content or len(content.strip()) < 200:
        return QualityResult(tier=1, score=0.0, passed=False, details={"reason": "too_short"})

    # Count real sentences
    sentences = [s.strip() for s in re.split(r'[.?!]', content) if len(s.strip()) > 20]
    if len(sentences) < 3:
        return QualityResult(tier=1, score=0.1, passed=False, details={"reason": "few_sentences", "count": len(sentences)})

    # Information density
    words = content.lower().split()
    unique_words = set(words)
    density = len(unique_words) / len(words) if words else 0
    if density < 0.3:
        return QualityResult(tier=1, score=0.2, passed=False, details={"reason": "low_density", "density": round(density, 3)})

    # Boilerplate penalty
    boilerplate_count = sum(1 for w in words if w in BOILERPLATE_WORDS)
    boilerplate_ratio = boilerplate_count / len(words) if words else 0

    score = min(1.0, density + (1 - boilerplate_ratio))
    passed = score >= 0.3

    return QualityResult(
        tier=1,
        score=round(score, 3),
        passed=passed,
        details={
            "chars": len(content),
            "sentences": len(sentences),
            "density": round(density, 3),
            "boilerplate_ratio": round(boilerplate_ratio, 3),
        },
    )


# --- Tier 2: Standard Score (6 Dimensions) ---


def standard_score(
    has_authority: bool,
    has_recency: bool,
    has_citations: bool,
    has_relevance: bool,
    has_counter_evidence: bool,
    has_consistency: bool,
) -> QualityResult:
    """Tier 2: 6-dimension binary check.

    Score = count of True / 6. Threshold: >= 0.5 to proceed.
    """
    dimensions = [has_authority, has_recency, has_citations, has_relevance, has_counter_evidence, has_consistency]
    score = sum(dimensions) / 6
    return QualityResult(
        tier=2,
        score=round(score, 3),
        passed=score >= 0.5,
        details={
            "authority": has_authority,
            "recency": has_recency,
            "citations": has_citations,
            "relevance": has_relevance,
            "counter_evidence": has_counter_evidence,
            "consistency": has_consistency,
        },
    )


# --- Tier 3: Deep Score (Full Rubric) ---


def deep_score(
    source_quality: int,  # 0-25
    coverage: int,  # 0-25
    synthesis: int,  # 0-25
    reliability: int,  # 0-25
) -> QualityResult:
    """Tier 3: Full rubric for high-quality findings.

    Each dimension 0-25 points. Total 0-100.
    """
    total = source_quality + coverage + synthesis + reliability
    score = total / 100
    return QualityResult(
        tier=3,
        score=round(score, 3),
        passed=score >= 0.7,
        details={
            "source_quality": source_quality,
            "coverage": coverage,
            "synthesis": synthesis,
            "reliability": reliability,
            "total": total,
        },
    )
