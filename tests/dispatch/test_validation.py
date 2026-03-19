"""Tests for validation + retry logic."""

import json

import pytest

from src.dispatch.schema import AuditResult
from src.dispatch.validation import EscalationRequired, validate_and_retry, validate_json


def test_validate_json_valid():
    raw = json.dumps({
        "findings": [],
        "files_audited": ["a.py"],
        "dimensions_checked": ["d"],
    })
    result = validate_json(raw, AuditResult)
    assert isinstance(result, AuditResult)


def test_validate_json_invalid_json():
    with pytest.raises(Exception):
        validate_json("not json", AuditResult)


def test_validate_json_wrong_schema():
    raw = json.dumps({"wrong": "shape"})
    with pytest.raises(Exception):
        validate_json(raw, AuditResult)


def test_validate_and_retry_success_first_try():
    valid = json.dumps({
        "findings": [],
        "files_audited": [],
        "dimensions_checked": [],
    })

    def llm_call(**kwargs):
        return valid

    result = validate_and_retry(llm_call, AuditResult)
    assert isinstance(result, AuditResult)


def test_validate_and_retry_fail_then_succeed():
    calls = []

    def llm_call(**kwargs):
        calls.append(kwargs)
        if len(calls) == 1:
            return "invalid json"
        return json.dumps({
            "findings": [],
            "files_audited": [],
            "dimensions_checked": [],
        })

    result = validate_and_retry(llm_call, AuditResult, max_retries=1)
    assert isinstance(result, AuditResult)
    assert len(calls) == 2
    assert "error_feedback" in calls[1]


def test_validate_and_retry_all_fail():
    def llm_call(**kwargs):
        return "always invalid"

    with pytest.raises(EscalationRequired, match="failed validation"):
        validate_and_retry(llm_call, AuditResult, max_retries=2)


def test_validate_and_retry_zero_retries():
    def llm_call(**kwargs):
        return "invalid"

    with pytest.raises(EscalationRequired):
        validate_and_retry(llm_call, AuditResult, max_retries=0)
