# CruxDev: Autonomous Convergence Framework for AI-Driven Development

**Version:** 0.1.0 (Design Document)
**Created:** 2026-03-18
**Status:** DESIGN PHASE

---

## 1. Vision and Philosophy

### 1.1 The Problem

Current AI coding agent frameworks fall into two camps:

1. **Ad hoc prompting** -- the user drives every step, saying "do it again" until quality converges. This is a powerful methodology, but one where the human is the loop controller. A disciplined user's prompt toolkit works because that user keeps saying "audit it again" until convergence. Most users are not that disciplined.

2. **Skill-based frameworks** (e.g., Superpowers) -- modular skills auto-activate based on context, subagents isolate tasks, and git worktrees provide safety. But the agent still executes tasks *linearly*. There is no built-in convergence engine. The agent does what it's told, skill by skill, but doesn't drive the audit-fix-re-audit loop autonomously.

Neither framework runs to convergence on its own. The human must either drive the loop (ad hoc prompting) or trust that a single pass is sufficient (Superpowers).

### 1.2 The CruxDev Thesis

**CruxDev is a framework where the agent drives itself to convergence.** The human provides the goal and constraints. The agent plans, audits the plan, executes, audits the code, audits the docs, fixes, re-audits, and repeats -- all without the human saying "do it again." The convergence loop is built into the framework, not the user's prompt.

The name "CruxDev" reflects the core idea: find the *crux* -- the critical path, the load-bearing decision, the thing that actually matters -- and converge on it through systematic iteration.

### 1.3 Design Principles

| Principle | Meaning |
|-----------|---------|
| **Convergence is the product** | The framework's output is not code -- it is *converged* code. Code that has survived multi-pass auditing across all dimensions. |
| **The agent drives the loop** | No human intervention required between "start" and "done." The human sets the goal; the agent converges. |
| **Honest checkpoints over optimistic status** | Every status claim is verified empirically. Coverage numbers come from tools, not assertions. |
| **Skills compose, loops orchestrate** | Skills are the building blocks. The convergence engine chains them into autonomous loops. |
| **Environment truth over document truth** | The viability check pattern: verify the actual machine, not just the plan document. |
| **Two consecutive clean passes** | One clean pass is not convergence. Anchoring bias from the previous fix round creates false confidence. |
| **Explicit over implicit** | Every state transition, every audit dimension, every termination condition is named and documented. |

### 1.4 How CruxDev Differs from Superpowers

| Dimension | Superpowers | CruxDev |
|-----------|-------------|---------|
| **Loop control** | Human-driven ("start a new task") | Agent-driven (convergence engine runs autonomously) |
| **Audit model** | Two-stage review (spec compliance + code quality) per task | Multi-dimensional audit (8 code + 5 doc dimensions) with convergence loop |
| **Convergence criterion** | Task passes review | Two consecutive independent clean passes across all dimensions |
| **Plan auditing** | Plans are reviewed once before execution | Plans undergo focused + full-plan + viability audit loops, each converging independently |
| **Environment verification** | Not present | Viability assessment checks actual machine state |
| **Coverage enforcement** | TDD skill enforces write-test-first | TDD + coverage-by-coincidence detection + empirical line-number verification |
| **Deferred item tracking** | Not formalized | Severity-based triage with explicit Known Gaps section and machine-readable state |
| **Checkpoint honesty** | Not addressed | Honest annotation rule: caveats on every checked box |
| **Agent-parallel auditing** | `dispatching-parallel-agents` skill exists | Integrated into convergence engine: code agent + doc agent + test agent run in parallel |
| **Prompt toolkit** | Not present (skills auto-activate) | Proven prompt patterns encoded as triggerable skill chains |
| **Token budget** | <2k core + skill retrieval | Similar: <2k bootstrap + on-demand skill loading |
| **Platform support** | Claude Code, Codex, OpenCode, Cursor, Gemini | Claude Code, Codex, OpenCode (extensible) |

---

## 2. Architecture

### 2.1 Directory Structure

```
cruxdev/
├── CRUXDEV.md                          # Bootstrap prompt (the one file agents always read)
├── LICENSE
├── README.md
│
├── engine/                             # The convergence engine (CruxDev's key innovation)
│   ├── CONVERGENCE.md                  # Master convergence loop specification
│   ├── PLAN_CONVERGENCE.md             # Plan audit convergence sub-loop
│   ├── CODE_CONVERGENCE.md             # Code audit convergence sub-loop
│   ├── DOC_CONVERGENCE.md              # Documentation audit convergence sub-loop
│   └── VIABILITY.md                    # Environment viability assessment protocol
│
├── skills/                             # Modular skills (Superpowers-compatible structure)
│   ├── planning/
│   │   ├── SKILL.md                    # Full planning cycle skill
│   │   ├── plan-template.md            # Plan document template
│   │   └── plan-audit-checklist.md     # 8-point audit checklist
│   │
│   ├── tdd/
│   │   ├── SKILL.md                    # Test-driven development enforcement
│   │   ├── coverage-verification.md    # Coverage-by-coincidence detection
│   │   └── anti-patterns.md            # TDD anti-patterns reference
│   │
│   ├── auditing/
│   │   ├── SKILL.md                    # Audit methodology (focused + full + viability)
│   │   ├── code-audit-dimensions.md    # 8 code audit dimensions
│   │   ├── doc-audit-dimensions.md     # 5 documentation audit dimensions
│   │   └── issue-tracking.md           # Issue ID format, severity triage
│   │
│   ├── executing/
│   │   ├── SKILL.md                    # Plan execution with task conversion
│   │   ├── checkpoint-honesty.md       # Honest checkpoint annotation rules
│   │   └── session-crash-recovery.md   # Resume protocol after context loss
│   │
│   ├── subagent-delegation/
│   │   ├── SKILL.md                    # Subagent dispatch and review
│   │   ├── implementer-prompt.md       # Implementer subagent template
│   │   ├── spec-reviewer-prompt.md     # Spec compliance reviewer template
│   │   ├── code-reviewer-prompt.md     # Code quality reviewer template
│   │   └── parallel-audit-prompt.md    # Parallel audit agent template
│   │
│   ├── git-worktrees/
│   │   ├── SKILL.md                    # Git worktree isolation
│   │   └── cleanup.md                  # Branch finishing protocol
│   │
│   ├── viability-assessment/
│   │   ├── SKILL.md                    # Environment verification
│   │   └── environment-checks.md       # Platform-specific check patterns
│   │
│   ├── convergence-driving/
│   │   ├── SKILL.md                    # How to drive a convergence loop
│   │   ├── termination-criteria.md     # Two-consecutive-clean-pass rule
│   │   └── safety-valves.md            # Max-round limits, triage escalation
│   │
│   ├── honest-tracking/
│   │   ├── SKILL.md                    # Progress tracking with integrity
│   │   ├── deferred-items.md           # Known Gaps pattern
│   │   └── coverage-verification.md    # Empirical coverage checking
│   │
│   ├── prompt-patterns/
│   │   ├── SKILL.md                    # Proven prompt patterns as a skill
│   │   └── prompt-library.md           # All prompt templates with usage
│   │
│   ├── patterns-capture/
│   │   ├── SKILL.md                    # Methodology extraction and preservation
│   │   └── patterns-template.md        # Template for project patterns files
│   │
│   ├── systematic-debugging/
│   │   ├── SKILL.md                    # Root-cause-first debugging (adapted from Superpowers)
│   │   └── diagnostic-checklist.md     # Structured diagnostic evidence gathering
│   │
│   ├── brainstorming/
│   │   ├── SKILL.md                    # Idea-to-design refinement (adapted from Superpowers)
│   │   └── design-doc-template.md      # Design document output format
│   │
│   ├── data-safety/
│   │   ├── SKILL.md                    # Atomic writes, backups, idempotency
│   │   └── patterns.md                 # Write-then-rename, rotating backups
│   │
│   ├── state-machines/
│   │   ├── SKILL.md                    # State machine design patterns
│   │   └── bridge-functions.md         # Cross-system state verification
│   │
│   └── writing-skills/
│       └── SKILL.md                    # Meta-skill: how to write CruxDev skills
│
├── adapters/                           # Platform-specific integration
│   ├── claude-code/
│   │   ├── CLAUDE.md                   # Claude Code bootstrap (injected into .claude/CLAUDE.md)
│   │   ├── hooks/
│   │   │   └── session-start.sh        # Session start hook
│   │   └── commands/
│   │       ├── cruxdev-plan.md         # /cruxdev-plan slash command
│   │       ├── cruxdev-execute.md      # /cruxdev-execute slash command
│   │       └── cruxdev-converge.md     # /cruxdev-converge slash command
│   │
│   ├── codex/
│   │   ├── codex-config.md             # Codex adapter configuration
│   │   └── INSTALL.md                  # Codex installation instructions
│   │
│   └── opencode/
│       ├── opencode-config.json        # OpenCode adapter configuration
│       └── INSTALL.md                  # OpenCode installation instructions
│
├── templates/                          # Reusable document templates
│   ├── BUILD_PLAN_TEMPLATE.md          # Plan document skeleton
│   ├── CODE_AUDIT_STATE.json           # Audit state file template
│   └── PATTERNS_TEMPLATE.md            # Patterns file template
│
└── tests/                              # Framework self-tests
    ├── test-skill-loading.md           # Verify skills load correctly
    ├── test-convergence-loop.md        # Verify convergence terminates
    └── test-platform-adapters.md       # Verify each platform works
```

