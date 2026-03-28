# CruxDev Adoption Playbook

A standardized plan for bringing an existing codebase to the standard it would be at if it had been developed with Crux/CruxDev from the beginning. This covers code, tests, documentation, infrastructure, and methodology — not just coverage.

**This is a template.** Each adoption is instantiated as a numbered build plan (e.g., `BUILD_PLAN_001_ADOPTION.md`) following the DEVELOPMENT_PATTERNS_CRUXDEV.md naming convention. The phases below are executed in order using the lights-out convergence methodology.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the execution methodology. This playbook says *what* to do; the patterns file says *how* to drive it to convergence.
- **E2E_TEST_PATTERNS.md** — the E2E testing methodology. Phase 7 of this playbook follows it.
- **CruxDev.md** — the framework architecture. Phase 1 installs the framework. All phases execute via the CruxDev convergence engine.

---

## Overview

An adopted repo passes through 9 phases. The first two install the development infrastructure. The next five upgrade the codebase. The last two verify and document the result.

```
Phase 1: Infrastructure Setup           ← Development environment
Phase 2: Codebase Assessment            ← Understand what exists + viability
Phase 3: Architecture Remediation        ← Structural fixes (brainstorming gate)
Phase 4: Code Hardening                  ← Safety, security, resilience
Phase 5: Test Suite Build-Out            ← Unit + integration to 100%
Phase 6: Documentation Convergence       ← Docs match code
Phase 7: E2E Test Suite                  ← User-facing journey coverage
Phase 8: Convergence Verification        ← Full audit to two clean passes (code + docs + EVERY PAGE)
Phase 9: Methodology Handoff             ← Patterns file + CLAUDE.md
```

Each phase has its own convergence loop. The full adoption runs lights-out per DEVELOPMENT_PATTERNS_CRUXDEV.md — the agent drives to completion autonomously.

### Execution Model (Cross-Cutting Rules)

These rules apply to ALL phases. They come from DEVELOPMENT_PATTERNS_CRUXDEV.md and are not repeated in individual phases.

| Rule | Reference | Summary |
|------|-----------|---------|
| **Git worktree isolation** | Section 2D | Every implementation task runs in an isolated worktree. Adoption changes are high-risk — isolation prevents contaminating main. |
| **Subagent delegation** | Section 2E | Independent modules can be processed in parallel via implementer + reviewer subagents. Especially useful in Phases 4 and 5. |
| **Auto-rollback** | Section 2B | 3 failed attempts on any task → rollback to last known good state, log as blocked, move on. |
| **15-minute timeout** | Section 2B | Max 15 minutes per task. Checkpoint and move on if exceeded. |
| **Honest tracking** | Section 3E | Annotate caveats on checkboxes. Maintain Known Gaps section for deferred items. |
| **Convergence state files** | Section 7C | Each phase persists convergence state to disk (JSON, atomic writes). If the agent crashes mid-phase, the next session resumes from the state file. |
| **Mid-execution checkpoint** | Section 2B | At ~50% completion of each phase, audit progress tracker honesty. |

### Escalation Rules

The agent runs autonomously EXCEPT when (per DEVELOPMENT_PATTERNS_CRUXDEV.md Section 0B):

| Condition | Action |
|-----------|--------|
| Safety valve hit (max rounds reached) | Stop, present triage report, ask for direction |
| Ambiguous requirement discovered | Stop, ask for clarification |
| Architecture decision with no clear winner | Stop, present 2-3 options with tradeoffs |
| Net-negative fix round (issues increasing 2 consecutive rounds) | Stop immediately, escalate |
| Protected file modification needed | Stop, present the change, ask for approval |
| Viability blocker that can't be auto-fixed | Stop, present the blocker |

### CruxDev Engine Mapping

| Playbook Phase | CruxDev Engine Phase | Skills Loaded |
|----------------|---------------------|---------------|
| 1-2 | Phase A (Plan Convergence) | `planning`, `viability-assessment` |
| 3-7 | Phase B (Execution) | Phase-specific skills (see phase headers) + cross-cutting: `executing`, `subagent-delegation`, `git-worktrees`, `systematic-debugging` |
| 8 | Phase C (Code+Doc+Page Convergence) | `auditing`, `convergence-driving`, `honest-tracking` (spans CODE_CONVERGENCE, DOC_CONVERGENCE, and PAGE_CONVERGENCE engines) |
| 9 | Phase D (Patterns Update) | `patterns-capture` |

---

## Phase 1: Infrastructure Setup

**Purpose:** Install the development environment so every subsequent phase has enforcement. Without this, code changes happen without safety nets.

**This phase runs first because everything downstream depends on it. Phase 1 does not have a convergence loop — it is complete when all checklist items are verified.**

### 1A. Project Configuration

| Item | Standard | Action |
|------|----------|--------|
| `pyproject.toml` (Python) or equivalent | `fail_under = 100`, `show_missing = true` | Create or update |
| `.claude/CLAUDE.md` | TDD rules, coverage enforcement, test commands, session protocol | Create |
| `DEVELOPMENT_PATTERNS_CRUXDEV.md` | Copy or symlink from CruxDev | Install |
| `.gitignore` | Coverage artifacts, temp files, `.crux/` state files | Update |
| Test directory | `tests/` with `conftest.py` for shared fixtures | Create if missing |
| CI configuration | Coverage gate in CI, E2E test triggers per E2E_TEST_PATTERNS.md Section 11 | Create or update |

### 1B. CruxDev Framework Installation

**Prerequisite:** CruxDev must be available locally (cloned repo or `$CRUXDEV_PATH`).

