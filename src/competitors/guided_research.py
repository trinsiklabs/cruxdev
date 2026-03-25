"""Guided competitor research — enforces 5-pass methodology via state machine.

The LLM has the web search tool, not the engine. So the engine can't
run research directly. Instead, it enforces the methodology by:
1. Issuing specific search instructions per pass
2. Refusing to advance until the current pass is complete
3. Requiring contrarian evidence before accepting results
4. Validating completeness before producing a profile

The LLM calls research_next_step() repeatedly. The engine tells it
exactly what to search, what to verify, and when it's done.
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import dataclass, field
from enum import Enum

from .research import CompetitorProfile, Feature


class ResearchPass(str, Enum):
    BROAD = "broad"
    ACADEMIC = "academic"
    PRACTITIONER = "practitioner"
    CONTRARIAN = "contrarian"
    PRIMARY = "primary"
    VERIFY = "verify"
    DONE = "done"


PASS_ORDER = [
    ResearchPass.BROAD,
    ResearchPass.ACADEMIC,
    ResearchPass.PRACTITIONER,
    ResearchPass.CONTRARIAN,
    ResearchPass.PRIMARY,
    ResearchPass.VERIFY,
    ResearchPass.DONE,
]


@dataclass
class PassResult:
    """Result submitted for a single research pass."""
    pass_name: str
    findings: list[str] = field(default_factory=list)
    sources: list[str] = field(default_factory=list)
    search_queries_used: list[str] = field(default_factory=list)


@dataclass
class ResearchState:
    """State of a guided research session for one competitor."""
    competitor_name: str
    competitor_url: str
    category: str
    current_pass: ResearchPass = ResearchPass.BROAD
    pass_results: list[PassResult] = field(default_factory=list)
    profile_data: dict = field(default_factory=dict)
    started_at: float = field(default_factory=time.time)

    @property
    def is_done(self) -> bool:
        return self.current_pass == ResearchPass.DONE

    @property
    def passes_completed(self) -> int:
        return len(self.pass_results)

    def to_dict(self) -> dict:
        return {
            "competitor_name": self.competitor_name,
            "competitor_url": self.competitor_url,
            "category": self.category,
            "current_pass": self.current_pass.value,
            "passes_completed": self.passes_completed,
            "is_done": self.is_done,
        }


# Search instructions per pass
PASS_INSTRUCTIONS: dict[ResearchPass, dict] = {
    ResearchPass.BROAD: {
        "goal": "Establish what this competitor is and does",
        "search_queries": [
            "{name} AI coding tool",
            "{name} {url} features",
            "{name} vs alternatives",
        ],
        "required_fields": ["description", "features"],
        "instructions": "Search for the competitor. Find: what it does, key features, pricing, tech stack. Return raw findings.",
    },
    ResearchPass.ACADEMIC: {
        "goal": "Find authoritative evidence — official docs, benchmarks, technical details",
        "search_queries": [
            "site:{domain} documentation",
            "{name} benchmark performance",
            "{name} architecture technical",
        ],
        "required_fields": ["tech_stack"],
        "instructions": "Find official documentation, benchmarks, technical architecture details. Look for specific numbers (users, stars, downloads). Return findings with source URLs.",
    },
    ResearchPass.PRACTITIONER: {
        "goal": "Find real-world user experience — reviews, complaints, praise",
        "search_queries": [
            "{name} review 2025 2026",
            "{name} problems issues",
            "{name} reddit experience",
        ],
        "required_fields": ["strengths", "weaknesses"],
        "instructions": "Find real user reviews, Reddit discussions, blog posts about using this tool. What do users praise? What do they complain about? Return findings with sources.",
    },
    ResearchPass.CONTRARIAN: {
        "goal": "Find evidence AGAINST this competitor — failures, limitations, criticism",
        "search_queries": [
            "{name} problems limitations criticism",
            "{name} not good why avoid",
            "{name} alternative better than",
        ],
        "required_fields": [],
        "instructions": "THIS PASS IS MANDATORY. Search for criticism, failures, and limitations. Find at least one negative finding. If you cannot find any criticism, state that explicitly — do not skip this pass.",
    },
    ResearchPass.PRIMARY: {
        "goal": "Verify claims from primary sources — official site, GitHub, pricing page",
        "search_queries": [
            "{url}",
            "{name} github stars",
            "{name} pricing",
        ],
        "required_fields": ["pricing"],
        "instructions": "Go to the primary sources: official website, GitHub repo, pricing page. Verify the claims from previous passes. Check: is the pricing accurate? Are the features real? Return verified data.",
    },
    ResearchPass.VERIFY: {
        "goal": "Cross-check all findings and compile final profile",
        "search_queries": [],
        "required_fields": [],
        "instructions": "No new searches needed. Review all findings from passes 1-5. Flag any contradictions. Compile the final profile with: description, features (list), strengths (list), weaknesses (list), pricing, revenue_model, tech_stack, differentiation. Return as structured data.",
    },
}


def start_research(
    competitor_name: str,
    competitor_url: str,
    category: str = "",
) -> ResearchState:
    """Start a guided research session for one competitor."""
    return ResearchState(
        competitor_name=competitor_name,
        competitor_url=competitor_url,
        category=category,
    )


def get_next_step(state: ResearchState) -> dict:
    """Get the next research instruction for the LLM.

    Returns a dict with:
    - pass_name: which pass this is
    - pass_number: 1-6
    - total_passes: 6
    - goal: what this pass aims to achieve
    - search_queries: specific queries to run
    - instructions: what to do and return
    - required_fields: what data must be found
    - is_done: whether research is complete
    """
    if state.is_done:
        return {
            "pass_name": "done",
            "is_done": True,
            "instructions": "Research complete. Call setup_competitive_analysis with the compiled data.",
            "profile_data": state.profile_data,
        }

    info = PASS_INSTRUCTIONS[state.current_pass]
    domain = state.competitor_url.replace("https://", "").replace("http://", "").split("/")[0]

    queries = [
        q.format(
            name=state.competitor_name,
            url=state.competitor_url,
            domain=domain,
        )
        for q in info["search_queries"]
    ]

    return {
        "pass_name": state.current_pass.value,
        "pass_number": PASS_ORDER.index(state.current_pass) + 1,
        "total_passes": len(PASS_ORDER) - 1,  # exclude DONE
        "goal": info["goal"],
        "search_queries": queries,
        "instructions": info["instructions"],
        "required_fields": info["required_fields"],
        "is_done": False,
        "competitor": state.competitor_name,
        "previous_findings": [
            {"pass": r.pass_name, "finding_count": len(r.findings)}
            for r in state.pass_results
        ],
    }


def submit_pass_result(
    state: ResearchState,
    findings: list[str],
    sources: list[str] | None = None,
    search_queries_used: list[str] | None = None,
    profile_updates: dict | None = None,
) -> dict:
    """Submit results for the current pass and advance to next.

    Args:
        state: Current research state
        findings: List of findings from this pass
        sources: URLs of sources consulted
        search_queries_used: Queries that were actually run
        profile_updates: Dict of profile fields to update (e.g., {"pricing": "$10/mo"})

    Returns dict with validation result and next step info.
    """
    # Validate contrarian pass has actual findings
    if state.current_pass == ResearchPass.CONTRARIAN and not findings:
        return {
            "accepted": False,
            "error": "Contrarian pass MUST have at least one finding. Search for criticism, limitations, or failures. This pass cannot be skipped.",
            "current_pass": state.current_pass.value,
        }

    # Record the pass result
    state.pass_results.append(PassResult(
        pass_name=state.current_pass.value,
        findings=findings,
        sources=sources or [],
        search_queries_used=search_queries_used or [],
    ))

    # Update profile data
    if profile_updates:
        state.profile_data.update(profile_updates)

    # Advance to next pass
    current_idx = PASS_ORDER.index(state.current_pass)
    state.current_pass = PASS_ORDER[current_idx + 1]

    return {
        "accepted": True,
        "passes_completed": state.passes_completed,
        "next": get_next_step(state),
    }


# In-memory state store for active research sessions
_active_sessions: dict[str, ResearchState] = {}


def get_or_create_session(
    competitor_name: str,
    competitor_url: str = "",
    category: str = "",
) -> tuple[ResearchState, bool]:
    """Get existing session or create new one. Returns (state, is_new)."""
    key = competitor_name.lower()
    if key in _active_sessions:
        return _active_sessions[key], False
    state = start_research(competitor_name, competitor_url, category)
    _active_sessions[key] = state
    return state, True


def clear_session(competitor_name: str) -> bool:
    """Remove a completed research session."""
    key = competitor_name.lower()
    if key in _active_sessions:
        del _active_sessions[key]
        return True
    return False


def list_sessions() -> list[dict]:
    """List all active research sessions."""
    return [s.to_dict() for s in _active_sessions.values()]