### 2.2 Token Budget

The bootstrap file (`CRUXDEV.md`) must stay under **2,000 tokens**. It contains:
- Framework identity and core principles (200 tokens)
- Convergence engine pointer (300 tokens)
- Skill index with trigger conditions (800 tokens)
- Platform adapter detection (200 tokens)
- Session start protocol (300 tokens)
- Emergency fallback if skills can't load (200 tokens)

Skills are loaded on demand, not at session start. The bootstrap file tells the agent *when* to load each skill, not what each skill contains.

**Per-Skill Token Budgets:** Each skill declares a `token-budget` in its frontmatter -- the maximum tokens for the SKILL.md plus all loaded supporting files combined. Target budgets: core skills (convergence-driving, auditing, executing) ~800-1000 tokens; supporting skills (data-safety, state-machines, brainstorming) ~400-600 tokens. If a skill exceeds its budget, split content into a supporting file loaded only when needed.

### 2.3 Skill Loading Protocol

```
Session Start
    │
    ▼
Read CRUXDEV.md (<2k tokens)
    │
    ▼
Detect platform (Claude Code / Codex / OpenCode)
    │
    ▼
Load platform adapter
    │
    ▼
Determine current phase:
    ├── No plan exists       → load skills/planning/SKILL.md
    ├── Plan exists, not run → load engine/CONVERGENCE.md + skills/executing/SKILL.md
    ├── Execution in progress → load engine/CONVERGENCE.md + resume from state file
    ├── Audit in progress    → load engine/*_CONVERGENCE.md for active audit type
    └── User says "plan X"  → load skills/planning/SKILL.md
    │
    ▼
Execute with convergence engine driving the loop
```

---

## 3. The Convergence Engine (CruxDev's Key Innovation)

### 3.1 The Master Loop

The convergence engine is the orchestrator. It chains skills together into autonomous loops that run without human intervention. The human provides a goal; the engine converges.

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MASTER CONVERGENCE LOOP                       │
│                                                                      │
│  Input: Goal + Constraints                                           │
│  Output: Converged code + docs + tests                               │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ PHASE A: PLAN CONVERGENCE                                    │    │
│  │                                                              │    │
│  │  write plan                                                  │    │
│  │      │                                                       │    │
│  │      ▼                                                       │    │
│  │  ┌─── FOCUSED AUDIT LOOP ──────────────────────────────┐    │    │
│  │  │ audit phase N → find issues → fix → re-audit        │    │    │
│  │  │ TERMINATE: zero issues found                        │    │    │
│  │  └─────────────────────────────────────────────────────┘    │    │
│  │      │ (for each new/modified phase)                        │    │
│  │      ▼                                                       │    │
│  │  ┌─── FULL-PLAN AUDIT LOOP ────────────────────────────┐    │    │
│  │  │ audit all phases → find cross-phase issues → fix    │    │    │
│  │  │ TERMINATE: zero issues found                        │    │    │
│  │  └─────────────────────────────────────────────────────┘    │    │
│  │      │                                                       │    │
│  │      ▼                                                       │    │
│  │  ┌─── VIABILITY LOOP ──────────────────────────────────┐    │    │
│  │  │ check environment → find blockers → fix plan → re-  │    │    │
│  │  │ check                                                │    │    │
│  │  │ TERMINATE: YES with zero caveats                    │    │    │
│  │  └─────────────────────────────────────────────────────┘    │    │
│  │      │                                                       │    │
│  │  Output: Converged plan, verified executable                 │    │
│  └──────────────────────────────────────────────────────────────┘    │
│         │                                                            │
│         ▼                                                            │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ PHASE B: EXECUTION                                           │    │
│  │                                                              │    │
│  │  convert checkboxes → tasks                                  │    │
│  │  for each task:                                              │    │
│  │    dispatch subagent → implement (TDD) → two-stage review   │    │
│  │    mark task complete                                        │    │
│  │  checkpoint: mid-execution audit of progress tracker         │    │
│  │                                                              │    │
│  │  Output: All tasks complete, all tests passing               │    │
│  └──────────────────────────────────────────────────────────────┘    │
│         │                                                            │
│         ▼                                                            │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ PHASE C: CODE + DOC CONVERGENCE                              │    │
│  │                                                              │    │
│  │  ┌─── CONVERGENCE LOOP ────────────────────────────────┐    │    │
│  │  │ Round N:                                             │    │    │
│  │  │   Agent 1: audit code (8 dimensions)                │    │    │
│  │  │   Agent 2: audit docs (5 dimensions)                │    │    │
│  │  │   Agent 3: run tests + coverage                     │    │    │
│  │  │   Synthesize findings → fix all → re-run tests      │    │    │
│  │  │                                                      │    │    │
│  │  │ TERMINATE: two consecutive clean passes              │    │    │
│  │  │   (second pass from independent agent/context)       │    │    │
│  │  │                                                      │    │    │
│  │  │ SAFETY VALVE: max 5 rounds → escalate to human      │    │    │
│  │  └─────────────────────────────────────────────────────┘    │    │
│  │                                                              │    │
│  │  Output: CONVERGED — code matches plan, tests pass,          │    │
│  │    docs match code, no phantom references, no stale counts   │    │
│  └──────────────────────────────────────────────────────────────┘    │
│         │                                                            │
│         ▼                                                            │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ PHASE D: PATTERNS UPDATE                                     │    │
│  │                                                              │    │
│  │  Review all work done → update project patterns file          │    │
│  │  Audit patterns file → fix gaps → re-audit                  │    │
│  │                                                              │    │
│  │  Output: Methodology captured for future rounds              │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  DONE: Report convergence status, issue counts, coverage numbers    │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Convergence State Machine

Each convergence loop tracks its state in a JSON file:

```json
{
  "loop_type": "code_convergence",
  "status": "in_progress",
  "current_round": 3,
  "max_rounds": 5,
  "rounds": [
    {
      "round": 1,
      "phase": "complete",
      "issues_found": 13,
      "issues_fixed": 13,
      "classes": ["placeholder_code", "missing_functions", "non_atomic_writes"]
    },
    {
      "round": 2,
      "phase": "complete",
      "issues_found": 9,
      "issues_fixed": 9,
      "classes": ["connection_leaks", "phantom_commands"]
    },
    {
      "round": 3,
      "phase": "auditing",
      "issues_found": 0,
      "issues_fixed": 0,
      "classes": [],
      "note": "First clean pass -- need one more independent pass"
    }
  ],
  "consecutive_clean_passes": 1,
  "convergence_threshold": 2,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T14:30:00Z",
  "error_state": null
}
```

#### Error States

Infrastructure failures can interrupt convergence. The state machine handles these explicitly:

```json
{
  "error_state": {
    "type": "test_suite_failure",
    "message": "Test runner exited with code 2: 'No module named pytest'",
    "occurred_at": "2026-03-18T15:00:00Z",
    "round_interrupted": 3,
    "recovery_action": "fix_infrastructure"
  }
}
```

| Error Type | Trigger | Recovery Action | Max Retries |
|------------|---------|-----------------|-------------|
| `test_suite_failure` | Test runner cannot execute (missing deps, config error, syntax error in test harness) | Fix the infrastructure issue, then resume from the interrupted round | 2 |
| `subagent_failure` | Subagent crashes, times out, or returns incoherent output | Retry with a fresh subagent. If 2 retries fail, escalate to human | 2 |
| `file_system_error` | Cannot read/write files (permissions, disk full, missing directories) | Log the error, attempt to fix (create dirs, check permissions). If unfixable, escalate to human | 1 |
| `context_overflow` | Agent context window exceeded mid-round | Write current state to convergence JSON, terminate session. Next session resumes from state file | 0 (resume in new session) |
| `build_failure` | Project build fails (compilation error, missing dependency) | Treat as a HIGH-severity audit finding. Apply systematic-debugging skill, then resume convergence | 2 |

**Error state transitions:**
- On error → set `error_state` in JSON, set `status` to `"error_recovery"`
- On recovery success → clear `error_state`, set `status` to `"in_progress"`, resume from interrupted round
- On max retries exceeded → set `status` to `"escalated"`, present full error context to human

### 3.3 Termination Conditions

| Loop Type | Termination Condition | Safety Valve |
|-----------|----------------------|--------------|
| Focused audit | Zero issues in one pass | Max 5 passes |
| Full-plan audit | Zero issues in one pass | Max 3 passes |
| Viability | YES with zero caveats | Max 3 passes |
| Code convergence | Two consecutive clean passes (second independent) | Max 5 rounds |
| Doc convergence | Two consecutive clean passes | Max 3 rounds |
| Combined code+doc | Two consecutive clean passes across ALL dimensions | Max 5 rounds |

