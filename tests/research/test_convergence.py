"""Tests for research convergence detection."""

from src.research.convergence import (
    calculate_novelty, check_research_convergence,
)
from src.research.session import ResearchFinding, ResearchSession, create_session


def test_calculate_novelty():
    assert calculate_novelty(3, 10) == 0.3
    assert calculate_novelty(0, 10) == 0.0
    assert calculate_novelty(0, 0) == 0.0
    assert calculate_novelty(10, 10) == 1.0


def test_natural_convergence():
    s = create_session("test")
    s.seen_urls = [f"https://example.com/{i}" for i in range(5)]
    s.novelty_scores = [0.8, 0.5, 0.3, 0.09, 0.05, 0.03, 0.02, 0.01]
    s.total_searches = 10
    result = check_research_convergence(s)
    assert result.converged is True
    assert result.reason == "natural_convergence"
    assert result.budget_exhausted is False


def test_budget_exhausted():
    s = create_session("test")
    s.total_searches = 50
    result = check_research_convergence(s, max_searches=50)
    assert result.converged is True
    assert result.reason == "budget_exhausted"
    assert result.budget_exhausted is True


def test_not_converged_low_coverage():
    s = create_session("test")
    s.seen_urls = ["https://one.com"]
    s.novelty_scores = [0.05, 0.03, 0.02]
    result = check_research_convergence(s, min_sources=3)
    assert result.converged is False
    assert "coverage" in result.reason


def test_not_converged_high_novelty():
    s = create_session("test")
    s.seen_urls = [f"https://example.com/{i}" for i in range(5)]
    s.novelty_scores = [0.8, 0.7, 0.6]
    result = check_research_convergence(s)
    assert result.converged is False
    assert "novelty" in result.reason


def test_not_converged_unresolved_contradictions():
    s = create_session("test")
    s.seen_urls = [f"https://example.com/{i}" for i in range(5)]
    s.novelty_scores = [0.05, 0.03, 0.02]
    s.findings.append(ResearchFinding(
        id="f1", content="contested claim", source_url="https://a.com",
        robustness="contested", counter_evidence=[],
    ))
    result = check_research_convergence(s)
    assert result.converged is False
    assert "contradiction" in result.reason


def test_not_converged_few_novelty_scores():
    s = create_session("test")
    s.seen_urls = [f"https://example.com/{i}" for i in range(5)]
    s.novelty_scores = [0.05]  # Need at least 3
    result = check_research_convergence(s)
    assert result.converged is False


def test_empty_session():
    s = create_session("test")
    result = check_research_convergence(s)
    assert result.converged is False
