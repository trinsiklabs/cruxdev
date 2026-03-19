"""Tests for subagent coordination — parallel dispatch + synthesis + conflict."""

import pytest

from src.dispatch.providers.stub import StubMode, StubProvider
from src.dispatch.schema import AuditFinding
from src.dispatch.subagent import SubagentCoordinator, resolve_conflict
from src.engine.state import Finding, FindingSeverity


def _make_finding(id="f1", file="a.py", dim="correctness", sev="high", desc="bug", fix="fix"):
    return AuditFinding(id=id, file=file, dimension=dim, severity=sev, description=desc, suggested_fix=fix)


def _make_engine_finding(id="f1", file="a.py", dim="correctness", sev=FindingSeverity.HIGH, desc="bug", fix="fix"):
    return Finding(id=id, file=file, dimension=dim, severity=sev, description=desc, suggested_fix=fix)


# --- Parallel audit ---


def test_parallel_audit_basic():
    coord = SubagentCoordinator()
    agents = [StubProvider(mode=StubMode.CLEAN), StubProvider(mode=StubMode.CLEAN)]
    file_sets = [["a.py"], ["b.py"]]

    results = coord.parallel_audit(agents, file_sets, ["correctness"])
    assert len(results) == 2
    assert all(len(r.findings) == 0 for r in results)


def test_parallel_audit_with_findings():
    findings = [_make_finding()]
    coord = SubagentCoordinator()
    agents = [
        StubProvider(mode=StubMode.FINDINGS, findings=findings),
        StubProvider(mode=StubMode.CLEAN),
    ]
    file_sets = [["a.py"], ["b.py"]]

    results = coord.parallel_audit(agents, file_sets, ["correctness"])
    assert len(results[0].findings) == 1
    assert len(results[1].findings) == 0


def test_parallel_audit_empty():
    coord = SubagentCoordinator()
    assert coord.parallel_audit([], [], ["d"]) == []


def test_parallel_audit_concurrency():
    """Verify agents actually run concurrently."""
    import time

    coord = SubagentCoordinator()
    agents = [
        StubProvider(mode=StubMode.SLOW, delay_seconds=0.2),
        StubProvider(mode=StubMode.SLOW, delay_seconds=0.2),
    ]
    file_sets = [["a.py"], ["b.py"]]

    start = time.time()
    results = coord.parallel_audit(agents, file_sets, ["d"], max_workers=2)
    elapsed = time.time() - start

    assert len(results) == 2
    # Should take ~0.2s (parallel), not ~0.4s (sequential)
    assert elapsed < 0.35


# --- Synthesis ---


def test_synthesize_dedup():
    coord = SubagentCoordinator()
    from src.dispatch.schema import AuditResult

    r1 = AuditResult(
        findings=[_make_finding(id="f1", desc="same bug")],
        files_audited=["a.py"], dimensions_checked=["d"],
    )
    r2 = AuditResult(
        findings=[_make_finding(id="f2", desc="same bug")],  # Same desc + file + dim
        files_audited=["a.py"], dimensions_checked=["d"],
    )

    findings = coord.synthesize([r1, r2])
    assert len(findings) == 1  # Deduplicated


def test_synthesize_keeps_different():
    coord = SubagentCoordinator()
    from src.dispatch.schema import AuditResult

    r1 = AuditResult(
        findings=[_make_finding(id="f1", desc="bug A")],
        files_audited=["a.py"], dimensions_checked=["d"],
    )
    r2 = AuditResult(
        findings=[_make_finding(id="f2", desc="bug B")],
        files_audited=["a.py"], dimensions_checked=["d"],
    )

    findings = coord.synthesize([r1, r2])
    assert len(findings) == 2


def test_synthesize_respects_size_limit():
    coord = SubagentCoordinator()
    from src.dispatch.schema import AuditResult

    many_findings = [_make_finding(id=f"f{i}", desc=f"bug {i}") for i in range(100)]
    r = AuditResult(findings=many_findings, files_audited=["a.py"], dimensions_checked=["d"])

    findings = coord.synthesize([r], max_synthesis_size=10)
    assert len(findings) == 10


def test_synthesize_prioritizes_high_severity():
    coord = SubagentCoordinator()
    from src.dispatch.schema import AuditResult

    findings = [
        _make_finding(id="low1", sev="low", desc="low 1"),
        _make_finding(id="high1", sev="high", desc="high 1"),
        _make_finding(id="med1", sev="medium", desc="med 1"),
    ]
    r = AuditResult(findings=findings, files_audited=["a.py"], dimensions_checked=["d"])

    result = coord.synthesize([r], max_synthesis_size=2)
    assert len(result) == 2
    # High should be first
    assert result[0].severity == FindingSeverity.HIGH


def test_synthesize_empty():
    coord = SubagentCoordinator()
    assert coord.synthesize([]) == []


# --- Conflict resolution ---


def test_conflict_safety_wins():
    a = _make_engine_finding(dim="security", sev=FindingSeverity.LOW, desc="security issue")
    b = _make_engine_finding(dim="performance", sev=FindingSeverity.HIGH, desc="slow code")
    assert resolve_conflict(a, b) == a


def test_conflict_safety_b_wins():
    a = _make_engine_finding(dim="performance", sev=FindingSeverity.HIGH, desc="slow")
    b = _make_engine_finding(dim="safety", sev=FindingSeverity.LOW, desc="unsafe")
    assert resolve_conflict(a, b) == b


def test_conflict_higher_severity_wins():
    a = _make_engine_finding(sev=FindingSeverity.HIGH, desc="high bug")
    b = _make_engine_finding(sev=FindingSeverity.LOW, desc="low bug")
    assert resolve_conflict(a, b) == a


def test_conflict_higher_severity_b_wins():
    a = _make_engine_finding(sev=FindingSeverity.LOW, desc="low bug")
    b = _make_engine_finding(sev=FindingSeverity.HIGH, desc="high bug")
    assert resolve_conflict(a, b) == b


def test_conflict_more_specific_wins():
    a = _make_engine_finding(desc="a very detailed description of the bug")
    b = _make_engine_finding(desc="short")
    assert resolve_conflict(a, b) == a


def test_conflict_more_specific_b_wins():
    a = _make_engine_finding(desc="short")
    b = _make_engine_finding(desc="a very detailed description of the bug")
    assert resolve_conflict(a, b) == b


def test_conflict_same_everything_first_wins():
    a = _make_engine_finding(id="first", desc="same")
    b = _make_engine_finding(id="second", desc="same")
    assert resolve_conflict(a, b) == a


def test_conflict_error_handling_is_safety():
    a = _make_engine_finding(dim="error_handling", sev=FindingSeverity.MEDIUM, desc="missing try")
    b = _make_engine_finding(dim="performance", sev=FindingSeverity.HIGH, desc="O(n^2)")
    assert resolve_conflict(a, b) == a
