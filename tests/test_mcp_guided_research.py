"""Tests for guided research MCP tools."""

import json

import pytest

from src.mcp_server import (
    init,
    research_competitor_list,
    research_competitor_next_step,
    research_competitor_start,
    research_competitor_submit,
)
from src.competitors.guided_research import _active_sessions


@pytest.fixture(autouse=True)
def setup(tmp_path):
    init(str(tmp_path / "state"))
    _active_sessions.clear()
    yield
    _active_sessions.clear()


class TestResearchCompetitorStart:
    def test_starts_session(self):
        result = json.loads(research_competitor_start("Rival", "https://rival.com", "AI tools"))
        assert result["session_created"] is True
        assert result["competitor"] == "Rival"
        assert result["step"]["pass_name"] == "broad"
        assert result["step"]["is_done"] is False

    def test_returns_existing(self):
        research_competitor_start("Rival", "https://rival.com")
        result = json.loads(research_competitor_start("Rival"))
        assert result["session_created"] is False


class TestResearchCompetitorNextStep:
    def test_returns_current_step(self):
        research_competitor_start("X", "https://x.com")
        result = json.loads(research_competitor_next_step("X"))
        assert result["step"]["pass_name"] == "broad"


class TestResearchCompetitorSubmit:
    def test_advances_pass(self):
        research_competitor_start("X", "https://x.com")
        result = json.loads(research_competitor_submit("X", "Found features|Found pricing"))
        assert result["accepted"] is True
        assert result["next"]["pass_name"] == "academic"

    def test_contrarian_enforced(self):
        research_competitor_start("X", "https://x.com")
        # Advance to contrarian
        research_competitor_submit("X", "broad findings")
        research_competitor_submit("X", "academic findings")
        research_competitor_submit("X", "practitioner findings")
        # Try empty contrarian
        result = json.loads(research_competitor_submit("X", ""))
        assert result["accepted"] is False
        assert "MUST" in result["error"]

    def test_with_sources(self):
        research_competitor_start("X", "https://x.com")
        result = json.loads(research_competitor_submit(
            "X", "finding", sources="https://src1.com,https://src2.com"
        ))
        assert result["accepted"] is True

    def test_with_profile_updates(self):
        research_competitor_start("X", "https://x.com")
        result = json.loads(research_competitor_submit(
            "X", "finding", profile_updates='{"pricing": "$10/mo"}'
        ))
        assert result["accepted"] is True

    def test_bad_profile_json(self):
        research_competitor_start("X", "https://x.com")
        result = json.loads(research_competitor_submit(
            "X", "finding", profile_updates="not json"
        ))
        assert result["accepted"] is True  # Still accepts, just ignores bad updates

    def test_full_flow(self):
        research_competitor_start("X", "https://x.com")
        research_competitor_submit("X", "broad")
        research_competitor_submit("X", "academic")
        research_competitor_submit("X", "practitioner")
        research_competitor_submit("X", "criticism found")
        research_competitor_submit("X", "primary verified")
        result = json.loads(research_competitor_submit("X", "compiled"))
        assert result["next"]["is_done"] is True


class TestResearchCompetitorList:
    def test_lists_sessions(self):
        research_competitor_start("A", "u")
        research_competitor_start("B", "u")
        result = json.loads(research_competitor_list())
        assert len(result) == 2
