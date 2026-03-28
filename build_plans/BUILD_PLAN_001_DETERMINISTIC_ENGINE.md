# BUILD_PLAN_001: Deterministic Convergence Engine

**Created:** 2026-03-18
**Last Updated:** 2026-03-18
**Status:** CONVERGED
**Goal:** Build the convergence engine as a Python MCP server with deterministic state management. The engine owns all loops, counters, timeouts, and termination. The LLM is a tool the engine calls for language understanding tasks only.

**Constraint:** All LLM work stays in Claude Code (Pro Max).
**Constraint:** LLM MINIMIZATION — if it can be code, it must be code. Enforced by architecture tests and CI, not by prompting.
**Constraint:** Engine exposed as MCP tools. Claude Code calls `converge()` once, engine runs in background, returns convergence_id for polling.
**Rule:** TDD. 100% coverage. Tests before code.
**Rule:** Follow DEVELOPMENT_PATTERNS_CRUXDEV.md methodology.
**Rule:** Every LLM provider call has an HTTP-level timeout. The engine can interrupt mid-call.

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│ Claude Code                                              │
│   "converge BUILD_PLAN_001.md"                          │
│        │                                                 │
│        ▼                                                 │
│   MCP call: converge(plan_file, constraints)            │
└────────┬────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────┐
│ Convergence Engine (Python MCP Server)                   │
│                                                          │
│ DETERMINISTIC (code, not LLM):                          │
│   ┌─────────────────────────────────┐                   │
│   │ State Machine                    │                   │
│   │   phase: planning → auditing →   │                   │
│   │     executing → converging →     │                   │
│   │     patterns → DONE             │                   │
│   │   round: int                     │                   │
│   │   consecutive_clean: int         │                   │
│   │   failures: dict[task, int]      │                   │
│   │   deadline: float (real clock)   │                   │
│   └─────────────────────────────────┘                   │
│                                                          │
│ NON-DETERMINISTIC (LLM via provider):                   │
│   audit(files, dimensions) → findings JSON              │
│   fix(finding) → code diff                              │
│   evaluate_independence(pass_a, pass_b) → bool          │
│   write(spec) → code or documentation                   │
│                                                          │
│ SCHEMA VALIDATION (code):                               │
│   Every LLM output validated before engine acts on it   │
└─────────────────────────────────────────────────────────┘
```

---

## Phase Ordering

```
Phase 1: State machine + persistence        ← The core loop, no LLM yet
Phase 2: LLM dispatch + schema validation   ← Wire LLM as a tool
Phase 3: Convergence sub-loops              ← Plan, code, doc convergence
Phase 4: MCP server interface               ← Expose as MCP tools
Phase 5: Dependency graph                   ← Context budget management
Phase 6: Subagent coordination              ← Real parallelism
Phase 7: Integration + E2E                  ← Wire to Crux MCP, end-to-end test
```

Phases 1-4 are sequential (each builds on prior).
Phases 5-6 can run in parallel after Phase 4.
Phase 7 requires all prior phases.

---

## Phase 1: State Machine + Persistence

**Purpose:** Build the deterministic core — state machine, counters, timeouts, rollback, persistence. No LLM calls yet. This phase proves the engine can manage a convergence loop with deterministic state.

### 1A. Data Structures

```python
# src/engine/state.py

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
    failures: dict = field(default_factory=dict)  # task_id → attempt count
    max_failures: int = 3
    deadline: Optional[float] = None  # time.time() + timeout_seconds
    timeout_per_task: float = 900.0   # 15 minutes
    history: list[RoundResult] = field(default_factory=list)
    escalation_reason: Optional[str] = None
    created_at: float = field(default_factory=time.time)
    updated_at: float = field(default_factory=time.time)
```

### 1B. State Persistence

```python
# src/engine/persistence.py

import json
import os
import tempfile
from pathlib import Path
from .state import ConvergenceState

