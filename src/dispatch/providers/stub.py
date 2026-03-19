"""Configurable stub LLM provider for testing.

Simulates 6 modes:
1. clean — always returns zero findings
2. findings — returns configurable findings
3. persistent — returns same findings every time (never fixes)
4. schema_invalid — returns malformed JSON
5. slow — sleeps before responding (for timeout testing)
6. intermittent — alternates between clean and findings
"""

import json
import time
from enum import Enum

from ..llm import LLMDispatcher
from ..schema import AuditFinding, AuditResult, EvaluationResult, FixResult, WriteResult


class StubMode(Enum):
    CLEAN = "clean"
    FINDINGS = "findings"
    PERSISTENT = "persistent"
    SCHEMA_INVALID = "schema_invalid"
    SLOW = "slow"
    INTERMITTENT = "intermittent"


class StubProvider(LLMDispatcher):
    """Configurable stub for deterministic testing."""

    def __init__(
        self,
        mode: StubMode = StubMode.CLEAN,
        findings: list[AuditFinding] | None = None,
        delay_seconds: float = 0.0,
    ):
        self.mode = mode
        self._findings = findings or []
        self._delay_seconds = delay_seconds
        self._call_count = 0

    def audit(
        self,
        files: list[str],
        dimensions: list[str],
        skill_context: str,
    ) -> AuditResult:
        self._call_count += 1
        self._maybe_delay()

        if self.mode == StubMode.CLEAN:
            return AuditResult(
                findings=[], files_audited=files, dimensions_checked=dimensions
            )

        if self.mode == StubMode.FINDINGS:
            return AuditResult(
                findings=self._findings,
                files_audited=files,
                dimensions_checked=dimensions,
            )

        if self.mode == StubMode.PERSISTENT:
            return AuditResult(
                findings=self._findings,
                files_audited=files,
                dimensions_checked=dimensions,
            )

        if self.mode == StubMode.SCHEMA_INVALID:
            raise ValueError("Simulated schema validation failure")

        if self.mode == StubMode.SLOW:
            return AuditResult(
                findings=[], files_audited=files, dimensions_checked=dimensions
            )

        if self.mode == StubMode.INTERMITTENT:
            if self._call_count % 2 == 0:
                return AuditResult(
                    findings=[],
                    files_audited=files,
                    dimensions_checked=dimensions,
                )
            return AuditResult(
                findings=self._findings,
                files_audited=files,
                dimensions_checked=dimensions,
            )

        return AuditResult(  # pragma: no cover
            findings=[], files_audited=files, dimensions_checked=dimensions
        )

    def fix(
        self,
        finding_id: str,
        finding_description: str,
        file_path: str,
        file_content: str,
        skill_context: str,
    ) -> FixResult:
        self._call_count += 1
        self._maybe_delay()

        if self.mode == StubMode.PERSISTENT:
            return FixResult(
                success=False, files_modified=[], description="Fix failed (persistent mode)"
            )

        if self.mode == StubMode.SCHEMA_INVALID:
            raise ValueError("Simulated schema validation failure")

        return FixResult(
            success=True,
            files_modified=[file_path],
            description=f"Fixed {finding_id}",
        )

    def evaluate_independence(
        self,
        pass_a: AuditResult,
        pass_b: AuditResult,
    ) -> EvaluationResult:
        self._call_count += 1
        return EvaluationResult(
            independent=True,
            rationale="Stub: always independent",
        )

    def write(self, spec: str, skill_context: str) -> WriteResult:
        self._call_count += 1
        return WriteResult(
            content=f"# Generated from spec\n{spec}",
            files_written=["output.md"],
            description="Stub write",
        )

    def _maybe_delay(self) -> None:
        if self._delay_seconds > 0:
            time.sleep(self._delay_seconds)

    @property
    def call_count(self) -> int:
        return self._call_count
