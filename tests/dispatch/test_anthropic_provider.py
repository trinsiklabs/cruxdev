"""Tests for Anthropic provider — mocked API calls + JSON extraction."""

import json
import os
from unittest.mock import MagicMock, patch

import pytest

from src.dispatch.providers.anthropic import (
    AnthropicProvider,
    _audit_system_prompt,
    _audit_user_prompt,
    _extract_json,
    _fix_system_prompt,
    _fix_user_prompt,
    _write_system_prompt,
)
from src.dispatch.schema import AuditResult, EvaluationResult, FixResult, WriteResult
from src.dispatch.validation import EscalationRequired


# --- JSON extraction ---


def test_extract_json_raw():
    raw = '{"findings": [], "files_audited": [], "dimensions_checked": []}'
    assert json.loads(_extract_json(raw)) is not None


def test_extract_json_code_block():
    raw = '```json\n{"key": "value"}\n```'
    assert json.loads(_extract_json(raw)) == {"key": "value"}


def test_extract_json_generic_code_block():
    raw = '```\n{"key": "value"}\n```'
    assert json.loads(_extract_json(raw)) == {"key": "value"}


def test_extract_json_with_surrounding_text():
    raw = 'Here is the result:\n{"key": "value"}\nDone.'
    assert json.loads(_extract_json(raw)) == {"key": "value"}


def test_extract_json_nested():
    raw = '{"a": {"b": [1, 2]}}'
    assert json.loads(_extract_json(raw)) == {"a": {"b": [1, 2]}}


def test_extract_json_no_json():
    raw = "no json here"
    assert _extract_json(raw) == "no json here"


def test_extract_json_array():
    raw = '[{"id": 1}]'
    assert json.loads(_extract_json(raw)) == [{"id": 1}]


# --- Prompt construction ---


def test_audit_system_prompt():
    prompt = _audit_system_prompt(["correctness", "security"], "skill ctx")
    assert "correctness" in prompt
    assert "security" in prompt
    assert "skill ctx" in prompt


def test_audit_system_prompt_no_context():
    prompt = _audit_system_prompt(["d"], "")
    assert "Context" not in prompt


def test_audit_user_prompt_file_not_found():
    prompt = _audit_user_prompt(["/nonexistent/file.py"])
    assert "file not found" in prompt


def test_audit_user_prompt_real_file(tmp_path):
    f = tmp_path / "test.py"
    f.write_text("print('hello')")
    prompt = _audit_user_prompt([str(f)])
    assert "print('hello')" in prompt


def test_fix_system_prompt():
    prompt = _fix_system_prompt("ctx")
    assert "fix" in prompt.lower()
    assert "ctx" in prompt


def test_fix_system_prompt_no_context():
    prompt = _fix_system_prompt("")
    assert "Context" not in prompt


def test_fix_user_prompt():
    prompt = _fix_user_prompt("f1", "bug found", "a.py", "code here")
    assert "f1" in prompt
    assert "bug found" in prompt
    assert "a.py" in prompt


def test_write_system_prompt():
    prompt = _write_system_prompt("ctx")
    assert "ctx" in prompt


def test_write_system_prompt_no_context():
    prompt = _write_system_prompt("")
    assert "Context" not in prompt


# --- Provider construction ---


def test_provider_requires_api_key():
    with patch.dict(os.environ, {}, clear=True):
        with pytest.raises(ValueError, match="ANTHROPIC_API_KEY"):
            AnthropicProvider(api_key="")


def test_provider_from_env():
    with patch.dict(os.environ, {"ANTHROPIC_API_KEY": "sk-test"}):
        p = AnthropicProvider()
        assert p.client is not None


def test_provider_explicit_key():
    p = AnthropicProvider(api_key="sk-test-key")
    assert p.model == "claude-sonnet-4-20250514"


# --- Mocked API calls ---


def _mock_provider():
    """Create a provider with a mocked Anthropic client."""
    p = AnthropicProvider(api_key="sk-test")
    p.client = MagicMock()
    return p


