# UAT Test Patterns

A convergence-driven methodology for automated user acceptance testing using AI agents. Stack-agnostic. Applies to any web application with role-based access and multi-step workflows.

This document captures **how** to plan, configure, execute, and converge AI-driven UAT testing — not which LLM or browser automation tool to use. The tools change; the methodology doesn't.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority for planning, auditing, and convergence. This document extends that methodology specifically to UAT testing.
- **E2E_TEST_PATTERNS.md** — companion document for developer-written end-to-end tests. UAT testing sits *above* E2E tests: E2E tests verify code correctness; UAT tests verify that real users (simulated by AI agents) can accomplish their goals without confusion.
- **CruxDev.md** — the autonomous convergence framework. UAT test convergence follows the same loop structure.

---

## 0. Philosophy

### Why UAT Needs Its Own Methodology

Unit, integration, and E2E tests verify that code works as specified. UAT tests verify that the **specification itself is correct** — that the features, workflows, forms, and navigation make sense to the humans who will use them.

- E2E tests ask: "Can a user register, log in, and see the dashboard?"
- UAT tests ask: "Does the dashboard show the right information for this user's role? Is the form intuitive? Are the labels confusing? Is anything missing?"

UAT testing is traditionally the most expensive phase because it requires real humans in each role to systematically explore the application. AI agents reduce this cost by simulating domain-aware users who navigate, reason about what they see, and produce structured critiques — before any human touches the system.

### AI Agents as Simulated Users

Modern LLM-based browser agents don't just click buttons — they **reason about the interface** from the perspective of their assigned role. They can:

- Navigate autonomously based on role-specific goals
- Identify confusing labels, awkward step sequences, and missing features
- Try edge cases (empty forms, invalid data, unauthorized access)
- Critique UX from a domain expert's perspective
- Produce structured findings with severity, screenshots, and reasoning

This makes them ideal UAT pre-filters. They catch a significant portion of the issues that would otherwise surface only during human UAT — the exact percentage depends on persona quality, application complexity, and the LLM's reasoning capability.

### The Testing Stack (Extended Pyramid)

```
           ╱╲
          ╱  ╲         UAT Tests (THIS DOCUMENT)
         ╱    ╲        AI agents simulating real users per role
        ╱──────╲       Run: before human UAT, after E2E passes
       ╱        ╲
      ╱          ╲      E2E Tests (E2E_TEST_PATTERNS.md)
     ╱            ╲     Deterministic critical path tests
    ╱──────────────╲    Run: merge gates, nightly
   ╱                ╲
  ╱                  ╲   Integration / Unit Tests
 ╱                    ╲  Deterministic code-level tests
╱────────────────────╲  Run: every commit
```

UAT tests are non-deterministic by nature (different LLM runs may explore different paths). This is a feature, not a bug — it increases the probability of finding real usability issues.

### The Convergence Principle

An AI-driven UAT cycle is not done when the agents finish running. It is done when:

1. The **role inventory** has been audited and is complete
2. The **agents have run** and produced findings
3. The **findings have been triaged** and all actionable items resolved
4. The **application has been re-tested** after fixes
5. The **re-test produces no new HIGH or CRITICAL findings**

Each of these is a step in a single convergence loop. The loop repeats until the re-test pass is clean.

**Independence rule:** For re-test rounds, vary the agent configuration between passes. Change the scenario order, adjust persona phrasing, or use a different LLM temperature. Running identical prompts against identical state may produce identical blind spots. The re-test must explore differently than the initial test to have value as a confirming pass.

---

## 1. Phase A: Role & Scenario Inventory

### 1A. Enumerate All User Roles

Before configuring any AI agent, list every distinct user role in the system. Include unauthenticated visitors.

```
Role: [name] ([auth level])
  Responsibilities: [what this person does day-to-day]
  Primary workflows: [the 3-5 things they do most often]
  Pain points: [what frustrates people in this role — informs agent behavior]
  Access: [what they can see/do]
  Cannot: [what they should NOT be able to see/do]
  Login: [credentials for the test environment]
```

**Quality bar:** The role description must be detailed enough that a person unfamiliar with the domain could play-act the role convincingly. If the description is too thin, the AI agent will test like a generic user instead of a domain expert.

### 1B. Define Scenario Categories

For each role, define scenarios across three categories:

| Category | Description | Example |
|----------|-------------|---------|
| **Happy path** | The intended workflow, executed correctly | Contractor submits an invoice with valid data |
| **Edge cases** | Unusual but valid inputs or sequences | Investor signs up with an email that has a `+` character |
| **Adversarial** | Invalid inputs, unauthorized access, broken workflows | Contractor tries to access admin pages; submits form with empty required fields |

### 1C. Define UX Critique Dimensions

Instruct agents to evaluate the interface along these dimensions:

| # | Dimension | What the agent should evaluate |
|---|-----------|-------------------------------|
| 1 | **Information relevance** | Does the dashboard show what this role needs? |
| 2 | **Label clarity** | Are form labels, button text, and headings clear and unambiguous? |
| 3 | **Step logic** | Is the order of steps in workflows logical for this role? |
| 4 | **Missing features** | What can't the user do that they should be able to? |
| 5 | **Error handling** | Do error messages explain what went wrong and how to fix it? |
| 6 | **Navigation** | Can the user find what they need within 2-3 clicks? |
| 7 | **Status clarity** | Are status badges, progress indicators, and state labels meaningful? |
| 8 | **Cross-role handoffs** | When work passes from one role to another, is the handoff clear? |
| 9 | **Form burden** | Do forms ask for the right amount of information at the right time? |
| 10 | **Feedback** | Does the interface provide feedback when actions are taken (or fail)? |
| 11 | **Basic accessibility** | Do images have alt text? Do form fields have labels? Is there sufficient color contrast? Are interactive elements keyboard-reachable? |

**Note on dimension 11:** AI agents can check structural accessibility (missing alt text, unlabeled form fields) but cannot evaluate experiential accessibility (screen reader flow, focus management quality). Dimension 11 covers structural checks only. Experiential accessibility remains a human UAT concern (Section 10).

### 1D. Build the Role-Scenario Matrix

| # | Role | Scenario Category | Scenario | Criticality |
|---|------|-------------------|----------|-------------|
| 1 | Investor | Happy path | Log in, view dashboard, check financials | CRITICAL |
| 2 | Investor | Edge case | Submit empty onboarding form | HIGH |
| 3 | Admin | Adversarial | Try to advance a deal — does it provide feedback? | HIGH |
| 4 | Contractor | Happy path | Submit invoice with line items | CRITICAL |

### 1E. Role Inventory Audit

Before configuring agents, audit the role-scenario matrix:

| # | Dimension | Question |
|---|-----------|----------|
| 1 | **Role completeness** | Is every user role represented, including unauthenticated visitors? |
| 2 | **Persona depth** | Could a stranger play-act each role from the description alone? |
| 3 | **Scenario coverage** | Does every role have happy-path, edge-case, AND adversarial scenarios? |
| 4 | **Criticality assignment** | Are CRITICAL/HIGH assignments justified? Is anything over- or under-weighted? |
| 5 | **Credential readiness** | Do all test accounts exist with correct roles and passwords? |
| 6 | **UX dimension coverage** | Are all 11 UX critique dimensions applicable to this application? Should any be added? |

**Process:** Review the matrix against all 6 dimensions. Fix gaps. One clean pass is sufficient (this is a checklist audit, not a convergence loop — the convergence happens in Phase E).

**Safety valve:** If the inventory requires more than 2 revision rounds, the requirements themselves are unclear. Stop and clarify requirements before proceeding.

---

## 2. Phase B: Agent Configuration

### 2A. Persona Definition

Each agent needs a persona that goes beyond role name. The persona should include:

```yaml
role: "General Contractor"
login_email: "gc@example.com"
login_password: "TestPass123!"

persona: |
  Background: You manage renovation projects. You spend most of your day
  on job sites and access the app from your phone. You need to quickly
  update project status, submit invoices, and check milestones.

  What you value: Speed over aesthetics. Clear status at a glance.
  What frustrates you: Too many form fields. Unclear invoice status.
  What you expect: A list of your active projects. A way to submit invoices.

scenarios:
  - "Log in, check dashboard — are active projects visible?"
  - "Submit an invoice — is the form reasonable?"
  - "Try submitting with empty fields — what error messages appear?"
  - "Navigate to all sidebar pages — anything missing?"
```

### 2B. Agent Behavior Instructions

