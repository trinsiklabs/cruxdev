# E2E Test Patterns

A convergence-driven methodology for end-to-end test suites. Stack-agnostic. Applies to any application where users interact through a UI, API, or CLI.

This document captures **how** to plan, write, audit, and converge an E2E test suite — not what tools to use. The tools change; the methodology doesn't.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority for planning, auditing, and convergence. This document extends that methodology specifically to E2E testing.
- **CruxDev.md** — the autonomous convergence framework. E2E test convergence follows the same loop structure.
- **Build plan files** — E2E testing is typically a phase within or after a build plan. This document defines how that phase works internally.

---

## 0. Philosophy

### Why E2E Tests Need Their Own Methodology

Unit and integration tests verify that code works. E2E tests verify that **users can accomplish their goals**. This is a fundamentally different concern:

- Unit tests ask: "Does this function return the right value?"
- Integration tests ask: "Do these components work together?"
- E2E tests ask: "Can a visitor register, receive a confirmation, and show up in the admin pipeline?"

E2E tests are the most expensive to write, the slowest to run, the most brittle to maintain, and the most valuable when they catch real bugs. The methodology exists to make that investment pay off.

### Accessibility in E2E Tests

E2E tests for critical journeys should include basic accessibility assertions where applicable: form fields have associated labels, modal dialogs trap focus, keyboard navigation works for core flows. This is not a full accessibility audit — it's a minimum bar for the journeys you're already testing.

### The E2E Testing Pyramid Constraint

E2E tests sit at the top of the testing pyramid. Keep the suite **small and focused on critical paths**. If a behavior can be tested at a lower level, test it there. E2E tests exist for:

1. **Cross-system journeys** — flows that span multiple pages, services, or state transitions
2. **Authentication and authorization boundaries** — can the right users access the right things?
3. **JavaScript-dependent interactions** — behaviors that only exist in a real browser
4. **Integration points** — external services, webhooks, embedded forms
5. **Critical business paths** — the flows that, if broken, mean the product is broken

### The Convergence Principle

An E2E test suite is not done when the tests pass. It is done when:

1. The **plan** has been audited to convergence (two consecutive clean passes)
2. The **tests** have been audited against the plan to convergence
3. The **test suite runs green** with no flaky tests
4. The **documentation** has been audited against the tests to convergence

Each of these is a separate convergence loop. They run sequentially, not in parallel.

**Independence rule:** For all convergence loops in this document, the two-consecutive-clean-pass rule includes an independence requirement: the second pass must come from a fresh context (different agent, different starting point) that has not seen the previous round's findings. This prevents anchoring bias.

---

## 1. Phase A: User Path Inventory

### 1A. Enumerate All User Roles

Before writing any test plan, list every distinct user role in the system. Include unauthenticated users.

```
Role: visitor (unauthenticated)
  Can: view public pages, submit registration forms, access embedded forms
  Cannot: access dashboards, edit profiles, view admin pages

Role: member (authenticated)
  Can: edit own profile, view directory, claim seats
  Cannot: manage users, view visitor pipeline

Role: chapter_admin (authenticated)
  Can: everything member can + manage chapter, view visitors, manage users
  Cannot: create chapters, grant platform_admin

Role: platform_admin (authenticated)
  Can: everything
```

### 1B. Enumerate All User Journeys

For each role, list the complete journeys — not individual pages, but **goal-oriented sequences**. A journey starts with a user intent and ends with a measurable outcome.

Format:

```
Journey: [ROLE] [verb] [goal]
  Entry: [where the user starts]
  Steps: [numbered sequence of actions]
  Exit: [measurable outcome — what changed in the system]
  Criticality: [CRITICAL / HIGH / MEDIUM / LOW]
```

**Criticality guide:**
- **CRITICAL**: If this breaks, the product is unusable. Ship-blocking.
- **HIGH**: Core functionality. Users will notice immediately.
- **MEDIUM**: Important but has workarounds.
- **LOW**: Edge case or convenience feature.

### 1C. Enumerate Cross-System Paths

Identify journeys that cross system boundaries:

- **Embedded forms** — forms loaded in iframes or on external sites
- **Webhook flows** — external service triggers a state change in the app
- **API integrations** — data flows between the app and third-party services
- **Email/SMS flows** — user receives a message and takes action from it
- **OAuth/SSO** — authentication through external providers