def save_state(state: ConvergenceState, path: str) -> None:
    """Atomic write of convergence state to disk."""
    state.updated_at = time.time()
    data = serialize(state)  # dataclass → dict
    parent = os.path.dirname(path)
    fd, tmp = tempfile.mkstemp(dir=parent, prefix='.convergence_')
    try:
        os.write(fd, json.dumps(data, indent=2).encode())
        os.close(fd)
        fd = -1
        os.rename(tmp, path)  # atomic on POSIX
    except Exception:
        if fd >= 0:
            os.close(fd)
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise

def load_state(path: str) -> ConvergenceState:
    """Load convergence state from disk."""
    with open(path) as f:
        data = json.load(f)
    return deserialize(data)  # dict → dataclass
```

### 1C. Core Loop Logic (No LLM)

```python
# src/engine/convergence.py

import time
from .state import ConvergenceState, ConvergencePhase

PHASE_ORDER = [
    ConvergencePhase.PLANNING,
    ConvergencePhase.PLAN_AUDITING,
    ConvergencePhase.VIABILITY,
    ConvergencePhase.EXECUTING,
    ConvergencePhase.CODE_AUDITING,
    ConvergencePhase.DOC_AUDITING,
    ConvergencePhase.E2E_TESTING,
    ConvergencePhase.PATTERNS_UPDATE,
    ConvergencePhase.CONVERGED,
]

def advance_phase(current: ConvergencePhase) -> ConvergencePhase:
    """Move to the next phase in the sequence."""
    idx = PHASE_ORDER.index(current)
    return PHASE_ORDER[idx + 1]

def is_terminal(phase: ConvergencePhase) -> bool:
    return phase in (ConvergencePhase.CONVERGED, ConvergencePhase.ESCALATED)

def check_timeout(state: ConvergenceState) -> bool:
    """Real clock check. Code, not LLM."""
    if state.deadline is None:
        return False
    return time.time() > state.deadline

def check_max_rounds(state: ConvergenceState) -> bool:
    """Integer comparison. Code, not LLM."""
    return state.round >= state.max_rounds

def check_net_negative(state: ConvergenceState) -> bool:
    """Compare last two rounds' issue counts. Code, not LLM."""
    if len(state.history) < 2:
        return False
    last = len(state.history[-1].findings)
    prev = len(state.history[-2].findings)
    if len(state.history) >= 3:
        prev_prev = len(state.history[-3].findings)
        return last > prev and prev > prev_prev
    return False

def check_convergence(state: ConvergenceState) -> bool:
    """Two consecutive clean passes. Code, not LLM."""
    return state.consecutive_clean >= state.convergence_threshold

def record_round(state: ConvergenceState, findings: list) -> None:
    """Record a round's results. Update counters. Code, not LLM."""
    result = RoundResult(
        round=state.round,
        phase=state.phase,
        findings=findings,
        findings_fixed=sum(1 for f in findings if f.fixed),
        timestamp=time.time(),
    )
    state.history.append(result)

    if len(findings) == 0:
        state.consecutive_clean += 1
    else:
        state.consecutive_clean = 0

    state.round += 1

def check_structural_independence(round_a: RoundResult, round_b: RoundResult) -> bool:
    """Code-level check: were two passes structurally independent?
    Checks prompts/file sets differ. Code, not LLM."""
    # Different round numbers (not same pass re-evaluated)
    if round_a.round == round_b.round:
        return False
    # Must be consecutive
    if abs(round_a.round - round_b.round) != 1:
        return False
    return True

def run_tests(project_dir: str, test_command: str) -> TestRunResult:
    """Run project test suite. CODE runs tests, not LLM.
    Subprocess call with timeout."""
    import subprocess
    result = subprocess.run(
        test_command, shell=False, capture_output=True,
        timeout=300, cwd=project_dir
    )
    # Parse test output for pass/fail/coverage — code, not LLM
    return parse_test_output(result.stdout.decode())

