# Development Patterns — Crux Ecosystem

Methodology for developing within the Crux ecosystem (Crux, CruxCLI, CruxDev). This document governs **how** we plan, build, audit, and converge code to completion.

This is the successor to DEVELOPMENT_PATTERNS.md. The core methodology is identical — multi-pass auditing, viability assessment, convergence to zero issues — but the execution model changes from **human-driven** ("do it again") to **agent-driven** (the process runs to completion autonomously).

**Relationship to other files:**
- **CLAUDE.md** — the authority. Hard rules override everything in this file.
- **CRUX_ECOSYSTEM_PLAN.md** — the current master plan (what to build, in what order).
- **CruxDev.md** — the CruxDev design document (architecture, skills, engine specs).
- **DEVELOPMENT_PATTERNS.md** — the original methodology. This file extends it for the Crux ecosystem. Both remain valid. Use the original for projects that don't need the Crux ecosystem context.
- **This file is living.** Updated at the end of each planning session.

**When to read this file:**
- At the start of any planning session in the Crux ecosystem
- Before writing any new plan document for Crux, CruxCLI, or CruxDev
- When User says "follow the patterns" or references methodology
- NOT needed for routine work covered by CLAUDE.md

---

## 0. The Lights-Out Execution Model

### 0A. The Core Difference from DEVELOPMENT_PATTERNS.md

DEVELOPMENT_PATTERNS.md documents a process where the user drives convergence through repeated prompts:

```
User: "audit it"
Agent: [audits, finds 14 issues, fixes]
User: "do it again"
Agent: [audits, finds 8 issues, fixes]
User: "do it again"
Agent: [audits, finds 0 issues]
User: "now audit the entire plan"
...
```

This works. It produces excellent results (79 issues caught across 11 passes). But it requires the user at every loop iteration.

**DEVELOPMENT_PATTERNS_CRUXDEV.md replaces the human loop with autonomous convergence:**

```
User: "Plan and build [goal] with [constraints]. Converge."
Agent: [drives the ENTIRE cycle to completion autonomously]
  → writes plan
  → focused audit loop (each phase, until zero issues)
  → full-plan audit loop (until zero issues)
  → viability assessment loop (until zero caveats)
  → task conversion
  → execution (TDD, per-task)
  → code audit convergence (until two consecutive clean passes)
  → doc audit convergence (until two consecutive clean passes)
  → patterns update
  → documentation convergence (audit all docs against new code, two clean passes)
  → website convergence (audit project website against WEBSITE_PLANNING.md standards)
  → deployment convergence (deploy using docs/DEPLOYMENT.md, create if missing)
  → patterns update
  → DONE: report convergence status
```

**Same quality. Zero intermediate prompting.** The agent knows the termination conditions, safety valves, and escalation rules. The user provides the goal and constraints. The agent runs to completion.

### 0B. When to Escalate to the User

The agent runs autonomously EXCEPT when:

| Condition | Action |
|-----------|--------|
| Safety valve hit (max rounds reached) | Stop, present triage report, ask for direction |
| Protected file modification needed | Stop, present the change, ask for approval |
| Ambiguous requirement discovered | Stop, ask for clarification |
| Net-negative fix round (issues increasing for 2 consecutive rounds) | Stop, present analysis, ask for direction |
| Viability blocker that can't be auto-fixed (e.g., missing paid service) | Stop, present the blocker |
| Architecture decision with no clear winner | Stop, present 2-3 options with tradeoffs |

Everything else — audit, fix, re-audit, test, coverage check, doc update — happens without interruption.

### 0C. The One Prompt

The ideal prompt to start a full development round:

```
"Plan and build [goal] with [constraints]. Converge."
```

Or, for an existing plan:

```
"Execute the plan. Converge."
```

Or, for code that's already written:

```
"Converge code and docs."
```

The agent reads DEVELOPMENT_PATTERNS_CRUXDEV.md and knows the full lifecycle. No further instructions needed.

---

## 1. The Autonomous Planning Cycle

### 1A. From Problem Statement to Converged Plan (No Human Loop)

When the agent receives a goal, it runs the full planning cycle autonomously:

```
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: PLAN WRITING                                         │
│                                                              │
│ Read project state, understand the goal.                     │
│ Write the comprehensive plan document:                       │
│   - Header (status, goal, constraints, rules)                │
│   - Architecture overview (before/after ASCII)               │
│   - Phases with data structures, functions, tests            │
│   - Progress tracker with checkboxes                         │
│   - File inventory, risks, dependencies                      │
│   - Definition of Done                                       │
│                                                              │
│ Output: BUILD_PLAN.md (first draft)                          │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: FOCUSED AUDIT CONVERGENCE (per phase)                │
│                                                              │
│ For each new or modified phase:                              │
│   Round 1: audit → find issues → fix                         │
│   Round 2: re-audit → find issues introduced by fixes → fix  │
│   Round N: re-audit → zero issues → MOVE ON                  │
│                                                              │
│ Safety valve: max 5 rounds per phase.                        │
│                                                              │
│ Empirical: 10 → 8 → 7 → 0 (3-4 rounds typical)             │
│ Round 1 catches structural issues.                           │
│ Round 2 catches logical gaps exposed by fixes.               │
│ Round 3 catches consistency/redundancy from prior rounds.    │
│                                                              │
│ Output: Each phase internally consistent                     │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: FULL-PLAN AUDIT CONVERGENCE                          │
│                                                              │
│ Audit ALL phases together for cross-phase issues:            │
│   Round 1: find stale references, state machine divergence,  │
│     dependency graph errors → fix                            │
│   Round N: re-audit → zero cross-phase issues → MOVE ON     │
│                                                              │
│ Safety valve: max 3 rounds.                                  │
│                                                              │
│ Empirical: 14 → 8 → 0 (2-3 rounds typical)                  │
│ Dominant bug class: stale cross-phase references.            │
│ One rename = N fixes across N phases.                        │
│                                                              │
│ Output: Plan consistent across all phases                    │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ STEP 4: VIABILITY ASSESSMENT CONVERGENCE                     │
│                                                              │
│ Verify plan is executable against the ACTUAL environment:    │
│   1. Re-read the entire plan with fresh eyes                 │
│   2. Simulate execution mentally, step by step               │
│   3. Verify the environment:                                 │
│      - Are referenced tools/deps installed?                  │
│      - Do import paths work?                                 │
│      - Does project config match assumptions?                │
│      - Do referenced files exist?                            │
│      - Does code already exist for files plan says to CREATE?│
│   4. Fix all caveats in the plan                             │
│   5. Re-verify → zero caveats → MOVE ON                     │
│                                                              │
│ Safety valve: max 3 rounds.                                  │
│                                                              │
│ Output: Plan verified executable against actual machine      │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
           PLAN IS CONVERGED — proceed to execution
```

**Key point:** The agent does NOT wait for the user to say "audit it" or "do it again" at any step. It drives each loop to its termination condition autonomously.

### 1B. Plan Document Format

Every plan follows the format established in DEVELOPMENT_PATTERNS.md Section 1A:

| Section | Purpose |
|---------|---------|
| Header | Status, goal, constraints, rules |
| Architecture Overview | Before/after ASCII diagrams |
| Phases | Numbered, ordered, with data structures, functions, tests |
| Phase Ordering & Dependencies | Explicit dependency graph |
| Progress Tracker | Checkbox list, one per atomic step |
| File Inventory | Every new file with phase and purpose |
| E2E Test Plan | User path inventory, journey matrix, test specs (per E2E_TEST_PATTERNS.md) — include unless no user-facing surface |
| Risks & Mitigations | What could go wrong |
| Definition of Done | Numbered acceptance criteria |

### 1C. Plan File Naming

Plan files are numbered sequentially with a descriptor. Never use a generic name like `BUILD_PLAN.md`.

```
BUILD_PLAN_001_COVERAGE_GAPS.md
BUILD_PLAN_002_GITHUB_ACTIONS.md
BUILD_PLAN_003_CRUXCLI_FORK.md
```

The number provides chronological ordering. The descriptor says what the plan is for. This prevents accumulation of ambiguously-named plan files and makes git history clear.

### 1D. Plan Status

| Status | Meaning |
|--------|---------|
| `NOT STARTED` | No code exists |
| `PARTIALLY BUILT (annotation)` | Some phases have existing code |
| `IN PROGRESS` | Active execution |
| `COMPLETE` | All phases done, all tests passing, coverage verified |

**Rule:** After any session that builds code, update the status before ending.

### 1D. Phase Design

Each phase must be:
- **Independent enough to resume** — if context crashes, next session picks up from last checked checkbox
- **Ordered by dependency** — prerequisites explicit
- **Testable in isolation** — own test suite that passes independently
- **Purposeful** — one-sentence purpose statement explaining WHY

### 1E. Sub-Session Breakdown

Break phases into sub-sessions when they touch 4+ modules or have 10+ build steps:

```
Sub-session 4A: steps 1-4 (needs: module_a, module_b)
Sub-session 4B: steps 5-8 (needs: module_b, module_c)
```

Each sub-session lists exactly which modules it needs and has a checkpoint instruction.

### 1F. Brainstorming Gate (from Superpowers)

Before any creative work — new features, component design, architecture changes — the agent must brainstorm before implementing:

1. **Explore context** — read project files, docs, recent commits. Don't ask the user questions you can answer from the codebase.
2. **Ask clarifying questions** — one at a time, prefer multiple-choice over open-ended
3. **Propose 2-3 approaches** — with tradeoffs (complexity, performance, maintainability). Apply YAGNI ruthlessly.
4. **Present design** — validate incrementally. Remain flexible.
5. **Get approval** — do NOT implement without design approval

**Critical gate:** No implementation skill, no code, no scaffolding until a design is approved. This prevents wasted effort on the wrong approach.

### 1G. Gap Analysis as North Star (from yoyo-evolve)

When planning work, start with a gap analysis: compare current state against target state. Work on the biggest gaps first. This prevents drift into low-value tasks.

```
For each planning session:
  1. What does the target state look like? (Definition of Done)
  2. What does the current state look like? (Read the code, run tests)
  3. What are the gaps? (Ordered by size/impact)
  4. Plan to close gaps in impact order
```

This replaces ad-hoc "what should I work on next?" with structured prioritization.

---

## 2. Autonomous Execution

### 2A. Checkbox → Task Conversion

Before execution, convert plan checkboxes to trackable tasks:

1. Audit checkboxes against plan for accuracy (iterate until clean)
2. Convert each checkbox to a task with context and dependencies
3. Execute from task list, not from scrolling the plan file

### 2B. Execution Loop with Safety Gates (from yoyo-evolve)