These are the highest-value E2E tests because they are impossible to verify at lower test levels.

### 1D. Build the Path Matrix

Organize all journeys into a matrix:

| # | Journey | Role | Criticality | Cross-system? | Steps |
|---|---------|------|-------------|---------------|-------|
| 1 | Visitor registers for chapter meeting | visitor | CRITICAL | Yes (GHL) | 5 |
| 2 | Member logs in and edits profile | member | HIGH | No | 4 |
| 3 | Admin deactivates a user | admin | MEDIUM | No | 3 |

This matrix IS the test plan. Every row becomes one E2E test (or a small group of related tests for the same journey).

---

## 2. Phase B: Test Plan Drafting

### 2A. Plan Structure

The E2E test plan is a document with one section per test. Each test section contains:

```markdown
### E2E-[N]: [Journey name]

**Role:** [who]
**Criticality:** [CRITICAL/HIGH/MEDIUM/LOW]
**Preconditions:** [what must exist before the test — seed data, user accounts, etc.]

**Steps:**
1. [Action the user takes]
   → [Expected result]
2. [Next action]
   → [Expected result]
...

**Assertions:**
- [Specific, verifiable assertion about system state after the journey]
- [Another assertion]

**Cleanup:** [What to reset after the test, if anything]
```

### 2B. Precondition Strategy

E2E tests need data. Define a precondition strategy:

- **Seed data** — data that exists before any test runs (chapters, seat rosters, admin users)
- **Per-test setup** — data created by the test itself (new users, new visitors)
- **Shared fixtures** — data shared across multiple tests (a chapter that multiple tests reference)

**Rule:** Per-test setup is preferred over shared fixtures. Shared fixtures create coupling between tests and ordering dependencies. A test that creates its own data can run in any order.

**Exception:** Seed data that represents the production environment (the chapter roster, the admin account) is acceptable as shared — it mirrors reality.

### 2C. Flakiness Prevention

Design each test to minimize flakiness:

- **Explicit waits over implicit** — wait for a specific element to appear, not a fixed time delay
- **Deterministic data** — use unique identifiers (timestamps, UUIDs) in test data to avoid collisions
- **Isolated state** — each test starts from a known state and doesn't depend on prior tests
- **Retry strategy** — 0 retries. Flaky tests must be fixed, not retried. See Section 6D for the zero-tolerance policy.
- **No sleep** — if you're sleeping, you're waiting for something. Wait for that thing explicitly.

### 2D. Environment Requirements

Document what the test environment needs:

- **Browser** — which browser(s) and viewport(s)
- **Services** — which external services need to be running or mocked
- **Database** — clean database per run, or seed data preserved across runs
- **Network** — does the test need real network access, or can everything be local

### 2E. Viewport Strategy

- **Default viewport:** Desktop (1280x720 or similar). All E2E tests run at desktop viewport unless specified otherwise.
- **Mobile viewport:** Add mobile-specific tests (e.g., 375x812) only for journeys where mobile behavior differs materially — responsive navigation, touch interactions, viewport-dependent layouts.
- **Each test declares its viewport** in the plan if it differs from the default.
- **Keep mobile tests separate** from desktop tests so they can be run independently or in parallel.

### 2F. Test Data Lifecycle

- **Database reset:** Truncate or reseed the database before each full suite run. Do not rely on accumulated state from previous runs.
- **External service accounts:** Use dedicated test accounts for external services (CRM, email, payment). Never use production accounts. Run cleanup scripts after each suite run.
- **CI isolation:** In CI environments where multiple test runs may overlap, use isolated databases per run (unique database names with run IDs, or container-per-run). Never share a test database between concurrent runs.
- **Idempotent seed data:** Seed scripts must be safe to run multiple times. Use find-or-create patterns, not insert-and-hope.

---

## 3. Phase C: Plan Audit (Convergence Loop 1)

