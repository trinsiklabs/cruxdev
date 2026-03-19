"""Tests for the stub LLM provider — all 6 simulation modes."""

import pytest

from src.dispatch.providers.stub import StubMode, StubProvider
from src.dispatch.schema import AuditFinding, AuditResult


def _make_findings() -> list[AuditFinding]:
    return [
        AuditFinding(
            id="f1", file="a.py", dimension="correctness",
            severity="high", description="bug", suggested_fix="fix it",
        )
    ]


# --- Clean mode ---

def test_clean_mode_audit():
    p = StubProvider(mode=StubMode.CLEAN)
    result = p.audit(["a.py"], ["correctness"], "ctx")
    assert result.findings == []
    assert result.files_audited == ["a.py"]


def test_clean_mode_fix():
    p = StubProvider(mode=StubMode.CLEAN)
    result = p.fix("f1", "bug", "a.py", "content", "ctx")
    assert result.success is True


# --- Findings mode ---

def test_findings_mode():
    findings = _make_findings()
    p = StubProvider(mode=StubMode.FINDINGS, findings=findings)
    result = p.audit(["a.py"], ["correctness"], "ctx")
    assert len(result.findings) == 1
    assert result.findings[0].id == "f1"


# --- Persistent mode ---

def test_persistent_mode_audit():
    findings = _make_findings()
    p = StubProvider(mode=StubMode.PERSISTENT, findings=findings)
    r1 = p.audit(["a.py"], ["d"], "ctx")
    r2 = p.audit(["a.py"], ["d"], "ctx")
    assert len(r1.findings) == 1
    assert len(r2.findings) == 1


def test_persistent_mode_fix_fails():
    p = StubProvider(mode=StubMode.PERSISTENT)
    result = p.fix("f1", "bug", "a.py", "content", "ctx")
    assert result.success is False


# --- Schema invalid mode ---

def test_schema_invalid_mode_audit():
    p = StubProvider(mode=StubMode.SCHEMA_INVALID)
    with pytest.raises(ValueError, match="schema validation"):
        p.audit(["a.py"], ["d"], "ctx")


def test_schema_invalid_mode_fix():
    p = StubProvider(mode=StubMode.SCHEMA_INVALID)
    with pytest.raises(ValueError, match="schema validation"):
        p.fix("f1", "bug", "a.py", "content", "ctx")


# --- Slow mode ---

def test_slow_mode():
    import time
    p = StubProvider(mode=StubMode.SLOW, delay_seconds=0.1)
    start = time.time()
    result = p.audit(["a.py"], ["d"], "ctx")
    elapsed = time.time() - start
    assert elapsed >= 0.1
    assert result.findings == []


# --- Intermittent mode ---

def test_intermittent_mode():
    findings = _make_findings()
    p = StubProvider(mode=StubMode.INTERMITTENT, findings=findings)
    r1 = p.audit(["a.py"], ["d"], "ctx")  # call 1: odd → findings
    r2 = p.audit(["a.py"], ["d"], "ctx")  # call 2: even → clean
    r3 = p.audit(["a.py"], ["d"], "ctx")  # call 3: odd → findings
    assert len(r1.findings) == 1
    assert len(r2.findings) == 0
    assert len(r3.findings) == 1


# --- Evaluate independence ---

def test_evaluate_independence():
    p = StubProvider(mode=StubMode.CLEAN)
    a = AuditResult(findings=[], files_audited=["a.py"], dimensions_checked=["d"])
    b = AuditResult(findings=[], files_audited=["b.py"], dimensions_checked=["d"])
    result = p.evaluate_independence(a, b)
    assert result.independent is True


# --- Write ---

def test_write():
    p = StubProvider(mode=StubMode.CLEAN)
    result = p.write("build a thing", "ctx")
    assert "build a thing" in result.content


# --- Call count ---

def test_call_count():
    p = StubProvider(mode=StubMode.CLEAN)
    assert p.call_count == 0
    p.audit(["a.py"], ["d"], "ctx")
    assert p.call_count == 1
    p.fix("f1", "d", "a.py", "c", "ctx")
    assert p.call_count == 2