**Safety valve behavior:** When max rounds are reached, the engine:
1. Logs all remaining issues with severity
2. Presents a triage report to the human
3. Asks for direction: fix specific issues, defer, or accept

**Diminishing returns policy:** If after 3 completed rounds only LOW-severity issues remain (no HIGH or MEDIUM), the engine does not continue to max rounds. Instead, it:
1. Compiles the list of remaining LOW-severity issues
2. Presents them to the user with: "Only low-severity issues remain after 3 rounds. Accept with known gaps, or continue auditing?"
3. If the user accepts, the engine marks convergence as `"converged_with_known_gaps"` and records all LOW items in the Known Gaps section
4. If the user says continue, the engine runs up to 2 more rounds (max 5 total) targeting only the remaining LOW issues

This prevents spending 2 additional full-audit rounds chasing cosmetic issues when the code is functionally converged.

### 3.4 How "Autonomous to Convergence" Works in Practice

The human says one thing:

```
"Plan and build [goal] with [constraints]. Converge."
```

The engine does the rest:

1. Loads `skills/planning/SKILL.md` and writes the plan
2. Loads `engine/PLAN_CONVERGENCE.md` and runs focused audits until clean, then full-plan audits until clean, then viability until clean
3. Loads `skills/executing/SKILL.md` and converts checkboxes to tasks
4. For each task, loads `skills/subagent-delegation/SKILL.md` and dispatches implementer + reviewers
5. Mid-execution, loads `skills/honest-tracking/SKILL.md` and audits the progress tracker
6. After execution, loads `engine/CODE_CONVERGENCE.md` and `engine/DOC_CONVERGENCE.md`
7. Runs parallel audit agents, synthesizes findings, fixes, re-audits
8. Repeats until two consecutive clean passes
9. Reviews all work done and updates the project patterns file with methodology learnings
10. Reports: CONVERGED, with issue counts, coverage numbers, and known gaps

No human intervention between steps 1 and 10 unless a safety valve triggers.

### 3.5 The Independence Requirement for Clean Passes

The two-consecutive-clean-pass rule has a critical nuance: the second pass must come from an **independent context**. This means:

- Different subagent (fresh context, no memory of previous fixes)
- Different starting assumptions (reads files fresh, doesn't know what was just fixed)
- Same audit dimensions and rigor

Without independence, the second pass suffers from anchoring bias -- the auditor just fixed problems and unconsciously skips areas they believe are clean. An independent agent has no such bias.

Implementation: the convergence engine dispatches a fresh subagent for the second clean pass, providing only the file list and audit dimensions, not the previous round's findings.

**Note on subagent independence:** Subagents share the filesystem, so 'independence' means the second-pass agent should NOT read the convergence state file's previous round details. It should perform a fresh audit from the codebase alone, then update the state file with its findings.

---

## 4. Skill Specifications

### 4.1 Skill Format

Every skill follows this structure:

```markdown
---
name: skill-name
description: "Use when [triggering conditions]"
loads: [list of supporting files in this skill directory]
chains-to: [list of skills that typically follow this one]
token-budget: <max tokens for SKILL.md + all loaded files combined>
---

# Skill Name

## Overview
[1-2 sentences: what this skill does and why]

## When to Activate
[Specific triggers -- conversation patterns, project state, user commands]

## Core Pattern
[The main workflow, as concise as possible]

## Verification
[How to confirm the skill was applied correctly]

## Anti-Patterns
[What NOT to do]

## Integration
[How this skill chains with others in the convergence engine]
```

### 4.2 Skill Index

#### Planning Skills

| Skill | Purpose | Triggers | Key Content |
|-------|---------|----------|-------------|
| `planning` | Full planning cycle from problem to audited plan | "plan X", "I want to build X", new project start | Plan document format, required sections, phase design, progress tracker conventions |
| `prompt-patterns` | Proven prompt patterns as reusable triggers | Any planning or audit session | Big bang prompt, viability question, "do it again", numbered decisions, checkpoint audit |
| `brainstorming` | Collaborative idea-to-design refinement before implementation | "I want to build X", new feature ideation, creative work | Incremental questioning, 2-3 approach proposals, design doc output, YAGNI enforcement |

#### Quality Skills

| Skill | Purpose | Triggers | Key Content |
|-------|---------|----------|-------------|
| `tdd` | Test-driven development enforcement | Any code implementation task | RED-GREEN-REFACTOR, no code without failing test, coverage-by-coincidence detection |
| `auditing` | Multi-dimensional audit methodology | "audit X", post-execution, convergence loop | 8 code dimensions, 5 doc dimensions, focused vs full-plan taxonomy, issue tracking format |
| `viability-assessment` | Environment verification against plan | Pre-execution, viability question | Check installed deps, import paths, file existence, config files, existing code |
| `honest-tracking` | Progress tracking with integrity | During execution, checkpoint audits | Honest annotation, Known Gaps section, deferred item triage, coverage verification |
| `systematic-debugging` | Root-cause-first debugging methodology | Bug reports, test failures, unexpected behavior, "it's broken" | 4-phase investigation (root cause → pattern analysis → hypothesis testing → implementation), no fixes before diagnosis |

#### Execution Skills

| Skill | Purpose | Triggers | Key Content |
|-------|---------|----------|-------------|
| `executing` | Plan execution with task conversion | "execute the plan", post-planning | Checkbox-to-task conversion, dual tracking, session crash recovery |
| `subagent-delegation` | Dispatch fresh agents per task | During execution of implementation tasks | Implementer dispatch, two-stage review, model selection, escalation handling |
| `git-worktrees` | Isolated workspace creation | Before any implementation work | Worktree creation, baseline testing, cleanup protocol |

#### Convergence Skills

| Skill | Purpose | Triggers | Key Content |
|-------|---------|----------|-------------|
| `convergence-driving` | How to drive any convergence loop | Engine-internal, any audit loop | Two-consecutive-clean-pass rule, safety valves, independence requirement |
| `patterns-capture` | Extract and preserve methodology from completed work | Phase D of master loop, "extract patterns", post-convergence | What to capture (decisions, anti-patterns, conventions, tooling), patterns file format, audit of captured patterns |

#### Infrastructure Skills

| Skill | Purpose | Triggers | Key Content |
|-------|---------|----------|-------------|
| `data-safety` | Atomic writes, backups, idempotency | Any code that writes to files or databases | Write-then-rename, rotating backups, dual-write pattern |
| `state-machines` | State machine design and bridge functions | Designing systems with explicit states | Terminal vs non-terminal states, cross-system bridge functions, rename propagation |
| `writing-skills` | Meta-skill: authoring new CruxDev skills | "create a new skill", framework extension | Skill template, naming conventions, testing requirements |

### 4.3 Skill Detail: `convergence-driving`

This is CruxDev's signature skill -- the one that doesn't exist in Superpowers.

```markdown
---
name: convergence-driving
description: "Use when running any audit-fix-re-audit loop to convergence"
chains-to: [auditing, honest-tracking]
token-budget: 1000
---

# Convergence Driving

## Overview
Drive an audit-fix-re-audit loop to termination without human intervention.
The agent controls the loop, not the human. This skill owns the *loop mechanics*
(when to re-audit, when to stop, when to escalate) but delegates the actual
audit work to the `auditing` skill.

## Core Pattern

1. Initialize convergence state file (JSON, atomic writes)
2. Run audit (dispatch parallel agents if scope > 5 files)
3. Log findings with IDs, dimensions, severity
4. Fix all findings
5. Re-run test suite + coverage
6. If fixes introduced new issues, log as fix-regressions in THIS round
7. Increment round counter
8. Check termination:
   - Zero issues found? Increment consecutive_clean counter
   - Two consecutive clean passes (second independent)? → CONVERGED
   - Max rounds reached? → ESCALATE to human
   - Otherwise → goto step 2
9. On CONVERGED: report final stats, mark state file complete

**Context Overflow Recovery:** If context overflows mid-convergence, the agent resumes from the convergence state JSON file in a new session, picking up at the current round. The state file contains the round number, issues found, fixes applied, and current phase (auditing/fixing/verifying).

## The Independence Requirement

The second clean pass MUST come from a fresh subagent that:
- Has not seen previous round findings
- Reads all files fresh
- Uses the same audit dimensions
- Reports findings independently

One clean pass after fixing is NOT convergence. It is the auditor confirming
their own fixes, which has ~30% false-negative rate (empirical: R2 was "clean"
for code but R3 found 3 doc issues that a fresh eye caught).

## Safety Valves

- Max 5 rounds for code convergence
- Max 3 rounds for doc convergence
- Max 3 rounds for plan convergence
- On max reached: stop, triage, escalate with full issue report

## Anti-Patterns

- Declaring convergence after one clean pass
- Running the same agent for the second "clean" pass (not independent)
- Fixing issues without re-running the full test suite
- Ignoring fix-regression issues
- Counting deferred items as "not issues"
- **Net-Negative Fix Round:** If issue count increases for two consecutive rounds (fixes are making things worse), immediately escalate to human rather than continuing. Do not wait for the max-rounds safety valve.
```

### 4.4 Skill Detail: `auditing`

```markdown
---
name: auditing
description: "Use when auditing plans, code, or documentation for quality"
loads: [code-audit-dimensions.md, doc-audit-dimensions.md, issue-tracking.md]
chains-to: [convergence-driving]
token-budget: 1000
---

# Auditing

## Overview
Systematic multi-dimensional audit of plans, code, and documentation.
Three audit types, each catching different bug classes. This skill owns
*what to audit and how to evaluate it* (dimensions, issue format, taxonomy)
but delegates loop control (re-audit timing, termination, escalation) to
the `convergence-driving` skill.

## Audit Taxonomy

| Type | Scope | Catches | Prompt Pattern |
|------|-------|---------|----------------|
| Focused (plan) | Single phase | Internal consistency, dead fields, schema errors | "audit phase N for issues" |
| Full-plan | All phases | Cross-phase refs, state divergence, dependency errors | "audit the entire plan" |
| Viability | Plan vs environment | Missing deps, wrong tools, broken imports | "will this succeed?" |
| Code (post-exec) | All source files | Logic bugs, leaks, missing impls, phantom refs | "audit code against plan" |
| Documentation | All doc files | Stale counts, phantom commands, architecture drift | "audit docs against code" |

## Code Audit Dimensions (8)

1. Plan alignment -- does code match spec?
2. Correctness -- logic errors, race conditions, edge cases?
3. Test coverage -- every public function tested? Coverage verified empirically?
4. BDD alignment -- feature files match behavior?
5. Cross-module integration -- modules call each other correctly?
6. Data safety -- critical writes atomic? Backups before imports?
7. Documentation compliance -- docs describe code accurately?
8. Regression -- ALL existing tests still pass?

## Documentation Audit Dimensions (5)

1. Accuracy -- function signatures match code?
2. Completeness -- all new modules/functions documented?
3. Staleness -- hardcoded counts that should be live queries?
4. Phantom references -- documented things that don't exist?
5. Architecture alignment -- docs describe current state, not pre-build?

## Issue Format

ID: R{round}-{sequence}
File: path/to/file
Dimension: one of the 8+5 above
Severity: high | medium | low
Description: specific, actionable
Fix applied: boolean

## The Focused-Then-Full Pattern

1. Audit new/changed phase (2-3 focused passes → converge)
2. Audit full plan (1-2 passes → converge)
3. Viability check (1-2 passes → converge)

Empirical issue curve: 10 → 8 → 7 (focused), 14 → 8 → 0 (full-plan).
Total: ~54 issues across 7 pre-execution passes.
```

### 4.5 Skill Detail: `viability-assessment`

```markdown
---
name: viability-assessment
description: "Use before executing any plan to verify it will actually work"
chains-to: [planning, executing]
token-budget: 800
---

# Viability Assessment

## Overview
Verify that a plan is executable against the ACTUAL environment, not just
internally consistent. Document audits catch ~87% of issues. Viability
checks catch the remaining ~13% -- and those 13% are all execution-blockers.

## The Five Steps

1. RE-READ the entire plan with fresh eyes (not skim)
2. SIMULATE execution mentally -- walk through each phase step by step
3. VERIFY the environment:
   - Are referenced tools/deps installed?
   - Do import paths work?
   - Does project config exist and match assumptions?
   - Do referenced files/directories exist?
   - Does code already exist for files the plan says to CREATE?
4. REPORT: YES/NO with SPECIFIC NUMBERED CAVEATS
5. FIX caveats in the plan, then RE-VERIFY

## Environment Check Commands (adapt per stack)

```bash
# Dependencies installed?
python3 -c "import module_name" 2>&1
pip list | grep package_name
mix deps.get --check    # Elixir
npm ls package_name     # Node

# Files exist that plan assumes?
ls path/to/expected/file 2>&1

# Files already exist that plan says to create?
ls path/to/file/plan/says/to/create 2>&1

# Project config correct?
cat mix.exs | grep "version"
cat package.json | grep "version"
```

## Critical Pattern: Existing Code Detection

For EVERY file the plan says to CREATE, check if it already exists.
If it does:
- Update plan status from "NOT STARTED" to "PARTIALLY BUILT (annotation)"
- Verify existing code against plan spec
- Do NOT overwrite working code with plan-spec code

This prevents the "session re-creates existing work" failure mode.
```

### 4.6 Skill Detail: `honest-tracking`

```markdown
---
name: honest-tracking
description: "Use when tracking progress to ensure checkpoints are truthful"
loads: [deferred-items.md, coverage-verification.md]
chains-to: [auditing, convergence-driving]
token-budget: 800
---

# Honest Tracking

## Overview
Progress checkboxes lie. Coverage numbers lie. This skill enforces empirical
verification of every status claim.

## Rules

1. NEVER mark a checkbox complete without verifying the claim
2. ALWAYS annotate caveats on checked boxes:
   - [x] Coverage check >= 100% -- module_a 100%, module_b 98% (2 edge cases deferred)
3. ALWAYS verify coverage with tool output, not memory:
   - Run coverage tool
   - Check specific line numbers in "Missing" column
   - Only then mark coverage checkbox
4. MAINTAIN a Known Gaps section for deferred items:
   - Issue ID, severity, description, reason for deferral

## Coverage by Coincidence Detection

A test can pass and appear to cover a code path while actually exercising
a different branch. The ONLY source of truth is the coverage tool's
line-by-line report.

After writing coverage-targeting tests:
1. Run coverage with line-missing report
2. Check: are the specific target line numbers gone from "Missing"?
3. If still missing: your test hit a different branch. Trace the actual
   execution path step by step.

## Deferred Item Triage

| Severity | Action |
|----------|--------|
| HIGH | Fix now, no deferral |
| MEDIUM | Fix if time, otherwise defer with tracking |
| LOW | Defer with tracking |

Every deferred item gets: ID, severity, description, entry in Known Gaps
section, entry in machine-readable state file.
```

### 4.7 Skill Detail: `prompt-patterns`

These are field-tested prompts discovered through iterative use of AI coding agents. Each prompt exploits a specific leverage point in how agents process instructions. They are not theoretical -- they emerged from real convergence cycles and have been validated across multiple projects.

```markdown
---
name: prompt-patterns
description: "Use when recognizing standard prompt triggers or when the user needs efficient agent interaction patterns"
loads: [prompt-library.md]
chains-to: [planning, convergence-driving, auditing]
token-budget: 900
---

# Prompt Patterns

## Overview
A library of proven prompt patterns that drive agent behavior efficiently.
These patterns are the manual precursors to CruxDev's automated convergence
engine -- they represent what a skilled user says to get convergence without
framework automation. CruxDev automates most of these, but they remain useful
for manual overrides and edge cases.

NOTE: This skill provides the user-facing prompt toolkit. The `convergence-driving`
skill provides the engine-internal automation of the same patterns. When the engine
is running autonomously, these prompts are unnecessary -- the engine applies the
underlying patterns automatically.

## When to Activate
- User asks "how do I drive the agent?"
- User is in a manual audit session without the convergence engine
- User needs to override or supplement the convergence engine
- Planning or audit session where the user wants direct control

## Prompt Library

### 1. The "Big Bang" Prompt

**Purpose:** A single prompt that drives full plan execution + audit + convergence.
This is the manual equivalent of what the convergence engine does automatically.

**The prompt:**
> "Run the build plan exactly as planned, including creating tasks planning the
> entire thing out, including documentation changes, code audits, documentation
> audits, and then after you run it, audit and fix any issues in the code, and
> in the docs, over and over, until you report there aren't any left to fix."

**Why it works:** It chains every phase (planning, execution, code audit, doc audit,
fix, re-audit) into a single instruction with an explicit termination condition
("until you report there aren't any left to fix"). The agent cannot stop early
because the prompt demands convergence.

**CruxDev equivalent:** `engine/CONVERGENCE.md` master loop. The engine does this
automatically without the user needing to say it.

### 2. The Viability Question

**Purpose:** Forces the agent to verify a plan against the actual environment
before execution begins, catching the ~13% of issues that document audits miss.

**The prompt:**
> "If I tell you to run the plan exactly as planned, and then after you run it,
> I tell you to audit and fix any issues, over and over, until you report there
> aren't any left to fix, will the plan succeed?"

**Why it works:** The conditional framing ("if I tell you to...") forces the agent
to mentally simulate execution end-to-end. It cannot just say "yes" -- it must
evaluate every dependency, import path, tool version, and file existence.

**CruxDev equivalent:** `skills/viability-assessment/SKILL.md` + `engine/VIABILITY.md`.

### 3. "Do It Again"

**Purpose:** Forces a re-audit pass. Each pass finds different classes of problems
because the agent's attention shifts after fixing the previous round's issues.

**The prompt:**
> "Do it again."

**Why it works:** Deceptively simple. After a fix round, the agent has anchoring
bias -- it believes the areas it just fixed are clean. "Do it again" forces a
fresh pass that catches issues the previous pass was blind to. Empirically,
round 2 finds ~70% as many issues as round 1, and round 3 finds ~30%. Each pass
surfaces a different *class* of problem (e.g., R1 finds placeholder code, R2 finds
connection leaks, R3 finds doc staleness).

**CruxDev equivalent:** The convergence engine's automatic re-audit loop with
independent second-pass agents.

### 4. Numbered Decisions

**Purpose:** Efficient multi-decision responses when the agent asks several
questions at once.

**The prompt:**
> "1. yes 2. skip it."

**Why it works:** Agents often present numbered options or ask multiple questions.
Responding with numbered answers maps directly to the agent's numbered list,
eliminating ambiguity and saving tokens. The agent processes each decision
without needing to re-ask.

**CruxDev equivalent:** Not automated -- this is a human-efficiency pattern for
interactive sessions.

### 5. Checkpoint Audit

**Purpose:** Catches drift between what the plan says was done and what was
actually done. Progress trackers accumulate lies over long sessions.

**The prompt:**
> "Audit your list of checkboxes against the plan and resolve any gaps."

**Why it works:** Forces the agent to cross-reference two sources of truth (the
progress tracker and the plan document) and reconcile them. Common findings:
checkboxes marked complete for unfinished work, checkboxes missing for work
that was done, and caveats omitted from checked items.

**CruxDev equivalent:** `skills/honest-tracking/SKILL.md` checkpoint verification.

### 6. Scope Correction

**Purpose:** Corrects the agent when it interprets a task too narrowly. Agents
tend to minimize scope to reduce risk, which produces incomplete work.

**The prompt pattern:**
> "Not just [thing]. [broader scope]."

**Example:**
> "Not just the database module. All modules that touch user data."

**Why it works:** The "Not just X. Y." structure is unambiguous. It tells the agent
exactly what scope it incorrectly assumed and exactly what the correct scope is.
No room for misinterpretation.

**CruxDev equivalent:** The auditing skill's "full-plan audit" mode, which forces
cross-phase scope rather than single-phase focus.

### 7. Extract Methodology

**Purpose:** Captures patterns and lessons learned from a development round into
a reusable document, preventing knowledge loss between sessions.

**The prompt:**
> "Extract everything about how we did this into a development patterns file."

**Why it works:** Forces the agent to reflect on the entire session and distill
methodology from specific actions. The output becomes the project's institutional
memory -- patterns, anti-patterns, conventions, and decisions that future sessions
can reference.

**CruxDev equivalent:** Phase D (Patterns Update) of the master convergence loop.

## Verification
- Each prompt pattern has a documented "why it works" explanation
- Each pattern maps to a CruxDev engine/skill equivalent
- Patterns are validated against empirical convergence data

## Anti-Patterns
- Using these prompts instead of the convergence engine (the engine is more reliable)
- Assuming one "do it again" pass is sufficient (empirically, 2-4 passes are needed)
- Using vague scope corrections ("also check other stuff") instead of specific ones
- Skipping the viability question before execution

## Integration
The convergence engine automates patterns 1, 2, 3, 5, and 7. Patterns 4 and 6
are interactive-only and remain useful during manual sessions or when overriding
the engine. When the engine is active, these prompts serve as documentation of
the underlying methodology rather than active tools.
```

### 4.8 Skill Detail: `patterns-capture`

```markdown
---
name: patterns-capture
description: "Use after convergence to extract and preserve methodology learnings"
loads: [patterns-template.md]
chains-to: [writing-skills]
token-budget: 800
---

# Patterns Capture

## Overview
Extract decisions, conventions, anti-patterns, and tooling learnings from a
completed development round into a reusable patterns file. This is Phase D
of the master convergence loop. Without explicit capture, methodology is lost
between sessions.

NOTE: This skill handles the *capture* of patterns from completed work. The
`convergence-driving` skill handles the *loop* that determines when work is
complete. Patterns capture runs AFTER convergence is achieved, not during it.

## When to Activate
- Phase D of the master convergence loop (after code+doc convergence)
- User says "extract patterns" or "what did we learn?"
- End of any significant development round

## What to Capture

| Category | Examples | Source |
|----------|----------|--------|
| Architecture decisions | "We chose X over Y because Z" | Plan document, commit messages |
| Naming conventions | "All API endpoints use snake_case" | Code review findings |
| Anti-patterns discovered | "Don't use X because it causes Y" | Audit findings, fix-regression data |
| Stack-specific patterns | "Always use write-then-rename for config files" | Data safety findings |
| Testing conventions | "Integration tests use factory functions, not fixtures" | TDD findings |
| Tooling decisions | "Use tool X version Y for Z" | Viability assessment findings |
| Convergence insights | "Module X consistently needs 3+ audit rounds" | Convergence state data |

## Core Pattern

1. Read the convergence state files for all completed loops
2. Read the plan document and any audit findings
3. Identify recurring themes across rounds (what kept breaking? what patterns emerged?)
4. Write or update the project patterns file using the template
5. Audit the patterns file for completeness and accuracy
6. Fix any gaps, re-audit until clean

## Patterns File Format

```
# [Project] Development Patterns

## Architecture Decisions
[Decision, rationale, date, alternatives considered]

## Conventions
[Naming, file structure, API design, testing]

## Anti-Patterns
[What not to do, why, what to do instead]

## Stack Patterns
[Framework-specific patterns, library usage, config conventions]

## Convergence History
[Rounds to convergence, common issue classes, safety valve triggers]
```

## Verification
- Every architecture decision has a rationale
- Every anti-pattern has a "what to do instead"
- Convergence data is pulled from actual state files, not memory
- Patterns file passes a self-audit (no stale references, no phantom items)

## Anti-Patterns
- Writing patterns from memory instead of from convergence data
- Capturing only positive patterns (anti-patterns are equally valuable)
- Writing patterns so specific they only apply to one project
- Skipping the audit of the patterns file itself
```

### 4.9 Skill Detail: `systematic-debugging`

Adapted from Superpowers' `systematic-debugging` skill, with CruxDev convergence integration.

```markdown
---
name: systematic-debugging
description: "Use when investigating bugs, test failures, or unexpected behavior -- always find root cause before attempting fixes"
loads: [diagnostic-checklist.md]
chains-to: [convergence-driving, auditing]
token-budget: 600
---

# Systematic Debugging

## Overview
A four-phase root-cause-first debugging methodology. The core rule: ALWAYS
find root cause before attempting fixes. Symptom fixes are failure. This skill
integrates with CruxDev's convergence engine by feeding confirmed root causes
into the audit-fix-re-audit loop.

NOTE: This skill handles *diagnosis* of bugs -- finding the root cause. The
`convergence-driving` skill handles the *fix-and-verify loop* after the root
cause is identified. Debugging finds the problem; convergence ensures the fix
is complete and doesn't regress.

## When to Activate
- Bug reports or unexpected behavior
- Test failures with unclear cause
- "It's broken" or "this doesn't work"
- Fix attempts that keep failing (3+ failed fixes = restart debugging)
- During convergence rounds when an audit finding's root cause is unclear

## The Four Phases

### Phase 1: Root Cause Investigation
- Examine error messages thoroughly (full stack traces, not just the top line)
- Reproduce the issue consistently (document exact reproduction steps)
- Review recent changes (git log, diff against last known good state)
- Gather diagnostic evidence across component boundaries
- For multi-layered systems: add logging at each transition point
- DO NOT propose solutions during this phase

### Phase 2: Pattern Analysis
- Locate functionally equivalent code that works correctly
- Read reference implementations completely (not skim)
- Document precise differences between working and broken code
- Map all dependencies of the broken component

### Phase 3: Hypothesis and Testing
- Form specific, falsifiable hypotheses
- Test with minimal changes (one variable at a time)
- Verify results against expected behavior
- NEVER add multiple modifications simultaneously
- If hypothesis is wrong, return to Phase 1 with new evidence

### Phase 4: Implementation
- Create a failing test case that reproduces the bug
- Implement a single fix addressing the root cause
- Verify the fix: original symptom test passes, all other tests still pass
- If 3+ fix attempts fail, question the underlying architecture rather than
  continuing symptomatic patches

## Red Flags (Restart from Phase 1)
- Attempting quick fixes without investigation
- Adding multiple changes simultaneously
- Proposing solutions before tracing data flow
- "It should work now" without verification evidence
- Frustration-driven changes ("just try this")

## Integration with Convergence Engine
When a convergence audit round identifies a bug:
1. The convergence engine pauses the fix step
2. This skill activates for root cause analysis
3. Once root cause is confirmed, the fix is applied
4. The convergence engine resumes its re-audit loop
5. If the fix introduces regressions, they are caught in the next round

## Verification
- Root cause is documented with evidence (not just "I think it's X")
- A failing test exists before the fix is applied
- The fix addresses root cause, not symptoms
- All existing tests still pass after the fix

## Anti-Patterns
- Fixing symptoms without understanding root cause
- Skipping Phase 1 because "I know what's wrong"
- Testing multiple hypotheses simultaneously
- Declaring "fixed" without running the full test suite
- 95% of "no root cause found" cases reflect incomplete investigation
```

### 4.10 Skill Detail: `brainstorming`

Adapted from Superpowers' `brainstorming` skill, with CruxDev convergence-aware design output.

```markdown
---
name: brainstorming
description: "Use before any creative work -- creating features, building components, adding functionality, or modifying behavior. Explores intent, requirements, and design before implementation."
chains-to: [planning]
token-budget: 500
---

# Brainstorming

## Overview
Transform ideas into fully formed designs through collaborative dialogue.
Understand project context, ask questions incrementally, and obtain user
approval before any implementation begins. Every project -- regardless of
apparent simplicity -- requires this process. Simple projects often contain
unexamined assumptions that cause wasted effort.

NOTE: This skill handles *design exploration* before planning begins. The
`planning` skill handles *plan creation* after the design is approved. Brainstorming
produces a design; planning produces an executable plan from that design.

## When to Activate
- "I want to build X" (new feature or project)
- "How should we approach X?"
- Any creative work: new features, component design, architecture changes
- When the user describes a goal but not a solution

## Critical Gate
Do NOT invoke any implementation skill, write any code, scaffold any project,
or take any implementation action until a design has been presented and the
user has approved it.

## Core Pattern

1. **Explore context** -- Read project files, docs, recent commits to understand
   the current state. Do not ask the user questions you can answer from the codebase.
2. **Ask clarifying questions** -- One question per message. Prefer multiple-choice
   over open-ended. Each question should narrow the design space.
3. **Propose approaches** -- Present 2-3 approaches with trade-offs (complexity,
   performance, maintainability, time). Apply YAGNI ruthlessly.
4. **Present design** -- Break design into sections. Validate incrementally with
   the user. Remain flexible -- the user may pivot.
5. **Write design doc** -- Capture the approved design in a document that the
   planning skill can consume.
6. **Hand off to planning** -- The ONLY skill invoked after brainstorming is
   `planning`. Do not skip ahead to execution.

## Verification
- User has explicitly approved the design (not just "looks good" -- confirmed scope)
- Design doc exists and is readable by the planning skill
- All user requirements are addressed (cross-reference against initial request)
- YAGNI applied: no features the user didn't ask for

## Anti-Patterns
- Jumping to implementation without design approval
- Asking open-ended questions when multiple-choice would work
- Presenting only one approach (always offer alternatives)
- Over-designing: adding features the user didn't request
- Skipping context exploration and asking questions the codebase already answers
```

---

## 5. Platform Adapters

### 5.1 Claude Code Adapter

Claude Code is the primary platform. The adapter provides:

**Bootstrap (`adapters/claude-code/CLAUDE.md`):**
```markdown
# CruxDev Framework

You are operating under the CruxDev autonomous convergence framework.

## Session Start
1. Read this file (you're doing it now)
2. Determine current phase from project state:
   - No plan file → planning phase
   - Plan exists, unchecked boxes → execution phase
   - Convergence state file exists → resume convergence loop
3. Load the appropriate skill from cruxdev/skills/
4. Load cruxdev/engine/CONVERGENCE.md if in any audit/convergence phase

## Core Rules
- Drive convergence loops autonomously (do not wait for "do it again")
- Two consecutive independent clean passes = convergence
- Verify all status claims empirically
- Log state to convergence JSON files (atomic writes)
- Max 5 rounds per convergence loop, then escalate

## Skill Loading
Load skills on demand. Do NOT load all skills at session start.
Read skill SKILL.md files only when entering that phase of work.

## Available Skills
[skill index with one-line descriptions and trigger conditions]
```

**Slash Commands:**
- `/cruxdev-plan [goal]` -- Start a new planning cycle
- `/cruxdev-execute` -- Begin execution of the current plan
- `/cruxdev-converge` -- Run code+doc convergence loop on current codebase
- `/cruxdev-audit [scope]` -- Run a single audit pass (focused or full)
- `/cruxdev-viability` -- Run viability assessment on current plan
- `/cruxdev-status` -- Report current convergence state

**Hooks:**
- `session-start.sh` -- Checks for convergence state files, warns about in-progress loops

**Installation:**
```bash
# Copy CruxDev into project
cp -r cruxdev/ .cruxdev/

# Add bootstrap to CLAUDE.md (append, don't replace)
# Installation commands must be idempotent. Before appending bootstrap content
# to CLAUDE.md, check if it already contains the CruxDev bootstrap marker
# (`# CruxDev Bootstrap`). If present, skip the append.
grep -q "# CruxDev Bootstrap" .claude/CLAUDE.md 2>/dev/null || cat .cruxdev/adapters/claude-code/CLAUDE.md >> .claude/CLAUDE.md

# Copy slash commands
cp .cruxdev/adapters/claude-code/commands/*.md .claude/commands/

# Copy hooks
cp .cruxdev/adapters/claude-code/hooks/*.sh .claude/hooks/
```

### 5.2 Codex Adapter

Codex (OpenAI's CLI agent) uses a different configuration mechanism. The adapter:

1. Symlinks `CRUXDEV.md` into the Codex instruction path (`AGENTS.md`)
2. Configures skill loading via Codex's file-read capabilities
3. Maps CruxDev convergence loops to Codex's task execution model

**Key Differences from Claude Code:**
- Codex uses `AGENTS.md` as its bootstrap file (equivalent to `CLAUDE.md`)
- Codex does not have native slash commands; convergence modes are triggered by natural language keywords in the bootstrap
- Subagent dispatch uses Codex's parallel task system rather than a subagent API -- each task runs in an isolated sandbox
- Codex sandboxes have network disabled by default, which affects viability checks that need to verify remote dependencies. The adapter includes a pre-flight step that caches dependency checks before entering the sandbox
- Convergence state JSON files work identically (filesystem is shared within a Codex session)
- Hooks are not natively supported; the adapter embeds session-start logic directly in `AGENTS.md`

**Bootstrap (`adapters/codex/codex-config.md`):**
The Codex bootstrap contains the same core rules as the Claude Code bootstrap but uses Codex-specific terminology:
- "Read file" instead of `Read` tool
- "Run shell command" instead of `Bash` tool
- Convergence keywords: "converge", "plan and build", "audit" trigger the same skill loading as Claude Code slash commands

**Installation:**
```bash
# Codex reads from .codex/ directory
mkdir -p .codex
ln -s ../.cruxdev/CRUXDEV.md .codex/AGENTS.md
cp .cruxdev/adapters/codex/codex-config.md .codex/

# Verify the symlink resolves
ls -la .codex/AGENTS.md
```

### 5.3 OpenCode Adapter

OpenCode uses JSON configuration for agent instructions. The adapter:

1. Generates an `opencode.json` configuration pointing to CruxDev skills
2. Maps skill activation triggers to OpenCode's pattern-matching system
3. Configures agent spawning for subagent delegation

**Key Differences from Claude Code:**
- OpenCode uses a JSON config file (`agents.json`) rather than a markdown bootstrap
- Skill triggers are defined as regex patterns in the JSON config, matched against user input and project state
- OpenCode's agent spawn mechanism is used for subagent delegation; each spawned agent receives a subset of the convergence context
- Git worktree support depends on OpenCode's shell access configuration; the adapter verifies shell access at session start
- The JSON config includes a `skill_paths` array pointing to each CruxDev skill directory, enabling OpenCode's file-read system to locate skills on demand
- Convergence state files work identically (JSON on the filesystem)

**Bootstrap (`adapters/opencode/opencode-config.json`):**
```json
{
  "framework": "cruxdev",
  "bootstrap_rules": [
    "Drive convergence loops autonomously",
    "Two consecutive independent clean passes = convergence",
    "Verify all status claims empirically",
    "Max 5 rounds per convergence loop, then escalate"
  ],
  "skill_paths": [
    ".cruxdev/skills/planning",
    ".cruxdev/skills/tdd",
    ".cruxdev/skills/auditing",
    ".cruxdev/skills/executing",
    ".cruxdev/skills/convergence-driving"
  ],
  "triggers": {
    "plan.*build|I want to build": "planning",
    "execute.*plan|run.*plan": "executing",
    "converge|audit.*fix": "convergence-driving",
    "audit": "auditing",
    "viability|will.*succeed": "viability-assessment"
  }
}
```

**Installation:**
```bash
# OpenCode reads from .opencode/ directory
mkdir -p .opencode
cp .cruxdev/adapters/opencode/opencode-config.json .opencode/agents.json

# Verify JSON is valid
python3 -m json.tool .opencode/agents.json > /dev/null && echo "Valid" || echo "Invalid JSON"
```

### 5.4 Platform Abstraction Layer

Skills reference platform capabilities abstractly. The adapter translates:

| Abstract Capability | Claude Code | Codex | OpenCode |
|---------------------|-------------|-------|----------|
| Create task | `TaskCreate` | TODO item in plan | Task in config |
| Update task | `TaskUpdate` | Edit plan checkbox | Edit task status |
| List tasks | `TaskList` | Read plan file | Read task list |
| Dispatch subagent | Claude subagent API | Codex parallel task | OpenCode agent spawn |
| Read file | `Read` tool | File read | File read |
| Run command | `Bash` tool | Shell execution | Shell execution |
| Git operations | `Bash` with git | Git via shell | Git via shell |

---

## 6. Migrating from an Existing Methodology

If your project already uses an established development patterns methodology (ad hoc prompting documentation, team playbooks, etc.), CruxDev is designed to absorb and formalize those patterns rather than discard them.

### 6.1 How Existing Methodology Maps to CruxDev

Most ad hoc prompting methodologies cover the same territory as CruxDev's skills. The typical mapping:

| Methodology Area | CruxDev Skill | What Changes |
|-----------------|---------------|-------------|
| Planning process & flowcharts | `skills/planning/SKILL.md` + `engine/PLAN_CONVERGENCE.md` | The planning cycle becomes a convergence loop, not a human-driven sequence |
| Viability assessment | `skills/viability-assessment/SKILL.md` | Extracted as a standalone, reusable skill |
| Prompt templates & sequences | `skills/prompt-patterns/SKILL.md` | Prompts become triggerable skill chains |
| Audit methodology & checklists | `skills/auditing/SKILL.md` | Integrated into multi-dimensional audit skill |
| "Do it again" convergence | `skills/convergence-driving/SKILL.md` | The human saying "do it again" becomes the engine doing it automatically |
| TDD/BDD patterns | `skills/tdd/SKILL.md` | Enhanced with coverage-by-coincidence detection |
| Data safety (atomic writes, backups) | `skills/data-safety/SKILL.md` | Extracted as standalone skill |
| Session crash resilience | `skills/executing/SKILL.md` + `session-crash-recovery.md` | Crash resilience is part of execution |
| State machine design | `skills/state-machines/SKILL.md` | Generalized state machine patterns |
| Code audit & convergence | `skills/auditing/SKILL.md` + `engine/CODE_CONVERGENCE.md` + `skills/honest-tracking/SKILL.md` | Split across multiple focused skills |
| Documentation patterns | `engine/DOC_CONVERGENCE.md` + auditing skill | Doc audit becomes a convergence sub-loop |
| Subagent delegation | `skills/subagent-delegation/SKILL.md` | Agent delegation for research and implementation |
| Plan lifecycle | `skills/planning/SKILL.md` (lifecycle section) | Part of planning skill |
| Execution-phase patterns | `engine/CONVERGENCE.md` + `engine/CODE_CONVERGENCE.md` | The "run until done" model IS the convergence engine |
| Communication, architecture, maintenance | Project-specific (not generalized) | Stay in your project's config files, not in CruxDev |

### 6.2 Stack-Specific Patterns

CruxDev does not include stack-specific skills. Instead, projects create their own stack patterns file alongside CruxDev:

```
project/
├── .cruxdev/              # CruxDev framework (methodology)
├── .claude/
│   └── CLAUDE.md          # Project rules + CruxDev bootstrap
├── STACK_PATTERNS.md       # Stack-specific patterns (per project)
└── BUILD_PLAN.md          # Current plan
```

Stack-specific patterns (e.g., framework conventions, component patterns, auth flows, integration details) belong in your project-level patterns file, not in CruxDev skills. CruxDev provides the methodology (how to plan, audit, converge); the stack patterns file provides the domain knowledge (what patterns to use in your particular technology stack).

### 6.3 Practical Migration Steps

For projects already using an established ad hoc prompting methodology:

**Step 1: Install CruxDev Alongside (Not Instead Of)**

```bash
# Don't replace your existing patterns file -- it contains project-specific context
cp -r cruxdev/ .cruxdev/

# Append CruxDev bootstrap to existing CLAUDE.md
echo "" >> .claude/CLAUDE.md
echo "## CruxDev Framework" >> .claude/CLAUDE.md
cat .cruxdev/adapters/claude-code/CLAUDE.md >> .claude/CLAUDE.md
```

**Step 2: Keep Project-Specific Content in Your Patterns File**

Your existing methodology file continues to hold:
- Project-specific architecture decisions
- Project-specific anti-patterns
- Project-specific maintenance patterns
- Communication preferences
- Session management rituals

**Step 3: Let CruxDev Handle Methodology**

CruxDev takes over:
- Planning methodology
- Audit methodology
- TDD enforcement
- Convergence loops
- Prompt patterns

**Step 4: The Handoff Test**

Run a development round using CruxDev. Compare:
- Did the plan converge faster? (Fewer human interventions)
- Did code convergence produce similar issue counts?
- Were there any cases where CruxDev's automation missed something the user would have caught manually?

If CruxDev matches or exceeds the manual methodology, the migration is complete. If not, the gaps become new skills or engine refinements.

---

## 7. Implementation Roadmap

### Phase 1: Core Engine (Week 1)

**Goal:** The convergence engine works end-to-end for a single project.

- [ ] Write `CRUXDEV.md` bootstrap file (<2k tokens)
- [ ] Write `engine/CONVERGENCE.md` (master loop specification)
- [ ] Write `engine/PLAN_CONVERGENCE.md` (plan audit loop)
- [ ] Write `engine/CODE_CONVERGENCE.md` (code audit loop)
- [ ] Write `engine/DOC_CONVERGENCE.md` (doc audit loop)
- [ ] Write `engine/VIABILITY.md` (viability assessment protocol)
- [ ] Write convergence state file template (`templates/CODE_AUDIT_STATE.json`)
- [ ] Write plan template (`templates/BUILD_PLAN_TEMPLATE.md`)
- [ ] Test: run the convergence engine on an existing project and verify it terminates

### Phase 2: Core Skills (Week 2)

**Goal:** All essential skills exist and chain correctly.

- [ ] Write `skills/planning/SKILL.md`
- [ ] Write `skills/tdd/SKILL.md`
- [ ] Write `skills/auditing/SKILL.md`
- [ ] Write `skills/executing/SKILL.md`
- [ ] Write `skills/convergence-driving/SKILL.md` (new -- CruxDev original)
- [ ] Write `skills/viability-assessment/SKILL.md`
- [ ] Write `skills/honest-tracking/SKILL.md`
- [ ] Write `skills/subagent-delegation/SKILL.md`
- [ ] Write `skills/git-worktrees/SKILL.md` (adapted from Superpowers)
- [ ] Write `skills/prompt-patterns/SKILL.md`
- [ ] Write `skills/patterns-capture/SKILL.md` (CruxDev original -- Phase D methodology extraction)
- [ ] Write `skills/systematic-debugging/SKILL.md` (adapted from Superpowers)
- [ ] Write `skills/brainstorming/SKILL.md` (adapted from Superpowers)
- [ ] Write `skills/data-safety/SKILL.md`
- [ ] Write `skills/state-machines/SKILL.md`
- [ ] Write `skills/writing-skills/SKILL.md` (adapted from Superpowers)
- [ ] Test: verify each skill loads correctly and triggers on appropriate context

### Phase 3: Platform Adapters (Week 3)

**Goal:** CruxDev works on Claude Code, Codex, and OpenCode.

- [ ] Write Claude Code adapter (CLAUDE.md bootstrap, slash commands, hooks)
- [ ] Write Codex adapter (symlink configuration)
- [ ] Write OpenCode adapter (JSON configuration)
- [ ] Test: install on each platform and verify skill loading + convergence
- [ ] Write installation documentation for each platform

### Phase 4: Validation (Week 4)

**Goal:** Run CruxDev on a real project and validate convergence.

- [ ] Select a real project with an existing codebase
- [ ] Install CruxDev alongside existing project methodology files
- [ ] Run a full planning + execution + convergence cycle
- [ ] Compare: issue counts, convergence rounds, human interventions required
- [ ] Update skills based on findings
- [ ] Write patterns file template for the patterns capture phase

### Phase 5: Polish and Release (Week 5)

**Goal:** Framework is usable by others.

- [ ] Write README.md with quick-start guide
- [ ] Write LICENSE (MIT)
- [ ] Clean up skill cross-references
- [ ] Verify token budget (<2k bootstrap, <1k per skill)
- [ ] Create GitHub repository
- [ ] Write contributing guide (using `skills/writing-skills/SKILL.md`)

---

## 8. The CruxDev Prompt (What the Human Actually Says)

The entire point of CruxDev is to reduce the human's role to a single prompt. Here are the prompts that drive each mode:

### 8.1 Full Lifecycle (The "One Prompt" Dream)

```
"Plan and build [goal] with [constraints]. Use CruxDev. Converge."
```

This triggers: planning → plan convergence → execution → code+doc convergence → patterns update.

### 8.2 Planning Only

```
"Plan [goal] with [constraints]. Converge the plan."
```

This triggers: planning → focused audits → full-plan audits → viability → converged plan.

### 8.3 Execution Only (Plan Already Exists)

```
"Execute the plan. Converge."
```

This triggers: task conversion → subagent execution → code+doc convergence.

### 8.4 Convergence Only (Code Already Written)

```
"Converge code and docs."
```

This triggers: code audit → doc audit → fix → re-audit → convergence loop.

### 8.5 Single Audit Pass

```
"Audit [scope] for issues."
```

This triggers a single audit pass without the convergence loop. For manual, human-driven auditing.

---

## 9. Open Questions and Future Work

### 9.1 Token Pressure in Long Convergence Loops

A convergence loop that runs 5 rounds with parallel audit agents consumes significant context. Strategies:
- Summarize previous rounds in convergence state file
- Keep only the current round's findings in context
- Use subagent dispatch to isolate each round's context

### 9.2 Multi-Agent Coordination

When three audit agents run in parallel, their findings may conflict (Agent 1 says "rename X", Agent 2 says "keep X").

**Conflict Resolution Protocol:** When parallel agents produce conflicting findings, the main agent applies these tie-breaking rules in priority order:

1. **Safety first:** Prefer the finding that prevents data loss, security vulnerabilities, or corruption. Example: Agent 1 says "inline the helper function for readability," Agent 2 says "keep the helper because it wraps a transaction boundary." Agent 2 wins -- the transaction boundary is a safety concern.

2. **Preserve existing behavior:** Prefer the finding that changes less existing, working code. Example: Agent 1 says "rename `processRecord` to `handleRecord` for consistency," Agent 2 says "keep `processRecord` because 14 call sites reference it." Agent 2 wins -- mass renames are high-risk and low-value.

3. **Tests over no tests:** If both findings change equal amounts of code, prefer the one that adds or preserves test coverage. Example: Agent 1 suggests refactoring a module with no new tests, Agent 2 suggests a smaller change with a test covering the edge case. Agent 2 wins.

4. **Specificity over generality:** Prefer the finding with a more specific, actionable description. Vague findings ("could be improved") lose to precise findings ("function X leaks a database connection when called with null input").

5. **Escalate genuine conflicts:** If both findings are equally valid and mutually exclusive (e.g., "use approach A" vs "use approach B" for a design decision), log both findings, mark the conflict in the convergence state file, and escalate to the human with a one-sentence summary of each position. Do not guess.

When documenting the resolution, the main agent records: which finding was selected, which tie-breaking rule applied, and a one-sentence rationale.

### 9.3 Non-Greenfield Projects

CruxDev is designed for new development rounds, but many projects need convergence on existing messy codebases. Future work: a "retrofit" skill that audits and converges an existing codebase without a prior plan.

### 9.4 Cross-Repository Convergence

Large projects span multiple repositories. The convergence engine currently operates within a single repository. Future work: cross-repo convergence state, distributed audit agents.

### 9.5 Learning from Convergence Data

Every convergence loop produces structured data: issue counts, classes, rounds to convergence, safety valve triggers. This data can inform:
- Which audit dimensions find the most issues (and should be checked first)
- Which issue classes are introduced by fixes (and should be watched for)
- Average rounds to convergence (for estimating time)
- Which skills are most frequently loaded (and should be optimized for token budget)

---

## Appendix A: Empirical Convergence Data

From a real-world pilot project (96 checkboxes, Phases 0-6):

### Plan Convergence

| Audit Type | Passes | Issues Found | Issue Classes |
|------------|--------|-------------|---------------|
| Focused (Phase 6) | 3 | 10 + 8 + 7 = 25 | Structural, logical, consistency |
| Full-plan | 2 | 14 + 8 = 22 | Cross-phase integration, stale references |
| Viability | 2 | 2 + 5 = 7 | Missing deps, wrong tools, broken imports |
| **Pre-execution total** | **7** | **54** | |

### Code + Doc Convergence

| Round | Scope | Issues | Classes |
|-------|-------|--------|---------|
| R1 | Code | 13 | Placeholder code, missing functions, non-atomic writes |
| R2 | Code | 9 | Connection leaks, phantom commands, migration bugs |
| R3 | Docs | 3 | Stale counts, phantom references |
| R4 | Both | 0 | **CONVERGED** |
| **Post-execution total** | | **25** | |

### Grand Total

| Phase | Passes | Issues |
|-------|--------|--------|
| Pre-execution | 7 | 54 |
| Post-execution | 4 | 25 |
| **Grand total** | **11** | **79** |

The convergence curve drops to zero within 2-4 passes per scope level. This is the empirical basis for CruxDev's safety valve settings (max 5 rounds for code, max 3 for docs and plans).

---

## Appendix B: Superpowers Compatibility

CruxDev is designed to coexist with Superpowers, not replace it. For teams already using Superpowers:

| Superpowers Skill | CruxDev Equivalent | Relationship |
|-------------------|-------------------|--------------|
| `test-driven-development` | `skills/tdd/SKILL.md` | CruxDev adds coverage-by-coincidence detection |
| `writing-plans` | `skills/planning/SKILL.md` | CruxDev adds convergence loop around planning |
| `executing-plans` | `skills/executing/SKILL.md` | CruxDev adds task conversion + honest tracking |
| `subagent-driven-development` | `skills/subagent-delegation/SKILL.md` | CruxDev adds parallel audit agents |
| `using-git-worktrees` | `skills/git-worktrees/SKILL.md` | Largely identical |
| `systematic-debugging` | `skills/systematic-debugging/SKILL.md` | CruxDev adapts this with convergence engine integration (debugging feeds into audit-fix loop) |
| `verification-before-completion` | `skills/convergence-driving/SKILL.md` | CruxDev generalizes this into full convergence loops |
| `brainstorming` | `skills/brainstorming/SKILL.md` | CruxDev adapts this with convergence-aware design output (brainstorming hands off to planning skill) |
| `writing-skills` | `skills/writing-skills/SKILL.md` | Similar but with CruxDev-specific template |
| N/A | `skills/auditing/SKILL.md` | **CruxDev only** -- multi-dimensional audit methodology |
| N/A | `skills/viability-assessment/SKILL.md` | **CruxDev only** -- environment verification |
| N/A | `skills/honest-tracking/SKILL.md` | **CruxDev only** -- checkpoint integrity |
| N/A | `skills/convergence-driving/SKILL.md` | **CruxDev only** -- autonomous convergence loops |
| N/A | `skills/prompt-patterns/SKILL.md` | **CruxDev only** -- prompt toolkit as skills |
| N/A | `skills/data-safety/SKILL.md` | **CruxDev only** -- atomic writes, backups |
| N/A | `skills/state-machines/SKILL.md` | **CruxDev only** -- state machine design |
| N/A | `engine/*` | **CruxDev only** -- the convergence engine |

Teams can use both frameworks together. CruxDev now includes adapted versions of Superpowers' brainstorming and systematic-debugging skills with convergence integration. For code review skills and other Superpowers-only capabilities, both frameworks coexist without conflict.

---

## Appendix C: The Bootstrap File (CRUXDEV.md Draft)

```markdown
# CruxDev: Autonomous Convergence Framework

You are operating under CruxDev. Your job is to drive development to convergence
without human intervention between "start" and "done."

## Core Rules
1. Drive convergence loops autonomously (never wait for "do it again")
2. Two consecutive independent clean passes = convergence
3. Verify all status claims empirically (coverage, test counts, file existence)
4. Log convergence state to JSON files (atomic writes)
5. Max 5 rounds per loop, then escalate to human

## Phase Detection
Check project state to determine current phase:
- No plan file → load skills/planning/SKILL.md
- Plan exists, unchecked boxes → load skills/executing/SKILL.md
- convergence_state.json exists → resume from state file
- User says "converge" → load engine/CONVERGENCE.md

## Skill Index (load on demand, not at start)
- planning: "Use when creating or auditing a development plan"
- tdd: "Use when writing any production code"
- auditing: "Use when checking quality of plans, code, or docs"
- executing: "Use when running a plan's tasks"
- convergence-driving: "Use when running any audit-fix-re-audit loop"
- viability-assessment: "Use before executing any plan"
- honest-tracking: "Use when marking any progress checkpoint"
- subagent-delegation: "Use when dispatching work to fresh agents"
- git-worktrees: "Use before starting any implementation work"
- prompt-patterns: "Use when recognizing standard prompt triggers"
- patterns-capture: "Use after convergence to extract methodology learnings"
- systematic-debugging: "Use when investigating bugs or unexpected behavior"
- brainstorming: "Use before any creative work to explore design before implementation"
- data-safety: "Use when writing to files or databases"
- state-machines: "Use when designing systems with explicit states"

## Convergence Engine
Read engine/CONVERGENCE.md for the master loop specification.
The engine chains skills into autonomous loops:
  plan → audit plan → fix → re-audit → converge plan
  execute → audit code → fix → re-audit → converge code
  audit docs → fix → re-audit → converge docs
All without human saying "do it again."

## Emergency Fallback
If skill files cannot be loaded, follow these minimum rules:
1. Write tests before code
2. After any work, audit it
3. After fixing audit findings, audit again
4. Repeat until clean twice in a row
5. Verify claims with tool output
```

This bootstrap is approximately 1,800 tokens -- within the 2,000 token budget.