def compact_history(state: ConvergenceState) -> None:
    """After a phase completes, summarize old rounds into aggregate stats.
    Prevents unbounded history growth across many rounds."""
    if len(state.history) <= 5:
        return
    # Keep last 3 rounds detailed, summarize the rest
    old = state.history[:-3]
    summary = RoundResult(
        round=-1,  # sentinel for "summary"
        phase=old[0].phase,
        findings=[],
        findings_fixed=sum(r.findings_fixed for r in old),
        timestamp=old[-1].timestamp,
    )
    state.history = [summary] + state.history[-3:]

def should_rollback(state: ConvergenceState, task_id: str) -> bool:
    """Three failures on same task. Code, not LLM."""
    return state.failures.get(task_id, 0) >= state.max_failures

def record_failure(state: ConvergenceState, task_id: str) -> None:
    """Increment failure counter. Code, not LLM."""
    state.failures[task_id] = state.failures.get(task_id, 0) + 1
```

### 1D. Timeout Enforcement

```python
# src/engine/timeout.py

import time
from .state import ConvergenceState

def set_task_deadline(state: ConvergenceState) -> None:
    """Set deadline for current task. Real clock."""
    state.deadline = time.time() + state.timeout_per_task

def set_phase_deadline(state: ConvergenceState, timeout: float) -> None:
    """Set deadline for entire phase."""
    state.deadline = time.time() + timeout

def remaining_seconds(state: ConvergenceState) -> float:
    """How much time is left. Real clock."""
    if state.deadline is None:
        return float('inf')
    return max(0, state.deadline - time.time())
```

### Checklist — Phase 1

- [ ] 1.1 `src/engine/state.py` — data structures (ConvergenceState, Finding, RoundResult, enums)
- [ ] 1.2 `src/engine/persistence.py` — atomic save/load of state
- [ ] 1.3 `src/engine/convergence.py` — core loop logic (advance, check, record, rollback)
- [ ] 1.4 `src/engine/timeout.py` — clock-based timeout enforcement
- [ ] 1.5 Tests for state serialization/deserialization roundtrip
- [ ] 1.6 Tests for phase advancement and ordering
- [ ] 1.7 Tests for convergence detection (0, 1, 2 clean passes)
- [ ] 1.8 Tests for timeout (real clock, mock time)
- [ ] 1.9 Tests for failure counting and rollback trigger
- [ ] 1.10 Tests for net-negative detection
- [ ] 1.11 Tests for atomic state persistence (crash safety)
- [ ] 1.12 Tests for structural independence check
- [ ] 1.13 Tests for test runner (subprocess, parse output)
- [ ] 1.14 Tests for history compaction
- [ ] 1.15 Coverage ≥ 100%

---

## Phase 2: LLM Dispatch + Schema Validation

**Purpose:** Build the interface between the deterministic engine and the non-deterministic LLM. Every LLM call has a defined input/output schema. Output is validated before the engine acts on it.

### 2A. LLM Dispatcher Interface

```python
# src/dispatch/llm.py

from abc import ABC, abstractmethod
from .schema import AuditResult, FixResult, EvaluationResult

class LLMDispatcher(ABC):
    """The ONLY way the engine calls an LLM.
    Grep for this interface to verify LLM scope."""

    @abstractmethod
    def audit(self, files: list[str], dimensions: list[str],
              skill_context: str) -> AuditResult:
        """Ask LLM to audit files. Returns schema-validated findings."""

    @abstractmethod
    def fix(self, finding: Finding, file_content: str,
            skill_context: str) -> FixResult:
        """Ask LLM to generate a fix. Returns schema-validated diff."""

    @abstractmethod
    def evaluate_independence(self, pass_a: AuditResult,
                               pass_b: AuditResult) -> EvaluationResult:
        """Ask LLM if two audit passes are independent.
        Called by the engine AFTER code-level structural independence check
        (different prompts, different file sets). LLM is secondary check only."""

    @abstractmethod
    def write(self, spec: str, skill_context: str) -> str:
        """Ask LLM to write code or documentation."""