```bash
# Install CruxDev into the project
CRUXDEV_SRC="${CRUXDEV_PATH:-../cruxdev}"
cp -r "$CRUXDEV_SRC" .cruxdev/

# Append bootstrap to CLAUDE.md
grep -q "# CruxDev Bootstrap" .claude/CLAUDE.md 2>/dev/null || \
  cat .cruxdev/adapters/claude-code/CLAUDE.md >> .claude/CLAUDE.md

# Copy slash commands and hooks
cp .cruxdev/adapters/claude-code/commands/*.md .claude/commands/ 2>/dev/null
cp .cruxdev/adapters/claude-code/hooks/*.sh .claude/hooks/ 2>/dev/null
```

### 1C. Baseline Measurements

Before changing anything, measure and record:

```
Coverage:           X% (from coverage tool)
Test count:         N tests
Test pass rate:     N passing / M total
Lint warnings:      N
Security issues:    N (from static analysis if available)
Doc staleness:      (manual assessment: fresh / stale / absent)
Atomic writes:      N of M critical file writes use write-then-rename
Connection safety:  N of M connections in context managers / try-finally
Input validation:   N of M boundary functions validate input
Pages/routes:       N total pages (N form, N content, N dashboard, N auth, N API)
Forms:              N total forms across all pages
Broken links:       N (from link scan)
Contrast failures:  N pages with issues (from initial scan)
Mobile failures:    N pages with issues (from initial viewport check)
Security headers:   present / missing (from HTTP check if deployed)
Hardcoded secrets:  N (from grep scan)
```

These become the "before" numbers for the Phase 8 before/after comparison.

### Checklist — Phase 1

- [ ] 1.1 `pyproject.toml` (or equivalent) with `fail_under = 100`
- [ ] 1.2 `.claude/CLAUDE.md` with TDD rules and test commands
- [ ] 1.3 CruxDev framework installed
- [ ] 1.4 Test directory structure exists
- [ ] 1.5 CI coverage gate configured
- [ ] 1.6 E2E test triggers configured per E2E_TEST_PATTERNS.md Section 11
- [ ] 1.7 Baseline measurements recorded (all rows including atomic writes, connections, validation, page counts, form counts, broken links, contrast failures, mobile failures, security headers, hardcoded secrets)
- [ ] 1.8 Git commit: "chore: install CruxDev infrastructure"

---

## Phase 2: Codebase Assessment

**Purpose:** Understand what exists before changing it. This is the gap analysis — compare current state against Crux/CruxDev standards, produce a prioritized remediation list, and verify the remediation plan is executable.

**Read before modifying. Always.**

### 2A. Architecture Inventory

Map the codebase:

| Item | What to Document |
|------|-----------------|
| Module map | Every source file, its purpose, its dependencies |
| Entry points | CLI commands, API endpoints, scheduled jobs, hooks |
| **Page/route inventory** | **Every user-facing page/route — classified by type (form, content, dashboard, auth, API). See ADOPTION_PROCESS.md Step 5.5A for stack-specific inventory methods.** |
| Data stores | Databases, JSON files, config files — which are authoritative? |
| External integrations | APIs, webhooks, OAuth, email/SMS |
| State machines | Any system with explicit states and transitions |
| File I/O | Which files are written, how, by what code paths |
| Test infrastructure | Test framework, existing fixtures, mocking approach, CI test commands |
| **Templates/components** | **Shared templates, layout files, component libraries — which exist, which are missing for this stack?** |

### 2B. Standards Gap Analysis

For each module, assess against Crux standards:

| Dimension | Standard | Common Gaps |
|-----------|----------|-------------|
| **Atomic writes** | All critical file writes use write-then-rename | Direct `open(f, 'w')` on important files |
| **Crash resilience** | State files for long-running processes, idempotent operations | No checkpointing, operations unsafe to re-run |
| **Input validation** | All external inputs validated at boundary | Raw user input passed through unchecked |
| **Path safety** | No path traversal, symlink-safe, canonicalized paths | `os.path.join` with user input, no traversal checks |
| **Error handling** | Explicit error paths, no bare `except:`, cleanup in `finally` | Swallowed exceptions, resource leaks |
| **Connection safety** | All connections in `try/finally` or context managers | SQLite/HTTP connections without cleanup |
| **State machine clarity** | States named explicitly, transitions documented | Implicit states buried in boolean flags |
| **Logging** | Structured logging, no sensitive data in logs | `print()` debugging, credentials in error messages |
| **Test coverage** | 100%, verified with `term-missing` | Low or no coverage, coverage-by-coincidence |
| **Documentation** | Accurate, no staleness, no phantom references | Stale counts, documented-but-unbuilt features |

### 2C. Prioritized Remediation List

Order by risk × effort:

| Priority | Category | Criteria |
|----------|----------|----------|
| **P0** | Security + data safety | Path traversal, injection, data corruption risks |
| **P1** | Crash resilience | Operations that lose data on crash |
| **P2** | Architecture | Structural issues that make everything else harder |
| **P3** | Code quality | Error handling, connection safety, logging |
| **P4** | Documentation | Staleness, phantom references |
| **P5** | Style + conventions | Naming, formatting, consistency |

### 2D. Viability Assessment

After the assessment is written and the remediation list prioritized, verify the plan is actually executable (per DEVELOPMENT_PATTERNS_CRUXDEV.md Section 1A Step 4):

1. Are required tools/deps installed? (test framework, linters, coverage tools)
2. Does the test suite run at all? Can you get a baseline coverage number?
3. Is CI access available for configuring gates?
4. Are there external service dependencies that need test accounts?
5. Is the codebase in a state where changes can be made? (no broken build, no locked branches)

Fix viability blockers before proceeding to Phase 3.

### 2E. Assessment Audit Dimensions

The assessment is audited against these dimensions (not the standard 8 code dimensions — this is a document, not code):

| # | Dimension | Question |
|---|-----------|----------|
| 1 | Completeness | Are all modules inventoried? |
| 2 | Accuracy | Do descriptions match code reality? |
| 3 | Prioritization validity | Is P0-P5 ordering defensible? |
| 4 | Actionability | Is each remediation item specific enough to execute? |

