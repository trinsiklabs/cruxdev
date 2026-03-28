# BUILD_PLAN_013: Convergence Engine Integrity Hardening

**Created:** 2026-03-26
**Status:** CONVERGED
**Goal:** Fix 5 integrity gaps that cause convergence runs to lose state, silently corrupt, or fail to resume.

**Rule:** TDD. Tests before code. 100% coverage.
**Rule:** All state mutations must be durable before returning to caller.
**Rule:** Fail closed on invalid input — never treat bad data as a clean pass.

---

## Phase 1: Project-Local State Storage

**Purpose:** Convergence state must live in the target project's `.cruxdev/`, not in the cruxdev repo root. This fixes cross-project state confusion.

- [ ] 1.1 `start_convergence` accepts `project_dir` and stores state at `<project_dir>/.cruxdev/convergence_state/<id>.json`
- [ ] 1.2 Default `project_dir` to cwd when not provided
- [ ] 1.3 `convergence_next_task`, `convergence_submit_result`, `convergence_status`, `convergence_cancel` resolve state path from convergence metadata (not hardcoded STATE_DIR)
- [ ] 1.4 Store `project_dir` in ConvergenceState so state path is recoverable from the state itself
- [ ] 1.5 Backward compat: if state file exists at old location, load from there (migration path)
- [ ] 1.6 Tests for project-local storage
- [ ] 1.7 Tests for backward compat loading
- [ ] 1.8 Coverage ≥ 100%

---

## Phase 2: Plan→Run Index

**Purpose:** Deterministic resume. Map plan files to active convergence IDs so restarting finds the existing run.

- [ ] 2.1 Create `convergence_index.json` at `<project_dir>/.cruxdev/convergence_index.json`
- [ ] 2.2 Index entry: `{ plan_file: str, convergence_id: str, status: str, started_at: float }`
- [ ] 2.3 `start_convergence` checks index — if active run exists for this plan, return it instead of creating new
- [ ] 2.4 `start_convergence` writes to index on create
- [ ] 2.5 Phase transitions and convergence/escalation update index status
- [ ] 2.6 `convergence_resume(plan_file)` MCP tool — find and resume active run by plan file
- [ ] 2.7 Tests for index create/read/update
- [ ] 2.8 Tests for resume vs new run
- [ ] 2.9 Coverage ≥ 100%

---

## Phase 3: Strict Submit Validation

**Purpose:** Malformed findings must fail closed, not silently become clean passes.

- [ ] 3.1 `convergence_submit_result` rejects invalid JSON with error response (not empty findings)
- [ ] 3.2 Validate findings array structure: each entry must have `id`, `file`, `dimension`, `severity`, `description`
- [ ] 3.3 Invalid entries logged and rejected — state not mutated
- [ ] 3.4 Return clear error message telling the caller what was wrong
- [ ] 3.5 Tests for malformed JSON rejection
- [ ] 3.6 Tests for missing required fields
- [ ] 3.7 Tests that valid submissions still work
- [ ] 3.8 Coverage ≥ 100%

---

## Phase 4: Plan Status as Derived Output

**Purpose:** The `**Status:**` line in build plan markdown must reflect actual engine state.

- [ ] 4.1 When convergence starts, update plan file `**Status:** CONVERGED` → `**Status:** CONVERGED`
- [ ] 4.2 When convergence completes, update → `**Status:** CONVERGED`
- [ ] 4.3 When escalated, update → `**Status:** ESCALATED`
- [ ] 4.4 Use regex replacement — don't rewrite the whole file
- [ ] 4.5 If no `**Status:**` line exists, don't fail — just skip
- [ ] 4.6 Tests for status line updates
- [ ] 4.7 Tests for missing status line
- [ ] 4.8 Coverage ≥ 100%

---

## Phase 5: Write-Ahead Log (WAL)

**Purpose:** Append-only event log for every convergence mutation. Crash recovery, audit trail, debugging.

- [ ] 5.1 Create `src/engine/wal.py` — append-only JSONL log
- [ ] 5.2 WAL file at `<project_dir>/.cruxdev/convergence_state/<id>.wal`
- [ ] 5.3 Events: start, next_task, submit_result, phase_change, escalate, converge
- [ ] 5.4 Each event: `{ timestamp, event_type, phase, round, findings_count, details }`
- [ ] 5.5 WAL append happens BEFORE state mutation (write-ahead)
- [ ] 5.6 fsync after each append
- [ ] 5.7 `convergence_status` includes WAL event count for observability
- [ ] 5.8 Tests for WAL append/read
- [ ] 5.9 Tests for crash recovery (WAL exists but state is stale)
- [ ] 5.10 Coverage ≥ 100%

---

## Document Alignment

### Product Docs:
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — convergence methodology
- docs/CruxDev.md — engine architecture

---

## Test Commands

```bash
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: update docs/CruxDev.md with new state storage and WAL
- [ ] Website convergence: update /engine page with WAL and integrity features
- [ ] Patterns update: capture learnings
- [ ] Inbox check: process messages

## Convergence Criteria

- All checklist items complete
- All tests pass, coverage ≥ 100%
- Malformed submissions rejected (not silent clean pass)
- State stored in project-local .cruxdev/
- Plan→run index enables deterministic resume
- Plan markdown status updated by engine
- WAL logs every mutation
- Two consecutive clean audit passes
