# Evolution Plan: Deterministic Convergence Engine

**Created:** 2026-03-18
**Status:** PLANNING
**Scope:** All three products (Crux, CruxCLI, CruxDev)
**Trigger:** Architecture review identified five structural concerns that the current ecosystem does not address

---

## The Problem

An external architecture review of CruxDev identified two root issues and three derivative concerns. The root issues are fundamental — they affect whether the system can reliably operate without human supervision.

### Root Issue 1: The Convergence Engine Is Not an Engine

The convergence engine is currently a collection of markdown files that instruct an LLM to behave like a state machine. Every component — loop counter, round tracking, termination condition, timeout, rollback trigger — is enforced by the LLM reading instructions and choosing to follow them.

An LLM is non-deterministic. It does not execute the same steps twice given the same inputs. This means:
- The round counter may be miscounted, skipped, or reset during context drift
- The two-consecutive-clean-pass rule may be declared satisfied prematurely
- The 15-minute timeout cannot be enforced by a system with no clock
- Rollback after 3 failed attempts requires reliable counting across long tool-call chains
- Safety valves are guidelines the LLM may or may not follow

A 50-100 step agentic process where each step has a 99% success rate compounds to ~60-37% overall success. The methodology is right. The execution substrate is wrong.

### Root Issue 2: LLM Minimization Has No Enforcement

Every LLM, when asked to design a system that uses LLMs, defaults to solving problems by calling itself. This is structural — the LLM has no incentive to remove itself from a process. It will architect systems where it is the loop controller, state tracker, counter, auditor, and executor simultaneously.

The responsibility for LLM minimization falls entirely on the human architect. It cannot be delegated to the model. It must be a hard rule enforced at every level.

### Derivative Concern 3: Codebase Scale and Context Windows

No mechanism exists for maintaining a dependency graph across session boundaries, identifying which files need audit context after a change, or detecting cross-module ripple effects when audit scope is split across subagents.

### Derivative Concern 4: Agent Coordination Is Described But Not Architected

Subagent dispatch, parallel auditing, output synthesis, and conflict resolution are described at the intent level. No specification exists for dispatch mechanisms, output schemas, error handling, parallelism implementation, or context limits during synthesis.

### Derivative Concern 5: Design-Phase vs. Production-Readiness Framing

Operational documents (DEVELOPMENT_PATTERNS_CRUXDEV.md, ADOPTION_PLAYBOOK.md) reference infrastructure that doesn't exist yet (the convergence engine, CruxDev skills, platform adapters) in an operational register. An LLM given these documents will attempt to use the described system, papering over gaps.

---

## The Principle

**If it can be code, it must be code.**

| Task | Must Be Code | LLM's Role |
|------|-------------|------------|
| Counting audit rounds | Integer in code | None |
| Checking if timeout elapsed | Clock call in code | None |
| Determining if two clean passes occurred | Boolean check against data structure | None |
| Triggering rollback after 3 failures | Failure counter in code | None |
| Writing state between sessions | File I/O in code | None |
| Reading state on resume | File read + deserialization in code | None |
| Enforcing max rounds safety valve | Conditional in code | None |
| Routing to the next phase | State machine transition in code | None |
| Parsing audit output into findings | JSON schema validation in code | None |
| Deciding if all findings are resolved | Empty list check in code | None |
| Reading source files and identifying issues | N/A | **This is the LLM's job** |
| Generating a fix for a specific issue | N/A | **This is the LLM's job** |
| Evaluating independence of clean passes | N/A | **This is the LLM's job** |
| Writing or rewriting code/documentation | N/A | **This is the LLM's job** |

The convergence engine must be **code that calls an LLM**. It cannot be an **LLM that describes a convergence engine**.

---

## Architecture: What the Engine Becomes

### Current (Prompt-Driven)

```
Agent reads CONVERGENCE.md
  → Agent decides to audit
  → Agent remembers round count
  → Agent decides termination
  → Agent writes state file
  → Agent decides next action
  → Agent counts failures
  → Agent enforces timeout (can't)
```

### Target (Code-Driven, MCP-Exposed)