Every agent should receive these universal instructions (adapted to the specific platform):

```
1. Log in using your credentials
2. Explore YOUR dashboard thoroughly
3. Navigate to every page accessible from your sidebar/nav
4. Try submitting forms — what happens with empty fields? Invalid data?
5. Look for: confusing labels, awkward step sequences, missing features
6. Try actions that SHOULD fail (e.g., accessing pages meant for other roles)
7. Check if notifications, status badges, and data make sense for your workflow
8. After testing, provide BRUTALLY HONEST feedback
```

### 2C. Tool Configuration

Configure the browser automation tool with:

- **Timeout:** Increase default timeouts for staging/UAT environments (cold starts, slower responses)
- **Screenshots:** Enable screenshot capture on every action
- **Session recording:** Enable if available — video evidence of issues is invaluable
- **Max steps:** Set high enough for thorough exploration (30-50 per role)
- **Headless mode:** Default headless for batch runs; headed for debugging

### 2D. Post-Task Questionnaire

Configure agents to answer structured questions after testing:

```yaml
questions:
  - "The dashboard shows information relevant to my role. (1-5)"
  - "I could find all the features I need. (1-5)"
  - "Navigation was intuitive. (1-5)"
  - "Forms were clear. (1-5)"
  - "Status indicators were meaningful. (1-5)"
  - "What was the most confusing part?"
  - "What feature is MISSING?"
  - "What would you change about the workflows?"
```

### 2D-1. Agent Output Format

Configure agents to produce structured findings that map to the triage template (Section 4C). Include in the agent's instructions:

"For each issue you find, report: (1) Brief description, (2) Page URL, (3) Category: Bug / UX issue / Missing feature / Cosmetic, (4) Severity: CRITICAL / HIGH / MEDIUM / LOW, (5) Your reasoning."

This reduces triage time and ensures no findings are lost in freeform output.

### 2E. Cost Management

AI agent runs have direct API costs. Estimate and budget before starting:

- **Estimate per-agent cost:** (max_steps × avg_tokens_per_step × model_price_per_token). Check current pricing for your chosen model.
- **Estimate per-round cost:** (number_of_roles × per_agent_cost).
- **Budget for 4 rounds:** The safety valve is 4 rounds, so budget 4x the per-round cost plus a 20% buffer for retries.
- **Cost guardrails:** Set max_steps per agent (Section 2C) to prevent runaway sessions. An agent stuck in a loop can consume unlimited tokens. A hard step limit is a cost ceiling.
- **Model selection trade-off:** Cheaper models (Haiku, GPT-4o-mini) find fewer issues but cost 10-20x less. Consider running cheap models first for broad coverage, then expensive models for critical roles only.

### 2F. Security: Prompt Injection from the Application

AI agents read and reason about application content. If the application contains user-generated text, that text could contain prompt injection attacks that alter agent behavior.

**Mitigations:**
- **Test environment should use controlled seed data only.** Do not run AI UAT against environments with real user-generated content.
- **Review agent output for signs of injection:** sudden behavioral changes, findings that contradict earlier findings, or agents that stop critiquing mid-session.
- **Scope agent permissions:** agents should not have write access to production data, admin capabilities, or the ability to modify other users' data. Use read-only test accounts where possible.
- **Do not use AI UAT findings as automated inputs to other systems** (e.g., auto-creating tickets from findings) without human review. A hijacked agent could generate misleading tickets.

### 2G. Stability Design

Reduce preventable instability in agent runs:

- **Fresh session per agent:** Each agent starts a new browser session. Do not reuse sessions or cookies between agents.
- **Database snapshot before runs:** Take a snapshot (or re-seed) before each round so every agent starts from the same known state.
- **Session timeout awareness:** If the application has session timeouts, ensure max_steps × avg_step_duration is well under the timeout. An agent that gets logged out mid-run produces garbage findings.
- **Environment health check:** The pre-flight checklist (Section 3A) must pass before every round, not just the first.

### 2H. Viewport Strategy

- **Default viewport:** Desktop (1280x720 or similar) unless the role description specifies mobile use.
- **Mobile-primary roles:** If a role is described as primarily mobile (e.g., a contractor on job sites), configure a second agent instance at mobile viewport (375x812) for that role's critical scenarios.
- **Do not test all roles at all viewports** — this multiplies cost without proportional value. Target mobile testing at roles whose personas describe mobile use.
- **Declare viewport in agent configuration** so it is visible in logs and finding reports.