```

### 2B. Provider Implementations

Extend the pattern from `crux_audit_backend.py`:

```python
# src/dispatch/providers/anthropic.py
# src/dispatch/providers/ollama.py
# src/dispatch/providers/openai.py
```

Each provider:
- Injects skill markdown as system context
- Sends the task (files + dimensions) as user context
- Receives structured JSON output
- Validates against schema before returning
- **Has an HTTP-level timeout** (e.g., `httpx` timeout, `asyncio.wait_for`) so the engine can interrupt a long-running LLM call that exceeds the per-task deadline. The engine's `check_timeout()` runs between steps, but provider timeouts catch mid-call hangs.
- **Reads API credentials** from environment variables (`ANTHROPIC_API_KEY`, `OPENAI_API_KEY`) or from Crux's credential management (`crux_typefully.py` key-file pattern). Startup validation checks required credentials are present before attempting convergence.

### 2C. Schema Definitions

```python
# src/dispatch/schema.py

from pydantic import BaseModel
from typing import Literal

class AuditFinding(BaseModel):
    id: str
    file: str
    dimension: str
    severity: Literal["high", "medium", "low"]
    description: str
    suggested_fix: str

class AuditResult(BaseModel):
    findings: list[AuditFinding]
    files_audited: list[str]
    dimensions_checked: list[str]

class FixResult(BaseModel):
    success: bool
    files_modified: list[str]
    description: str

class TestRunResult(BaseModel):
    """Produced by CODE (subprocess), not LLM. Engine runs tests itself."""
    passed: bool
    total: int
    failures: int
    coverage_percent: float

class EvaluationResult(BaseModel):
    independent: bool
    rationale: str
```

### 2D. Validation + Retry

```python
# src/dispatch/validation.py

def validate_and_retry(llm_call, schema, max_retries=2):
    """Call LLM, validate output, retry on schema failure."""
    for attempt in range(max_retries + 1):
        raw = llm_call()
        try:
            return schema.model_validate_json(raw)
        except ValidationError as e:
            if attempt == max_retries:
                raise EscalationRequired(f"LLM output failed validation after {max_retries} retries: {e}")
            # Retry with error feedback
            llm_call = partial(llm_call, error_feedback=str(e))
```

### Checklist — Phase 2

- [ ] 2.1 `src/dispatch/llm.py` — LLMDispatcher abstract interface
- [ ] 2.2 `src/dispatch/schema.py` — Pydantic models for all LLM I/O
- [ ] 2.3 `src/dispatch/validation.py` — validate + retry logic
- [ ] 2.4 `src/dispatch/providers/anthropic.py` — Anthropic provider
- [ ] 2.5 `src/dispatch/providers/ollama.py` — Ollama provider
- [ ] 2.6 `src/dispatch/providers/stub.py` — Configurable stub provider for testing (must simulate: clean passes, findings, persistent findings, schema-invalid output, slow responses, intermittent failures)
- [ ] 2.7 `src/dispatch/credentials.py` — Credential loading + startup validation
- [ ] 2.8 Tests for schema validation (valid, invalid, edge cases)
- [ ] 2.9 Tests for retry logic (succeed, fail once + succeed, fail all)
- [ ] 2.10 Tests for stub provider (all 6 simulation modes)
- [ ] 2.11 Tests for provider HTTP-level timeout (mock slow response)
- [ ] 2.12 Tests for credential validation (present, missing, malformed)
- [ ] 2.13 Coverage ≥ 100%

---

## Phase 3: Convergence Sub-Loops

**Purpose:** Implement the specific convergence loops (plan, code, doc) as code that calls the LLM dispatcher. Each loop uses the Phase 1 state machine for all control flow.

### 3A. Plan Convergence

```python
# src/engine/plan_convergence.py

def run_plan_convergence(state, llm, plan_file):
    """Focused audit → full-plan audit → viability. All loops code-driven."""
    # Focused audit per phase
    for phase_section in extract_phases(plan_file):
        state.round = 0
        state.consecutive_clean = 0
        while not check_convergence(state):
            if check_max_rounds(state): escalate(state, "focused_max_rounds"); return
            findings = llm.audit(files=[plan_file], dimensions=PLAN_DIMENSIONS,
                                  skill_context=load_skill("auditing"))
            validated = validate(findings)
            if validated.findings:
                for f in validated.findings:
                    llm.fix(f, read(plan_file), load_skill("planning"))
            record_round(state, validated.findings)
            save_state(state)
    # ... full-plan audit, viability (same pattern)
