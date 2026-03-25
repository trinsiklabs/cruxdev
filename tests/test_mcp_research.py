"""Tests for MCP research tool wrappers."""

import json

import pytest

from src.mcp_server import (
    counter_research,
    init,
    research_status,
    research_topic,
    verify_research_sources,
)


@pytest.fixture(autouse=True)
def setup(tmp_path):
    init(str(tmp_path / "state"))
    yield


class TestResearchTopic:
    def test_creates_session(self):
        result = json.loads(research_topic("AI convergence tools"))
        assert "session_id" in result
        assert result["topic"] == "AI convergence tools"
        assert result["current_pass"] == 1

    def test_with_sub_questions(self):
        result = json.loads(research_topic(
            "Testing frameworks",
            sub_questions="coverage tools,mutation testing",
        ))
        assert len(result["sub_questions"]) == 2


class TestResearchStatus:
    def test_returns_instructions(self):
        result = json.loads(research_status("abc12345"))
        assert result["session_id"] == "abc12345"
        assert "instructions" in result


class TestVerifyResearchSources:
    def test_verifies_sources(self):
        result = json.loads(verify_research_sources(
            "f1",
            "https://example.com,https://test.io",
        ))
        assert result["finding_id"] == "f1"
        assert result["total_sources"] == 2

    def test_empty_sources(self):
        result = json.loads(verify_research_sources("f1", ""))
        assert result["total_sources"] == 0
        assert result["overall_verified"] is False


class TestCounterResearch:
    def test_basic_claim(self):
        result = json.loads(counter_research("Python is fast"))
        assert result["claim"] == "Python is fast"
        assert result["robustness"] == "robust"
        assert len(result["negation_queries"]) >= 3

    def test_with_counter_evidence(self):
        result = json.loads(counter_research(
            "X is best",
            counter_evidence="Benchmark shows Y is faster|Study found Z better",
            supporting_count=1,
        ))
        assert result["is_contested"] is True
        assert len(result["counter_evidence"]) == 2

    def test_with_alternatives(self):
        result = json.loads(counter_research(
            "TDD always helps",
            alternative_explanations="Depends on team size|Domain matters",
        ))
        assert len(result["alternative_explanations"]) == 2