### 3A. Plan Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | **Completeness** | Is every user journey from the path matrix represented as a test? |
| 2 | **Criticality coverage** | Are all CRITICAL and HIGH journeys covered? Are any MEDIUM/LOW journeys missing that should be included? |
| 3 | **Cross-system coverage** | Are all cross-system paths tested? Embedded forms, webhooks, API integrations? |
| 4 | **Step specificity** | Is every step specific enough that a developer could implement it without asking questions? |
| 5 | **Assertion completeness** | Does every test assert a measurable outcome? Not just "page loads" but "visitor appears in admin pipeline at Registered stage"? |
| 6 | **Precondition clarity** | Is the required state for each test explicitly listed? No hidden assumptions? |
| 7 | **Negative paths** | Are failure/error paths tested? Invalid input, unauthorized access, missing data? |
| 8 | **Independence** | Can each test run independently in any order? Are there hidden dependencies between tests? |
| 9 | **Redundancy** | Is any test duplicating coverage already provided by unit/integration tests? If so, is it justified? |
| 10 | **Feasibility** | Can every test actually be implemented with available tools? Are there steps that require manual intervention? |

### 3B. Audit Process

```
Round N:
  1. Audit the plan against all 10 dimensions
  2. Log issues as numbered items with severity and fix
  3. Apply all fixes to the plan
  4. If issues found → Round N+1
  5. If zero issues in TWO CONSECUTIVE independent passes → plan is converged
```

**Two-consecutive-clean-pass rule applies.** One clean pass is not convergence — the auditor has anchoring bias from just having fixed the previous round's issues. The second pass must come from fresh eyes (different agent context, different starting point).

**Safety valve:** Max 5 audit rounds on the plan. If still finding issues at round 5, the plan has deeper structural problems. Stop and reassess the journey inventory.

### 3C. Common Plan Audit Findings

| Round | Typical finding class |
|-------|----------------------|
| 1 | Missing journeys, vague steps, no assertions |
| 2 | Assertion gaps exposed by step refinement, precondition conflicts |
| 3 | Independence violations, redundancy with lower-level tests |
| 4+ | Diminishing returns — cosmetic or style issues |

---

## 4. Phase D: Test Implementation

### 4A. Implementation Order

Write tests in criticality order:

1. All CRITICAL tests first
2. All HIGH tests second
3. MEDIUM and LOW as time allows

Within each criticality level, write cross-system tests before same-system tests. Cross-system tests have the highest risk of revealing environment issues early.

### 4B. One Test at a Time

Write one test. Run it. Make it pass. Then write the next. Do NOT write all tests and then run them all — you'll spend hours debugging cascading failures from a single root cause.

```
For each test in the plan:
  1. Write the test (following the plan's steps and assertions exactly)
  2. Run the test
  3. If it fails:
     a. Is the test wrong? Fix the test.
     b. Is the application wrong? Fix the application.
     c. Is the plan wrong? Fix the plan AND the test.
  4. Verify the test passes
  5. Run all previous tests to check for regressions
  6. Commit
```

### 4C. Handling Application Bugs Found During E2E Testing

E2E tests frequently discover real bugs — this is their purpose. When a test fails because of an application bug:

1. **Do not skip the test.** The test is correct; the application is wrong.
2. **Fix the application bug** at the appropriate level (model, controller, view).
3. **Add a lower-level test** that catches the same bug (unit or integration). The E2E test found it; a faster test should prevent regression.
4. **Re-run the E2E test** to verify the fix.
5. **Document the bug** in the test plan as a finding.

### 4D. Test Naming Convention

Tests should read as user stories:

```
test "visitor registers from chapter page and sees confirmation"
test "member logs in and edits their business name"
test "admin deactivates a user who can no longer log in"
test "visitor registers via embedded form on external site"
```

Not:

```
test "test_registration"
test "login_test_1"
test "admin_features"
```

---

## 5. Phase E: Test-Plan Alignment Audit (Convergence Loop 2)

### 5A. After all tests are written, audit alignment

| # | Dimension | Question |
|---|-----------|----------|
| 1 | **Plan coverage** | Does every test in the plan have a corresponding implemented test? |
| 2 | **Step fidelity** | Does every implemented test follow its plan's steps in order? |
| 3 | **Assertion fidelity** | Does every implemented test assert everything the plan specifies? |
| 4 | **Extra tests** | Are there implemented tests not in the plan? If so, add them to the plan or remove them. |
| 5 | **Precondition match** | Does the test setup match the plan's preconditions? |

### 5B. Alignment Audit Process