def _mock_response(text: str):
    """Create a mock Anthropic response."""
    block = MagicMock()
    block.type = "text"
    block.text = text
    response = MagicMock()
    response.content = [block]
    return response


def test_audit_mocked():
    p = _mock_provider()
    p.client.messages.create.return_value = _mock_response(
        json.dumps({
            "findings": [],
            "files_audited": ["a.py"],
            "dimensions_checked": ["correctness"],
        })
    )

    result = p.audit(["a.py"], ["correctness"], "")
    assert isinstance(result, AuditResult)
    assert result.findings == []
    assert p.client.messages.create.called


def test_audit_with_findings_mocked():
    p = _mock_provider()
    p.client.messages.create.return_value = _mock_response(
        json.dumps({
            "findings": [{
                "id": "f1", "file": "a.py", "dimension": "correctness",
                "severity": "high", "description": "bug", "suggested_fix": "fix",
            }],
            "files_audited": ["a.py"],
            "dimensions_checked": ["correctness"],
        })
    )

    result = p.audit(["a.py"], ["correctness"], "")
    assert len(result.findings) == 1


def test_fix_mocked():
    p = _mock_provider()
    p.client.messages.create.return_value = _mock_response(
        json.dumps({
            "success": True,
            "files_modified": ["a.py"],
            "description": "Fixed the bug",
        })
    )

    result = p.fix("f1", "bug", "a.py", "code", "")
    assert isinstance(result, FixResult)
    assert result.success is True


def test_evaluate_independence_mocked():
    p = _mock_provider()
    p.client.messages.create.return_value = _mock_response(
        json.dumps({
            "independent": True,
            "rationale": "Different files and dimensions",
        })
    )

    a = AuditResult(findings=[], files_audited=["a.py"], dimensions_checked=["d"])
    b = AuditResult(findings=[], files_audited=["b.py"], dimensions_checked=["d"])
    result = p.evaluate_independence(a, b)
    assert isinstance(result, EvaluationResult)
    assert result.independent is True


def test_write_mocked():
    p = _mock_provider()
    p.client.messages.create.return_value = _mock_response(
        json.dumps({
            "content": "# Generated",
            "files_written": ["output.md"],
            "description": "Wrote docs",
        })
    )

    result = p.write("write docs", "")
    assert isinstance(result, WriteResult)


def test_retry_on_invalid_then_succeed():
    p = _mock_provider()
    p.max_retries = 1

    # First call returns invalid, second returns valid
    p.client.messages.create.side_effect = [
        _mock_response("not json"),
        _mock_response(json.dumps({
            "findings": [],
            "files_audited": ["a.py"],
            "dimensions_checked": ["d"],
        })),
    ]

    result = p.audit(["a.py"], ["d"], "")
    assert isinstance(result, AuditResult)
    assert p.client.messages.create.call_count == 2


def test_retry_exhausted():
    p = _mock_provider()
    p.max_retries = 1
    p.client.messages.create.return_value = _mock_response("always invalid")

    with pytest.raises(EscalationRequired):
        p.audit(["a.py"], ["d"], "")


def test_error_feedback_sent_on_retry():
    p = _mock_provider()
    p.max_retries = 1

    p.client.messages.create.side_effect = [
        _mock_response("bad json"),
        _mock_response(json.dumps({
            "findings": [], "files_audited": [], "dimensions_checked": [],
        })),
    ]

    p.audit(["a.py"], ["d"], "")

    # Second call should have 3 messages (original + assistant + error feedback)
    second_call = p.client.messages.create.call_args_list[1]
    messages = second_call.kwargs.get("messages") or second_call[1].get("messages", [])
    assert len(messages) == 3


# --- Integration test (requires API key, skipped in CI) ---


@pytest.mark.skipif(
    not os.environ.get("ANTHROPIC_API_KEY"),
    reason="ANTHROPIC_API_KEY not set",
)
def test_real_audit():
    """Integration test with real API — only runs when key is available."""
    p = AnthropicProvider(model="claude-haiku-4-5-20251001", max_tokens=1024)
    result = p.audit(
        files=[],
        dimensions=["correctness"],
        skill_context="",
    )
    assert isinstance(result, AuditResult)