```
Convergence Engine = Python MCP Server
  │
  │  Claude Code calls converge() once → engine runs to completion
  │
  ├── MCP Tools (the interface):
  │     converge(plan_file, constraints) → runs full lifecycle, returns report
  │     check_convergence(id)            → poll progress while running
  │     get_convergence_state(id)        → current state snapshot
  │     cancel_convergence(id)           → abort with state preserved
  │
  ├── Deterministic internals (code, not LLM):
  │     State machine: states, transitions, counters
  │     Clock: timeouts, deadlines (real time)
  │     Failure tracking: per-task attempt counts (integers)
  │     State persistence: JSON read/write (file I/O)
  │     Schema validation: LLM outputs validated before use
  │     Dependency graph: AST-based, code-maintained
  │     Subagent coordination: real parallelism, structured outputs
  │
  └── Calls LLM via provider abstraction (Anthropic, Ollama, OpenAI, etc.):
      ├── "Audit these files against these dimensions" → structured JSON
      ├── "Generate a fix for this issue" → code diff
      ├── "Are these two audit results independent?" → boolean + rationale
      └── "Write documentation for this module" → text
```

### Where This Lives in the Ecosystem

```
┌─────────────────────────────────────────────────────────────┐
│ CruxCLI (terminal runtime)                                   │
│   └── Launches the convergence engine as a subprocess        │
│       or integrates it as a library                          │
│                                                              │
│ Crux (intelligence layer)                                    │
│   └── Provides modes, MCP tools, safety gates, knowledge     │
│       The engine calls Crux MCP tools for auditing,          │
│       mode switching, knowledge lookup                       │
│                                                              │
│ CruxDev (convergence engine + skills)                        │
│   └── The ENGINE is now Python/TypeScript code               │
│       Skills remain markdown (they are LLM instructions      │
│       for the tasks the LLM is actually good at)             │
│       The engine CALLS skills by injecting them into         │
│       LLM context when dispatching audit/fix/write tasks     │
└─────────────────────────────────────────────────────────────┘
```

---

## What Changes, What Stays

### What Becomes Code (Currently Markdown "Engine")

| Component | Current | Target |
|-----------|---------|--------|
| Master convergence loop | `engine/CONVERGENCE.md` | `src/engine/convergence.py` — real state machine |
| Plan convergence sub-loop | `engine/PLAN_CONVERGENCE.md` | `src/engine/plan_convergence.py` |
| Code convergence sub-loop | `engine/CODE_CONVERGENCE.md` | `src/engine/code_convergence.py` |
| Doc convergence sub-loop | `engine/DOC_CONVERGENCE.md` | `src/engine/doc_convergence.py` |
| Viability assessment | `engine/VIABILITY.md` | `src/engine/viability.py` |
| Round counter | LLM memory | `int` in `ConvergenceState` |
| Timeout enforcement | LLM told to respect 15 min | `time.time()` + deadline check |
| Failure counter | LLM counting attempts | `int` per task in state |
| Rollback trigger | LLM deciding after 3 failures | `if failures >= 3: rollback()` |
| Termination condition | LLM checking clean pass count | `if consecutive_clean >= 2: return CONVERGED` |
| Safety valve | LLM checking max rounds | `if round > max_rounds: escalate()` |
| Net-negative detection | LLM comparing issue counts | `if issues[n] > issues[n-1] and issues[n-1] > issues[n-2]: escalate()` |
| State persistence | LLM writing JSON | `json.dump()` in code |
| State resume | LLM reading JSON | `json.load()` in code |
| Schema validation | Trust LLM output | `jsonschema.validate()` or Pydantic |

### What Stays as Markdown (LLM Instructions for LLM Tasks)

| Component | Why It Stays Markdown |
|-----------|---------------------|
| Skills (16 skill SKILL.md files) | Skills are instructions for how the LLM should approach specific tasks (auditing, planning, TDD). The LLM genuinely needs these — they shape its reasoning. |
| Audit dimensions (8 code + 5 doc) | These are evaluation criteria the LLM applies. Code can't evaluate "does this function have logic errors?" |
| Prompt patterns | Templates for LLM invocations. The engine injects these into LLM calls. |
| Plan templates | Document structure for LLM-generated plans. |
| CRUXDEV.md bootstrap | Still needed for agents that aren't using the code engine (e.g., vanilla Claude Code without CruxDev installed as a runtime). |

### The Key Insight