```
Round N:
  1. For each test in the plan, verify the implementation matches
  2. For each implemented test, verify it has a plan entry
  3. Log mismatches as issues
  4. Fix issues (update test, update plan, or both)
  5. If issues found → Round N+1
  6. If zero issues in two consecutive passes → alignment converged
```

**Safety valve:** Max 5 rounds. If still finding mismatches at round 5, stop and reassess whether the plan or the implementation has structural issues.

**Note:** If alignment audit reveals plan defects affecting multiple tests, apply the backflow rules in Section 8A.

---

## 6. Phase F: Suite Execution (Convergence Loop 3)

### 6A. Run the Full Suite

Run all E2E tests in sequence (E2E tests are typically not parallelized due to shared state concerns).

### 6B. Categorize Failures

Every failure falls into one of four categories:

| Category | Meaning | Action |
|----------|---------|--------|
| **Application bug** | Test is correct, app is wrong | Fix app + add unit/integration test |
| **Test bug** | Test is wrong, app is correct | Fix test + update plan if needed |
| **Environment issue** | Test and app are correct, environment is wrong | Fix environment (service down, wrong config, stale data) |
| **Flaky test** | Test sometimes passes, sometimes fails | Investigate root cause — usually a timing issue. Fix the test. Do NOT add retries. |

### 6C. Execution Convergence Loop

```
Round N:
  1. Run full suite
  2. Categorize each failure
  3. Fix all failures
  4. Run full suite again
  5. If new failures → Round N+1
  6. If zero failures in TWO CONSECUTIVE runs → suite is green-converged
```

**Why two consecutive green runs?** A single green run may contain tests that passed by coincidence (timing, data ordering, cache state). Two consecutive runs with the same result confirms stability.

**Safety valve:** Max 5 rounds. If still finding new failures at round 5, the test suite or the application has systemic issues. Stop and investigate the pattern — are the same kinds of failures recurring?

### 6D. Flakiness Zero Tolerance

A flaky test is worse than no test. It erodes trust in the entire suite. When a test is flaky:

1. **Identify the root cause** — timing dependency, shared state, non-deterministic data, external service
2. **Fix it** — add explicit waits, isolate state, mock the service, use deterministic data
3. **Never add retries as a fix** — retries hide the problem. A test that needs retries needs rewriting.
4. **If unfixable, delete it** — a test that randomly fails provides negative value

---

## 7. Phase G: Documentation Alignment (Convergence Loop 4)

### 7A. What to Update

After the E2E suite is green-converged, update project documentation:

- **README / getting started** — how to run E2E tests
- **Development patterns** — new patterns learned during E2E test writing
- **Architecture docs** — if E2E testing revealed architectural issues
- **Test plan** — final version with all findings annotated

### 7B. Documentation Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | **Accuracy** | Does the documentation describe the tests as they actually exist? |
| 2 | **Completeness** | Are all E2E-discovered bugs documented? Are all environment requirements listed? |
| 3 | **Staleness** | Are there references to tests that were removed or renamed? |
| 4 | **Run instructions** | Can a new developer run the E2E suite from the docs alone? |

### 7C. Documentation Convergence Loop

```
Round N:
  1. Audit all project docs against the test suite
  2. Fix documentation issues
  3. If issues found → Round N+1
  4. If zero issues in two consecutive passes → documentation converged
```

Two passes are required because documentation fixes can introduce new staleness (e.g., updating a test count in one place while missing it in another).

**Safety valve:** Max 3 rounds. If documentation issues persist after 3 rounds, the documentation structure itself needs rethinking.

---

## 8. The Complete E2E Convergence Flowchart

