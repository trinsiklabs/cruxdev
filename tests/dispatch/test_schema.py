"""Tests for schema validation."""

import pytest
from pydantic import ValidationError

from src.dispatch.schema import (
    AuditFinding,
    AuditResult,
    EvaluationResult,
    FixResult,
    WriteResult,
)


def test_audit_finding_valid():
    f = AuditFinding(
        id="f1", file="foo.py", dimension="correctness",
        severity="high", description="bug", suggested_fix="fix it",
    )
    assert f.severity == "high"


def test_audit_finding_invalid_severity():
    with pytest.raises(ValidationError):
        AuditFinding(
            id="f1", file="foo.py", dimension="correctness",
            severity="critical", description="bug", suggested_fix="fix",
        )


def test_audit_result_valid():
    r = AuditResult(
        findings=[
            AuditFinding(
                id="f1", file="a.py", dimension="d",
                severity="low", description="d", suggested_fix="f",
            )
        ],
        files_audited=["a.py"],
        dimensions_checked=["correctness"],
    )
    assert len(r.findings) == 1


def test_audit_result_empty():
    r = AuditResult(findings=[], files_audited=[], dimensions_checked=[])
    assert r.findings == []


def test_fix_result_valid():
    r = FixResult(success=True, files_modified=["a.py"], description="fixed")
    assert r.success is True


def test_evaluation_result_valid():
    r = EvaluationResult(independent=True, rationale="different scope")
    assert r.independent is True


def test_write_result_valid():
    r = WriteResult(content="hello", files_written=["a.md"], description="wrote it")
    assert r.content == "hello"


def test_audit_finding_missing_field():
    with pytest.raises(ValidationError):
        AuditFinding(id="f1", file="foo.py", dimension="d", severity="high")


def test_audit_result_from_dict():
    data = {
        "findings": [
            {"id": "f1", "file": "a.py", "dimension": "d",
             "severity": "medium", "description": "d", "suggested_fix": "f"}
        ],
        "files_audited": ["a.py"],
        "dimensions_checked": ["d"],
    }
    r = AuditResult.model_validate(data)
    assert r.findings[0].severity == "medium"