Skills and engine are different things:
- **Skills** = what the LLM should think about (audit dimensions, TDD rules, planning methodology). These are correctly markdown.
- **Engine** = what controls the process (loop, count, timeout, terminate, rollback, route). This must be code.

The current architecture conflates these. The fix is to separate them cleanly.

---

## Implementation Phases

### Phase 1: Core Engine (Python)

Build the deterministic convergence engine as a Python package in the CruxDev repo.

```
src/
├── engine/
│   ├── __init__.py
│   ├── state.py              # ConvergenceState dataclass + persistence
│   ├── convergence.py        # Master loop state machine
│   ├── plan_convergence.py   # Plan audit sub-loop
│   ├── code_convergence.py   # Code audit sub-loop
│   ├── doc_convergence.py    # Doc audit sub-loop
│   ├── viability.py          # Environment verification
│   ├── timeout.py            # Clock-based timeout enforcement
│   ├── rollback.py           # Git-based rollback on failure
│   └── schema.py             # JSON schemas for LLM output validation
├── dispatch/
│   ├── __init__.py
│   ├── llm.py                # LLM invocation (inject skill + context, get structured output)
│   ├── subagent.py           # Subagent dispatch (parallel via subprocess/threads)
│   └── synthesis.py          # Merge parallel agent outputs, conflict resolution
└── graph/
    ├── __init__.py
    ├── dependency.py          # Code-maintained dependency graph
    ├── change_impact.py       # Given a change, what else needs auditing?
    └── context_budget.py      # What fits in context? What gets split?
```

**State machine for the master loop:**

```python
class ConvergencePhase(Enum):
    PLANNING = "planning"
    PLAN_AUDITING = "plan_auditing"
    EXECUTING = "executing"
    CODE_AUDITING = "code_auditing"
    DOC_AUDITING = "doc_auditing"
    E2E_TESTING = "e2e_testing"
    PATTERNS_UPDATE = "patterns_update"
    CONVERGED = "converged"
    ESCALATED = "escalated"

class ConvergenceState:
    phase: ConvergencePhase
    round: int                    # Owned by code, not LLM
    max_rounds: int
    consecutive_clean: int        # Owned by code
    failures_per_task: dict       # Owned by code
    deadline: float               # time.time() + timeout_seconds
    issues: list                  # Schema-validated LLM output
    history: list                 # Append-only round history
```

**The loop, in code:**

```python
def run_convergence(state: ConvergenceState, llm: LLMDispatcher):
    while state.phase not in (ConvergencePhase.CONVERGED, ConvergencePhase.ESCALATED):
        if time.time() > state.deadline:
            state.phase = ConvergencePhase.ESCALATED
            state.escalation_reason = "timeout"
            break

        if state.round > state.max_rounds:
            state.phase = ConvergencePhase.ESCALATED
            state.escalation_reason = "max_rounds"
            break

        # Dispatch LLM for the language-understanding part
        findings = llm.audit(
            files=get_audit_scope(state),
            dimensions=get_dimensions(state.phase),
            skill=load_skill(state.phase),        # Markdown skill injected as context
        )

        # Validate LLM output against schema
        validated = validate_findings(findings)    # Code, not LLM

        if len(validated) == 0:
            state.consecutive_clean += 1           # Code counts, not LLM
        else:
            state.consecutive_clean = 0
            # Dispatch LLM to fix each finding
            for finding in validated:
                fix_result = llm.fix(finding)
                if not fix_result.success:
                    state.failures_per_task[finding.id] += 1
                    if state.failures_per_task[finding.id] >= 3:
                        rollback(finding)          # Code triggers, not LLM

        # Check termination — code decides, not LLM
        if state.consecutive_clean >= 2:
            state.phase = advance_phase(state.phase)
            state.round = 0
            state.consecutive_clean = 0
        else:
            state.round += 1

        # Net-negative detection — code checks, not LLM
        if is_net_negative(state.history):
            state.phase = ConvergencePhase.ESCALATED
            state.escalation_reason = "net_negative"

        # Persist state — code writes, not LLM
        save_state(state)
```

### Phase 2: LLM Dispatch Layer

Build the interface between the deterministic engine and the non-deterministic LLM.

**Key design rules:**
- Every LLM call has a defined input schema and output schema
- Output is validated before the engine acts on it
- Failed validation → retry with error feedback, max 2 retries → escalate
- LLM receives the relevant skill markdown as system context
- LLM receives only the files needed for the current task (context budget managed by code)