```
┌─────────────────────────────────────────────────────────┐
│  PHASE A: USER PATH INVENTORY                           │
│                                                         │
│  Enumerate roles → journeys → cross-system paths        │
│  Build the path matrix                                  │
│  Output: Complete journey inventory                     │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE B: TEST PLAN DRAFTING                            │
│                                                         │
│  One section per test: steps, assertions, preconditions │
│  Define precondition strategy + environment reqs        │
│  Output: E2E test plan document                         │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE C: PLAN AUDIT — CONVERGENCE LOOP 1               │
│                                                         │
│  Audit plan (10 dimensions) → fix → re-audit            │
│  LOOP until two consecutive clean passes                │
│  Safety valve: max 5 rounds                             │
│  Output: Converged test plan                            │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE D: TEST IMPLEMENTATION                           │
│                                                         │
│  Write tests in criticality order                       │
│  One at a time: write → run → pass → commit             │
│  Fix app bugs when found (add lower-level regression)   │
│  Output: Complete test suite (each passes individually;  │
│          full-suite run may reveal interaction issues)   │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE E: TEST-PLAN ALIGNMENT — CONVERGENCE LOOP 2      │
│                                                         │
│  Verify tests match plan exactly                        │
│  LOOP until two consecutive clean passes                │
│  Safety valve: max 5 rounds                             │
│  Output: Tests aligned with plan                        │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE F: SUITE EXECUTION — CONVERGENCE LOOP 3          │
│                                                         │
│  Run full suite → categorize failures → fix → re-run    │
│  LOOP until two consecutive green runs                  │
│  Safety valve: max 5 rounds                             │
│  Flakiness zero tolerance                               │
│  Output: Stable, passing E2E suite                      │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE G: DOCUMENTATION — CONVERGENCE LOOP 4            │
│                                                         │
│  Update all project docs → audit alignment → fix        │
│  LOOP until two consecutive clean passes                │
│  Safety valve: max 3 rounds                             │
│  Output: Documentation matches reality                  │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  DONE                                                   │
│                                                         │
│  Four convergence loops completed:                      │
│  ✓ Plan converged                                       │
│  ✓ Tests aligned with plan                              │
│  ✓ Suite runs green (stable)                            │
│  ✓ Documentation matches tests                          │
└─────────────────────────────────────────────────────────┘
```

(Note: Backflow between phases is expected. See Section 8A.)

### 8A. Backflow Rules

The flowchart above shows the happy path. In practice, later phases regularly trigger returns to earlier phases:

1. **Phase C → Phase A**: Plan audit reveals the journey inventory is incomplete (missing role, missing cross-system path). Return to Phase A, update the inventory, then restart Phases B and C.
2. **Phase D → Phase C**: Test implementation reveals the plan is wrong. If the change affects a single test, fix the plan and continue. If it affects multiple tests or the path matrix, re-run Phase C (plan audit convergence) on the affected sections.
3. **Phase F → Phase C**: Execution reveals systematic environment or application issues that invalidate the plan. Escalate to plan reassessment.
4. **Phase G → Phase F**: Documentation audit reveals a test that should exist but doesn't. Add the test, then re-run Phase F to verify it passes.

**Rule:** When backflow occurs, only re-converge the affected loop — not the entire sequence. If Phase D fixes one test plan entry, re-audit only that entry in Phase E, not the entire plan.

---

## 9. Anti-Patterns

| Anti-Pattern | What Happens | Rule |
|-------------|--------------|------|
| Writing all tests before running any | Hours of debugging cascading failures from one root cause | Write one, run one, pass one, commit |
| Testing implementation details in E2E | Tests break on every UI refactor | Test user goals, not CSS selectors or DOM structure |
| Sharing mutable state between tests | Test A creates data that test B depends on; test B fails when run alone | Each test owns its own data |
| Using `sleep` for synchronization | Tests are slow and still flaky | Wait for specific conditions (element visible, API response received) |
| Retrying flaky tests instead of fixing them | Trust in the suite erodes; real failures get ignored | Fix the root cause or delete the test |
| Skipping the plan audit | Tests cover the wrong things or miss critical paths | Audit the plan before writing any tests |
| Testing everything at E2E level | Suite takes 30 minutes, nobody runs it | Test at the lowest level possible; E2E is for cross-system journeys |
| No cleanup between tests | Database fills up, tests slow down, ghost data causes false failures | Reset or isolate state per test |
| Auditing tests without running them | "Looks correct" is not the same as "passes" | Always run after auditing |
| Declaring done after first green run | Flaky tests and timing coincidences produce false green | Two consecutive green runs required |
| Hardcoded test data (real emails, production IDs) | Data collisions, security risk, tests break in other environments | Use generated unique data (timestamps, UUIDs) per test. Never use production credentials. |
| Setting up preconditions via UI instead of API/database | Tests are slow, brittle, and test more than the target journey | Seed precondition data directly (API calls, database inserts). Only use the UI for the journey being tested. |
| Fragile CSS selectors (`.btn:nth-child(3)`) | Tests break on any layout change | Use `data-testid` attributes or semantic selectors (role, label, text content) |
| Tests pass locally but fail in CI | Environment differences cause phantom failures | Script the entire environment setup. Match CI and local environments exactly. Test in CI early and often. |

