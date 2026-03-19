"""Convergence engine state data structures.

All state is deterministic — no LLM calls, no AI inference.
Pure data structures with enums, dataclasses, and simple logic.
"""

from enum import Enum
from dataclasses import dataclass, field
from typing import Optional
import time


class ConvergencePhase(Enum):
    PLANNING = "planning"
    PLAN_AUDITING = "plan_auditing"
    VIABILITY = "viability"
    EXECUTING = "executing"
    CODE_AUDITING = "code_auditing"
    DOC_AUDITING = "doc_auditing"
    E2E_TESTING = "e2e_testing"
    PATTERNS_UPDATE = "patterns_update"
    CONVERGED = "converged"
    ESCALATED = "escalated"


class FindingSeverity(Enum):
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


@dataclass
class Finding:
    id: str
    file: str
    dimension: str
    severity: FindingSeverity
    description: str
    suggested_fix: str
    fixed: bool = False


@dataclass
class TestRunResult:
    passed: bool
    total: int
    failures: int
    coverage: Optional[float] = None
    output: str = ""
    duration_seconds: float = 0.0


@dataclass
class RoundResult:
    round: int
    phase: ConvergencePhase
    findings: list[Finding]
    findings_fixed: int
    timestamp: float


@dataclass
class ConvergenceState:
    plan_file: str
    phase: ConvergencePhase = ConvergencePhase.PLANNING
    round: int = 0
    max_rounds: int = 5
    consecutive_clean: int = 0
    convergence_threshold: int = 2  # Two consecutive clean passes
    failures: dict[str, int] = field(default_factory=dict)
    max_failures: int = 3
    deadline: Optional[float] = None
    timeout_per_task: float = 900.0  # 15 minutes
    history: list[RoundResult] = field(default_factory=list)
    escalation_reason: Optional[str] = None
    created_at: float = field(default_factory=time.time)
    updated_at: float = field(default_factory=time.time)