```

### 3B. Code Convergence

8-dimension audit with parallel agent support (Phase 6 wires real parallelism).

**Independence check wiring:** When `consecutive_clean` reaches 1 (first clean pass), the engine dispatches the SECOND pass with different parameters (different system prompt seed, different file ordering). After the second pass returns clean, the engine:
1. Checks structural independence via `check_structural_independence()` (code)
2. If structurally independent → CONVERGED
3. If not structurally independent → calls `llm.evaluate_independence()` as secondary check
4. If LLM says not independent → reset `consecutive_clean`, continue

**Test verification after fixes:** After every `llm.fix()` call, the engine runs `run_tests()` (code, subprocess). If tests fail, the fix is rolled back and counted as a failure. The LLM never claims "tests pass" — code verifies.

### 3C. Doc Convergence

5-dimension audit. Same loop structure.

### 3D. Pre-Audit Analysis (Context Scoping)

Before each `llm.audit()` call, the engine calls `dependency_graph.impact_set(changed_files)` to narrow the audit scope. The LLM receives only the files that could be affected by recent changes, not the entire codebase. This is a code-level decision, not an LLM decision.

### Checklist — Phase 3

- [ ] 3.1 `src/engine/plan_convergence.py` — focused + full-plan + viability loops
- [ ] 3.2 `src/engine/code_convergence.py` — 8-dimension code audit loop
- [ ] 3.3 `src/engine/doc_convergence.py` — 5-dimension doc audit loop
- [ ] 3.4 `src/engine/e2e_convergence.py` — 4-loop E2E convergence
- [ ] 3.5 `src/engine/runner.py` — master runner that chains sub-loops
- [ ] 3.6 Tests with stub LLM provider (deterministic convergence scenarios)
- [ ] 3.7 Tests for escalation paths (max rounds, timeout, net-negative)
- [ ] 3.8 Tests for phase transitions through the full lifecycle
- [ ] 3.9 Tests for independence check wiring (structural check + LLM fallback)
- [ ] 3.10 Tests for test verification after fix (run_tests, rollback on failure)
- [ ] 3.11 Tests for pre-audit scope narrowing via dependency graph
- [ ] 3.12 Test: engine does NOT converge when two clean passes are not independent
- [ ] 3.13 Coverage ≥ 100%

---

## Phase 4: MCP Server Interface

**Purpose:** Expose the engine as MCP tools that Claude Code can call.

### 4A. MCP Tools

The `converge()` tool spawns a background thread and returns immediately with a `convergence_id`. Claude Code remains available for other work and can poll progress. This is non-blocking by design.

```python
# src/mcp_server.py

import threading
import uuid

_active_runs: dict[str, threading.Thread] = {}

@mcp.tool()
async def converge(plan_file: str, constraints: str = "",
                   timeout_minutes: int = 120) -> dict:
    """Start convergence. Returns immediately with convergence_id.
    Poll with check_convergence_status(). Non-blocking."""
    convergence_id = str(uuid.uuid4())[:8]
    state = ConvergenceState(
        plan_file=plan_file,
        deadline=time.time() + (timeout_minutes * 60),
    )
    save_state(state, state_path(convergence_id))

    def run_in_background():
        runner = ConvergenceRunner(state, llm=get_provider())
        runner.run()  # Blocks in this thread, not the MCP server
        save_state(state, state_path(convergence_id))

    thread = threading.Thread(target=run_in_background, daemon=True)
    thread.start()
    _active_runs[convergence_id] = thread

    return {"convergence_id": convergence_id, "status": "started"}

@mcp.tool()
async def check_convergence_status(convergence_id: str) -> dict:
    """Check progress of a running convergence."""
    state = load_state(state_path(convergence_id))
    thread = _active_runs.get(convergence_id)
    running = thread.is_alive() if thread else False
    return {
        "convergence_id": convergence_id,
        "running": running,
        "phase": state.phase.value,
        "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "total_findings": sum(len(r.findings) for r in state.history),
        "elapsed_seconds": time.time() - state.created_at,
        "escalation_reason": state.escalation_reason,
    }