### Checklist — Phase 2

- [ ] 2.1 Architecture inventory complete (including test infrastructure)
- [ ] 2.1b Page/route inventory complete (every page enumerated and classified by type — form, content, dashboard, auth, API)
- [ ] 2.1c Applicable pattern docs mapped per page type (FORM_PATTERNS, COLOR_CONTRAST_PATTERNS, MOBILE_WEB_PATTERNS, DASHBOARD_PATTERNS, WCAG AA)
- [ ] 2.1d Template/component inventory: all required templates for this stack identified, missing ones flagged
- [ ] 2.2 Standards gap analysis per module
- [ ] 2.2b Standards gap analysis per page (each page checked against its applicable pattern docs)
- [ ] 2.3 Prioritized remediation list
- [ ] 2.4 Viability assessment passed (tools, deps, CI, test accounts)
- [ ] 2.5 Assessment document written (lives in plan file)
- [ ] 2.6 Assessment audited (two consecutive clean passes on 4 dimensions above)

---

## Phase 3: Architecture Remediation

**Purpose:** Fix structural issues that affect multiple modules. These must be fixed before code hardening because they change interfaces that downstream code depends on.

**Loads skills:** `state-machines`, `data-safety`, `brainstorming`

### 3.0. Brainstorming Gate

Before making any architecture decision (authoritative data source, state machine extraction, config consolidation), run the brainstorming gate per DEVELOPMENT_PATTERNS_CRUXDEV.md Section 1F:

1. Explore context from codebase
2. Propose 2-3 approaches with tradeoffs
3. **GATE:** Design must be approved before proceeding

Architecture decisions are exactly the kind that benefit from "propose 2-3 approaches" — they are hard to reverse and affect everything downstream.

### 3A. Common Architecture Fixes

| Pattern | From | To |
|---------|------|----|
| **Authoritative data source** | Ambiguous (DB and files both "truth") | JSON authoritative, DB is read-optimized index rebuilt from JSON. Or: DB authoritative, exported to JSON for portability. Pick one. |
| **State machine extraction** | Boolean flags (`is_done`, `is_processing`) | Explicit state enum, named transitions, terminal states documented |
| **Configuration consolidation** | Scattered config across env vars, files, hardcoded values | Single config module that loads from one source with explicit defaults |
| **Entry point standardization** | Multiple ways to invoke the same operation | Single canonical entry point per operation, aliases redirect |
| **Dependency direction** | Circular imports, utility modules importing domain modules | Dependency flows one direction: domain → service → infrastructure |

### 3B. Interface Stabilization

Before hardening individual modules, stabilize the interfaces between them:

1. Document every public function signature (name, params, return type). A function is "public" if it is called from outside its own module — use grep/import analysis to identify cross-module calls.
2. Write interface tests that verify the contract (input → output)
3. Now the interfaces are locked — internal changes can't accidentally break callers

### 3C. Data Flow Documentation

For every data path (user input → processing → storage → output):

```
Input: [where does data enter?]
Validation: [where is it validated? is it validated at all?]
Processing: [what transforms it?]
Storage: [where does it land? is the write atomic?]
Output: [where does it go? is it sanitized?]
```

Gaps in this chain become Phase 4 work items.

### Checklist — Phase 3

- [ ] 3.0 Brainstorming gate passed for each major architecture decision
- [ ] 3.1 Authoritative data source decided and documented
- [ ] 3.2 State machines extracted from boolean flags
- [ ] 3.3 Configuration consolidated
- [ ] 3.4 Entry points standardized
- [ ] 3.5 Dependency direction cleaned up
- [ ] 3.6 Public interfaces documented and tested
- [ ] 3.7 Data flow paths documented
- [ ] 3.8 All existing tests still pass
- [ ] 3.9 Architecture changes audited (two consecutive clean passes)

---

## Phase 4: Code Hardening

**Purpose:** Bring every module up to Crux safety standards. This is the line-by-line remediation work.

**Loads skills:** `data-safety`, `tdd`, `systematic-debugging`

**Note on TDD exception:** During adoption, hardening existing code is a refactoring activity where existing behavior is preserved. Hardening before writing new tests (Phase 5) is acceptable because we're fixing safety issues in existing code, not building new features. However, new behavior introduced during hardening (e.g., new error paths, new validation checks) should still follow TDD — write the test for the new behavior first, then implement it.

### 4A. Atomic Writes

Every write to a critical file must use write-then-rename:

```python
def atomic_write(path, data):
    tmp = path.with_suffix('.tmp')
    tmp.write_text(json.dumps(data, indent=2))
    tmp.rename(path)  # atomic on POSIX
```

**Audit:** grep for `open(.*'w')` and `json.dump(.*open` across the codebase. Each hit is a candidate for atomic write conversion.

### 4B. Connection Safety

Every connection must be in a context manager or try/finally:

```python
# Before (leaks on exception):
conn = sqlite3.connect(db_path)
cursor = conn.execute(query)
results = cursor.fetchall()
conn.close()

# After:
with sqlite3.connect(db_path) as conn:
    cursor = conn.execute(query)
    results = cursor.fetchall()
```

**Audit:** grep for `sqlite3.connect`, `http.client`, `urllib.request`, `requests.` — verify each is in a context manager or try/finally.

### 4C. Input Validation at Boundaries

Every function that accepts external input (user input, API data, file contents, environment variables) must validate at the boundary:

| Input Type | Validation |
|-----------|------------|
| File paths | Canonicalize, check within base dir, no traversal |
| Strings | Length limit, type check, sanitize for output context |
| Lists | Length limit, item type check |
| JSON | Schema validation or explicit field extraction with defaults |
| Environment variables | Existence check, type coercion, default values |

### 4D. Error Handling