### 2I. Test Data Lifecycle

- **Snapshot before each round:** Take a database snapshot (or re-seed) before each round so every round starts from identical state. Agent-created data from Round 1 must not persist into Round 2.
- **Within a round:** Agents may create data (submitted forms, new records). For sequential runs, later agents will see data created by earlier agents. For cross-role scenarios (Section 3E), this is intentional. For independent role testing, it is acceptable noise.
- **External service accounts:** If the application integrates with external services, use dedicated test accounts. AI agents with write access to real CRM or payment systems can create real records.
- **Cleanup between rounds is mandatory.** Do not rely on agents to undo their changes.

---

## 3. Phase C: Execution

### 3A. Pre-Flight Checklist

Before running agents:

- [ ] Test environment is up and responsive (health check)
- [ ] All test accounts are seeded with correct roles and credentials
- [ ] Login flow works (verify manually or via curl)
- [ ] 2FA, CAPTCHA, and OAuth redirects are disabled or bypassed in the test environment (AI agents cannot complete these flows)
- [ ] Browser automation dependencies installed (Playwright/Chromium)
- [ ] LLM API key is set and has sufficient quota
- [ ] Results directory is writable
- [ ] E2E test suite passes (AI UAT should not run against a system with known E2E failures)

### 3B. Execution Order

Run agents in this order:

1. **Admin first** — broadest access, tests most pages, reveals systemic issues
2. **Primary user roles** — the roles that use the platform most (e.g., Investor)
3. **Specialist roles** — roles with narrow, specific dashboards (e.g., Inspector)
4. **Cross-role scenarios** — if supported, run agents that hand off work to each other

### 3C. Parallelism

- **Sequential by default** — each agent runs to completion before the next starts. This avoids database contention and makes logs easier to read.
- **Parallel when stable** — once the test suite is proven stable, run up to 3-5 agents concurrently for speed.

### 3D. Handling Agent Failures

| Failure type | Action |
|-------------|--------|
| Agent can't log in | Fix credentials or auth flow; re-run |
| Agent gets stuck on a page | Check for timeouts, broken navigation, or missing elements; increase timeout or fix the page |
| Agent terminates early | Check max_steps, review the termination reason; may indicate a blocking issue |
| LLM API error | Retry; check quota/rate limits |
| Browser crash | Check memory; reduce parallel agents |
| Agent blocked by 2FA/CAPTCHA/OAuth | Disable or bypass these in the test environment; create a test-only login path if needed |

### 3E. Cross-Role Scenarios

When testing handoffs between roles:

1. **Sequential handoff:** Agent A completes an action (e.g., contractor submits invoice). Agent B then tests from the receiving end (e.g., admin reviews invoice). Run these sequentially with Agent A first.
2. **Shared state verification:** After Agent A acts, verify the state change is visible to Agent B. The handoff scenario should explicitly check that the receiving role sees the expected data.
3. **Do not assume agents share context.** Each agent starts fresh. Agent B does not know what Agent A did — it should discover the handed-off item through normal navigation.
4. **Handoff dependency:** If Agent A fails to complete the prerequisite action, mark Agent B's dependent scenario as BLOCKED. Do not run Agent B on stale state — the findings would be meaningless. Log the blocked scenario as a finding against Agent A's failure.

### 3F. Managing Non-Determinism

AI agents are non-deterministic. The same agent may find different issues on different runs. This is acceptable for exploration but must be managed during triage:

- **Reproducibility check:** Every finding must be manually reproduced (Section 4B, step 2). If a finding cannot be reproduced in the application, it is a false positive regardless of the agent's confidence.
- **Multi-run strategy:** For CRITICAL roles, consider running the agent 2-3 times and taking the union of findings. This increases coverage at the cost of more triage.
- **Finding stability:** If a finding appears in one run but not the next, it is still valid if manually reproducible. Non-deterministic exploration is how agents find edge cases.
- **Do not retry to make findings disappear.** Unlike E2E tests where a retry indicates flakiness, a UAT finding that does not reproduce in a second agent run may still be a real issue. Always verify manually.
- **Convergence interaction:** If a confirming pass produces a new CRITICAL or HIGH finding — whether novel or simply missed in the previous pass — it resets the convergence counter. New MEDIUM or LOW findings are noted but do not reset the counter (per Section 5B). All new findings must pass manual reproduction (Section 4B) before being counted; false positives do not reset convergence.

