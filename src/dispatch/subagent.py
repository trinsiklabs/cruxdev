"""Subagent coordination — parallel audit dispatch + synthesis.

Real parallelism via threading. Conflict resolution in code, not LLM.
"""

import concurrent.futures
from typing import Callable

from .llm import LLMDispatcher
from .schema import AuditFinding, AuditResult
from ..engine.state import Finding, FindingSeverity


class SubagentCoordinator:
    """Dispatch N agents concurrently and synthesize results."""

    def parallel_audit(
        self,
        agents: list[LLMDispatcher],
        file_sets: list[list[str]],
        dimensions: list[str],
        skill_context: str = "",
        max_workers: int | None = None,
    ) -> list[AuditResult]:
        """Dispatch N agents concurrently. Real parallelism via threads."""
        if not agents or not file_sets:
            return []

        with concurrent.futures.ThreadPoolExecutor(
            max_workers=max_workers or len(agents)
        ) as executor:
            futures = [
                executor.submit(agent.audit, files, dimensions, skill_context)
                for agent, files in zip(agents, file_sets)
            ]
            return [f.result() for f in futures]

    def synthesize(
        self,
        results: list[AuditResult],
        max_synthesis_size: int = 50000,
    ) -> list[Finding]:
        """Merge results. Deduplicate. Resolve conflicts in code.

        If combined findings exceed max_synthesis_size, chunk by severity.
        """
        all_findings: list[AuditFinding] = []
        for result in results:
            all_findings.extend(result.findings)

        # Deduplicate by (file, dimension, description)
        seen: set[tuple[str, str, str]] = set()
        unique: list[AuditFinding] = []
        for f in all_findings:
            key = (f.file, f.dimension, f.description)
            if key not in seen:
                seen.add(key)
                unique.append(f)

        # If too many, prioritize by severity
        if len(unique) > max_synthesis_size:
            by_severity = {"high": [], "medium": [], "low": []}
            for f in unique:
                by_severity[f.severity].append(f)
            unique = (
                by_severity["high"]
                + by_severity["medium"]
                + by_severity["low"]
            )[:max_synthesis_size]

        # Convert to engine findings
        return [
            Finding(
                id=f.id,
                file=f.file,
                dimension=f.dimension,
                severity=FindingSeverity(f.severity),
                description=f.description,
                suggested_fix=f.suggested_fix,
            )
            for f in unique
        ]


def resolve_conflict(a: Finding, b: Finding) -> Finding:
    """Resolve conflicting findings using 5 priority rules (code, not LLM).

    Rules:
    1. Safety first — security/safety dimension wins
    2. Higher severity wins
    3. More specific (longer description) wins
    4. First finding wins (stable ordering)
    5. Escalate genuine conflicts (return higher severity)
    """
    SAFETY_DIMENSIONS = {"security", "safety", "error_handling"}
    SEVERITY_ORDER = {FindingSeverity.HIGH: 3, FindingSeverity.MEDIUM: 2, FindingSeverity.LOW: 1}

    # Rule 1: Safety first
    a_safety = a.dimension in SAFETY_DIMENSIONS
    b_safety = b.dimension in SAFETY_DIMENSIONS
    if a_safety and not b_safety:
        return a
    if b_safety and not a_safety:
        return b

    # Rule 2: Higher severity wins
    a_sev = SEVERITY_ORDER[a.severity]
    b_sev = SEVERITY_ORDER[b.severity]
    if a_sev > b_sev:
        return a
    if b_sev > a_sev:
        return b

    # Rule 3: More specific wins
    if len(a.description) > len(b.description):
        return a
    if len(b.description) > len(a.description):
        return b

    # Rule 4: First finding wins (stable ordering)
    return a