| Anti-Pattern | Fix |
|-------------|-----|
| Bare `except:` | Catch specific exceptions |
| `except Exception: pass` | Log the error, re-raise or handle explicitly |
| No `finally` for cleanup | Add `finally` block for resource cleanup |
| Error messages expose internals | Sanitize error messages at API boundaries |
| No error path at all | Add explicit error handling for every I/O operation |

### 4E. Crash Resilience

For any operation that takes > 1 second or processes multiple items:

1. Write state to disk before starting
2. Update state after each significant step
3. On resume, read state and pick up where you left off
4. Every operation is idempotent (safe to re-run)

### 4F. Security Hardening

| Check | Standard |
|-------|----------|
| No hardcoded credentials | Credentials from env vars or config files only. Grep for API keys, tokens, passwords in source. Check `.env` files are gitignored. |
| No path traversal | All user-supplied paths validated and canonicalized |
| No command injection | Subprocess calls use list args, never `shell=True` with user input |
| No SQL injection | Parameterized queries, never f-string SQL |
| Sensitive data in logs | Sanitize before logging, never log credentials/tokens |
| File permissions | Config files 0o600, directories 0o700 |
| **CSRF protection** | **Every state-changing form has a CSRF token. Framework CSRF middleware enabled.** |
| **Security headers** | **CSP, HSTS, X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Permissions-Policy configured in the web server or framework middleware.** |
| **XSS prevention** | **No raw HTML injection with user data (`dangerouslySetInnerHTML`, `{!! !!}`, `| safe`). Output escaped by default.** |
| **Auth verification** | **Auth-protected pages checked server-side, not just hidden in the UI. Middleware/guards enforced.** |
| **Dependency vulnerabilities** | **Run `npm audit` / `pip-audit` / `mix audit` / `cargo audit`. No known critical vulnerabilities.** |

### Checklist — Phase 4

- [ ] 4.1 All critical writes converted to atomic (write-then-rename)
- [ ] 4.2 All connections in context managers or try/finally
- [ ] 4.3 All external input validated at boundaries
- [ ] 4.4 All bare `except:` replaced with specific exception handling
- [ ] 4.5 All long-running operations have crash-resilient state files
- [ ] 4.6 All idempotency requirements met (safe to re-run)
- [ ] 4.7 Security hardening checklist passed (including CSRF, security headers, XSS, auth, dependency audit)
- [ ] 4.7b No hardcoded secrets in source (grep verified, not assumed)
- [ ] 4.7c Security headers configured and verified (CSP, HSTS, X-Content-Type-Options, X-Frame-Options)
- [ ] 4.8 All existing tests still pass
- [ ] 4.9 Code hardening audited (two consecutive clean passes)

---

## Phase 5: Test Suite Build-Out

**Purpose:** Bring test coverage to 100%, verified empirically with `--cov-report=term-missing`. This phase also fixes coverage-by-coincidence and adds missing test categories.

**Loads skills:** `tdd`, `honest-tracking`

### 5A. Coverage Gap Closure

Same methodology proven in the Crux project's BUILD_PLAN_001_COVERAGE_CLOSURE:

1. Run `--cov-report=term-missing` to get exact uncovered lines
2. For each uncovered line range, understand the code path
3. Write a test that exercises that specific path
4. Verify with `term-missing` that the lines disappeared from "Missing"
5. Never trust test names as proof of coverage

### 5B. Test Categories

Ensure all categories exist:

| Category | What It Tests | Typical Location |
|----------|--------------|-----------------|
| Unit tests | Individual functions in isolation | `tests/test_<module>.py` |
| Integration tests | Module interactions, data flow | `tests/test_<flow>_integration.py` |
| Edge case tests | Boundary conditions, error paths, empty inputs | Within unit test files |
| Security tests | Path traversal, injection, auth bypass | `tests/test_<module>_security.py` or inline |
| Crash recovery tests | Resume from interrupted state, idempotent re-runs | `tests/test_<module>_recovery.py` or inline |

### 5C. Test Quality Standards

| Standard | Enforcement |
|----------|------------|
| Each test owns its own data | No shared mutable state between tests |
| Tests run in any order | No ordering dependencies |
| Tests are fast | Mock I/O-heavy operations; real filesystem via `tmp_path` |
| Test names read as specifications | `test_visitor_registers_and_sees_confirmation` not `test_reg_1` |
| No `sleep` in tests | Wait for specific conditions |
| No retries for flaky tests | Fix root cause or delete |

### Checklist — Phase 5

- [ ] 5.1 Coverage gap analysis (list all uncovered lines per module)
- [ ] 5.2 Tests written for all uncovered lines
- [ ] 5.3 Coverage verified at 100% with `term-missing`
- [ ] 5.4 Coverage-by-coincidence check (verify specific lines, not just percentages)
- [ ] 5.5 All test categories present (unit, integration, edge, security, recovery)
- [ ] 5.6 Test quality standards met
- [ ] 5.7 Full suite passes: 0 failures, 100% coverage
- [ ] 5.8 Mid-adoption checkpoint: audit Phases 1-5 for honest tracking (coverage claims verified, deferred items tracked)

---

## Phase 6: Documentation Convergence

**Purpose:** Make documentation match reality. Not aspirational — descriptive.

**Loads skills:** `auditing`

### 6A. Documentation Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | Accuracy | Do function signatures in docs match code? |
| 2 | Completeness | Are all modules/functions documented? |
| 3 | Staleness | Are there hardcoded counts or state that should be live? |
| 4 | Phantom references | Do documented features/commands actually exist? |
| 5 | Architecture alignment | Do docs describe the current architecture? |

### 6B. Common Doc Fixes

| Issue | Fix |
|-------|-----|
| Hardcoded counts ("4,483 entries") | Replace with query command or remove |
| Documented commands that don't exist | Remove or implement |
| Stale architecture diagrams | Redraw from current code |
| README describes planned features as existing | Move to roadmap or remove |
| Out-of-date install instructions | Verify by following them in a fresh environment |