---

## 4. Phase D: Finding Triage

### 4A. Finding Categories

Every agent finding falls into one of these categories:

| Category | Severity | Example | Action |
|----------|----------|---------|--------|
| **Bug** | CRITICAL/HIGH | Button does nothing when clicked; form submits without validation | Fix immediately |
| **UX issue** | HIGH/MEDIUM | Confusing label; awkward step order; too many clicks | Prioritize for next iteration |
| **Missing feature** | MEDIUM/LOW | "I expected to see X but it doesn't exist" | Evaluate against requirements |
| **Cosmetic** | LOW | Alignment, color, spacing issues | Fix when convenient |
| **False positive** | N/A | Agent misunderstood the interface due to LLM limitations | Ignore; improve persona if recurrent |

### 4B. Triage Process

```
For each finding:
  1. Read the agent's reasoning and screenshot
  2. Reproduce the issue manually (1 minute)
  3. Categorize: Bug / UX issue / Missing feature / Cosmetic / False positive
  4. Assign severity: CRITICAL / HIGH / MEDIUM / LOW
  5. If actionable → add to fix list
  6. If false positive → note it and move on
```

### 4C. Finding Report Format

```markdown
### UAT-[N]: [Brief description]

**Role:** [which role found it]
**Category:** [Bug / UX issue / Missing feature / Cosmetic]
**Severity:** [CRITICAL / HIGH / MEDIUM / LOW]
**Page:** [URL where the issue occurs]
**Agent reasoning:** [what the agent said about it]
**Screenshot:** [path to screenshot]
**Status:** [Open / Fixed / Won't fix / False positive]
```

### 4D. False Positive Management

LLM agents hallucinate. They will report issues that do not exist. Manage this actively:

- **Track the false positive rate per role.** If >30% of a role's findings are false positives, the persona is too vague or the agent is exploring outside its competence. Refine the persona (backflow to Phase B).
- **Track the false positive rate per finding category.** If most false positives are in "Missing feature," the agent may lack domain context. If most are in "Bug," the agent may be misreading the UI.
- **Never skip manual reproduction.** The 1-minute reproduction check (Section 4B) is the only reliable filter. Do not batch-accept findings because "the agent seems reliable."
- **Screenshot evidence is necessary but not sufficient.** Agents can misinterpret screenshots. Always verify the screenshot matches the described issue.
- **Escalation threshold:** If the aggregate false positive rate exceeds 40%, stop the UAT cycle and re-examine agent configuration before continuing.

---

## 5. Phase E: Fix & Re-Test (Convergence Loop)

### 5A. Fix Priority

Fix in severity order:

1. **CRITICAL bugs** — application is broken; blocks testing
2. **HIGH bugs** — core functionality doesn't work
3. **HIGH UX issues** — confusing workflows that will definitely confuse real users
4. **MEDIUM** — important but not blocking
5. **LOW** — address when convenient

### 5B. Re-Test After Fixes

After fixing all CRITICAL and HIGH issues:

```
Round N:
  1. Re-deploy to test environment
  2. Re-run ALL agents (not just the ones that found issues)
  3. Triage new findings per Section 4B (including manual reproduction — only confirmed findings count)
  4. If new confirmed CRITICAL or HIGH findings → fix and Round N+1
  5. If no new CRITICAL or HIGH findings → run one more confirming pass (Round N+1)
  6. If TWO CONSECUTIVE clean passes → UAT is converged
```

**Independence rule:** Both consecutive clean passes must use different agent configurations — change scenario order, adjust persona phrasing, or use a different LLM temperature. The confirming pass (step 5) must vary from the previous round. See Section 0 for rationale. Running identical prompts may produce identical blind spots.

### 5B-1. Handling Zero Findings

If the initial agent run produces zero findings across all roles:

1. **Verify agents actually explored.** Check logs and screenshots to confirm agents navigated beyond the login page and visited multiple pages per role.
2. **Check step counts.** If agents completed far fewer steps than max_steps, they may have gotten stuck or terminated early without reporting an error.
3. **Run one role manually** and compare your experience to the agent's logs. If you find issues the agent missed, the persona is too shallow (backflow to Phase B).
4. **If agents explored thoroughly and the application is genuinely clean,** treat this as a clean pass and run the confirming second pass per Section 5B.

Do not accept zero findings at face value without verification.

### 5C. Convergence Criteria

UAT testing is converged when:

- [ ] All roles have been tested by AI agents
- [ ] All CRITICAL findings are fixed and verified
- [ ] All HIGH findings are fixed or explicitly deferred with justification
- [ ] Two consecutive re-test runs produce no new CRITICAL or HIGH findings
- [ ] The finding report is complete and archived
- [ ] Human UAT handoff is documented (what was tested, what was fixed, what to focus on)

### 5D. Safety Valve

Max 4 full rounds. If the fourth round still produces new CRITICAL/HIGH findings, the application has systemic issues that require architectural review, not more UAT cycles. Note: two consecutive clean passes require a minimum of 2 rounds (initial clean + confirmation). The budget of 4 rounds allows up to 2 rounds of fixes before the confirmation sequence must begin.

---

## 6. The Complete UAT Convergence Flowchart

```
┌─────────────────────────────────────────────────────────┐
│  PHASE A: ROLE & SCENARIO INVENTORY                     │
│                                                         │
│  Enumerate roles → scenarios → UX critique dimensions   │
│  Build the role-scenario matrix                         │
│  Output: Complete role inventory                        │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE B: AGENT CONFIGURATION                           │
│                                                         │
│  Define personas with domain knowledge + pain points    │
│  Configure browser automation + LLM                     │
│  Set up post-task questionnaire                         │
│  Output: Configured agent suite                         │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE C: EXECUTION                                     │
│                                                         │
│  Pre-flight checks → run agents per role                │
│  Capture: logs, screenshots, agent reasoning            │
│  Output: Raw findings per role                          │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE D: FINDING TRIAGE                                │
│                                                         │
│  Categorize each finding (bug/UX/missing/cosmetic/FP)   │
│  Assign severity (CRITICAL/HIGH/MEDIUM/LOW)             │
│  Reproduce manually (1 min per finding)                 │
│  Output: Prioritized finding report                     │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  PHASE E: FIX & RE-TEST — CONVERGENCE LOOP              │
│                                                         │
│  Fix CRITICAL + HIGH → re-deploy → re-run ALL agents    │
│  Triage new findings                                    │
│  LOOP until TWO CONSECUTIVE clean passes (no new        │
│  CRITICAL/HIGH)                                         │
│  Safety valve: max 4 rounds                             │
│  Output: Converged application                          │
└──────────────────────────┬──────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│  DONE                                                   │
│                                                         │
│  ✓ All roles tested by AI agents                        │
│  ✓ All CRITICAL/HIGH findings resolved                  │
│  ✓ Two consecutive re-test passes clean (no new CRITICAL/HIGH) │
│  ✓ Finding report archived                              │
│  ✓ Post-convergence documentation complete (Section 6B) │
│  ✓ Ready for human UAT                                  │
└─────────────────────────────────────────────────────────┘
```

### 6A. Backflow Rules

1. **Phase C → Phase A**: Agent execution reveals a missing role or scenario (e.g., agent discovers a page accessible only to a role not in the inventory). Return to Phase A, update the inventory, configure a new agent, re-run.
2. **Phase D → Phase B**: High false-positive rate (>30%) for a specific role indicates a weak persona. Return to Phase B, refine the persona description, re-run that role.
3. **Phase E → Phase A**: Fix cycle reveals that the scenario matrix missed an entire workflow category. Return to Phase A, add scenarios, then re-run from Phase C.

**Rule:** When backflow occurs, only re-run the affected role(s) — not the entire suite. If Phase D triggers persona refinement for the Contractor role, re-run only the Contractor agent.

### 6B. Post-Convergence Documentation

UAT documentation is a checklist, not a convergence loop. The primary artifact (the finding report) is already converged through Phase E's fix-retest cycle. No separate documentation convergence loop is needed — unlike E2E where documentation spans README, dev patterns, and architecture docs.

After UAT converges, verify:

1. **Finding report is complete** — every finding has a final status (Fixed / Won't fix / False positive). No findings are left as "Open."
2. **Deferred items are tracked** — any HIGH findings deferred with justification are logged in the project backlog, not just in the UAT report.
3. **Role inventory is updated** — if backflow changed the inventory during testing, the final version reflects all changes.
4. **Human UAT handoff is documented** — the finding report clearly states what was tested by AI, what was fixed, and what human testers should focus on.

---

## 7. Anti-Patterns

| Anti-Pattern | What Happens | Rule |
|-------------|--------------|------|
| Generic personas ("test user") | Agent tests like a tourist, misses role-specific issues | Write detailed personas with domain knowledge and pain points |
| Happy path only | Finds no bugs; validates the already-working paths | Require edge case and adversarial scenarios per role |
| Running all roles in parallel on first run | Hard to debug; database contention; overwhelming output | Run admin first, then sequentially until stable |
| Treating all findings equally | Everything feels urgent; nothing gets fixed | Triage by severity; fix CRITICAL first |
| Skipping manual reproduction | AI agent may have misunderstood; false positives waste dev time | Spend 1 minute reproducing before fixing |
| Fixing without re-testing | Fixes may introduce new issues; regression is common | Always re-run agents after a fix batch |
| No post-task questionnaire | Lose structured feedback; findings are only in logs | Configure structured exit questions per agent |
| Running UAT before E2E passes | Agents waste step budget on basic failures (broken login, missing pages, form crashes). Findings are E2E-level bugs, not UAT-level UX issues. Entire round cost is wasted. | Require a green E2E suite as a Phase C pre-flight gate (Section 3A). |
| Ignoring "missing feature" findings | These are the most valuable — they reveal spec gaps | Evaluate against requirements; update backlog |
| Over-relying on AI findings | AI agents miss some things humans catch (aesthetics, "feel") | AI UAT is a pre-filter, not a replacement for human UAT |
| LLM version drift changes results | Findings differ between runs due to model updates, not app changes | Pin the model version in agent configuration (e.g., `claude-sonnet-4-20250514` not `claude-sonnet-4-latest`). When upgrading models, re-baseline by running against a known state and comparing findings. |
| Over-trusting AI findings | CRITICAL bugs reported that are actually agent misunderstandings; developers waste time | Always manually reproduce before fixing (Section 4B) |
| Cost runaway from long sessions | Agent gets stuck in a loop, burns through API budget | Set hard max_steps limit; monitor token usage per session |
| Same prompt every round | Agent explores the same paths, misses the same issues, gives false confidence | Vary persona phrasing, scenario order, or LLM temperature between rounds |
| Using production data in test environment | Agent reads real PII; prompt injection from real user content | Use synthetic seed data only |
| No screenshot evidence | Finding says "button is broken" with no proof; impossible to triage | Require screenshots on every action (Section 2C) |

---

## 8. Metrics

| Metric | What it tells you | Target |
|--------|-------------------|--------|
| **Roles tested** | Coverage of the user base | 100% of defined roles |
| **Findings per role** | Which roles have the most issues | Decreasing per round |
| **CRITICAL findings** | Ship-blocking issues | 0 after convergence |
| **HIGH findings** | Core UX issues | 0 after convergence |
| **Fix rate** | % of findings resolved | 100% for CRITICAL/HIGH |
| **False positive rate** | Agent accuracy | < 20% |
| **Rounds to convergence** | How many fix-retest cycles | 1-3 |
| **Time per role** | Agent execution efficiency | 5-15 minutes |
| **Total UAT cycle time** | End-to-end from first run to convergence | < 1 day |
| **Cost per round** | Budget tracking; identifies cost trends | Decreasing (fewer issues = faster runs) |
| **Cost per finding** | Efficiency of AI UAT investment | Varies by application maturity; $2-10 per actionable finding in early rounds, increasing as easy issues are resolved |
| **Finding overlap between roles** | Whether multiple agents find the same issue | Low overlap = good persona differentiation |
| **Backflow count** | Times a later phase triggered return to earlier phase | 0-1 |
| **Human UAT delta** | New findings from human UAT that AI missed | Decreasing over time as personas improve |

---

## 9. Tool Selection Guidance

This methodology is tool-agnostic. Any combination of LLM + browser automation works. Requirements: the LLM must be vision-capable (screenshot analysis), the browser automation must support programmatic control (Playwright, Puppeteer), and the orchestration layer must support per-agent configuration of personas and step limits. See the project's tool-specific documentation for setup instructions.

Required tool capabilities:
- LLM can analyze screenshots (vision-capable model)
- Browser automation supports programmatic control
- Per-agent persona configuration supported
- Per-agent step limits enforced (cost ceiling)
- Screenshots captured on every action or on demand
- Session logs capture agent reasoning, not just actions
- Headless and headed modes both supported
- Agent sessions configurable with specific viewport dimensions

---

## 10. Relationship to Human UAT

AI-driven UAT does not replace human testing. It is a **pre-filter** that catches the obvious issues before humans spend their time.

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│ AI UAT           │ ──→ │ Fix & Re-test     │ ──→ │ Human UAT        │
│                  │     │                  │     │                  │
│ • 10 AI agents   │     │ • Fix CRIT/HIGH  │     │ • Real users     │
│ • Significant %   │     │ • Re-deploy      │     │ • Final sign-off │
│   of issues found │     │ • Re-test clean  │     │ • Edge cases AI  │
│ • Hours, not     │     │                  │     │   missed         │
│   weeks          │     │                  │     │ • Subjective UX  │
└──────────────────┘     └──────────────────┘     └──────────────────┘
```

Human UAT should focus on:
- **Subjective experience** — does the app *feel* right? Is it *pleasant* to use?
- **Domain nuance** — edge cases specific to the business that AI personas don't know about
- **Cross-device testing** — mobile, tablet, different browsers
- **Accessibility** — screen readers, keyboard navigation, color contrast
- **Performance** — does the app feel fast under real network conditions?

---

## 11. CruxDev Integration

When using the CruxDev convergence framework, UAT testing maps to:

| UAT Phase | CruxDev Phase | CruxDev Engine |
|-----------|---------------|----------------|
| A (Role Inventory) | Phase A (Planning) | PLAN_CONVERGENCE — audit role matrix |
| B (Agent Config) | Phase A continued | PLAN_CONVERGENCE — generate agent configs |
| C (Execution) | Phase B (Execution) | Subagent dispatch — run browser agents |
| D (Triage) | Phase C (Convergence) | CODE_CONVERGENCE — triage + classify findings |
| E (Fix & Re-test) | Phase C continued | CODE_CONVERGENCE — fix, re-run, verify |

CruxDev can drive the UAT convergence loop autonomously: run agents → triage findings → fix application → re-run agents → repeat until clean. The finding report becomes the audit artifact.

### 11A. Sequencing with E2E

AI UAT should run **after** the E2E test suite converges (see E2E_TEST_PATTERNS.md). The sequence:

1. E2E suite converges (all critical paths pass deterministically)
2. AI UAT Phase A-D runs (agents explore, findings triaged)
3. Fixes applied, E2E re-verified (fixes must not break E2E)
4. AI UAT Phase E converges (two consecutive clean passes)
5. Human UAT begins

If an AI UAT fix breaks an E2E test, the E2E failure takes priority. Fix the E2E regression first, then re-run AI UAT. An E2E regression fix that changes application behavior resets the UAT convergence counter — the prior clean pass was against different code.

### 11B. Big Bang Prompt

"Run AI UAT agents for all roles defined in the role inventory. Triage findings using the severity framework in Section 4A. Fix all CRITICAL and HIGH issues. Re-run ALL agents and verify. Loop until two consecutive re-test passes (with varied agent configuration per the independence rule) produce no new CRITICAL or HIGH findings. Verify E2E suite still passes after each fix batch. Produce the final finding report per Section 4C."

---

## 12. When to Run AI UAT

| Trigger | Which roles | Why |
|---------|-------------|-----|
| **After E2E suite converges** | All roles | Full pre-human-UAT sweep |
| **After major UI changes** | Affected roles only | Verify UX is still coherent |
| **Before human UAT sessions** | All roles | Pre-filter to maximize human tester value |
| **Before release** | All CRITICAL roles | Final AI sanity check |
| **After new role is added** | New role + roles with cross-role handoffs to/from it | Verify the new role's portal/dashboard and handoff visibility |

Do NOT run AI UAT on every commit or every merge — it is expensive and slow. Run it at the cadence above, and rely on E2E tests for continuous regression protection.