@mcp.tool()
async def cancel_convergence(convergence_id: str) -> dict:
    """Cancel a running convergence, preserving state for resume."""
    state = load_state(state_path(convergence_id))
    state.phase = ConvergencePhase.ESCALATED
    state.escalation_reason = "cancelled_by_user"
    save_state(state, state_path(convergence_id))
    return {"status": "cancelled", "convergence_id": convergence_id}
```

### 4B. MCP Server Configuration

The engine runs as a separate MCP server alongside Crux's existing one:

```json
{
  "mcpServers": {
    "crux": { "...existing crux config..." },
    "cruxdev": {
      "command": "python3",
      "args": ["-m", "src.mcp_server"],
      "env": {
        "CRUXDEV_PROJECT": ".",
        "PYTHONPATH": "/Users/user/personal/cruxdev"
      }
    }
  }
}
```

### Checklist — Phase 4

- [ ] 4.1 `src/mcp_server.py` — MCP server with converge, check_status, cancel tools
- [ ] 4.2 State file management (per-convergence ID)
- [ ] 4.3 Provider selection from config/env
- [ ] 4.4 Tests for MCP tool invocation
- [ ] 4.5 Tests for concurrent convergence runs
- [ ] 4.6 Tests for cancel + resume
- [ ] 4.7 Integration test: Claude Code → MCP → engine → stub LLM → result
- [ ] 4.8 Coverage ≥ 100%

---

## Phase 5: Dependency Graph

**Purpose:** Code-maintained dependency graph for context budget management. The engine (not LLM) decides what files to include in each audit call.

### 5A. Architecture

```python
# src/graph/dependency.py

import ast  # Python AST for Python projects
# tree-sitter for multi-language support (future)

class DependencyGraph:
    def __init__(self, project_root: str):
        self.root = project_root
        self.edges: dict[str, set[str]] = {}  # file → files it imports

    def build(self) -> None:
        """Parse all source files, build import graph."""

    def update(self, changed_file: str) -> None:
        """Re-parse one file, update edges."""

    def impact_set(self, changed_files: list[str]) -> set[str]:
        """Given changes, what other files might be affected?"""

    def audit_context(self, target_files: list[str],
                       token_budget: int) -> list[str]:
        """Given files to audit and a token budget,
        return the optimal set of context files."""

    def assign_scopes(self, n_agents: int) -> list[set[str]]:
        """Partition files into N non-overlapping scopes with minimal
        cross-scope dependencies. Feeds into Phase 6 subagent coordination."""
```

### Checklist — Phase 5

- [ ] 5.1 `src/graph/dependency.py` — graph construction from Python AST
- [ ] 5.2 `src/graph/change_impact.py` — impact set calculation
- [ ] 5.3 `src/graph/context_budget.py` — token-budget-aware context selection
- [ ] 5.4 Tests for graph construction (real Python files)
- [ ] 5.5 Tests for impact set (change A affects B which affects C)
- [ ] 5.6 Tests for context budget (large codebase, limited tokens)
- [ ] 5.7 `assign_scopes()` — partition files for parallel agents with minimal cross-scope deps
- [ ] 5.8 Tests for scope assignment (verify non-overlapping, minimal cross-deps)
- [ ] 5.9 Coverage ≥ 100%

---

## Phase 6: Subagent Coordination

**Purpose:** Real parallelism for multi-agent audit passes.

### 6A. Architecture

```python
# src/dispatch/subagent.py

import asyncio