---

## 10. Metrics

Track these across E2E test suite runs:

| Metric | What it tells you | Target |
|--------|-------------------|--------|
| **Suite pass rate** | % of runs where all tests pass | 100% (every run) |
| **Suite duration** | Total wall-clock time for full suite | < 10 minutes (adjust per project) |
| **Flaky test count** | Tests that pass/fail non-deterministically | 0 |
| **Journey coverage** | % of CRITICAL+HIGH journeys with E2E tests | 100% |
| **Failure-to-fix time** | Time from E2E failure to fix committed | < 1 hour for CRITICAL |
| **Tests per journey** | Average tests per user journey | 1-2 (keep it focused) |
| **App bugs found** | Bugs discovered by E2E tests (not lower levels) | Track to justify the investment |
| **Plan audit rounds** | Rounds to plan convergence | 2-3 |
| **Alignment audit rounds** | Rounds to test-plan alignment convergence | 1-2 |
| **Execution convergence rounds** | Rounds to green-converged suite | 2-3 |
| **Documentation convergence rounds** | Rounds to documentation convergence | 1-2 |
| **Backflow count** | Times a later phase triggered return to an earlier phase | 0-1 |

---

## 11. When to Run E2E Tests

| Trigger | Which tests | Why |
|---------|-------------|-----|
| **Before merge to main** | All CRITICAL | Gate on ship-blocking paths |
| **Nightly CI** | Full suite | Catch regressions from the day's work |
| **Before release** | Full suite | Final verification |
| **After infrastructure change** | Cross-system tests | Verify integrations survived |
| **During development** | The test you're working on | Fast feedback loop |

Do NOT run the full E2E suite on every commit — it's too slow. Use the testing pyramid: unit tests on every commit, integration tests on every push, E2E tests on merge/nightly/release.

---

## 12. Relationship to the Testing Pyramid

```
          ╱╲
         ╱  ╲         E2E Tests (THIS DOCUMENT)
        ╱    ╲        ~10-20 critical journeys
       ╱──────╲       Run: merge gates, nightly, pre-release
      ╱        ╲
     ╱          ╲      Integration / Feature Tests
    ╱            ╲     ~50-200 tests
   ╱──────────────╲    Run: every push
  ╱                ╲
 ╱                  ╲   Unit Tests
╱                    ╲  ~200-500 tests
╱────────────────────╲  Run: every commit
```

E2E tests are the apex. They are few, slow, and high-value. Everything that CAN be tested below the E2E level SHOULD be tested below the E2E level. E2E tests exist for what nothing else can catch.

The convergence methodology applies at every level — but it matters most at the E2E level, where the cost of a flaky or missing test is highest.

---

## 13. CruxDev Integration

When using the CruxDev convergence framework, the E2E test phases map to the master loop:

| E2E Phase | CruxDev Phase | CruxDev Engine |
|-----------|---------------|----------------|
| A-B (Inventory + Plan) | Phase A (Planning) | PLAN_CONVERGENCE |
| C (Plan Audit) | Phase A continued | PLAN_CONVERGENCE |
| D (Implementation) | Phase B (Execution) | Subagent dispatch per test |
| E (Alignment Audit) | Phase C (Convergence) | CODE_CONVERGENCE |
| F (Suite Execution) | Phase C continued | CODE_CONVERGENCE (run + fix loop) |
| G (Documentation) | Phase D (Patterns) | DOC_CONVERGENCE |

CruxDev can drive the entire E2E test convergence as a sub-invocation within a larger build plan. The E2E test plan becomes a "plan within a plan" — the outer build plan has a checkbox for "E2E suite converged," and CruxDev's convergence engine runs the inner E2E loops (C, E, F, G) autonomously.

Alternatively, E2E test convergence can run as a standalone CruxDev invocation with the "Big Bang" prompt: "Write E2E tests following the test plan, audit alignment, run to green, and update documentation — over and over until there are no more issues."