**Structured output schemas (examples):**

```python
class AuditFinding(BaseModel):
    id: str
    file: str
    dimension: str          # One of the 8 code or 5 doc dimensions
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
    tests_pass: bool        # Did the fix break anything?
```

### Phase 3: Dependency Graph

Build a code-maintained (not LLM-maintained) dependency graph that solves the codebase scale problem.

**Approach:** Use tree-sitter or AST parsing (not LLM) to build an import/call graph. Update on every file write. When a file changes, the graph identifies what else needs to be included in the audit context.

```python
class DependencyGraph:
    def update(self, changed_file: str) -> None:
        """Re-parse changed file, update edges."""

    def impact_set(self, changed_files: list[str]) -> set[str]:
        """Given changes, what other files are affected?"""

    def audit_context(self, files: list[str], budget: int) -> list[str]:
        """Given files to audit and a token budget, what context to include?"""
```

This is the answer to Concern 3. The engine — not the LLM — decides what context each audit call receives.

### Phase 4: Subagent Coordination

Build real parallelism for multi-agent audit passes.

**Architecture:**
- Engine spawns subagent processes (Python `multiprocessing` or `asyncio`)
- Each subagent receives: file list, skill context, output schema
- Subagents return structured JSON validated by the engine
- Engine synthesizes outputs using the conflict resolution rules (in code, not LLM)
- If combined output exceeds context limits, engine chunks the synthesis

```python
class SubagentCoordinator:
    async def parallel_audit(
        self,
        agents: list[AuditAgent],
        files: list[str],
        dimensions: list[str],
    ) -> list[AuditResult]:
        """Dispatch N agents in parallel, collect validated results."""

    def synthesize(self, results: list[AuditResult]) -> list[AuditFinding]:
        """Merge, deduplicate, resolve conflicts per priority rules."""
```

The conflict resolution protocol (5 priority rules from CruxDev.md Section 9.2) becomes code:

```python
def resolve_conflict(finding_a: AuditFinding, finding_b: AuditFinding) -> AuditFinding:
    # 1. Safety first
    if finding_a.is_safety_concern and not finding_b.is_safety_concern:
        return finding_a
    # 2. Preserve existing behavior
    # 3. Tests over no tests
    # 4. Specificity over generality
    # 5. Escalate genuine conflicts
    ...
```

### Phase 5: Crux MCP Integration

Wire the deterministic engine to Crux's MCP server. The engine calls Crux MCP tools for:

| MCP Tool | Engine Uses It For |
|----------|--------------------|
| `verify_health` | Viability assessment |
| `get_session_state` / `update_session` | Session context for LLM calls |
| `lookup_knowledge` | Skill and knowledge injection into LLM context |
| `get_mode_prompt` | Mode prompt for LLM system context |
| `start_tdd_gate` / `check_tdd_status` | TDD enforcement during execution phase |
| `start_security_audit` | Security gate in safety pipeline |
| `log_correction` | Recording findings for continuous learning |
| `bip_generate` | Build-in-public content from convergence results |

### Phase 6: Document Status Annotations

Add explicit "not yet implemented" annotations to operational documents that reference infrastructure in design phase.

Every reference to the convergence engine, skills, or platform adapters in DEVELOPMENT_PATTERNS_CRUXDEV.md, ADOPTION_PLAYBOOK.md, and ADOPTION_PROCESS.md gets annotated:

```markdown
<!-- ENGINE: This step is currently LLM-driven. The deterministic engine
     (CRUX_EVOLUTION_PLAN_001) will replace this with code-driven execution. -->
```

This protects both humans and LLMs from treating the described system as more complete than it is.

---

## LLM Minimization Enforcement

### Hard Rule (Add to CLAUDE.md, DEVELOPMENT_PATTERNS_CRUXDEV.md, every build plan)

```
RULE: LLM MINIMIZATION

If it can be code, it must be code. The LLM is invoked ONLY for tasks
that require language understanding:
- Reading code and identifying issues
- Generating fixes for identified issues
- Evaluating audit independence
- Writing or rewriting code/documentation

Everything else — loops, counters, timeouts, state transitions, rollback
triggers, termination conditions, schema validation, file I/O, dependency
tracking — is deterministic code.

Any proposal where the LLM handles something code could handle is a red
flag requiring explicit justification.
```

