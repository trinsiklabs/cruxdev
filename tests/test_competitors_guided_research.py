"""Tests for guided competitor research — 5-pass enforcer."""

from src.competitors.guided_research import (
    ResearchPass,
    ResearchState,
    clear_session,
    get_next_step,
    get_or_create_session,
    list_sessions,
    start_research,
    submit_pass_result,
    _active_sessions,
)


class TestStartResearch:
    def test_creates_state(self):
        state = start_research("Rival", "https://rival.com", "AI tools")
        assert state.competitor_name == "Rival"
        assert state.current_pass == ResearchPass.BROAD
        assert state.passes_completed == 0
        assert state.is_done is False


class TestGetNextStep:
    def test_first_step_is_broad(self):
        state = start_research("Rival", "https://rival.com")
        step = get_next_step(state)
        assert step["pass_name"] == "broad"
        assert step["pass_number"] == 1
        assert step["is_done"] is False
        assert len(step["search_queries"]) >= 1
        assert "Rival" in step["search_queries"][0]

    def test_done_state(self):
        state = start_research("X", "u")
        state.current_pass = ResearchPass.DONE
        step = get_next_step(state)
        assert step["is_done"] is True

    def test_queries_use_competitor_name(self):
        state = start_research("Acme Tool", "https://acme.io")
        step = get_next_step(state)
        assert any("Acme Tool" in q for q in step["search_queries"])

    def test_queries_use_domain(self):
        state = start_research("X", "https://example.com/path")
        state.current_pass = ResearchPass.ACADEMIC
        step = get_next_step(state)
        assert any("example.com" in q for q in step["search_queries"])

    def test_previous_findings_tracked(self):
        state = start_research("X", "u")
        submit_pass_result(state, ["found something"])
        step = get_next_step(state)
        assert len(step["previous_findings"]) == 1


class TestSubmitPassResult:
    def test_advances_pass(self):
        state = start_research("X", "u")
        result = submit_pass_result(state, ["Found feature list"])
        assert result["accepted"] is True
        assert state.current_pass == ResearchPass.ACADEMIC

    def test_full_sequence(self):
        state = start_research("X", "u")
        # Broad
        submit_pass_result(state, ["desc"])
        assert state.current_pass == ResearchPass.ACADEMIC
        # Academic
        submit_pass_result(state, ["tech details"])
        assert state.current_pass == ResearchPass.PRACTITIONER
        # Practitioner
        submit_pass_result(state, ["user review"])
        assert state.current_pass == ResearchPass.CONTRARIAN
        # Contrarian — MUST have findings
        submit_pass_result(state, ["criticism found"])
        assert state.current_pass == ResearchPass.PRIMARY
        # Primary
        submit_pass_result(state, ["verified"])
        assert state.current_pass == ResearchPass.VERIFY
        # Verify
        submit_pass_result(state, ["compiled"])
        assert state.current_pass == ResearchPass.DONE
        assert state.is_done

    def test_contrarian_rejects_empty(self):
        state = start_research("X", "u")
        # Advance to contrarian
        submit_pass_result(state, ["a"])
        submit_pass_result(state, ["b"])
        submit_pass_result(state, ["c"])
        assert state.current_pass == ResearchPass.CONTRARIAN
        # Try to submit empty contrarian
        result = submit_pass_result(state, [])
        assert result["accepted"] is False
        assert "MUST" in result["error"]
        # Still on contrarian
        assert state.current_pass == ResearchPass.CONTRARIAN

    def test_records_sources(self):
        state = start_research("X", "u")
        submit_pass_result(state, ["found"], sources=["https://src.com"])
        assert state.pass_results[0].sources == ["https://src.com"]

    def test_records_queries(self):
        state = start_research("X", "u")
        submit_pass_result(state, ["found"], search_queries_used=["query 1"])
        assert state.pass_results[0].search_queries_used == ["query 1"]

    def test_profile_updates(self):
        state = start_research("X", "u")
        submit_pass_result(state, ["found"], profile_updates={"pricing": "$10/mo"})
        assert state.profile_data["pricing"] == "$10/mo"

    def test_profile_updates_accumulate(self):
        state = start_research("X", "u")
        submit_pass_result(state, ["a"], profile_updates={"pricing": "$10"})
        submit_pass_result(state, ["b"], profile_updates={"tech_stack": "Python"})
        assert state.profile_data["pricing"] == "$10"
        assert state.profile_data["tech_stack"] == "Python"


class TestSessionManagement:
    def setup_method(self):
        _active_sessions.clear()

    def test_create_new(self):
        state, is_new = get_or_create_session("Rival", "https://rival.com")
        assert is_new is True
        assert state.competitor_name == "Rival"

    def test_get_existing(self):
        get_or_create_session("Rival", "https://rival.com")
        state, is_new = get_or_create_session("Rival")
        assert is_new is False

    def test_case_insensitive(self):
        get_or_create_session("Rival", "https://rival.com")
        _, is_new = get_or_create_session("rival")
        assert is_new is False

    def test_clear(self):
        get_or_create_session("Rival", "u")
        assert clear_session("Rival") is True
        assert clear_session("Rival") is False

    def test_list_sessions(self):
        _active_sessions.clear()
        get_or_create_session("A", "u")
        get_or_create_session("B", "u")
        sessions = list_sessions()
        assert len(sessions) == 2

    def teardown_method(self):
        _active_sessions.clear()


class TestResearchState:
    def test_to_dict(self):
        state = start_research("X", "https://x.com", "AI")
        d = state.to_dict()
        assert d["competitor_name"] == "X"
        assert d["current_pass"] == "broad"
        assert d["is_done"] is False