### 6C. CLAUDE.md Accuracy

The project's CLAUDE.md must accurately describe:
- Current test count (not a stale number)
- Available commands and tools
- File structure
- Active constraints and rules

### Checklist — Phase 6

- [ ] 6.1 All documentation audited against code (5 dimensions)
- [ ] 6.2 Phantom references removed
- [ ] 6.3 Stale counts replaced or removed
- [ ] 6.4 Architecture docs match current state
- [ ] 6.5 CLAUDE.md verified accurate
- [ ] 6.6 README install instructions verified in fresh environment
- [ ] 6.7 Documentation audited to convergence (two consecutive clean passes)

---

## Phase 7: E2E Test Suite

**Purpose:** Verify that users can accomplish their goals through the product's interface. Follows E2E_TEST_PATTERNS.md methodology.

**Loads skills:** `tdd`, `auditing`, `convergence-driving`

### 7A. Applicability

Include E2E tests if the product has any user-facing surface:
- Web UI → browser-based E2E tests
- CLI → subprocess-based E2E tests
- API → HTTP-based E2E tests
- MCP server → tool-call-based E2E tests

Skip only for pure libraries with no user-facing interface.

**Testing pyramid constraint:** Before writing E2E tests, verify the behavior isn't already covered by Phase 5 unit/integration tests. E2E tests are for cross-system journeys only — things nothing else can catch.

**Viewport strategy (web UI only):** For web UI products, define a viewport strategy per E2E_TEST_PATTERNS.md Section 2E — default desktop viewport, mobile only when behavior differs materially, each test declares its viewport.

### 7B. E2E Convergence (Four Loops)

Per E2E_TEST_PATTERNS.md:

1. **Plan audit** — 10 dimensions, two consecutive clean passes
2. **Test-plan alignment** — every test matches its plan spec
3. **Suite execution** — two consecutive green runs
4. **Documentation** — docs match test reality

**Backflow:** Later loops can trigger return to earlier loops. See E2E_TEST_PATTERNS.md Section 8A. When backflow occurs, only re-converge the affected loop — not the entire sequence.

**Accessibility:** E2E tests for CRITICAL journeys should include basic accessibility assertions per E2E_TEST_PATTERNS.md Section 0 (form labels, focus trapping, keyboard navigation).

### Checklist — Phase 7

- [ ] 7.1 User roles enumerated
- [ ] 7.2 User journeys inventoried with criticality
- [ ] 7.3 E2E test plan written
- [ ] 7.4 Plan audited to convergence
- [ ] 7.5 Tests implemented (CRITICAL first, then HIGH)
- [ ] 7.5b Testing pyramid constraint verified (no E2E test duplicates unit/integration coverage)
- [ ] 7.6 Accessibility assertions included for CRITICAL journeys
- [ ] 7.7 Test-plan alignment verified
- [ ] 7.8 Suite runs green (two consecutive runs)
- [ ] 7.9 E2E metrics baseline recorded per E2E_TEST_PATTERNS.md Section 10
- [ ] 7.10 Documentation updated and converged

---

## Phase 8: Convergence Verification

**Purpose:** Full-codebase audit across all dimensions PLUS per-page audit of every user-facing page. This is the final quality gate — the adopted codebase must survive the same audit rigor as code built from scratch with CruxDev. **Project-level "PASS" is not convergence. Every individual page must pass every applicable dimension.**

**Loads skills:** `auditing`, `convergence-driving`, `honest-tracking`

### 8A. Full Code Audit (8 Dimensions)

Run the standard 8-dimension code audit from DEVELOPMENT_PATTERNS_CRUXDEV.md Section 3B across the ENTIRE codebase — not just the files changed during adoption:

1. Plan alignment
2. Correctness
3. Test coverage
4. BDD alignment (skip if project has no BDD feature files)
5. Cross-module integration
6. Data safety
7. Documentation compliance
8. Regression

### 8B. Full Doc Audit (5 Dimensions)

Run the standard 5-dimension doc audit from Section 3C across ALL documentation.

### 8C. Page-Level Audit (Web Projects)

**This is the audit that prevents rubber-stamping.** For every project with user-facing web pages, run the full per-page audit from ADOPTION_PROCESS.md Step 5.5. This is NOT optional and NOT a sampling exercise.

#### 8C.1 Route Inventory Verification

Re-verify the route inventory from Phase 2. New routes may have been added during Phases 3-7. The inventory must be current.

#### 8C.2 Per-Page Form Audit

For EACH form on EACH page, audit all 17 form dimensions from FORM_PATTERNS.md. Read the SPECIFIC source file for that page — not just shared form components.

**The audit question is NOT:** "Does this project have form components that follow patterns?"
**The audit question IS:** "On `/visit/westlake-select`, is the email field's label above the input, does it have `autocomplete="email"`, is the textarea correctly sized, and are touch targets 44x44px?"

Every form field on every page must pass. Document findings per form, per page.

#### 8C.3 Per-Page Contrast Audit

For EACH page, verify contrast ratios on RENDERED output — not by reading CSS variables or semantic tokens.

| Check | Method |
|-------|--------|
| Body text contrast | Read the actual hex values used on this page, compute ratio |
| Secondary text | Same — do not assume "uses text-gray-600" means it passes |
| Links | Verify against the actual background color on this page |
| UI components | Buttons, inputs, icons — 3:1 minimum |
| Dark mode | If the site has dark mode, check BOTH modes per page |

**Tool:** Use `check_contrast` MCP tool or manual hex-pair computation. "Semantic tokens are defined" is not evidence of per-page compliance.

#### 8C.4 Per-Page Mobile Audit

For EACH page, verify mobile usability against MOBILE_WEB_PATTERNS.md:

- [ ] Navigation accessible at mobile viewport (not just "hamburger component exists" — does THIS page render it?)
- [ ] No horizontal scroll at 320px width
- [ ] All touch targets 44x44px with 8px spacing
- [ ] Body text 16px+ (not just "the CSS variable is set")
- [ ] Forms usable on mobile (correct keyboard types, adequate input sizing)
- [ ] Images responsive (no overflow)

#### 8C.5 Per-Page Link Validation

For EACH page, verify:

- [ ] Every internal link resolves to an existing route in the route inventory
- [ ] No broken anchor links (`#section` points to an element with that ID)
- [ ] External links use `rel="noopener noreferrer"` on `target="_blank"`
- [ ] No links to non-existent pages (404s)

**Method:** Parse all `<a href="...">` in each page's rendered output. Cross-reference internal links against the route inventory. For deployed sites, HTTP HEAD each link.

#### 8C.6 Per-Page Security Audit

For EACH page:

- [ ] No hardcoded API keys, tokens, or passwords visible in source or rendered HTML
- [ ] CSRF token present on every state-changing form
- [ ] No raw user data in `dangerouslySetInnerHTML` / `{!! !!}` / `| safe`
- [ ] Auth-protected pages enforce auth server-side
- [ ] No sensitive data in HTML comments or hidden fields

#### 8C.7 Per-Page SEO Audit

For EACH page:

- [ ] Unique `<title>` (not duplicated from another page)
- [ ] Unique `<meta name="description">` (not duplicated)
- [ ] Single `<h1>`, logical heading hierarchy
- [ ] All non-decorative images have alt text
- [ ] Open Graph tags present (og:title, og:description, og:image)
- [ ] Canonical URL set

#### 8C.8 Pattern Doc Coverage Verification

Verify that ALL applicable pattern docs were checked during the adoption. Cross-reference:

| Pattern Doc | Checked During Phase | Verified Per-Page |
|------------|---------------------|-------------------|
| FORM_PATTERNS.md | Phase 8C.2 | Yes — per form, per page |
| COLOR_CONTRAST_PATTERNS.md | Phase 8C.3 | Yes — per page |
| MOBILE_WEB_PATTERNS.md | Phase 8C.4 | Yes — per page |
| DASHBOARD_PATTERNS.md | Phase 8C.2 (dashboard pages) | Yes — per dashboard page |
| E2E_TEST_PATTERNS.md | Phase 7 | Yes — per journey |
| WEBSITE_PLANNING.md | Step 5 | Yes — per page for applicable sections |
| SEO_AND_GEO_REFERENCE.md | Phase 8C.7 | Yes — per page |
| POST_DEPLOYMENT_PATTERNS.md | Phase 8E | Yes — live verification |
| I18N_PATTERNS.md | Phase 8C (if applicable) | Yes — per page if multi-language |
| DRY_UI_COMPONENT_PATTERNS.md | Phase 3 | Yes — component reuse verified |
| BLOG_PATTERNS.md / BLOG_POST_PATTERNS.md | Phase 8C (if blog) | Yes — per blog page |

If ANY pattern doc applies to the project but was NOT checked, that is a gap. Fix it before convergence.

### 8D. Live Site Verification (Deployed Projects)

For projects with a deployed site, source code audit alone is NOT sufficient. Verify the live site.

#### 8D.1 HTTP Verification

Fetch each page and verify:

- [ ] HTTP 200 response (not redirect loops, not soft 404s)
- [ ] Correct `Content-Type` header
- [ ] Security headers present:
  - `Strict-Transport-Security` (HSTS)
  - `Content-Security-Policy` (CSP)
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY` or `SAMEORIGIN`
  - `Referrer-Policy`
  - `Permissions-Policy`
- [ ] HTTPS enforced (HTTP redirects to HTTPS)
- [ ] No mixed content warnings

#### 8D.2 Rendered vs Source Comparison

For a representative sample of pages (minimum: all form pages + homepage + one content page):

- [ ] Fetch the rendered HTML
- [ ] Verify form labels are above inputs in the DOM (not just in source)
- [ ] Verify mobile nav is functional (JS-dependent nav must actually work)
- [ ] Verify contrast values in rendered CSS match expectations
- [ ] Verify no server-side errors leaked into HTML

#### 8D.3 Performance Spot-Check

- [ ] Core Web Vitals (LCP < 2.5s, INP < 200ms, CLS < 0.1) — use PageSpeed Insights or `check_pagespeed` MCP tool
- [ ] Time to First Byte < 800ms
- [ ] No render-blocking resources that could be deferred

### 8E. GTV on Every Adoption Claim

**Every "PASS" in the adoption audit must be verifiable.** This means:

| Claim | NOT Acceptable Evidence | Acceptable Evidence |
|-------|------------------------|---------------------|
| "Forms follow patterns" | "Form components exist and use Tailwind" | "On `/contact`, the name field has `<label for='name'>` above the input, `autocomplete='name'`, 44px touch target" |
| "Contrast passes WCAG AA" | "Uses semantic color tokens" | "On `/about`, body text #111827 on background #FAFAFA = 15.4:1 ratio" |
| "Mobile responsive" | "Uses responsive CSS classes" | "At 320px viewport, `/pricing` has no horizontal scroll, nav shows hamburger, touch targets are 48px" |
| "No security issues" | "Framework handles security" | "Grepped for API keys: 0 matches. CSRF middleware enabled in `middleware.py`. All forms have `{% csrf_token %}`" |
| "Links work" | "Internal links use relative paths" | "Parsed 47 internal links across 12 pages. All resolve to routes in the inventory. 0 broken." |
| "SEO complete" | "Meta tags exist" | "Each of 12 pages has unique title and description. No duplicates. All images have alt text." |

**If you cannot produce the specific evidence column, the claim is not verified.**

### 8F. Convergence Criterion

Two consecutive independent clean passes across all dimensions INCLUDING the per-page audit. The second pass must come from a fresh agent context.

**Convergence means:**
- ALL code dimensions pass (8 dimensions, entire codebase)
- ALL doc dimensions pass (5 dimensions, all documentation)
- ALL pages pass (every page, every applicable dimension)
- ALL live site checks pass (if deployed)
- ALL claims have GTV evidence

### 8G. Before/After Comparison

Compare against Phase 1 baseline:

```
                    Before    After