### Enforcement Mechanism

This rule cannot be enforced by the LLM (the LLM will route around it). It must be enforced by:

1. **Code review** — human reviews every component for LLM minimization
2. **Architecture tests** — the engine's test suite verifies that state transitions, counting, timing, and rollback are deterministic (no LLM calls in those paths)
3. **Interface boundary** — the `LLMDispatcher` interface is the ONLY way to call the LLM. If code doesn't go through that interface, it doesn't involve the LLM. Grep for the interface to verify scope.

---

## Impact on Existing Documents

### CRUX_ECOSYSTEM_PLAN.md

Phase 2 (CruxDev convergence engine core) changes completely. It was "write markdown engine files." It becomes "write Python engine code." The markdown engine files become specifications that the code implements, not instructions the LLM follows.

### DEVELOPMENT_PATTERNS_CRUXDEV.md

The methodology stays. The lights-out execution model stays. But Section 11 (The Complete Autonomous Lifecycle) gains a critical qualifier: the lifecycle is driven by the deterministic engine, not by the LLM following instructions. The LLM is a tool the engine calls.

### CruxDev.md

The architecture section changes. The `engine/` directory contains Python code, not markdown specs. The markdown specs move to `specs/` as reference documents for what the code implements.

### ADOPTION_PLAYBOOK.md / ADOPTION_PROCESS.md

These reference the convergence engine. Until the code engine exists, they should annotate which steps are LLM-driven (current) vs. engine-driven (target).

---

## Sequencing Within the Ecosystem

```
This plan runs AFTER or IN PARALLEL with:
  - CruxCLI hard fork (BUILD_PLAN_001_HARD_FORK)
  - CruxDev skills writing (CRUX_ECOSYSTEM_PLAN Phase 3)

This plan REPLACES:
  - CRUX_ECOSYSTEM_PLAN Phase 2 (engine core was markdown, now code)

This plan ENABLES:
  - CRUX_ECOSYSTEM_PLAN Phase 5 (autonomous evolution requires deterministic engine)
  - Reliable lights-out operation (the entire point)
```

---

## Open Questions

### Q1: Language for the Engine

Python is the natural choice (Crux is Python, tests are pytest, MCP server is Python). But CruxCLI is TypeScript/Bun. Should the engine be:

a) Python (consistent with Crux, can use Crux MCP directly)
b) TypeScript (consistent with CruxCLI, can be embedded in the terminal agent)
c) Both (Python engine for standalone use, TypeScript engine embedded in CruxCLI)

**Proposed answer:** Python first. The engine calls CruxCLI/Claude Code as a subprocess. If embedding in CruxCLI proves necessary, port later.

### Q2: How Does the Engine Call the LLM?

Options:
a) Through Crux MCP (audit tools already exist)
b) Direct Anthropic API calls (structured output, tool use)
c) Through CruxCLI (launch a Claude Code session as subprocess)
d) Through any LLM via a provider abstraction (like Crux's audit backend)

**Proposed answer:** Provider abstraction (d). The engine shouldn't be locked to one LLM. Crux already has `crux_audit_backend.py` with Ollama, Anthropic, OpenAI, and Claude subagent backends. Extend that pattern.

### Q3: Does the Markdown Engine Still Have Value?

**Answer: No.** The deterministic engine is exposed as an MCP server. Claude Code calls `converge()` once and the engine runs to completion. No markdown fallback needed — the engine is always available via MCP. Markdown engine files are retired and replaced by code. Skills remain markdown (they are LLM instructions for LLM tasks, which is the correct use of markdown).

---

## Definition of Done

1. `src/engine/convergence.py` drives a real convergence loop with deterministic state management
2. Round counting, timeouts, rollback, termination — all in code, zero LLM involvement
3. LLM invoked only through `LLMDispatcher` interface for language-understanding tasks
4. LLM outputs validated against JSON schemas before engine acts on them
5. Dependency graph maintained by code (tree-sitter or AST), not LLM
6. Subagent coordination in code with real parallelism
7. Conflict resolution protocol implemented in code
8. LLM minimization rule in CLAUDE.md and enforced by architecture tests
9. Operational documents annotated with implementation status
10. Engine tested against a real project and verified to converge deterministically
11. 100% test coverage on all engine code