class SubagentCoordinator:
    async def parallel_audit(self, agents: list[LLMDispatcher],
                              file_sets: list[list[str]],
                              dimensions: list[str]) -> list[AuditResult]:
        """Dispatch N agents concurrently. Real parallelism."""
        tasks = [
            agent.audit(files, dimensions, skill_context)
            for agent, files in zip(agents, file_sets)
        ]
        return await asyncio.gather(*tasks)

    def synthesize(self, results: list[AuditResult],
                    max_synthesis_size: int = 50000) -> list[Finding]:
        """Merge results. Deduplicate. Resolve conflicts in code.
        If combined findings exceed max_synthesis_size tokens,
        chunk by severity (HIGH first) or by file group."""

    def resolve_conflict(self, a: Finding, b: Finding) -> Finding:
        """5 priority rules, implemented in code."""
        # 1. Safety first
        # 2. Preserve existing behavior
        # 3. Tests over no tests
        # 4. Specificity over generality
        # 5. Escalate genuine conflicts
```

### Checklist — Phase 6

- [ ] 6.1 `src/dispatch/subagent.py` — parallel dispatch + synthesis
- [ ] 6.2 `src/dispatch/conflict.py` — conflict resolution (5 rules in code)
- [ ] 6.3 Tests for parallel dispatch (mock agents, verify concurrency)
- [ ] 6.4 Tests for synthesis (dedup, merge, conflict cases)
- [ ] 6.5 Tests for each conflict resolution rule
- [ ] 6.6 Tests for synthesis when combined output exceeds context limit (chunking)
- [ ] 6.7 Coverage ≥ 100%

---

## Phase 7: Integration + E2E

**Purpose:** Wire everything together. Test against a real project. Prove the engine converges deterministically.

### 7A. Integration with Crux MCP

The engine imports Crux Python modules directly (both are Python, same machine). No MCP client needed — direct function calls to:
- `crux_status.verify_health()` — viability assessment
- `crux_mcp_handlers.handle_lookup_knowledge()` — skill context injection
- `crux_mcp_handlers.handle_get_mode_prompt()` — mode context
- `crux_mcp_handlers.handle_log_correction()` — continuous learning from findings

The CruxDev MCP server and Crux MCP server run as separate processes for Claude Code. But internally, the engine imports Crux as a library.

### 7B. End-to-End Test

Run the engine against the Crux project itself:
1. Create a deliberate defect (e.g., remove a test, break a docstring)
2. Call `converge(plan_file="test_plan.md")`
3. Verify: engine detects the defect, generates a fix, verifies the fix, converges
4. Verify: all state transitions were deterministic (round counts correct, timeouts enforced, no LLM in control flow)

### 7C. Architecture Test

Verify LLM minimization:
```python
def test_no_llm_in_control_flow():
    """The engine NEVER calls LLM for state transitions,
    counting, timing, or rollback."""
    # Grep src/engine/ for LLMDispatcher calls
    # They should ONLY appear in the convergence sub-loops
    # (plan_convergence, code_convergence, doc_convergence)
    # where they dispatch audit/fix tasks.
    # They should NEVER appear in convergence.py, timeout.py,
    # persistence.py, or state.py.