```
For each task (in dependency order):
  1. Mark task in_progress
  2. Create git worktree for isolation (see 2D)
  3. Do the work (TDD: tests first, then code)
  4. BUILD/TEST GATE: full test suite must pass
     - If tests fail: fix and retry (max 3 attempts)
     - If 3 attempts fail: AUTO-ROLLBACK to pre-task state
       and move to next task. Log the failure.
  5. TIMEOUT GATE: 15 minutes max per task
     - If exceeded: checkpoint state, move to next task
     - Prevents infinite loops on stuck tasks
  6. Merge worktree back to main branch
  7. Verify: tests still pass on main (catch merge conflicts)
  8. Mark task completed
  9. Mark corresponding plan checkbox [x]
  10. Log progress

Mid-execution checkpoint (at ~50% completion):
  Audit progress tracker honesty:
    - Are checked boxes truly complete?
    - Are deferred items tracked?
    - Are coverage claims verified?
```

**Auto-rollback rule (from yoyo-evolve):** Don't brute-force a stuck task. After 3 failed attempts, the code goes back to the last known good state and the task is logged as blocked. This prevents fix cascades where each attempt introduces new problems.

### 2C. TDD Build Order (Always)

```
1. Plan the full test suite FIRST
2. Write ALL unit tests (RED — they must fail)
3. Implement minimum code to pass tests (GREEN)
4. Refactor
5. Coverage check ≥ 100%
```

Never write production code before its tests exist. If context/time forces a tradeoff, escalate to the user.

### 2D. Git Worktree Isolation (from Superpowers)

Every implementation task runs in an isolated git worktree:

```
1. Create worktree: git worktree add .worktrees/<task-id> -b task/<task-id>
2. Run baseline tests in worktree (verify clean starting point)
3. Do all implementation work in the worktree
4. Run full test suite in worktree
5. If all pass: merge back to main branch
6. If tests fail after 3 attempts: delete worktree (auto-rollback)
7. Clean up: git worktree remove .worktrees/<task-id>
```

**Why:** Isolation prevents a broken task from contaminating the main branch. If anything goes wrong, the worktree is disposable. The main branch always has passing tests.

### 2E. Subagent-Driven Development (from Superpowers)

For implementation tasks, dispatch fresh subagents:

```
For each task:
  1. Dispatch IMPLEMENTER subagent with:
     - Task description from plan
     - Relevant file paths
     - Test requirements
     - TDD constraint
  2. Implementer produces code + tests in worktree
  3. Dispatch SPEC REVIEWER subagent with:
     - Plan specification for this task
     - Implemented code
     - Question: "Does this match the spec?"
  4. Dispatch CODE REVIEWER subagent with:
     - Implemented code
     - Question: "Is this correct, safe, and clean?"
  5. If either reviewer finds issues: fix and re-review
  6. If both reviewers pass: merge worktree
```

**Two-stage review:** Spec compliance (does it match the plan?) and code quality (is it well-written?) are separate concerns checked by separate agents. This catches both "wrong thing built right" and "right thing built wrong."

**When to use subagents:** Tasks that are self-contained enough to describe in a prompt. When NOT to use: tasks that require deep cross-module understanding built up over the session.

### 2F. Learnings Admission Gate (from yoyo-evolve)

When the patterns update phase (Section 4) captures new patterns, apply a quality gate:

**A learning is only admitted if it is:**
1. **Genuinely novel** — not already captured in this file or CLAUDE.md
2. **Would change future behavior** — if knowing this wouldn't change what you do next time, it's not a learning

This prevents patterns files from bloating with obvious or non-actionable observations. Every pattern in this file should pass this gate.

---

## 3. Post-Execution Convergence (Mandatory)

Every plan execution MUST complete ALL of these convergence steps before reporting done. Skipping any step is a violation.

### 3.0 Post-Execution Convergence Requirements

After code convergence, four additional convergence loops are mandatory:

**A. Documentation Convergence:**
- Audit ALL documentation files against the current codebase
- Check: accuracy (does the doc match the code?), completeness (are new features documented?), staleness (do docs reference removed code?), phantoms (do docs describe things that don't exist?)
- Two consecutive clean passes required
- Files to audit: CLAUDE.md, all docs in `docs/`, all build plans in `build_plans/`, README if present

**B. Website Convergence (if project has a website or web app):**
- Audit the project website against `docs/WEBSITE_PLANNING.md` standards
- Check: SEO (meta tags, structured data, llms.txt), performance (Core Web Vitals), accessibility (WCAG 2.1 AA), content accuracy (does the site match what the code actually does?)
- Update any metrics on the site (test counts, feature counts, etc.)
- Two consecutive clean passes required
- Reference: `docs/WEBSITE_UPGRADE_DIRECTIVE.md` for the full checklist

**C. Deployment Convergence (if project has a website or web app):**
- After website/webapp convergence, deploy using `docs/DEPLOYMENT.md`
- If `docs/DEPLOYMENT.md` does not exist, **prompt the user** to create one by asking:
  1. Where is this deployed? (Vercel, Cloudflare Pages, Netlify, AWS, self-hosted, etc.)
  2. What's the deploy command or CI/CD pipeline? (git push, GitHub Actions, manual, etc.)
  3. What environment variables or secrets are needed?
  4. Is there a staging environment?
  5. What's the domain and DNS provider?
- Write `docs/DEPLOYMENT.md` from the answers, then deploy
- If `docs/DEPLOYMENT.md` exists, follow it exactly — run the deploy commands, verify the deployment is live and correct
- After deployment, verify: site is accessible, SSL works, no broken links, content matches the converged version

**D. Patterns Update:**
- Capture any new learnings from this plan execution
- Apply the Learnings Admission Gate (Section 2F)
- Update this file (DEVELOPMENT_PATTERNS_CRUXDEV.md) if novel patterns were discovered

**The lifecycle is not complete until all four have converged.** Code convergence alone is insufficient.

### 3A. The Convergence Loop (Post-Execution)

After all tasks are complete, the agent drives code and doc auditing to convergence autonomously:

```
┌─────────────────────────────────────────────────────────────┐
│ CODE + DOC CONVERGENCE LOOP                                  │
│                                                              │
│ Round N:                                                     │
│   1. Audit code (8 dimensions — see 3B)                      │
│   2. Audit docs (5 dimensions — see 3C)                      │
│   3. Run full test suite + coverage                          │
│   4. Log findings with IDs, dimensions, severity             │
│   5. Fix all findings                                        │
│   6. Re-run test suite + coverage                            │
│   7. If fixes introduced new issues → log as fix-regressions │
│   8. Increment round counter                                 │
│                                                              │
│ Termination: TWO consecutive clean passes                    │
│   - The second pass MUST come from independent context       │
│   - Fresh agent/subagent with no memory of previous fixes    │
│   - Same audit dimensions, same rigor                        │
│   - One clean pass after fixing is NOT convergence            │
│     (anchoring bias: ~30% false-negative rate)               │
│                                                              │
│ Safety valve: max 5 rounds → escalate to the user               │
│                                                              │
│ Diminishing returns: if after 3 rounds only LOW-severity     │
│ issues remain, present to the user: "Accept with known gaps     │
│ or continue?" Don't spend 2 more rounds on cosmetics.        │
│                                                              │
│ Net-negative: if issue count increases for 2 consecutive     │
│ rounds, STOP immediately and escalate.                       │
│                                                              │
│ Empirical convergence curve:                                 │
│   R1: 13 issues (placeholder code, missing functions)        │
│   R2: 9 issues (connection leaks, phantom commands)          │
│   R3: 3 issues (stale doc counts)                            │
│   R4: 0 issues (CONVERGED)                                   │
└─────────────────────────────────────────────────────────────┘
```

### 3B. Eight Code Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | Plan alignment | Does the code match the plan's specifications? |
| 2 | Correctness | Are there logic errors, race conditions, edge cases? |
| 3 | Test coverage | Every public function tested? Coverage ≥ 100%? |
| 4 | BDD alignment | Do feature files match implemented behavior? |
| 5 | Cross-module integration | Do modules call each other correctly? |
| 6 | Data safety | All critical writes atomic? Backups before imports? |
| 7 | Documentation compliance | Do docs accurately describe the code? |
| 8 | Regression | Do ALL existing tests still pass? |

### 3C. Five Doc Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | Accuracy | Do function signatures match code? |
| 2 | Completeness | Are all new modules/functions documented? |
| 3 | Staleness | Are there hardcoded counts that should be live queries? |
| 4 | Phantom references | Do documented things actually exist? |
| 5 | Architecture alignment | Does the doc describe the current state? |

### 3D. Issue Tracking Format

```json
{
    "id": "R1-001",
    "file": "path/to/file",
    "dimension": "plan_alignment",
    "severity": "high",
    "description": "specific, actionable description",
    "fix_applied": false
}
```

### 3E. Honest Checkpoint Tracking

**Rule:** Never mark a checkbox complete without verifying the claim empirically.

When marking a checkbox, annotate caveats:
```markdown
- [x] 6.5 Coverage check ≥ 100% — module_a 100%, module_b 98% (2 edge cases deferred)
```

Maintain a **Known Gaps** section for deferred items:
```markdown
## Known Gaps (deferred from Phase 6 audit)
1. **R1-005 (medium):** CLI commands deferred — core present but wiring incomplete
```

### 3F. Coverage by Coincidence Detection

A test can pass, look correct, and appear to cover a code path while actually exercising a different branch. The ONLY source of truth is `--cov-report=term-missing` output.

After writing coverage-targeting tests:
1. Run coverage with line-missing report
2. Check: are the specific target line numbers gone from "Missing"?
3. If still missing: your test hit a different branch — trace the actual execution path

### 3G. Agent-Parallel Auditing

For audits across 5+ files, use parallel agents:

```
Agent 1: Audit production code against plan specs
Agent 2: Audit documentation against code reality
Agent 3: Run full test suite + coverage report
```

Results synthesized in main conversation. Prevents context blowup.

### 3H. Deferred Item Triage

| Severity | Action |
|----------|--------|
| HIGH | Fix now, no deferral |
| MEDIUM | Fix if time, otherwise defer with tracking |
| LOW | Defer with tracking |

Every deferred item gets: ID, severity, description, entry in Known Gaps section.

---

## 3I. E2E Test Convergence

Every build plan should include E2E tests unless there is a clear reason they don't apply (e.g., pure library with no user-facing surface). If the product has users interacting through a UI, API, or CLI, E2E tests are expected. The agent drives all four E2E convergence loops autonomously following E2E_TEST_PATTERNS.md.

### The Four E2E Convergence Loops

```
Loop 1: PLAN AUDIT (10 dimensions)
  Completeness, criticality coverage, cross-system coverage,
  step specificity, assertion completeness, precondition clarity,
  negative paths, independence, redundancy, feasibility
  → Two consecutive clean passes → plan converged

Loop 2: TEST-PLAN ALIGNMENT
  Every plan test has implementation, steps match, assertions match,
  no extra/missing tests, preconditions match
  → Two consecutive clean passes → alignment converged

Loop 3: SUITE EXECUTION
  Run full suite → categorize failures (app bug / test bug /
  environment / flaky) → fix → re-run
  → Two consecutive green runs → suite green-converged

Loop 4: DOCUMENTATION
  Update docs → audit against tests → fix
  → Two consecutive clean passes → docs converged
```

### E2E-Specific Rules

- **Implementation order:** CRITICAL first, then HIGH, then MEDIUM/LOW
- **One test at a time:** Write one, run it, make it pass, commit, then next
- **Flakiness zero tolerance:** Fix root cause or delete the test. Never add retries.
- **App bugs found during E2E → fix AND add lower-level regression test**
- **Backflow:** Later phases can trigger return to earlier phases (fix affected section only, don't restart everything)
- **Per-test data isolation:** Each test owns its own data. No ordering dependencies.
- **No sleep:** Wait for specific conditions, not fixed time delays.
- **Testing pyramid constraint:** If it can be tested at a lower level, test it there. E2E is for what nothing else can catch.

### CruxDev Engine Integration

E2E convergence runs as a sub-invocation within the master convergence loop:

| E2E Phase | CruxDev Phase | Engine |
|-----------|---------------|--------|
| Inventory + Plan + Plan Audit | Phase A (Planning) | PLAN_CONVERGENCE |
| Implementation | Phase B (Execution) | Subagent dispatch per test |
| Alignment + Execution | Phase C (Convergence) | CODE_CONVERGENCE |
| Documentation | Phase D (Patterns) | DOC_CONVERGENCE |

The outer build plan has a checkbox for "E2E suite converged." The engine runs the inner loops autonomously.

**Full reference:** E2E_TEST_PATTERNS.md contains the complete methodology — user path inventory, test plan format, precondition strategy, viewport strategy, test data lifecycle, flakiness prevention, metrics, and anti-patterns.

---

## 4. Patterns Update Phase

After convergence, the agent reviews all work done and updates this file:

1. Read convergence state files for completed loops
2. Read plan document and audit findings
3. Identify recurring themes (what kept breaking? what patterns emerged?)
4. Update this file with new patterns, anti-patterns, conventions
5. Audit the patterns file for completeness
6. Fix gaps, re-audit until clean

This ensures methodology is captured while it's fresh.

---

## 5. Three-Product Development Strategy

### 5A. The Three Products

| Product | Repo | Competes With | Arena |
|---------|------|---------------|-------|
| **Crux** | `/Users/user/personal/crux` | No direct competitor | Intelligence / safety platform |
| **CruxCLI** | `/Users/user/personal/cruxcli` | OpenCode (124k stars) | AI coding terminal agent |
| **CruxDev** | `/Users/user/personal/cruxdev` | Superpowers (93.9k stars) | Agentic methodology framework |

### 5B. Integration Model

```
CruxCLI (the runtime)
    │
    ├── Crux (modes, MCP, safety, knowledge) — the intelligence layer
    │
    └── CruxDev (skills, convergence engine) — the methodology layer
```

- Crux provides the intelligence layer (modes, knowledge, safety, sessions)
- CruxCLI provides the terminal UI and agent interaction
- CruxDev provides the convergence methodology that makes the agent self-correcting
- Each is independently installable and valuable
- Crux is the platform that makes both CruxDev and CruxCLI better than their competitors

### 5C. CruxDev Skills vs Crux Modes

They're complementary layers:

| Concern | Crux Modes | CruxDev Skills |
|---------|-----------|---------------|
| What they control | Agent personality + tool access | Development methodology + process |
| Example | `build` mode: "write production code" | `tdd` skill: "tests before code, coverage ≥ 100%" |
| Activation | Session state, user commands | Project phase, convergence engine |
| Scope | Per-session behavior | Per-development-round process |

A CruxCLI session can be in Crux's `build` mode while executing CruxDev's `tdd` skill. The mode governs how the agent behaves; the skill governs what process it follows.

CruxDev works standalone without Crux (like Superpowers works without any framework). When Crux is present, modes provide safety gates and MCP tools provide infrastructure.

### 5D. Upstream Strategy: Digest, Don't Track

Do NOT track upstream OpenCode via rebasing/merging. OpenCode releases every 1-3 days with 823+ contributors. Merge hell is not worth it.

Instead: the inspiration registry (see CRUX_ECOSYSTEM_PLAN.md Phase 4) periodically reads the git changes of monitored repos, analyzes for concepts and approaches, and selectively integrates on merit.

This is intelligence gathering, not fork maintenance. Decision rationale accumulates as knowledge.

### 5E. Cross-Repo Development

When a change spans multiple repos (e.g., new Crux MCP tool + CruxCLI integration + CruxDev skill):

1. Plan the change across all affected repos in one plan document
2. Build bottom-up: Crux (platform) → CruxCLI (runtime) → CruxDev (methodology)
3. Test integration at each layer before moving up
4. Each repo's changes must pass independently (no cross-repo test dependencies)

---

## 6. Data Safety Patterns

### 6A. Atomic Writes

Every write to a critical file must use write-then-rename:

```python
def atomic_write(path, data):
    tmp = path.with_suffix('.tmp')
    tmp.write_text(json.dumps(data, indent=2))
    tmp.rename(path)  # atomic on POSIX
```

### 6B. Rotating Backups

Before every batch import or destructive operation: backup with max 5 retained, prune oldest.

### 6C. Dual-Write for Indexed Data

JSON is authoritative. SQLite is read-optimized index. On rebuild, load from JSON.

### 6D. Crash-Resilient State Files

Any long-running process writes state to disk after every significant action. On resume, read the state file and pick up where it left off. The state file IS the resume protocol.

---

## 7. Crash Resilience & Idempotency

### 7A. Assume the Session Will Crash

- Write state to disk BEFORE doing the next thing
- Log after every substantive action
- Checkpoint work in progress to durable storage
- Never hold important results only in conversation context

### 7B. Idempotent Operations

Every operation must be safe to run twice. Creating a duplicate → error. Importing duplicates → skip. Checking a checked checkbox → no-op.

### 7C. Session Crash Recovery

On resume after crash/compaction:

1. Read convergence state files — they tell you exactly where you are
2. Read plan file — progress tracker checkboxes show what's done
3. Read task list — shows in-progress and pending work
4. Pick up from last checkpoint, don't restart

---

## 8. Communication Patterns

### 8A. Max Chatty Mode During Development

Narrate what you're doing and why. Think out loud. Flag concerns proactively. Don't silently make decisions.

### 8B. Convergence Status Reports

At the end of each convergence round, report:
- Round number
- Issues found (count + classes)
- Issues fixed
- Consecutive clean passes (0, 1, or 2 = CONVERGED)
- Coverage numbers

### 8C. Escalation Format

When escalating to the user, present:
1. What happened (one sentence)
2. Why the agent can't proceed (specific blocker)
3. Options (numbered, 2-3 max, with tradeoffs)
4. Recommended option

---

## 9. Anti-Patterns (Crux Ecosystem Specific)

| Anti-Pattern | What Happens | Rule |
|-------------|-------------|------|
| Absorbing CruxDev into Crux | Loses competitive positioning as separate product | Three products, three repos, three star counts. Always. |
| Tracking upstream OpenCode | Merge hell, 823+ contributors, releases every 1-3 days | Digest, don't track. Inspiration registry. |
| Single clean pass = convergence | ~30% false-negative rate from anchoring bias | Two consecutive independent clean passes required |
| Human-driven audit loops | Requires the user at every iteration | Agent drives to completion autonomously |
| Premature convergence declaration | Undiscovered issues ship | Two clean passes, second from fresh agent |
| Mixing skills and modes | Confusion about what controls what | Skills = process. Modes = behavior. Complementary layers. |
| Cross-repo test dependencies | One repo's tests break when another changes | Each repo's tests pass independently |
| All skills loaded at session start | Token budget blown on irrelevant skills | Load skills on demand based on current phase |
| Non-atomic writes | File corruption on crash | Write-then-rename for all critical files |
| Coverage checkbox dishonesty | False confidence in test coverage | Verify with tool output, annotate caveats |
| Skipping viability assessment | Plan fails on first step due to missing deps | Always check actual environment before execution |
| Modifying protected files during evolution | Safety constraints removed or weakened | Protected file gate, escalate to human |
| Net-negative fix rounds | Fixes making things worse | Stop after 2 consecutive increasing rounds, escalate |
| Brute-forcing stuck tasks | Fix cascades, each attempt introduces new problems | 3 failed attempts → auto-rollback, log as blocked (yoyo) |
| Implementing without design approval | Wasted effort on wrong approach | Brainstorming gate: design first, then implement (Superpowers) |
| Working on main branch during implementation | Broken task contaminates all other work | Git worktree isolation per task (Superpowers) |
| No timeout on stuck tasks | Infinite loop burns entire session context | 15-min timeout per task, checkpoint and move on (yoyo) |
| Bloating patterns file with non-actionable observations | Patterns file becomes noise | Learnings admission gate: genuinely novel AND changes behavior (yoyo) |
| Ad-hoc task ordering | Low-value work done before high-impact work | Gap analysis as north star: work on biggest gaps first (yoyo) |
| Skipping code review after implementation | "It compiles" ≠ "it's correct" | Two-stage review: spec compliance + code quality (Superpowers) |
| Structural-only website audit | Checks page existence/layout but not content accuracy. Stale tech refs, wrong dimension names, old prerequisites survive. BP027 failure: "Python 3.12+" on a Rust project, wrong code dimension names on 12 pages. | Website convergence = doc convergence on every page. Read every page, verify every claim against source code. |
| Metric-only content update | Grep for numbers to update without reading surrounding prose. Misses wrong names, stale technology refs, inconsistent terminology. | Read full content. Numbers AND prose must match codebase. |
| Aspirational documentation | Writing claims about features that don't exist as if they're implemented. BP030 failure: OpenClaw page claimed Jest/Vitest integration, ESLint support, TypeScript strict mode, Docker testing — none implemented. 8 of 18 claims FALSE. | Ship the code first, then document it. Every claim must reference the implementing code. "Roadmap" section for planned features. |
| Personal names in product docs | Developer/owner names in documentation, website, or code comments. Product is not tied to a person. | Names never appear in product docs. Exception: CONTRIBUTORS.md, git author, competitor analysis citing public statements. Convergence check: `grep -ri "[name]" docs/ src/` = 0. |
| Ecosystem lock-in language | Presenting one MCP client as the primary/default. Users on OpenClaw+GPT, Cursor+local models, etc. must see themselves as first-class. | Always list multiple clients. Per-client config in labeled sections. No "Install [specific client]" as prerequisite. Integration specifics managed per-integration, not in primary docs. |

---

## 10. User Collaboration Patterns

### 10A. Concise, Imperative Prompts

The user gives direction, not detail:

| User says | What it means |
|-----------|---------------|
| "converge" | Run the full autonomous cycle. Don't stop until done or escalation needed. |
| "fix these" | Apply fixes directly. Don't ask for permission, don't present options. |
| "will this succeed?" | Viability assessment. Check environment. YES/NO with specific numbered caveats. |
| "plan and build X" | Full lifecycle: plan → converge plan → execute → converge code+docs. |
| "update patterns with everything you've seen" | Review all sessions, find gaps, update. Don't ask what to add. |
| "1. yes 2. skip it." | Numbered responses to numbered decisions. Efficient, unambiguous. |
| "not just [thing]. [broader scope]." | Scope correction. The agent went too narrow. Expand. |

### 10B. User Corrects Scope, Not Detail

The user specifies what's missing, what's wrong, and what to do next. The agent fills in the detail autonomously. When the user corrects scope, that correction becomes a pattern.

### 10C. Quality Standard

- 100% coverage, no "90%+ floor"
- Two consecutive clean passes, not one
- Viability assessment before execution, always
- Honest checkpoints, annotated caveats
- Plans converge to zero known issues before building starts

---

## 11. The Complete Autonomous Lifecycle (Reference)

This is the full sequence the agent follows when the user says "Plan and build X. Converge."

```
1. BRAINSTORM (if new feature / creative work)
   ├── Explore context from codebase
   ├── Ask clarifying questions (multiple-choice preferred)
   ├── Propose 2-3 approaches with tradeoffs
   ├── GATE: design must be approved before proceeding
   └── Gap analysis: order work by biggest gaps first

2. PLAN
   └── Write comprehensive plan document

3. CONVERGE THE PLAN (autonomous, no human prompting)
   ├── Focused audit each phase → loop until 0 issues (max 5)
   ├── Full-plan audit → loop until 0 cross-phase issues (max 3)
   └── Viability assessment → loop until 0 caveats (max 3)

4. EXECUTE (autonomous, with safety gates)
   ├── Audit checkboxes for accuracy
   ├── Convert checkboxes to tasks
   └── For each task:
       ├── Create git worktree for isolation
       ├── Dispatch implementer subagent (TDD)
       ├── Two-stage review (spec compliance + code quality)
       ├── BUILD/TEST GATE: must pass to merge
       │   └── 3 failed attempts → auto-rollback, log as blocked
       ├── TIMEOUT GATE: 15 min max per task
       ├── Merge worktree back to main
       └── Mid-execution: checkpoint audit at ~50%

5. CONVERGE CODE + DOCS (autonomous)
   ├── Round N: audit code (8 dimensions) + docs (5 dimensions)
   ├── Fix all issues, re-run tests
   ├── Loop until two consecutive clean passes
   │   (second pass from independent agent)
   └── Safety valve: max 5 rounds → escalate

5b. CONVERGE E2E TESTS (autonomous)
    ├── Loop 1: plan audit (10 dimensions) → two clean passes
    ├── Loop 2: test-plan alignment → two clean passes
    ├── Loop 3: suite execution → two consecutive green runs
    └── Loop 4: documentation alignment → two clean passes
    (See Section 3I and E2E_TEST_PATTERNS.md)

6. UPDATE PATTERNS (with learnings admission gate)
   ├── Review work done
   ├── Capture only genuinely novel learnings that change behavior
   └── Update this file

7. REPORT
   └── Convergence status, issue counts, coverage numbers, known gaps
```

The agent does NOT wait for the user between any of these steps. It drives to completion and reports the result.

---

## 12. Plan Lifecycle

### 12A. One Plan Per Development Round

Each round gets one plan document. The plan lives until all checkboxes are checked and code audit converges. Then: mark COMPLETE, update patterns, archive, start fresh.

### 12B. What Carries Forward

| Carries forward | Lives in |
|----------------|----------|
| Methodology, patterns, anti-patterns | This file (DEVELOPMENT_PATTERNS_CRUXDEV.md) |
| Hard rules, session protocols | CLAUDE.md |
| What was built, when, why | Git history |
| Ecosystem plan, phases, open questions | CRUX_ECOSYSTEM_PLAN.md |

What does NOT carry forward: plan checkboxes, specific function lists, per-round artifacts.

---

## 13. Empirical Convergence Data

From a real-world pilot project (96 checkboxes, Phases 0-6):

### Plan Convergence

| Audit Type | Passes | Issues |
|------------|--------|--------|
| Focused (Phase 6) | 3 | 10 + 8 + 7 = 25 |
| Full-plan | 2 | 14 + 8 = 22 |
| Viability | 2 | 2 + 5 = 7 |
| **Pre-execution total** | **7** | **54** |

### Code + Doc Convergence

| Round | Scope | Issues |
|-------|-------|--------|
| R1 | Code | 13 |
| R2 | Code | 9 |
| R3 | Docs | 3 |
| R4 | Both | 0 (CONVERGED) |
| **Post-execution total** | | **25** |

### Grand Total: 11 passes, 79 issues, zero remaining.

The convergence curve drops to zero within 2-4 passes per scope level. This is the empirical basis for safety valve settings.

---

## 14. Working with Existing Code

### 14A. Read Before Modifying

Never propose changes to code you haven't read. Before modifying a function:
1. Read the current implementation
2. Read its callers
3. Read its tests
4. Document the baseline in the plan

### 14B. Document Current Implementation Before Modifying

When modifying an existing function, document what it looks like now — not just what it should become. Future sessions need the baseline to apply the delta.

### 14C. Agent Delegation for Research

When checking existing code without loading it into main context, use explore agents. Keeps context clean for plan writing.

## 15. DRY Principle — Don't Repeat Yourself

**Research basis:** Hunt/Thomas (The Pragmatic Programmer), Sandi Metz ("The Wrong Abstraction"), Dan Abramov (AHA Programming), Martin Fowler (Refactoring), Google Testing Blog

### 15A. The Real Definition

DRY means: "Every piece of **knowledge** must have a single, unambiguous, authoritative representation within a system." (Hunt/Thomas, 1999)

DRY is about **knowledge and intent**, not code text. Two identical code blocks representing different domain concepts are NOT violations. Two different-looking code blocks encoding the same business rule ARE a violation.

### 15B. Where to Apply DRY

| Domain | Apply DRY | Example |
|--------|----------|---------|
| Business logic | Yes | Pricing rules, validation, domain calculations — one authoritative source |
| Configuration | Yes | DB strings, feature flags, env values — config in environment, referenced not copied |
| Schemas/contracts | Yes | API contracts defined once, generate server stubs + client SDKs + docs |
| Infrastructure | Yes | Terraform modules, CI/CD templates — parameterized, not copy-pasted |
| Documentation restating code | Yes | Comments that restate logic (not intent) will drift |

### 15C. Where NOT to Apply DRY

| Domain | Why | Authority |
|--------|-----|-----------|
| **Tests** | Tests should be DAMP (Descriptive And Meaningful Phrases). Each test is a specification readable in isolation. Shared helpers create coupling. | Google Testing Blog, Kent Beck, Martin Fowler |
| **Prototypes/spikes** | Abstracting before understanding the domain locks in wrong abstractions | Rule of Three |
| **Cross-service boundaries** | Shared libraries create deployment coupling. Operational cost > maintenance cost of duplication. | Sam Newman, microservices literature |
| **Incidental similarity** | Code that looks identical today but represents different concepts will evolve differently | Sandi Metz |
| **< 3 instances** | You don't know the true shape of the abstraction yet | Rule of Three (Fowler/Roberts) |

### 15D. Rule of Three

- **First time:** Just do it.
- **Second time:** Note the duplication, do it anyway.
- **Third time:** Now extract — you have enough examples to see the true abstraction.

The three instances must represent the **same concept changing for the same reason**. Three things that look similar but serve different domains are not candidates.

### 15E. The Wrong Abstraction (Critical)

> "Duplication is far cheaper than the wrong abstraction." — Sandi Metz

The lifecycle of a wrong abstraction:
1. See duplication → extract abstraction
2. New requirement almost fits → add parameter/conditional
3. Repeat until the abstraction is a complex, parameterized mess
4. Nobody dares refactor because everything depends on it
5. **Sunk cost fallacy keeps it alive**

**Prescription:** When you find a wrong abstraction, **inline it back** (re-duplicate), then re-extract the correct abstraction from the now-visible concrete cases.

### 15F. AHA Programming

**Avoid Hasty Abstractions** (Kent C. Dodds, building on Metz). The middle ground: prefer duplication over the wrong abstraction, but don't be afraid to abstract when the pattern is truly clear after 3+ instances.

### 15G. DRY in AI-Generated Code

LLMs produce duplication by default — each function/module is generated somewhat independently. Specific risks:
- Utility functions reimplemented across generated files
- Similar error handling copied rather than shared
- Schema/contract duplication across client and server

**Mitigations:**
- Post-generation duplication scanning (jscpd, SonarQube) in CI
- Prompt engineering that references existing abstractions
- Architecture tests enforcing structural rules
- Code review specifically focused on DRY for AI-generated PRs

### 15H. Detection Tools

| Tool | Scope | Threshold |
|------|-------|-----------|
| jscpd | 150+ languages, tokenization | Configurable min tokens/lines |
| SonarQube | Enterprise, tracks over time | Default: flag >3% |
| PMD CPD | Language-agnostic | Configurable |
| Clippy (Rust) | Redundant patterns | Built-in |
| Pylint R0801 | Python duplicate-code | Configurable min lines |

Industry benchmarks: < 5% excellent, 5-10% normal, > 15% maintenance risk.

### 15I. Convergence Audit Integration

The `duplication` dimension in code audits checks:
- Duplication percentage (flag > 5%, fail > 15%)
- Business logic has single authoritative representation
- Schemas defined once with generation for other representations
- Configuration values not hardcoded in multiple places

**Exclusions:** Test code (DAMP), intentional boundary duplication (annotated), < 3 instances (Rule of Three).

### 15J. Anti-Patterns

| Anti-Pattern | Problem |
|-------------|---------|
| **Premature abstraction** | Extracting before 3 instances — wrong shape |
| **Wrong abstraction** | Functions with boolean switches, growing parameter lists |
| **Coupling through sharing** | Shared library prevents independent deployment |
| **DRY cargo cult** | Extracting constants for strings used twice in different contexts |
| **Utils dumping ground** | Unrelated functions in a "utils" module |
| **Abstraction addiction** | Optimizing for line count over comprehension |