Coverage:           X%        100%
Test count:         N         N+M
Test pass rate:     N/M       (N+M)/(N+M)
Atomic writes:      A/K       K/K
Connection safety:  B/J       J/J
Input validation:   C/L       L/L
Doc accuracy:       stale     converged
E2E journeys:       0         P (all CRITICAL+HIGH)
Pages audited:      0         R/R (all pages, all dimensions)
Forms passing:      ?/T       T/T (every form, every dimension)
Contrast passing:   ?/R       R/R (every page)
Mobile passing:     ?/R       R/R (every page)
Broken links:       ?         0
Security issues:    ?         0
Live site verified: no        yes (all pages, all headers)
```

### Checklist — Phase 8

- [ ] 8.1 Full code audit pass 1 (entire codebase, not just changed files)
- [ ] 8.2 Full code audit pass 2 (independent, clean)
- [ ] 8.3 Full doc audit pass 1
- [ ] 8.4 Full doc audit pass 2 (independent, clean)
- [ ] 8.5 Page-level form audit: every form on every page passes 17 dimensions
- [ ] 8.6 Page-level contrast audit: every page passes WCAG AA on rendered output
- [ ] 8.7 Page-level mobile audit: every page works at 320px viewport
- [ ] 8.8 Page-level link validation: every internal link resolves, 0 broken links
- [ ] 8.9 Page-level SEO audit: unique title/description per page, heading hierarchy, alt text
- [ ] 8.10 Page-level security audit: no hardcoded secrets, CSRF on all forms, no XSS vectors
- [ ] 8.11 Pattern doc coverage: every applicable pattern doc was checked, no gaps
- [ ] 8.12 Live site verification (if deployed): HTTP checks, security headers, rendered output
- [ ] 8.13 GTV evidence documented for every "PASS" claim
- [ ] 8.14 Before/after comparison documented (including page-level metrics)
- [ ] 8.15 All tests pass (unit + integration + E2E)
- [ ] 8.16 Coverage at 100%
- [ ] 8.17 Known Gaps reconciled (all deferred items from earlier phases resolved or explicitly accepted)
- [ ] 8.18 Two consecutive clean passes on ALL of the above (code + docs + pages + live site)

---

## Phase 9: Methodology Handoff

**Purpose:** Leave the project in a state where future development follows CruxDev methodology. The project should be self-sustaining — any agent that reads the CLAUDE.md and patterns file can continue at the same quality level.

**Loads skills:** `patterns-capture`

### 9A. Project Patterns File

Create `DEVELOPMENT_PATTERNS_<PROJECT>.md` capturing (per DEVELOPMENT_PATTERNS_CRUXDEV.md Section 4, governed by the learnings admission gate in Section 2F — only genuinely novel learnings that would change future behavior):

- Architecture decisions made during adoption (with rationale)
- Stack-specific patterns discovered
- Anti-patterns found and fixed (so they don't recur)
- Test conventions established
- Any project-specific deviations from CruxDev defaults

### 9B. CLAUDE.md Finalization

The project's CLAUDE.md should contain:

| Section | Content |
|---------|---------|
| Identity | What the project is, who owns it |
| Core rules | TDD, 100% coverage, convergence methodology |
| Test commands | Exact commands to run tests with coverage |
| Key files | Patterns file, plan files, design docs |
| Session protocol | What to do at session start |
| E2E test triggers | When to run E2E tests per E2E_TEST_PATTERNS.md Section 11 |

### 9C. Future Development Gate

After adoption, ALL future development on this project follows DEVELOPMENT_PATTERNS_CRUXDEV.md:

- Plans are numbered with descriptors
- Plans are audited to convergence before execution
- Execution uses TDD with safety gates
- Code+docs converge to two consecutive clean passes
- E2E tests are included when applicable
- Patterns file is updated after each development round

### Checklist — Phase 9

- [ ] 9.1 Project patterns file created (learnings admission gate applied)
- [ ] 9.2 CLAUDE.md finalized and accurate
- [ ] 9.3 CruxDev framework installed and functional
- [ ] 9.4 First future development round planned using CruxDev methodology
- [ ] 9.5 Adoption complete — project is at CruxDev standard

---

## The Full Adoption Flowchart

```
┌──────────────────────────────────────────────────────────────┐
│ Phase 1: INFRASTRUCTURE                                      │
│   Install CruxDev, configure coverage enforcement,           │
│   record baseline measurements (including atomic writes,     │
│   connection safety, input validation counts)                │
│   No convergence loop — complete when checklist verified     │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 2: ASSESSMENT + VIABILITY                              │
│   Architecture inventory (including test infrastructure),    │
│   PAGE/ROUTE INVENTORY (every page, classified by type),     │
│   standards gap analysis (per module AND per page),          │
│   template/component inventory, prioritized remediation list │
│   Viability check: tools, deps, CI, test accounts            │
│   → Audit to convergence (two clean passes, 4 dimensions)   │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 3: ARCHITECTURE REMEDIATION                            │
│   Brainstorming gate before each major decision              │
│   Structural fixes: data sources, state machines,            │
│   config, entry points, dependency direction                 │
│   → Audit to convergence                                     │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 4: CODE HARDENING                                      │
│   Atomic writes, connection safety, input validation,        │
│   error handling, crash resilience, security                 │
│   TDD for new behavior; refactoring preserves existing       │
│   → Audit to convergence                                     │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 5: TEST SUITE BUILD-OUT                                │
│   Coverage gap closure to 100%, all test categories,         │
│   coverage-by-coincidence elimination                        │
│   → Verify with --cov-fail-under=100                         │
│   Mid-adoption checkpoint: audit Phases 1-5 honesty          │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 6: DOCUMENTATION CONVERGENCE                           │
│   Accuracy, completeness, staleness, phantoms, architecture  │
│   → Audit to convergence (two clean passes)                  │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 7: E2E TEST SUITE                                      │
│   Four convergence loops per E2E_TEST_PATTERNS.md            │
│   Plan audit → alignment → execution → docs                 │
│   Backflow between loops expected (Section 8A)               │
│   Accessibility assertions for CRITICAL journeys             │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 8: CONVERGENCE VERIFICATION                            │
│   Full 8+5 dimension audit across ENTIRE codebase            │
│   Per-page audit: forms, contrast, mobile, links, SEO,      │
│   security — every page, every applicable dimension          │
│   Live site verification: fetch pages, check rendered output │
│   GTV on every "PASS" — specific evidence, not assumptions   │
│   Two consecutive independent clean passes                   │
│   Before/after comparison against Phase 1 baseline           │
└────────────────────────┬─────────────────────────────────────┘
                         ▼