```

### 7D. Document Status Annotations

Annotate operational documents that reference the convergence engine with implementation status:

- `DEVELOPMENT_PATTERNS_CRUXDEV.md` — every reference to "the engine drives..." gets annotated with whether that step is now code-driven (this build plan) or still LLM-driven (pending)
- `ADOPTION_PLAYBOOK.md` — engine references annotated
- `ADOPTION_PROCESS.md` — engine references annotated

### 7E. LLM Minimization Rule Propagation

Add the LLM MINIMIZATION rule to:
- `/Users/user/personal/cruxdev/.claude/CLAUDE.md`
- `/Users/user/personal/cruxdev/DEVELOPMENT_PATTERNS_CRUXDEV.md`

This is a hard rule, not a guideline. Enforced by architecture tests (7.6, 7.7) and CI.

### 7F. Markdown Engine Retirement

Move existing markdown engine spec files from `engine/` to `specs/` (or `archive/`). Add a note in each that they are superseded by the code engine. Prevents LLMs from following the markdown instructions instead of calling the MCP tools.

### Checklist — Phase 7

- [ ] 7.1 Crux library integration (direct imports, not MCP client)
- [ ] 7.2 E2E test: deliberate defect → converge → fixed
- [ ] 7.3 E2E test: timeout triggers escalation (including mid-call provider timeout)
- [ ] 7.4 E2E test: 3 failures triggers rollback
- [ ] 7.5 E2E test: net-negative triggers escalation
- [ ] 7.6 Architecture test: no LLM in control flow (CI gate, not one-time check)
- [ ] 7.7 Architecture test: all LLM calls go through LLMDispatcher (CI gate)
- [ ] 7.8 Annotate DEVELOPMENT_PATTERNS_CRUXDEV.md with engine implementation status
- [ ] 7.9 Annotate ADOPTION_PLAYBOOK.md with engine implementation status
- [ ] 7.10 Annotate ADOPTION_PROCESS.md with engine implementation status
- [ ] 7.11 Add LLM MINIMIZATION rule to .claude/CLAUDE.md
- [ ] 7.12 Add LLM MINIMIZATION rule to DEVELOPMENT_PATTERNS_CRUXDEV.md
- [ ] 7.13 Retire markdown engine files (move to specs/ or archive/)
- [ ] 7.14 Full suite passes, coverage ≥ 100%
- [ ] 7.15 Code + doc convergence (two consecutive clean passes)

---

## Progress Tracker

**Phase 1: State Machine (15 items)** — [ ] 1.1 – 1.15
**Phase 2: LLM Dispatch (13 items)** — [ ] 2.1 – 2.13
**Phase 3: Sub-Loops (13 items)** — [ ] 3.1 – 3.13
**Phase 4: MCP Server (8 items)** — [ ] 4.1 – 4.8
**Phase 5: Dependency Graph (9 items)** — [ ] 5.1 – 5.9
**Phase 6: Subagent Coordination (7 items)** — [ ] 6.1 – 6.7
**Phase 7: Integration + E2E (15 items)** — [ ] 7.1 – 7.15

**Total: 80 checkboxes**

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Pydantic/schema overhead on LLM output | Start with simple JSON validation, graduate to Pydantic if needed |
| LLM providers return different formats | Provider abstraction normalizes output before validation |
| Long-running LLM call exceeds task timeout | HTTP-level timeout on every provider call (not just between-step checks) |
| API credentials missing at runtime | Startup validation in `credentials.py` — fail fast before convergence starts |
| Claude Code unavailable during convergence | Non-blocking: `converge()` returns immediately, runs in background thread, poll with `check_status()` |
| History grows unbounded across many rounds | `compact_history()` summarizes old rounds, keeps last 3 detailed |
| Combined subagent output exceeds context | `synthesize()` chunks by severity when over `max_synthesis_size` |
| Dependency graph only covers Python | Start with `ast` module. Track as known gap. Add tree-sitter for multi-language later. |
| LLM falsely claims two passes are independent | Code checks structural independence first. LLM is secondary check only. |
| LLM claims "tests pass" but they don't | Engine runs tests itself via subprocess. Never trusts LLM's claim about test status. |
| Existing markdown engine files confuse LLMs | Retire to `specs/` with "superseded by code engine" notice |
| AST-based dep graph only works for Python | Start with Python (ast module). Add tree-sitter for multi-language later. |
| Engine timeout kills a long but productive LLM call | Per-task timeout (15 min) is generous. Phase-level deadline is separate. |
| `converge()` MCP call blocks for hours | Return convergence_id immediately, run in background thread, poll with `check_status()` |

---

## Definition of Done

1. Engine runs a convergence loop with deterministic state management
2. Round counting, timeouts, rollback, termination — all in code, zero LLM
3. LLM invoked only through `LLMDispatcher` for language-understanding tasks
4. LLM outputs validated against schemas before engine acts on them
5. Dependency graph maintained by code (AST), not LLM
6. Subagent coordination with real parallelism and code-level conflict resolution
7. Exposed as MCP tools (`converge`, `check_status`, `cancel`)
8. Architecture tests verify LLM minimization
9. E2E test: engine converges on a real project with a deliberate defect
10. 100% test coverage
