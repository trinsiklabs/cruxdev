"""Tests for Ollama provider — mocked HTTP calls."""

import json
from unittest.mock import MagicMock, patch

import pytest

from src.dispatch.providers.ollama import OllamaProvider
from src.dispatch.schema import AuditResult, EvaluationResult, FixResult, WriteResult
from src.dispatch.validation import EscalationRequired


def _mock_provider():
    """Create provider with mocked httpx client."""
    p = OllamaProvider(model="test-model")
    p._client = MagicMock()
    return p


def _mock_response(json_str: str):
    """Create a mock httpx response."""
    resp = MagicMock()
    resp.json.return_value = {"response": json_str}
    resp.raise_for_status = MagicMock()
    return resp


def test_audit_mocked():
    p = _mock_provider()
    p._client.post.return_value = _mock_response(
        json.dumps({
            "findings": [],
            "files_audited": ["a.py"],
            "dimensions_checked": ["d"],
        })
    )

    result = p.audit(["a.py"], ["d"], "")
    assert isinstance(result, AuditResult)
    assert result.findings == []


def test_fix_mocked():
    p = _mock_provider()
    p._client.post.return_value = _mock_response(
        json.dumps({
            "success": True,
            "files_modified": ["a.py"],
            "description": "fixed",
        })
    )

    result = p.fix("f1", "bug", "a.py", "code", "")
    assert isinstance(result, FixResult)
    assert result.success is True


def test_evaluate_independence_mocked():
    p = _mock_provider()
    p._client.post.return_value = _mock_response(
        json.dumps({"independent": True, "rationale": "different"})
    )

    a = AuditResult(findings=[], files_audited=["a.py"], dimensions_checked=["d"])
    b = AuditResult(findings=[], files_audited=["b.py"], dimensions_checked=["d"])
    result = p.evaluate_independence(a, b)
    assert isinstance(result, EvaluationResult)


def test_write_mocked():
    p = _mock_provider()
    p._client.post.return_value = _mock_response(
        json.dumps({
            "content": "output",
            "files_written": ["out.md"],
            "description": "wrote it",
        })
    )

    result = p.write("spec", "")
    assert isinstance(result, WriteResult)


def test_retry_with_error_feedback():
    p = _mock_provider()
    p.max_retries = 1
    p._client.post.side_effect = [
        _mock_response("bad json"),
        _mock_response(json.dumps({
            "findings": [], "files_audited": [], "dimensions_checked": [],
        })),
    ]

    result = p.audit(["a.py"], ["d"], "")
    assert isinstance(result, AuditResult)
    assert p._client.post.call_count == 2


def test_retry_exhausted():
    p = _mock_provider()
    p.max_retries = 0
    p._client.post.return_value = _mock_response("invalid")

    with pytest.raises(EscalationRequired):
        p.audit(["a.py"], ["d"], "")


def test_provider_default_config():
    p = OllamaProvider()
    assert p.model == "llama3.2"
    assert "localhost" in p.base_url