┌──────────────────────────────────────────────────────────────┐
│ Phase 9: METHODOLOGY HANDOFF                                 │
│   Project patterns file (learnings admission gate),          │
│   CLAUDE.md finalization, future development gated           │
│                                                              │
│   ADOPTION COMPLETE                                          │
└──────────────────────────────────────────────────────────────┘
```

---

## Adoption Scope Variants

Not every adoption needs all 9 phases at full depth. The playbook scales.

**Partial adoptions may escalate to full adoption** if Phase 2 assessment reveals structural issues requiring Phase 3. Document the decision to escalate.

### Full Adoption (Production System)

All 9 phases, full depth. For systems that handle user data, run in production, or will be actively developed going forward.

### Partial Adoption (Internal Tool)

Phases 1, 2, 4, 5, 8 (lightweight), 9. Skip architecture remediation (3) if the tool is small and well-structured. Skip E2E (7) if no user-facing surface. Skip doc convergence (6) if docs are minimal and accurate. Phase 8 runs a lightweight convergence: full-codebase audit, but only on the dimensions corresponding to executed phases (e.g., skip doc dimensions if Phase 6 was skipped).

### Assessment Only

Phases 1-2 only. Produces the gap analysis and prioritized remediation list without making changes. Useful for scoping the effort before committing.

### Test-Only Adoption

Phases 1, 5, 8. Install infrastructure, close coverage gaps to 100%, verify. For codebases where the code is solid but tests are missing. This is what BUILD_PLAN_001_COVERAGE_CLOSURE does for Crux itself.

---

## Anti-Patterns in Adoption

| Anti-Pattern | What Happens | Rule |
|-------------|-------------|------|
| Skipping assessment | Changes break things you didn't know existed | Phase 2 before Phase 3. Always. |
| Hardening before architecture | Atomic-writing a function that shouldn't exist | Fix structure first, then harden the survivors |
| Writing tests before hardening | Tests encode current (broken) behavior | Harden code, then test the hardened version |
| "Good enough" coverage | 90% feels like 100% but isn't | `fail_under = 100`. No exceptions. |
| Changing too much at once | Can't tell which change broke what | One module at a time, tests after each |
| Skipping E2E for CLI tools | "It's just a CLI" — but users interact through it | CLI is a user interface. Test the journeys. |
| No patterns file at handoff | Next developer starts from scratch | Phase 9 exists for a reason |
| Adopting without CruxDev installed | No enforcement, standards drift immediately | Phase 1 installs enforcement. It's first for a reason. |
| Boiling the ocean on Phase 2 | Assessment takes forever on a large codebase | Timebox Phase 2 to 1-2 sessions. It's a living document updated as later phases reveal more. |
| Auditing only changed files in Phase 8 | Adoption gaps in untouched code | Phase 8 covers the ENTIRE codebase, not just changed files |
| Adopting a fork without understanding divergence | Re-hardening code that was intentionally changed | Document fork-specific changes before hardening |
| Skipping viability assessment | Plan fails on first step due to missing deps | Phase 2D verifies plan is executable before Phase 3 starts |
| No brainstorming gate for architecture decisions | Wrong structure chosen, expensive to reverse | Phase 3.0 requires 2-3 approach proposals before deciding |
| Hardening code that should be deleted | Wasted effort on dead code | Phase 2 should identify modules for deletion, not just remediation. Phase 3 removes them before hardening. |
| **Project-level auditing for page-level concerns** | **"Project has forms → PASS" while individual pages have broken forms** | **Phase 8 audits EACH page against EACH applicable dimension. "The project" does not pass — individual pages pass or fail.** |
| **Trusting CSS variables as contrast proof** | **"Uses semantic color tokens → PASS" while rendered pages have low contrast** | **Phase 8C.3 computes actual contrast ratios on rendered output per page.** |
| **Trusting component existence as mobile proof** | **"Hamburger component exists → PASS" while specific pages don't render it** | **Phase 8C.4 checks each page at mobile viewport, not just shared components.** |
| **Skipping live site verification** | **Source code looks correct but deployed site has missing headers, broken JS, stale cache** | **Phase 8D fetches the deployed site and verifies rendered output matches source claims.** |
| **Accepting unverifiable claims** | **"PASS" without specific evidence, agent hallucinating compliance** | **Phase 8E requires GTV: specific hex values, specific line numbers, specific URLs, specific dimensions. Not "looks good."** |
| **Skipping pattern doc coverage check** | **FORM_PATTERNS exists but was never checked during adoption** | **Phase 8C.8 cross-references every applicable pattern doc against what was actually audited.** |
