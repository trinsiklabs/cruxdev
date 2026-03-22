# Development Patterns

Extracted from BUILD_PLAN.md and the collaborative process between Bryan and Claude that produced it. These are the patterns, conventions, and methodologies we've established for Claude Code development on this project. Follow these for all future development rounds.

This document captures both **what** we build and **how** we plan and collaborate. The meta-process is as important as the technical patterns.

**Relationship to other files:**
- **CLAUDE.md** — the authority. Contains mandatory session rules, logging requirements, pipeline tools, and project-specific constraints. DEVELOPMENT_PATTERNS.md captures reusable methodology; CLAUDE.md contains the hard rules that override everything.
- **BUILD_PLAN.md** — the current plan. Created using these patterns. When the plan is complete, it gets archived and a new one is created for the next development round.
- **This file is living.** Bryan asks "update DEVELOPMENT_PATTERNS.md with everything you've seen" at the end of each planning session. New patterns are added, existing ones refined. This self-updating cycle is itself a pattern.

**When to read this file:**
- At the start of any planning session (creating or auditing a BUILD_PLAN.md)
- Before writing any new plan document
- When Bryan says "follow the patterns" or references development methodology
- NOT needed for routine pipeline work (extraction, synthesis, etc.) — CLAUDE.md covers that

---

## 0. Planning Process (How We Build Plans)

### 0A. The Full Planning Cycle

Building a plan is a multi-step collaborative process, not a single document write. The cycle we've established:

```
1. Bryan describes the problem and goals (conversation)
2. Claude asks clarifying questions, proposes approach
3. Bryan corrects misconceptions, adds constraints
   — e.g., "this burns API tokens, it needs to stay in Claude Code"
   — e.g., "facts need to be queued for review, corrections ripple"
4. Claude writes the comprehensive plan document
5. Bryan demands the plan be audited
6. Claude audits against all known concerns, finds gaps (14 gaps in round 1)
7. Bryan says "fix all"
8. Claude applies all fixes
9. Bryan demands a SECOND audit
10. Claude re-audits, finds MORE gaps introduced by fixes (8 in round 2)
11. Bryan approves fixes
12. Claude applies, verifies totals
13. Bryan adds new requirements (e.g., "add a code audit phase")
14. Repeat audit cycle as needed
15. Only THEN does building start
```

**Key insight:** The first draft of a plan is never right. The audit cycle is where the plan becomes real. Don't resist being audited — embrace it. The plan gets dramatically better through each pass.

### 0B. Bryan's Role in Planning

Bryan is not a passive consumer of plans. He:

- **Corrects architectural assumptions** — Claude proposed API calls; Bryan caught that it would burn tokens outside Pro Max and killed it. This changed the entire architecture.
- **Adds requirements mid-stream** — "facts get queued for review, corrections ripple to all related entries" was added during discussion, not in the initial scope.
- **Demands audits** — "audit BUILD_PLAN.md. is it sane, coherent, complete?" This is not optional. Bryan will ask for this, and Claude should proactively offer it.
- **Demands re-audits** — Fixing 14 gaps can introduce new gaps. Bryan knows this and will ask again. Claude should expect this.
- **Holds quality gates** — Bryan won't say "good enough." The plan must converge to zero known issues before execution starts.

### 0C. Claude's Role in Planning

Claude must:

- **Think out loud** — share reasoning, tradeoffs, concerns as they arise. Don't silently make decisions.
- **Push back when appropriate** — if something won't work technically, say so with specifics.
- **Be honest about limitations** — "I can't do full TDD in this session, here's why" is acceptable. Silently skipping it is not.
- **Proactively audit** — don't wait to be asked. After writing a plan, offer to audit it. After fixing gaps, offer to re-audit.
- **Count things precisely** — checkbox totals, file counts, test counts. Grep to verify, don't eyeball. The plan said "63 items" when it had 82. This was caught because we counted.

### 0D. Constraint Discovery Through Conversation

Some of the most important constraints emerge from Bryan correcting Claude's assumptions:

| What Claude Proposed | What Bryan Corrected | Resulting Constraint |
|---------------------|---------------------|---------------------|
| Call Claude API for automated analysis | "This burns API tokens instead of happening in Claude Code with my Pro Max subscription" | All LLM work stays in Claude Code conversation |
| Simple import pipeline | "Facts get queued for review, corrections ripple to all related entries" | Full review queue with correction propagation |
| Skip testing for small scripts | CLAUDE.md mandates TDD for everything | TDD/BDD with 100% coverage, no exceptions without Bryan's approval |

**Pattern:** Claude proposes → Bryan corrects → constraint is codified in plan header → never violated again.

### 0E. The Complete Planning Flowchart

This is the full cycle from problem statement to execution-ready plan, with the exact prompts Bryan uses at each stage. Every future development round should follow this flowchart.

```
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 1: PROBLEM DEFINITION                  │
│                                                                 │
│  Bryan: "I want X. Here's the problem. Here are my goals."     │
│  Claude: Asks clarifying questions, proposes approach           │
│  Bryan: Corrects assumptions, adds constraints                  │
│    → "this burns API tokens, it needs to stay in Claude Code"   │
│    → "facts need to be queued for review, corrections ripple"   │
│  Claude: Revises approach based on corrections                  │
│                                                                 │
│  LOOP until Bryan says "write the plan":                        │
│    Claude proposes → Bryan corrects → constraint codified       │
│                                                                 │
│  Output: Shared understanding of scope + constraints            │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 2: PLAN WRITING                        │
│                                                                 │
│  Claude writes the comprehensive plan document:                 │
│    - Architecture overview (before/after)                       │
│    - Phases with data structures, functions, tests              │
│    - Progress tracker with checkboxes                           │
│    - File inventory, risks, dependencies                        │
│    - Definition of Done                                         │
│                                                                 │
│  Bryan: "extract everything about how we planned this into a    │
│    development patterns file"                                   │
│  Claude: Writes DEVELOPMENT_PATTERNS.md capturing methodology   │
│                                                                 │
│  Output: BUILD_PLAN.md + DEVELOPMENT_PATTERNS.md                │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 3: FOCUSED AUDITS                      │
│                                                                 │
│  Bryan: "audit phase 6 for any and all issues"                  │
│  Claude: Audits single phase → finds issues → fixes them        │
│    Round 1: 10 structural issues                                │
│                                                                 │
│  Bryan: "now do it again"                                       │
│  Claude: Re-audits same phase → finds issues fixes introduced   │
│    Round 2: 8 logical issues                                    │
│                                                                 │
│  Bryan: "now do it again"                                       │
│  Claude: Re-audits → finds consistency/redundancy issues        │
│    Round 3: 7 consistency issues                                │
│                                                                 │
│  LOOP: "do it again" until a round finds 0 or near-0 issues    │
│                                                                 │
│  Output: Phase internally consistent                            │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 4: FULL-PLAN AUDITS                    │
│                                                                 │
│  Bryan: "now audit the entire plan for issues, again"           │
│  Claude: Audits ALL phases together → finds cross-phase issues  │
│    Round 1: 14 cross-phase integration issues                   │
│    (stale references, state machine name divergence,            │
│     dependency graph errors, missing test coverage)             │
│                                                                 │
│  Bryan: "audit it again for any remaining issues"               │
│  Claude: Re-audits full plan → finds residual issues            │
│    Round 2: 8 more stale references + missing edge cases        │
│                                                                 │
│  LOOP: "audit it again" until a round finds 0                   │
│                                                                 │
│  Output: Plan internally consistent across all phases           │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 5: VIABILITY ASSESSMENT                 │
│                                                                 │
│  Bryan: "if i tell you to run the plan exactly as planned,      │
│    and then after you run it, i tell you to audit and fix any   │
│    issues, over and over, until you report there aren't any     │
│    left to fix, will the plan succeed?"                         │
│                                                                 │
│  Claude must:                                                   │
│    1. Re-read the ENTIRE plan with fresh eyes                   │
│    2. Simulate execution mentally — walk through each phase     │
│    3. Check the actual environment:                             │
│       - Are referenced tools/deps installed? (pip list, etc.)   │
│       - Do import paths work? (python3 -c "import X")           │
│       - Does the project config exist? (pyproject.toml? etc.)   │
│    4. Answer YES or NO with SPECIFIC numbered caveats           │
│    5. Each caveat must be concrete and fixable                  │
│                                                                 │
│  Bryan: "fix these in the plan as much as possible"             │
│  Claude: Fixes each caveat directly in the plan                 │
│                                                                 │
│  Example caveats found across two viability passes:             │
│    Pass 1:                                                      │
│      - Context pressure in Phase 4 (fixed: sub-sessions)        │
│      - Undocumented baseline code (fixed: inline current impl)  │
│    Pass 2:                                                      │
│      - BDD framework not installed (fixed: chose pytest-bdd)    │
│      - Progress tracker order wrong (fixed: swapped steps)      │
│      - No Definition of Done section (fixed: added)             │
│      - Interaction-required steps unmarked (fixed: flags added) │
│      - Wrong CLI tool referenced (fixed: uv → python3)          │
│                                                                 │
│  Output: Plan verified executable against actual environment    │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 5B: UPDATE PATTERNS FILE                │
│                                                                 │
│  Bryan: "update DEVELOPMENT_PATTERNS.md with everything         │
│    you've seen since we created it"                             │
│  Claude: Reviews all new patterns from audits + viability       │
│    → Adds new sections, updates existing ones                   │
│                                                                 │
│  Bryan: "now audit the DEVELOPMENT_PATTERNS.md file for gaps    │
│    in capturing what we've done"                                │
│  Claude: Audits patterns file against actual work done          │
│    → Finds missing patterns, redundancies, gaps                 │
│    → Fixes them                                                 │
│                                                                 │
│  (This step happens at least once per planning session.         │
│   The patterns file is a living document.)                      │
│                                                                 │
│  Output: DEVELOPMENT_PATTERNS.md current for this round         │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 6: CHECKBOX → TASK CONVERSION            │
│                                                                 │
│  Before execution, convert plan checkboxes to Claude Code tasks: │
│                                                                 │
│  Step 1: Audit checkboxes against the plan for accuracy         │
│    - Are all plan steps represented as checkboxes?              │
│    - Are checkbox descriptions precise and actionable?           │
│    - Are dependencies correct? (Phase 2 before Phase 3, etc.)   │
│    - Iterate audit until checkboxes are verified correct         │
│                                                                 │
│  Step 2: Convert each checkbox to a Claude Code task (TaskCreate)│
│    - Subject = checkbox text (imperative form)                   │
│    - Description = context from plan (what files, what to do)    │
│    - Set blockedBy for dependency ordering                       │
│    - Group by phase                                             │
│                                                                 │
│  Step 3: Execute from task list, not from the plan file          │
│    - TaskUpdate → in_progress when starting                     │
│    - TaskUpdate → completed when done                           │
│    - Tasks give Bryan real-time progress visibility              │
│    - Tasks persist across sessions without reading the plan      │
│                                                                 │
│  Output: Task list mirrors plan, execution tracked in tasks      │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 6B: EXECUTION                            │
│                                                                 │
│  Bryan: "ok, execute the plan"                                  │
│  Claude: Works from Claude Code task list:                      │
│    - TaskList to find next available task                        │
│    - TaskUpdate → in_progress                                   │
│    - Do the work                                                │
│    - TaskUpdate → completed                                     │
│    - Log to memory DB                                           │
│    - Also mark checkbox in plan file (dual tracking)             │
│    - Repeats until all tasks done                               │
│                                                                 │
│  Output: All code written, tests passing, docs updated          │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 7: CODE + DOC AUDIT CONVERGENCE         │
│                                                                 │
│  (Built into plan as Phase 6 / final phase)                     │
│                                                                 │
│  Bryan: "audit and fix any issues in the code, and in the docs, │
│    over and over, until you report there aren't any left to fix" │
│  Claude: Audits code + docs in parallel (8 dimensions + 5 doc   │
│    dimensions) → Finds N issues → fixes → re-runs tests         │
│                                                                 │
│  Round 1: Code issues (logic bugs, missing impls, leaks)        │
│  Round 2: More code issues (cross-cutting: connection leaks)    │
│  Round 3: Doc issues only (stale counts, phantom references)    │
│  Round 4: CLEAN PASS on both — TWO consecutive clean needed     │
│                                                                 │
│  LOOP until: two consecutive independent clean passes           │
│    (one clean pass is NOT convergence — anchoring bias)          │
│                                                                 │
│  Output: DONE — code matches plan, tests pass, coverage ≥ 100%, │
│    docs match code, no phantom references, no stale counts      │
└─────────────────────────────────────────────────────────────────┘
```

### 0E-2. Viability Assessment Methodology

The viability check is not just "does this look right" — it's a structured verification against the actual environment.

**Step 1: Re-read the entire plan.** Not skim — read every phase, every function signature, every test name. Fresh eyes catch things that editing sessions miss.

**Step 2: Simulate execution.** Walk through Phase 0 step 0.1. What commands would you run? What imports would you write? Where would files go? Do this mentally for every phase. Catch "this step assumes X but X doesn't exist yet" errors.

**Step 3: Verify the environment.** This is what distinguishes a viability check from a plan audit. Check the ACTUAL machine:

```bash
# Are referenced tools installed?
python3 -c "import pytest_bdd" 2>&1       # BDD framework?
python3 -c "import pytest_cov" 2>&1       # Coverage tool?
pip list | grep -i behave                  # Alternative BDD?

# Do import paths work?
python3 -c "import memory.mem_pipeline"    # Is memory/ importable?
ls memory/__init__.py                      # Explicit package or namespace?

# Does project config match plan assumptions?
ls pyproject.toml                          # Does it exist?
which uv                                   # Is uv available?

# Do referenced files/directories exist?
ls memory/tests/                           # Test directory?
ls memory/work_queue/                      # Work queue directory?

# Does code already exist for files the plan says to create?
# (Glob for every file in the plan's "Files to Create" table)
ls app/chat_importers/base.py 2>&1        # Does it already exist?
ls app/chat_db.py 2>&1                    # Already implemented?
```

**Step 3b: Check for existing implementations.** A plan that says "NOT STARTED" may be partially or fully built from a prior session. This happened: CHAT_PROVENANCE_PLAN.md said "NOT STARTED" when Phases 1-2 were 80% built with 75 passing tests. For every file the plan says to CREATE, check whether it already exists. If it does, the plan status must be updated, and existing code must be verified against the plan spec rather than written from scratch. Failure to do this wastes an entire session re-creating existing work or, worse, overwrites working code with plan-spec code that may be less complete.

**Step 4: Report caveats.** Not "it should work" but "YES, with these specific issues that need fixing: [numbered list]." Each must be actionable.

**Step 5: Fix and re-verify.** After fixing caveats, the viability check should be run again. This is why Bryan asked the viability question TWICE — the first pass found 2 caveats, the second pass (after DEVELOPMENT_PATTERNS.md was updated) found 5 more. Environment verification catches things that pure document auditing misses.

### 0E-3. The Full Prompt Sequence (Reference)

These are Bryan's actual prompts in order. Use this as a template for future development rounds:

```
PLANNING PHASE:
  "I want [goal]. [Problem description]. [Constraints]."
  → Claude writes plan

  "extract everything about how we planned this into a development
   patterns file"
  → Claude writes DEVELOPMENT_PATTERNS.md

FOCUSED AUDIT PHASE:
  "audit phase N for any and all issues"
  → Claude audits, finds/fixes issues

  "now do it again"
  → Claude re-audits, finds issues introduced by fixes

  "now do it again"
  → Claude re-audits, finds consistency issues
  (repeat until Claude reports 0 issues)

FULL-PLAN AUDIT PHASE:
  "now audit the entire plan for issues, again"
  → Claude audits all phases, finds cross-phase issues

  "audit it again for any remaining issues"
  → Claude re-audits, finds residual cross-phase issues
  (repeat until Claude reports 0 issues)

VIABILITY PHASE:
  "if i tell you to run the plan exactly as planned, and then after
   you run it, i tell you to audit and fix any issues, over and over,
   until you report there aren't any left to fix, will the plan
   succeed?"
  → Claude answers YES/NO with specific numbered caveats

  "fix these in the plan as much as possible"
  → Claude fixes all caveats in the plan document

  (Bryan may ask viability question AGAIN after fixes — this is
   correct, because fixes can introduce new issues or reveal
   additional environment gaps. In our case, the second viability
   pass found 5 issues the first pass missed.)

PATTERNS UPDATE PHASE:
  "update DEVELOPMENT_PATTERNS.md with everything you've seen
   since we created it"
  → Claude reviews all sessions, adds new patterns

  "now audit the DEVELOPMENT_PATTERNS.md file for gaps in
   capturing what we've done/been doing"
  → Claude audits patterns against actual work, fixes gaps
  (repeat if Bryan asks)

EXECUTION PHASE:
  "ok, execute the plan"
  → Claude follows plan exactly, checkbox by checkbox

TASK CONVERSION PHASE (before execution):
  Claude audits plan checkboxes for accuracy (iterate until clean),
  then converts each checkbox to a Claude Code task via TaskCreate,
  with dependencies set via addBlockedBy.

EXECUTION + CONVERGENCE PHASE (preferred — single "big bang" prompt):
  "run the build plan exactly as planned, including creating claude
   code tasks planning the entire thing out, including documentation
   changes, code audits, documentation audits, and then after you
   run it, audit and fix any issues in the code, and in the docs,
   over and over, until you report there aren't any left to fix."
  → Claude: audit checkboxes → convert to tasks → execute from tasks →
    audit code → audit docs → fix → repeat → converge

  (Or if you want to be precise about convergence criterion:)
  "...until two consecutive independent audit passes find zero issues
   across all files."

MID-EXECUTION CHECKPOINT:
  "audit your list of checkboxes for following the build plan against
   the build plan and resolve any gaps or errors."
  → Claude audits progress tracker honesty (coverage claims, deferred items)

POST-EXECUTION AUDIT PHASE (if not using big bang prompt):
  "audit and fix any issues in the code, and in the docs"
  → Claude audits code + docs against plan, fixes issues

  "do it again"
  → Repeat until convergence (two consecutive clean passes)

POST-EXECUTION PATTERNS UPDATE:
  "update the DEVELOPMENT_PATTERNS.md file with patterns to this point,
   including prompts"
  → Claude reviews everything that happened, captures new patterns
```

**Key insight:** The viability question is the most powerful quality gate. It forces Claude to think about execution, not just documentation. "Will this succeed?" is fundamentally different from "is this consistent?" — it requires simulating the actual build process and checking the actual environment.

### 0F. Audit Methodology for Plans

When auditing a plan document, check:

1. **Internal consistency** — Do phases reference each other correctly? Does Phase 4 use the data structures defined in Phase 1?
2. **Completeness** — Is every file listed in File Inventory? Is every function in the function tables tested in the test lists?
3. **Checkbox accuracy** — Run `grep -c "^\- \[ \]" BUILD_PLAN.md` and verify it matches Definition of Done.
4. **Dependency correctness** — Does the dependency graph match the phase prerequisites? Can Phase 3 really run in parallel?
5. **Missing edge cases** — What happens on crash? On re-run? On bad data? Is re-extraction defined? Is recovery defined?
6. **Constraint compliance** — Does every phase respect the constraints in the header? (No API calls, TDD everywhere, etc.)
7. **Build order validity** — Can you follow the numbered build order within each phase and arrive at working code? Are steps in the right order?
8. **Cross-reference accuracy** — When Phase 4 says "modifies mem_pipeline.py" does Phase 4's test list include tests for those modifications?

**Two-pass minimum.** First pass finds obvious gaps. Fixing those gaps introduces new gaps (updating a function signature without updating its test list). Second pass catches these. Each pass should produce a numbered list of specific, actionable fixes.

**Findings format:** List gaps as numbered items with specific, actionable fixes:

```markdown
**Gap 3:** Phase 2 test list doesn't include test for re-extraction workflow.
**Fix:** Add `test_re_extract_creates_new_unit_with_version_suffix` and 4 related tests to Section 2G.
```

Apply all fixes, re-count checkboxes, verify totals match.

### 0G. When Plans Must Be Updated

The plan document is living during execution. Update it when:

- A checkbox is completed (mark it `[x]`)
- A new requirement is discovered during building
- A technical approach changes (update the phase, not just the code)
- An assumption is invalidated

Do NOT update the plan for ephemeral state (current session progress, temp file locations). The plan describes WHAT and HOW, not WHERE WE ARE RIGHT NOW — that's what the progress tracker checkboxes are for.

### 0H. "Do It Again" — Forcing Convergence Through Repetition

Bryan's most powerful quality tool is simply saying "do it again." After Claude audits and fixes issues, Bryan demands a re-audit. This is not distrust — it's the recognition that:

1. **Fixes introduce new issues.** Updating a function signature in Phase 1 without updating the Phase 4 test list creates a new gap. The first audit can't catch what doesn't exist yet.
2. **Each pass finds different classes of problems.** Round 1 catches structural issues. Round 2 catches logical gaps exposed by structural fixes. Round 3 catches consistency/redundancy problems introduced by the first two rounds.
3. **Convergence is empirically fast.** In our experience: 10 → 8 → 7 → 14 (broader scope) → 8 → 0 for document audits (5 passes, 47 issues). Adding viability passes: 2 + 5 = 7 more environment issues across 2 passes. For post-execution code+doc audits: 13 → 9 → 3 → 0 (4 passes, 25 issues). Grand total across all audit types: 11 passes, 79 issues. The curve drops to zero within 2-4 passes per scope level.

**Bryan's exact pattern:**
```
"audit phase 6 for issues"     → 10 issues found, fixed
"now do it again"               → 8 issues found, fixed
"now do it again"               → 7 issues found, fixed
"now audit the entire plan"     → 14 cross-phase issues found, fixed
"audit it again"                → 8 more issues found, fixed
```

**Rule:** Never push back on "do it again." Each pass genuinely finds real problems. The marginal cost of an audit pass is low; the marginal value of catching a latent issue before execution is high.

### 0I. Focused vs. Full-Plan Audits

Two different audit scopes produce different types of findings:

| Scope | What it catches | Example |
|-------|----------------|---------|
| **Focused audit** (single phase) | Internal consistency, dead fields, redundant steps, schema issues | Phase 6 audit found `fix_verified` field referenced in schema but never checked by any logic |
| **Full-plan audit** (all phases) | Cross-phase integration, stale references, state machine inconsistencies | Phase 4 referenced `imported` as terminal state when Phase 1 had changed it to `review_queued` |

**Pattern:** Audit the new/changed phase first (2-3 focused passes), THEN audit the full plan (1-2 passes). Focused passes clean up internal mess so the full-plan pass can focus on integration issues.

**The dominant bug class in full-plan audits is stale cross-phase references.** When one phase changes a name, constant, state, or signature, every other phase that references it must be updated. In our experience, a single rename (e.g., `imported` → `review_queued` as terminal state) propagated into 5+ fixes across the plan.

### 0J. Three Types of Auditing (Taxonomy)

We discovered three fundamentally different audit types, each catching different classes of bugs. All three are required before execution.

| Type | What it checks | What it catches | Example prompt |
|------|---------------|----------------|----------------|
| **Document audit** (focused) | Single phase internal consistency | Dead fields, redundant steps, schema errors, wrong sequences | "audit phase 6 for issues" |
| **Document audit** (full-plan) | Cross-phase consistency | Stale references, state machine divergence, dependency errors | "audit the entire plan for issues" |
| **Environment audit** (viability) | Plan vs. actual machine state | Missing deps, wrong CLI tools, non-existent config files, broken import paths | "will this plan succeed?" |

**Critical insight:** Document auditing — no matter how many passes — will never catch environment issues. The plan can be perfectly internally consistent and still fail because `pytest-bdd` isn't installed, or `uv` doesn't exist on the machine, or `memory/` isn't importable as a package. The viability question forces environment verification.

**Issue counts by audit type in our experience:**

| Audit type | Passes | Issues found |
|------------|--------|-------------|
| Focused (Phase 6) | 3 | 10 + 8 + 7 = 25 |
| Full-plan | 2 | 14 + 8 = 22 |
| Environment (viability) | 2 | 2 + 5 = 7 |
| **Pre-execution total** | **7** | **54** |
| Post-execution code audit | 2 | 13 + 9 = 22 |
| Post-execution doc audit | 1 | 3 |
| Post-execution clean pass | 1 | 0 (convergence confirmed) |
| **Post-execution total** | **4** | **25** |
| **Grand total** | **11** | **79** |

The viability passes found fewer issues but they were all execution-blockers — things that would have caused immediate failure during Phase 0 step 1. The post-execution code audit found a different class of bugs entirely (connection leaks, placeholder code, phantom references) that no amount of plan auditing could have caught — they only exist after code is written.

---

## 1. Plan Structure

### 1A. Plan Document Format

Every development round starts with a plan document (`BUILD_PLAN.md` or similar). The plan is the single source of truth for what to build, how to build it, and whether it's done.

**Required sections:**

| Section | Purpose |
|---------|---------|
| **Header** | Created date, last updated (with change summary), status (see below), goal (1-2 sentences), constraints, rules |
| **Architecture Overview** | Before/after diagram showing current flow vs target flow. ASCII art, not prose. |
| **Phases** | Numbered, ordered. Each phase has: purpose statement, data structures, function signatures, test lists, build order. |
| **Phase Ordering & Dependencies** | Explicit dependency graph. Which phases can run in parallel. ASCII timeline. |
| **Session Execution Protocol** | Step-by-step instructions for any session picking up this plan. |
| **Progress Tracker** | Checkbox list, one per atomic step. This is what gets checked off. |
| **File Inventory** | Every new file the plan creates, with phase and purpose. |
| **Risks & Mitigations** | Table of what could go wrong and how we handle it. |
| **Definition of Done** | Numbered list of acceptance criteria. Plan is NOT complete until all are met. |

### 1A-2. Plan Status as a Living Field

The plan header status is NOT a static label — it's a living field that must be updated as execution proceeds. Valid statuses:

| Status | Meaning |
|--------|---------|
| `NOT STARTED` | No code exists for this plan |
| `PARTIALLY BUILT (annotation)` | Some phases have existing code. Annotation describes what exists. Example: `PARTIALLY BUILT (Phases 1-2 have existing production code + 75 passing tests)` |
| `IN PROGRESS` | Active execution in current or recent session |
| `COMPLETE` | All phases done, all tests passing, coverage verified |

**Why this matters:** CHAT_PROVENANCE_PLAN.md said "NOT STARTED" when Phases 1-2 were 80% built from a prior session. A new session saw "NOT STARTED" and nearly re-created all existing code. The annotation on `PARTIALLY BUILT` tells future sessions exactly what exists, so they can verify against spec rather than build from scratch.

**Rule:** After any session that builds code for a plan, update the status field before ending. If viability assessment discovers existing code, update the status BEFORE execution begins.

### 1B. Plan Constraints

State constraints up front in the header. These are non-negotiable rules that override all other decisions:

```markdown
**Constraint:** All LLM work stays in Claude Code (Pro Max). No API calls.
**Rule:** TDD/BDD for everything. Tests before code. 100% coverage.
```

Constraints flow from user decisions. If Bryan says "no API calls," that's a constraint. If CLAUDE.md says "TDD for everything," that's a rule. Don't bury these in phase details — they go in the header where every session sees them first.

### 1C. Phase Design

Each phase must be:

- **Independent enough to resume** — if a session crashes mid-phase, the next session can pick up from the last checked checkbox
- **Ordered by dependency** — Phase N's prerequisites are explicit ("needs Phase M")
- **Testable in isolation** — each phase has its own test suite that passes independently
- **Purposeful** — every phase has a one-sentence purpose statement explaining WHY, not just WHAT

Phase internal structure:

```markdown
## Phase N: Name

**Purpose:** One sentence explaining why this phase exists.

**Prerequisite:** Phase M (specific dependency).

### NA. Data structures / schema changes
### NB. Functions (table: name, purpose)
### NC. Tests (write FIRST — full test list)
### ND. Build order (numbered, RED→GREEN sequence)
```

### 1D. Sub-Session Breakdown for Large Phases

When a phase integrates multiple upstream modules (like Phase 4 wiring together Phases 0-3), context pressure can crash the session. Break large phases into sub-sessions:

```markdown
**Sub-session 4A:** Migration + stage gates (steps 1-4)
  Needs: work_unit.py, review_queue.py, pipeline_state.json
  Checkpoint after: step 4

**Sub-session 4B:** Commands (steps 5-8)
  Needs: work_unit.py, source_chunker.py, mem_pipeline.py
  Checkpoint after: step 8

**Sub-session 4C:** Recovery + integration (steps 9-14)
  Needs: session_check.py, work_unit.py, review_queue.py
  Checkpoint after: step 13
```

Each sub-session:
- Lists exactly which modules it needs to read (so the session doesn't load everything)
- Has a checkpoint instruction at the end
- Can be completed independently if context gets heavy

**When to use:** Any phase that touches 4+ modules, or any phase whose build order has 10+ steps. If in doubt, break it up — the cost of sub-sessions is near zero, the cost of a context crash mid-integration is a full restart.

### 1E. Interface Reference Sections

When a phase must call functions from modules built in earlier phases, document the current function signatures **inline in the plan**. This prevents future sessions from needing to re-read source files just to know what parameters a function takes.

```markdown
### 4-REF. Existing interfaces Phase 4 must integrate with

**`memory/mem_pipeline.py`:**
extract_corpus(person, entries_path=None) → str
build_synthesis_input(person, level, period, entries_path=None) → str
render_stage_prompt(person, stage, dimension) → str

**`memory/add_entry.py`:**
add_to_json(entries: list) → int
rebuild_db_from_json() → int
```

**When to add:** Any phase that modifies or calls functions defined in other files. Especially important for the integration phase (Phase 4 in our plan) where everything comes together.

**What to include:** Function name, parameters with types, return type, and a one-line description of behavior. Also document current implementations if the plan is modifying them (see Section 3D).

### 1F. Progress Tracker Conventions

- One checkbox per atomic action (not "implement module" — instead "implement function_a", "implement function_b")
- Checkbox IDs match phase numbers: `0.1`, `0.2`, `1.1`, `1.5.3`, etc.
- Last two checkboxes of every phase are always "All tests GREEN" and "Coverage check ≥ 100%"
- Total checkbox count is stated in Definition of Done and kept accurate
- Checkboxes are checked by editing the plan file directly during the session

---

## 2. TDD/BDD Methodology

### 2A. Build Order (always)

```
1. Plan the full test suite FIRST
2. Write BDD feature files for user-facing workflows
3. Write ALL unit tests (RED — they must fail)
4. Implement minimum code to pass tests (GREEN)
5. Refactor
6. Coverage check ≥ 100%
```

Never write production code before its tests exist. If context/time forces a tradeoff, tell Bryan explicitly and let him decide.

### 2B. Test File Naming

```
memory/tests/test_<module_name>.py          — unit tests
memory/tests/test_pipeline_integration.py   — integration tests
memory/tests/features/<workflow>.feature    — BDD feature files (Gherkin)
memory/tests/steps/test_<workflow>_steps.py — BDD step definitions (pytest-bdd)
memory/tests/conftest.py                    — shared fixtures
```

### 2C. Test List Format in Plans

List every test by name in the plan, grouped by concern:

```markdown
### Tests (write FIRST)

**File:** `memory/tests/test_work_unit.py`

# Lifecycle
test_create_unit_writes_metadata
test_create_unit_rejects_duplicate
test_complete_unit_transitions_state

# Crash safety
test_checkpoint_writes_entries_atomically
test_checkpoint_survives_simulated_crash

# Import
test_import_unit_backs_up_entries_json_first
test_import_unit_calls_add_entry
```

This is the contract. If a test isn't in the plan, it's optional. If it IS in the plan, it must exist and pass.

### 2D. Shared Test Fixtures

Create `conftest.py` in Phase 0 with fixtures used across all test files:

- Temp entries.json (isolated, doesn't touch real data)
- Temp DB (in-memory or tmpdir SQLite)
- Sample entries (representative data for testing)
- Sample reviews.json
- Any other shared setup

This prevents test files from duplicating setup logic and ensures tests are isolated from production data.

### 2E. BDD Feature Files

**Framework:** `pytest-bdd` — integrates with existing pytest runner and pytest-cov for coverage. Install: `pip install pytest-bdd`.

Write Gherkin feature files for user-facing workflows. These describe BEHAVIOR, not implementation:

```gherkin
Feature: Work unit lifecycle
  Scenario: Create and checkpoint a work unit
    Given a fresh work queue directory
    When I create a work unit for "danielle" "imessage" "2017-06"
    Then a metadata file exists in work_queue/
    And the unit state is "created"
```

**File layout:**
```
memory/tests/features/           — .feature files (Gherkin)
memory/tests/steps/              — step definition files (Python)
  test_work_unit_steps.py        — steps for work_unit_lifecycle.feature
  test_review_steps.py           — steps for review_workflow.feature
  test_pipeline_steps.py         — steps for pipeline_processing.feature
```

One feature file per major workflow. One step definition file per feature file. Step files are regular pytest files that `pytest-bdd` discovers automatically. They run alongside unit tests in the same `pytest` invocation.

---

## 3. Data Safety Patterns

### 3A. Atomic Writes

**Every write to a critical JSON file must use write-then-rename:**

```python
def atomic_write(path, data):
    tmp = path.with_suffix('.tmp')
    tmp.write_text(json.dumps(data, indent=2))
    tmp.rename(path)  # atomic on same filesystem
```

Critical files: `entries.json`, `relationships.json`, `reviews.json`, work unit metadata files.

**Why:** A crash during `write_text()` or `json.dump()` can truncate the file to 0 bytes or leave partial JSON. `rename()` is atomic on POSIX — the old file is replaced in one operation.

### 3B. Rotating Backups

Before every batch import to entries.json:

```python
def backup_entries(max_backups=5):
    backup_dir = PERSIST_DIR / "backups"
    backup_dir.mkdir(exist_ok=True)
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    dest = backup_dir / f"entries_{ts}.json"
    shutil.copy2(JSON_PATH, dest)
    backups = sorted(backup_dir.glob("entries_*.json"))
    for old in backups[:-max_backups]:
        old.unlink()
    return dest
```

Keep last 5. Prune oldest. This is the safety net for atomic write failures, corruption, or bad imports.

### 3C. Dual-Write for Indexed Data

When data lives in both a JSON file (authoritative) and SQLite (read-optimized index):

- **Write to JSON first** (atomic write)
- **Then write to SQLite**
- **On DB rebuild, load from JSON** — never trust SQLite as authoritative

This pattern applies to: entries (entries.json → memory.db), reviews (reviews.json → entry_reviews table), relationships (relationships.json → memory.db).

### 3D. Document Current Implementation Before Modifying

When the plan modifies an existing function, document **what it looks like now** — not just what it should look like after. Future sessions may not have context about the baseline.

```markdown
### 0D. Atomic _save_reviews

**Current implementation** (app/memory_db.py lines 189-192):
def _save_reviews(reviews_path, data):
    """Atomically save reviews.json."""  # docstring is aspirational, NOT actual
    with open(reviews_path, 'w') as f:
        json.dump(data, f, indent=2)

**Callers (2 total):**
- save_entry_review() line 206
- submit_month_review() line 234

**Fix — atomic write, same signature:**
def _save_reviews(reviews_path, data):
    tmp = reviews_path + '.tmp'
    with open(tmp, 'w') as f:
        json.dump(data, f, indent=2)
    os.rename(tmp, reviews_path)
```

This prevents two failure modes:
1. **"The plan describes a delta but not the original"** — a session that didn't read the file can't apply the fix correctly
2. **"The fix changes the signature unknowingly"** — by documenting callers, we confirm the fix is safe (no caller changes needed)

### 3E. Crash-Resilient State Files

Any long-running process that can be interrupted must write state to disk:

```python
{
    "status": "in_progress",      # in_progress | complete | converged
    "last_action": "...",         # what was done last
    "checkpoint": { ... },        # enough info to resume
    "created_at": "ISO timestamp",
    "updated_at": "ISO timestamp"
}
```

Update the state file after every significant action. On resume, read the state file and pick up where it left off. The state file IS the resume protocol.

---

## 4. Crash Resilience & Idempotency

### 4A. Assume the Session Will Crash

Every session has crashed or run out of context. Treat every interaction as potentially the last one:

- Write state to disk BEFORE doing the next thing
- Log to memory DB after every substantive action
- Checkpoint work in progress to durable storage
- Never hold important results only in conversation context

### 4B. Idempotent Operations

Every operation in the plan must be safe to run twice:

- Creating a work unit that already exists → error (not duplicate)
- Importing entries that already exist → dedup by UID (skip duplicates)
- Running backup before import → harmless if run twice
- Checking a checkbox that's already checked → no-op
- Rebuilding DB from JSON → always produces same result from same input

### 4C. Session Execution Protocol

Every session that works on a plan:

1. Run `TaskList` — find next available task (pending, not blocked)
2. `TaskUpdate` → `in_progress`
3. Do the work
4. `TaskUpdate` → `completed`
5. Mark corresponding checkbox in plan file (dual tracking)
6. Log to memory DB
7. If context is getting heavy, checkpoint and stop

**Why dual tracking (tasks + checkboxes):** Tasks give Bryan real-time progress visibility and persist across sessions without reading the plan. Checkboxes keep the plan file self-contained for archival. Both are updated — tasks are the primary execution tracker, checkboxes are the audit trail.

**On resume:** `TaskList` shows exactly what's done, in progress, and pending. No need to grep checkboxes or re-read the plan.

### 4C-2. Pre-Execution: Checkbox Audit → Task Conversion

Before executing ANY plan, the checkboxes must be converted to Claude Code tasks. The sequence:

1. **Audit checkboxes** — verify every plan step has a checkbox, descriptions are actionable, dependencies are correct. Iterate until clean.
2. **Convert to tasks** — one `TaskCreate` per checkbox. Include plan context in the description. Set `addBlockedBy` for ordering.
3. **Execute from tasks** — work from `TaskList`, not from scrolling the plan file.

This was a missing pattern in the first BUILD_PLAN execution — checkboxes existed but were never converted to tasks, losing the real-time progress tracking that tasks provide.

### 4D. Work Unit Pattern

For any multi-step processing that produces outputs:

```
create → checkpoint → checkpoint → ... → complete → import → review
```

Each step writes to disk. A crash at any point loses at most one step of work. The work unit metadata file tracks state, so recovery is just "read the state, resume from last checkpoint."

Work units live in a dedicated directory (`memory/work_queue/`) and are self-describing — the metadata file contains everything needed to understand and resume the unit.

---

## 5. Review & Correction Patterns

### 5A. Nothing Is Assumed Correct

All generated entries enter as `pending_review`. Bryan approves, corrects, or rejects. Only approved entries feed into downstream analysis.

### 5B. Correction Propagation

When a fact is corrected, its derived entries may be wrong:

```
fact ──derived_from──→ inference ──derived_from──→ synthesis
```

Correction propagation walks `derived_from` relationships to flag affected entries for re-review. **Depth limit: 2 levels.** Beyond that, flag but don't auto-propagate.

### 5C. Rejection Semantics

Rejected entries are NOT deleted. They stay in the database with `status=rejected` for audit trail. They are excluded from corpus assembly and synthesis input — invisible to analysis but preserved for history.

### 5D. Stage Gates

Pipeline stages are gated on review completion:

- Extraction → always allowed
- Cross-cutting → blocked until ALL extraction entries are approved
- Synthesis → blocked until ALL cross-cutting entries are approved

This prevents building analysis on unverified facts.

### 5E. Re-extraction Workflow

When >30% of a batch is rejected, create a versioned work unit (`_v2`) that includes:
- Approved entries from the old batch (don't re-extract what's correct)
- Rejection reasons (so the same mistakes aren't repeated)
- Original source material

---

## 6. State Machine Design

### 6A. Name Every State Explicitly

When a system has states, define them as an explicit list in one place:

```python
UNIT_STATES = ["created", "in_progress", "entries_written", "imported", "review_queued", "superseded"]
```

Any code that checks state must reference this list. Any rename must update the list AND grep the entire codebase for the old name.

### 6B. Bridge Between State Machines

When two systems have their own state machines, the bridge between them must be explicitly defined. In our case:

- **Work unit states:** `created → in_progress → entries_written → imported → review_queued`
- **Entry review statuses:** `pending_review → approved | corrected | rejected | needs_re_review`

The bridge is `_unit_fully_approved(unit)` — it checks BOTH systems:
- Unit state must be `review_queued` (work unit side)
- Every entry UID in the unit must have `review_status='approved'` (entry review side)

**Pattern:** When two state machines interact, write a single bridge function that checks both. Don't scatter cross-system checks through the codebase. Stage gates, for example, only call `_unit_fully_approved()` — they don't independently check unit states and review statuses.

### 6C. Terminal vs. Non-Terminal States

Clearly distinguish terminal states (no further transitions) from non-terminal states:

- **Terminal:** `review_queued` (success path), `superseded` (replacement path)
- **Non-terminal:** everything else

Terminal states are important for convergence checks: "are all units done?" means "are all units in a terminal state?" If you rename a terminal state (as we did, `imported` → `review_queued`), every convergence/gate check must be updated.

---

## 7. Code Audit & Convergence

### 7A. Audit at Completion

Every plan ends with a code audit phase. This is NOT optional. The plan is not complete until the audit converges.

### 7B. Eight Audit Dimensions

| # | Dimension | Question |
|---|-----------|----------|
| 1 | Plan alignment | Does the code match the plan's specifications? |
| 2 | Correctness | Are there logic errors, race conditions, edge cases? |
| 3 | Test coverage | Every public function tested? Coverage ≥ 100%? |
| 4 | BDD alignment | Do feature files match implemented behavior? |
| 5 | Cross-module integration | Do modules call each other correctly? Data structures match? |
| 6 | Data safety | All critical writes atomic? Backups before imports? |
| 7 | Documentation compliance | Do docs accurately describe the code? |
| 8 | Regression | Do ALL existing tests still pass? |

### 7C. Convergence Loop

```
Round N:
  1. Audit every file → log issues (phase: "auditing")
  2. Fix every issue → verify each fix (phase: "fixing")
     — if a fix breaks something, log it as a new issue in THIS round
     — if coverage drops below 100%, log it and fix immediately
  3. Re-run full test suite + coverage (phase: "verifying")
  4. If issues found → Round N+1
  5. If zero issues → CONVERGED → DONE
```

State tracked in a JSON file (`code_audit_state.json`) with per-round, per-file, per-issue granularity. The state file itself must use **atomic write-then-rename** — it's updated frequently and a crash mid-write loses all audit progress.

Each round has a `phase` field (`"auditing"` / `"fixing"` / `"verifying"`) so crash recovery knows exactly where to resume. Without this, "completed is null" is ambiguous — are we still auditing files or fixing issues?

**Progress tracker note:** Checkboxes represent the FINAL round only. Per-round progress lives in the state file. Don't check boxes until convergence.

**Safety valve:** Max 5 rounds. If still finding issues at round 5, stop and triage with Bryan.

### 7D. Issue Tracking Format

```json
{
    "id": "R1-001",
    "file": "memory/work_unit.py",
    "dimension": "plan_alignment",
    "severity": "high",
    "description": "create_unit() missing stage_type parameter",
    "fix_applied": false
}
```

Every issue gets a unique ID (`R{round}-{sequence}`), a dimension, a severity, and a fix tracking flag. Additional dimension `"fix_regression"` for issues introduced by fixes within the same round. Don't add redundant derived fields (like `fix_verified` or `issues_fixed`) — derive them from the issue list.

### 7E. Audit the Audit (Recursive Quality Checking)

The audit phase itself must be audited before the plan is executed — at least twice. Each pass finds different classes of issues:

**Phase 6 focused audits (3 rounds):**

| Round | Issues | Class of problems |
|-------|--------|-------------------|
| 1 | 10 | Structural — progress tracker can't model loops, state file not atomic, no phase distinction for crash recovery, missing files, redundant fields |
| 2 | 8 | Logical — state machine transitions incomplete, convergence metric confused by fix-regressions, audit ordering undefined, dead schema fields |
| 3 | 7 | Consistency — duplicate instructions from prior fixes, schema example creates fake issue, per-fix DB logging causes 30 rebuilds, vacuous checkboxes |

**Full-plan audits (2 rounds):**

| Round | Issues | Class of problems |
|-------|--------|-------------------|
| 1 | 14 | Cross-phase integration — terminal state renamed in Phase 1 but 5 Phase 4 references used old name, `list_units` missing params, dependency graph wrong, build order invalid |
| 2 | 8 | Stale references — `_unit_fully_approved` untested, `superseded` missing from state list, `imported`/`review_queued` confusion in 3 more places |

**Total: 5 passes, 47 issues, zero remaining.** (This counts document audits only. Including viability passes: 7 passes, 54 issues total — see Section 0J for the full breakdown.)

**Pattern:** First pass catches structural gaps. Second pass catches logical gaps exposed by the structural fixes. Third pass catches consistency/redundancy problems introduced by the first two rounds. Full-plan passes catch cross-phase integration issues invisible to single-phase audits. Each pass produces diminishing returns but non-zero findings. Keep auditing until a round finds nothing.

**Empirical rule of thumb:** Budget 2-3 focused passes per new/modified phase, then 1-2 full-plan passes. Total issues per pass drops roughly 10 → 8 → 7 for focused, 14 → 8 for full-plan.

### 7D. Code Audit Convergence (Post-Execution Empirical Data)

The plan audit and the code audit are different beasts. Plan audits check document consistency. Code audits check whether the built code matches the plan AND is correct. Here's what actually happened when we ran the code audit after building 96 checkboxes:

| Round | Scope | Issues Found | Classes of Problems |
|-------|-------|-------------|---------------------|
| R1 | Code only | 13 | Placeholder `pass` in loop body (R1-001), missing planned function (R1-002), non-atomic writes (R1-003), missing BDD step definitions (R1-004), 9 deferred items |
| R2 | Code only | 9 | Connection leaks in 5 SQLite functions (missing try/finally), phantom CLI commands in usage strings, migration copy-vs-rename bug |
| R3 | Code + Docs | 3 (docs only) | Stale hardcoded counts in DATABASE_SCHEMA.md (5 places), PROJECT_INDEX.md (2 places), phantom command references in CLAUDE.md |
| R4 | Code + Docs | 0 | **CLEAN PASS — convergence confirmed** |

**Key finding:** Code audit convergence required 4 rounds, not the 2-3 that plan audits needed. The dominant bug class in R2 was connection leaks — a cross-cutting concern invisible to plan-level auditing. R3 revealed that documentation auditing must happen alongside code auditing, not separately.

### 7E. Honest Checkpoint Annotation

During execution, the progress tracker checkboxes can become dishonest. Common failure mode: checking a box when the work is "mostly done" or coverage is "close enough." This happened — coverage was marked as meeting the 100% target when two modules were at 82% and 81%.

**Rule:** When marking a checkbox complete, annotate honestly if there are caveats:

```markdown
- [x] 6.5 Coverage check ≥ 100% — source_chunker 90%, pipeline_integration 92% (raised from 82%/81% in R2)
```

Better: maintain a **Known Gaps** section at the bottom of the plan for deferred items that are tracked but not blocking:

```markdown
## Known Gaps (deferred from Phase 6 audit)
1. **R1-005 (medium):** CLI commands deferred — core infrastructure present but CLI wiring incomplete
2. **R1-006 (low):** `_plan_quip_units()` and `_plan_blog_units()` are stubs
```

This pattern prevents the "plan says COMPLETE but 11 things are unfinished" problem. The plan IS complete — and here's exactly what's deferred and why.

### 7F. Agent-Based Parallel Auditing

For code audits across many files, use parallel agents:

```
Agent 1: Audit production code (9 files) against plan specs
Agent 2: Audit documentation (7 files) against code reality
Agent 3: Run full test suite + coverage report
```

Each agent works independently with its own context. Results are synthesized in the main conversation. This prevents context blowup from loading 16+ files into one conversation.

**When to parallelize:** When auditing 5+ files, or when code audit and doc audit are independent (they usually are). When NOT to parallelize: when fixing issues that span multiple files (fixes must be sequential to avoid conflicts).

### 7G. Coverage Gap Resolution

When coverage is below target for specific modules, don't accept it — write targeted tests:

| Gap | Root Cause | Test Added | Coverage Impact |
|-----|-----------|------------|-----------------|
| source_chunker 82% | `_split_by_week()` never triggered | 2100-message test that triggers weekly splitting | 82% → 100% |
| source_chunker 82% | Missing file path not tested | Remove chat_messages.json, verify empty result | +2% |
| pipeline_integration 81% | `recover()` untested | Two tests for incomplete units and all return categories | 81% → 88% |
| pipeline_integration 81% | Stage gate approval flow untested | Full flow: create→checkpoint→complete→review→approve→gate opens | 88% → 92% |
| pipeline_integration 81% | Edge cases untested | No entries file, empty entries, wrong state, unknown stage, re-migration | +4% |

**Pattern:** Read the coverage report, identify which functions/branches are uncovered, write the minimal tests that exercise those paths. Don't write redundant tests for already-covered code.

### 7H. Coverage by Coincidence

A test can pass, look correct, and appear to cover a code path — while actually exercising a completely different branch. This happened: `test_returns_none_on_struct_error` was written to hit an `except (IndexError, struct.error)` handler (lines 82-83), but the test input actually triggered an early-return guard (line 64). The test passed. The name suggested coverage. But `--cov-report=term-missing` showed lines 82-83 still uncovered.

**Rule:** Never trust test names or intent as proof of coverage. The ONLY source of truth is `--cov-report=term-missing` output. After writing coverage-closing tests, re-run coverage and verify the specific line numbers disappeared from the "Missing" column.

**Diagnostic pattern:**
1. Write the test targeting specific uncovered lines
2. Run `pytest --cov=module --cov-report=term-missing`
3. Check: are those exact line numbers still in "Missing"?
4. If yes: your test hit a different branch. Read the code path more carefully, trace what your test input actually does step-by-step.

This is a variant of checkbox dishonesty — you believe you've covered the code, but you haven't verified it empirically.

### 7I. Deferred Item Tracking

Not everything found in an audit needs to be fixed immediately. The triage:

| Severity | Action | Example |
|----------|--------|---------|
| **HIGH** | Fix now | `pass` placeholder in production code path |
| **MEDIUM** | Fix if time, otherwise defer with tracking | Missing CLI wiring for planned commands |
| **LOW** | Defer with tracking | Stub functions for future source types |

Deferred items get:
1. An issue ID (R1-005, R1-006, etc.)
2. A severity
3. A description in the Known Gaps section of the plan
4. An entry in `code_audit_state.json` for machine-readable tracking

This is NOT "sweeping things under the rug." It's explicit, tracked deferral with a clear paper trail.

---

## 8. Documentation Patterns

### 8A. Documentation as Audit Target

Documentation is not aspirational — it describes what EXISTS. After building, documentation is updated to match code, then audited for accuracy:

```
For each documented function:
  1. Does the function exist? (name match)
  2. Does the signature match? (parameters)
  3. Does the description match behavior? (read implementation)
  4. Are there undocumented functions? (gap check)
  5. Are there documented functions that don't exist? (stale check)
```

### 8B. No Stale State in Markdown

Never cache counts, statuses, or computed values in markdown files. The database is the truth. Documentation describes structure and behavior, not current state.

### 8C. Documentation Update Follows Code

The plan includes a documentation phase AFTER all code is written. Don't update docs as you build — wait until the code is stable, then update everything at once. This prevents docs from describing intermediate states.

### 8D. Documentation Audit Dimensions

When auditing documentation files post-build, check each file against these dimensions:

| Dimension | What to Check |
|-----------|--------------|
| **Accuracy** | Do function signatures match code? Do return types match? |
| **Completeness** | Are all new modules documented? All new functions? |
| **Staleness** | Are there hardcoded counts that should be live queries? |
| **Phantom references** | Do documented CLI commands/functions actually exist? |
| **Architecture alignment** | Does the doc describe the current architecture, not the pre-build one? |

**Empirical finding:** The dominant doc bug class is **phantom references** — commands, functions, or parameters that were planned but never implemented, or were renamed during development. In our audit: `propagate <uid>` command documented in 3 files but never built; `batch` subcommand listed in usage string with no handler.

### 8E. Replace Hardcoded Counts with Live Queries

Documentation should never contain lines like "4,483 entries" or "113K messages." These go stale instantly. Replace with query directives:

```markdown
<!-- Instead of: -->
The database contains 4,483 entries across 12 types.

<!-- Use: -->
Entry count: `python3 -c "import json; print(len(json.load(open('memory/entries.json'))))"`
```

In our audit, DATABASE_SCHEMA.md had "4,483" hardcoded in 5 places and PROJECT_INDEX.md had "4,485" in 2 places. All were stale by the time the audit ran.

---

## 9. Communication Patterns

### 9A. Max Chatty Mode

During development: narrate what you're doing and why. Think out loud. Don't silently do things. Flag concerns proactively.

### 9B. Surface Findings

After any batch import of non-meta entries, present a 500+ word narrative synthesis to Bryan. Entry counts are not findings. Bryan wants the insights, not the numbers.

### 9C. Persist Narratives

Any analytical prose written for Bryan must be logged as a reflection entry BEFORE being displayed. Sessions crash. Compaction erases context. The narrative is the work product — treat it as durable, not ephemeral.

### 9D. Log Everything

After every substantive interaction, log to the memory DB. One command:

```bash
python3 memory/add_entry.py --type meta --tags session_log --date "YYYY-MM-DD" \
  --content "Session: [1-2 sentence summary]" --source "session"
```

---

## 10. Architecture Decisions

### 10A. LLM Work Stays in Claude Code

All analytical work (extraction, cross-cutting, synthesis) happens in Claude Code conversation using Pro Max. The pipeline is scaffolding — it renders prompts, tracks state, deduplicates. It does NOT call any LLM API.

### 10B. JSON as Authoritative Store

`entries.json`, `relationships.json`, `reviews.json` are the ground truth. SQLite is a read-optimized index rebuilt from JSON. If they diverge, JSON wins.

### 10C. Collision-Safe Temp Outputs

Use collision-safe directories for temp outputs: `/tmp/pp_pipeline_<run_id>/`. Never write to shared temp paths that could collide across sessions.

### 10D. File Locking for Concurrent Safety

`import-results` uses `fcntl.LOCK_EX` for concurrent safety. Any operation that modifies shared files must either use file locking or be designed to run exclusively.

---

## 11. Anti-Patterns (Things That Have Gone Wrong)

| Anti-Pattern | What Happened | Rule |
|-------------|---------------|------|
| Non-atomic writes | Lost iMessage processing — file corruption during write | Always write-then-rename for critical files |
| No backups | No way to recover from bad imports or corruption | Rotating backups before every batch import |
| Silently skipping tests | "Just a quick script" led to bugs | TDD for everything, Bryan decides exceptions |
| Deferred logging | Session crashed, work lost because it wasn't logged yet | Log BEFORE moving to next task |
| Stale markdown state | Documentation said 2,780 entries when DB had 4,493 | Query live, never cache counts |
| DB rebuild drops data | `rebuild_db_from_json()` didn't create review tables | All tables must be created AND populated during rebuild |
| Assumed entry correctness | Imported entries fed straight into analysis with no review | Everything enters as `pending_review` |
| Infinite propagation | Correction could cascade through entire entry graph | 2-level depth limit on propagation |
| Plan checkbox mismatch | Plan said 63 items but had 82 | Count with `grep`, verify after every edit |
| Missing crash recovery | No protocol for resuming after session death | State files + progress tracker = resume protocol |
| State machine name divergence | Phase 1 renamed terminal state to `review_queued` but Phase 4 still said `imported` in 5 places | When renaming states/constants, grep the ENTIRE plan for the old name. One rename = N fixes across N phases. |
| Plan describes delta without baseline | Plan said "make _save_reviews atomic" but no session knew what _save_reviews currently looked like | Document current implementation inline: code, callers, line numbers. Future sessions need the baseline to apply the delta. |
| Single-phase audit only | Audited Phase 6 three times but never checked cross-phase integration | After focused audits converge, always do at least one full-plan audit. Cross-phase stale references are invisible to single-phase audits. |
| Undocumented context pressure | Phase 4 needed to read 6+ modules but no guidance on managing context | Break large integration phases into sub-sessions. List which modules each sub-session needs. |
| Aspirational docstrings | `_save_reviews` docstring said "Atomically save" but the implementation was not atomic | Treat docstrings as claims to verify, not as documentation of behavior. If the docstring says atomic, check if it actually is. |
| No environment verification | Plan referenced `uv run` but `uv` wasn't installed; plan said "write BDD files" but no BDD framework existed | Viability assessment must check the actual machine, not just the document. Run `pip list`, `which`, `python3 -c "import X"` |
| Premature convergence declaration | First "converged" call missed 5 connection leaks, phantom commands, and stale doc counts — subsequent independent audits found real issues | True convergence requires two consecutive clean passes from independent audit runs, not "I just fixed everything and it looks clean" |
| Coverage checkbox dishonesty | Marked "coverage ≥ 100%" when two modules were at 82% and 81% | Verify actual numbers before checking the box. Annotate honestly. |
| Connection leaks in SQLite | 5 functions opened SQLite connections without try/finally — any exception leaked the connection | EVERY `sqlite3.connect()` must be in a try/finally block. No exceptions. Pattern: `conn = sqlite3.connect(path)` / `try:` / `...` / `finally:` / `conn.close()` |
| Phantom CLI commands in docs | `review_queue.py` usage string listed `batch` and `propagate` commands that had no handler. `correct-entry.md` referenced `propagate <uid>` | When adding CLI usage strings, verify every listed command has an actual handler. When removing features, grep docs for references. |
| Orphaned relationships accumulate | 4,678 of 11,886 relationships referenced UIDs that no longer existed in entries.json — nearly 40% | Run periodic relationship integrity checks. Build `relationship_repair.py` for automated detection and cleanup. |
| Copy instead of rename in migration | `migrate_from_pipeline_state()` used `shutil.copy2` instead of `rename()`, leaving original file in place. Re-running migration created duplicates. | When a migration is "move and mark done," use `rename()` not `copy()`. The original must be gone. |
| Skipping Claude Code tasks | Prompt explicitly said "including creating claude code tasks" but Claude used plan checkboxes only — no TaskCreate calls, no real-time progress visibility | Always convert plan checkboxes to Claude Code tasks before execution. Audit checkboxes first, then TaskCreate for each. Execute from task list. |
| Plan said "NOT STARTED" with existing code | CHAT_PROVENANCE_PLAN said "NOT STARTED" but Phases 1-2 were 80% built with 75 passing tests from a prior session | Viability assessment Step 3b: glob for every file the plan says to create. If files exist, update plan status and verify existing code against spec. |
| Destructive rebuild without downstream chain | `build_memory.py --fast` drops chat tables silently. Server returned 500 because chat table rebuild wasn't chained after the main rebuild. | Any destructive operation must document (and ideally automate) its downstream rebuild requirements. "Drops X" must be paired with "then rebuild X by running Y." |
| Coverage floor became the ceiling | Doc said "90%+" as the floor, but in practice that number became the target. Coverage was reported at 91% and treated as acceptable. Bryan's intent was always 100%. | If the coverage target is 100%, say 100%. Floors like "90%+" invite building to the floor. Explicit targets prevent ambiguity. |
| Coverage by coincidence | Test named `test_returns_none_on_struct_error` passed and appeared to cover the except handler, but actually hit an early-return guard on a different line. Lines 82-83 remained uncovered. | Never trust test names or intent as proof of coverage. Verify with `--cov-report=term-missing` that specific line numbers actually disappeared from the Missing column. |

---

## 12. Session Management Patterns

These patterns are codified in CLAUDE.md as hard rules. They're repeated here because they represent hard-won lessons from actual session crashes.

### 12A. Mandatory Session Start

Every session begins with the same ritual:

```bash
python3 memory/session_check.py              # detect gaps from crashes
python3 memory/query_memory.py "session_log" --type meta --limit 5  # recent history
python3 memory/mem_pipeline.py status danielle  # pipeline state
```

Read `memory/config.json` for API keys and paths. Never ask Bryan for these. Tell Bryan exactly what you know and what's missing.

### 12B. Mandatory Logging

After every substantive interaction, log to the memory DB BEFORE doing anything else. One command:

```bash
python3 memory/add_entry.py --type meta --tags session_log --date "YYYY-MM-DD" \
  --content "Session: [1-2 sentence summary]" --source "session"
```

This is the single most violated rule. Sessions 6, 10, and multiple continuations lost work because logging was deferred. The rule is absolute: log BEFORE moving to the next task.

### 12C. Surfacing Ledger

After any batch import of non-meta entries, `add_entry.py` prints a `SURFACING REQUIRED` banner. The surfacing ledger (`memory/surfacing_ledger.json`) tracks what's been surfaced vs not. Check with:

```bash
python3 memory/add_entry.py --check-unsurfaced  # are there unsurfaced findings?
python3 memory/add_entry.py --mark-surfaced       # after presenting to Bryan
```

Hooks at `.claude/hooks/session-start-check.sh` and `.claude/hooks/session-logger.sh` enforce this — session start warns about unsurfaced findings, session stop checks and logs.

### 12D. Context Compaction and Recovery

Sessions crash when context gets too large. When resuming after compaction:

1. The conversation summary replaces detailed context — you've lost specific code snippets, error messages, and in-progress reasoning
2. Run `python3 memory/session_check.py` to detect gaps
3. Query recent session logs to see what was done
4. Check `memory/surfacing_ledger.json` — if the last import has no surfacing entry, STOP and surface before doing anything else
5. Read the plan file — the progress tracker checkboxes tell you exactly where work stopped

**Prevention:** Process large sources in chunks, log after each chunk. If context feels heavy, proactively tell Bryan and offer to checkpoint. Before any large processing operation, log first.

### 12E. Deprecated Files — Do Not Write To

These files were used in earlier sessions but are now deprecated. The memory DB is the single source of truth:

- ~~`documentation_and_analysis/SESSION_LOG.md`~~
- ~~`memory/session_log.json`~~
- ~~`memory/SESSION_STATE.json`~~
- ~~`RESUME.md`~~ (static recovery doc, don't update per interaction)
- ~~`python3 memory/post_interaction.py`~~

---

## 13. Working with Existing Code

### 13A. Read Before Modifying

Never propose changes to code you haven't read. The project has 92 production files and 70 test files. Before modifying a function:

1. Read the function's current implementation
2. Read its callers (grep for the function name)
3. Read its tests (grep in the test directory)
4. Document the current state in the plan (see Section 3D)

### 13B. Existing Test Patterns

The existing codebase uses inline imports inside test methods:

```python
class TestConfig:
    def test_load_config(self):
        from memory.mem_pipeline import load_config
        config = load_config()
        assert isinstance(config, dict)
```

This pattern avoids import-time failures (if a dependency is missing, only that test fails, not the entire file). New tests should follow the same pattern unless there's a reason not to.

### 13C. Namespace Package

`memory/` has no `__init__.py` but is importable as a Python 3 namespace package. `python3 -c "import memory.mem_pipeline"` works. New modules added to `memory/` are automatically importable. Do NOT add `memory/__init__.py` unless needed — it could break existing import behavior.

### 13D. Existing Tools and Slash Commands

The project has 11 slash commands (defined in `.claude/commands/*.md`), hooks (`.claude/hooks/*.sh`), and the pipeline CLI (`memory/mem_pipeline.py`). Before building new automation, check if an existing tool already does what you need. Key tools:

- `/process <person> <stage>` — run pipeline stage
- `/review <person> <view>` — surface analytical findings
- `/report` — live project statistics
- `/correct-entry` — fix incorrect entry
- `memory/mem_pipeline.py` — master pipeline (status, run, import, prompts)
- `memory/add_entry.py` — entry management (--batch for bulk)
- `memory/query_memory.py` — search entries

### 13E. Agent Delegation for Research

When you need to check existing code without loading it into the main conversation context, use explore agents:

- "Check what functions exist in app/memory_db.py and their signatures"
- "Find all callers of _save_reviews across the codebase"
- "What test patterns does app/tests/conftest.py establish?"

This keeps the main context clean for plan writing/editing while still gathering the information needed. Especially important during Phase 4-style integration work where multiple modules need to be understood but not all need to be in context simultaneously.

---

## 14. Bryan's Collaboration Style

### 14A. Concise, Imperative Prompts

Bryan gives direction, not detail. His prompts are short and assume Claude knows the methodology:

| Bryan says | What it means |
|-----------|---------------|
| "do it again" | Re-run the same audit type on the same scope. Find issues the fixes introduced. |
| "fix these" | Apply fixes directly to the plan/code. Don't ask for permission, don't present options. Just fix them. |
| "audit it" | Check for internal consistency, completeness, correctness. Use the 8-dimension framework if it's code, the plan audit checklist if it's a plan. |
| "will this succeed?" | Viability assessment. Check environment. Give YES/NO with specific numbered caveats. |
| "update X with everything you've seen" | Review all sessions since X was last written. Add new patterns, update existing ones. Don't ask what to add — find the gaps yourself. |
| "extract everything about how we did Y" | Create a new document capturing methodology, not just outcomes. Include the meta-process. |
| "not just [thing]. [broader scope]." | Bryan is correcting Claude's scope. Claude went too narrow. Expand to include what Bryan specified. |
| "[viability question] fix any issues you find" | Combined prompt — answer the viability question AND fix issues in one pass. Claude must do both: assess, then fix. Saves a round-trip. |
| "run the build plan... over and over, until... no issues left" | The "big bang" prompt. Execute entire plan + audit + converge in one go. See Section 17B. |
| "continue until done, and all audits pass" | Resume after interruption with same convergence requirement. |
| "1. yes 2. skip it." | Numbered responses to numbered decisions. Efficient, unambiguous. See Section 17D. |
| "audit your checkboxes against the plan" | Meta-audit: is the progress tracker honest? Are deferred items tracked? |
| "how should i have phrased my prompt to..." | Prompt refinement request. Help Bryan improve his prompt toolkit. See Section 17C. |
| "including prompts" | When updating patterns, explicitly capture the prompts used. The prompts ARE the methodology. |

### 14B. Bryan Corrects Scope, Not Detail

Bryan rarely specifies implementation details. He specifies:
- **What's missing** — "not just the plan. how you and i interacted building the plan."
- **What's wrong** — "this burns API tokens"
- **What to do next** — "do it again", "fix these", "now audit the entire plan"

Claude must fill in the detail autonomously. When Bryan says "extract everything about how we planned this into a development patterns file," Claude must decide what sections to create, what examples to include, what anti-patterns to document. If Claude goes too narrow, Bryan corrects scope — and that correction itself becomes a pattern (see Section 0D).

---

## 15. Plan Lifecycle

### 15A. One Plan Per Development Round

Each development round gets one plan document (e.g., `BUILD_PLAN.md`). The plan lives until all checkboxes are checked and the code audit converges. Then:

1. Mark the plan status as COMPLETE
2. Update DEVELOPMENT_PATTERNS.md with new patterns from this round
3. Archive or keep the plan file for reference
4. Start fresh for the next round

### 15B. Plans Don't Accumulate

Don't append to an old plan. Each round is a fresh plan with fresh checkboxes. The patterns file captures methodology across rounds; the plan file captures the specific work of one round.

### 15C. What Carries Forward

| Carries forward | Lives in |
|----------------|----------|
| Methodology, patterns, anti-patterns | DEVELOPMENT_PATTERNS.md |
| Hard rules, session protocols, tool docs | CLAUDE.md |
| Project state, entry counts, processing status | Memory DB (query live) |
| What was built, when, why | Git history + session log entries |

What does NOT carry forward: plan checkboxes, phase details, specific function lists. These are per-round artifacts.

---

## 16. Execution-Phase Patterns (Learned from Running the Build)

These patterns were discovered during actual execution of BUILD_PLAN.md (96 checkboxes, Phases 0-6). They complement the planning-phase patterns in Section 0.

### 16A. The "Run Until Done" Execution Model

The most effective execution prompt is a single instruction that encompasses the full lifecycle:

```
"Run the build plan exactly as planned, including creating Claude Code tasks
planning the entire thing out, including documentation changes, code audits,
documentation audits, and then after you run it, audit and fix any issues in
the code, and in the docs, over and over, until you report there aren't any
left to fix."
```

This single prompt drives: plan execution → code audit → doc audit → fix → re-audit → convergence. The key phrase is **"over and over, until you report there aren't any left to fix"** — this forces the convergence loop.

**Why this works:** It eliminates round-trips. Instead of Bryan saying "audit" then "fix" then "audit again" then "fix" four times, one prompt covers the entire loop. Claude knows the termination condition (zero issues) and drives toward it autonomously.

**Why the first version was insufficient:** Bryan's initial prompt said "run the build plan" but didn't explicitly include documentation audits or the multi-pass convergence requirement. This led to premature completion claims. The refined version above explicitly calls out:
1. Code audits
2. Documentation audits
3. The iterative fix-audit loop
4. The termination condition

### 16B. Multi-Pass Convergence on Code (Not Just Plans)

Plan audits and code audits follow the same convergence pattern but find different types of issues:

```
Plan audits:    consistency errors, stale references, missing sections
Code audits:    logic bugs, connection leaks, missing implementations, phantom references
Doc audits:     stale counts, phantom commands, missing new modules, architecture drift
```

**Critical insight:** A "clean" code audit doesn't mean docs are clean, and vice versa. In our execution:
- R1 and R2 found only code issues (13 + 9 = 22)
- R3 found 0 code issues but 3 doc issues
- R4 found 0 of both → convergence

The temptation after R2 (0 new code issues) was to declare convergence. But docs hadn't been audited yet. **True convergence requires clean passes across ALL audit dimensions in a single round.**

### 16C. The Two-Consecutive-Clean-Pass Rule

One clean pass is not enough. A single clean pass might mean:
- The auditor missed something (fatigue, scope blindness)
- The previous fix round was clean but introduced subtle issues
- The audit scope was too narrow

**Rule:** Convergence requires **two consecutive clean passes** where the second is from an **independent audit** (different agent, different starting context, fresh eyes). In our execution:
- R2 was "clean" for code → but R3 found 3 doc issues
- R3 was clean for code → R4 confirmed (independent agent, clean on both)
- R4 was the true convergence point

### 16D. Relationship Maintenance as Recurring Pattern

The project accumulates relationships (derived_from, contains, etc.) that reference entry UIDs. As entries are corrected, re-extracted, or removed, relationships become orphaned. This is not a bug — it's entropy.

**Pattern:** Include relationship integrity checks in `session_check.py` and run `relationship_repair.py` periodically:

```bash
# Dry run — see what would be cleaned
python3 memory/relationship_repair.py --dry-run

# Execute cleanup (with Bryan's approval)
python3 memory/relationship_repair.py --execute
```

In our execution: 4,678 of 11,886 relationships were both-orphaned (neither from_uid nor to_uid existed in entries). Cleanup reduced to 7,208 (5,821 valid, 1,387 one-sided kept as safer option).

**When to run:** After any large re-extraction or bulk entry deletion. After any migration. At session start if the count seems high.

### 16E. Session Crash During Execution

During execution of 96 checkboxes, the session ran out of context and required continuation. The recovery protocol:

1. The conversation summary preserves: what was done, what was pending, key decisions
2. The plan file's progress tracker shows exactly which checkboxes are complete
3. `code_audit_state.json` tracks audit round state (issues found, fixes applied, current phase)
4. Memory DB session logs provide narrative history

**What worked:** The plan file + progress tracker was sufficient to resume. No work was duplicated. The summary was accurate enough to continue without re-reading code.

**What didn't work:** The summary lost specific code snippets and error messages. For complex fixes, the next session had to re-read files to understand context. This is acceptable — the alternative (keeping everything in context) causes the crash in the first place.

### 16F. Fixture Isolation in Integration Tests

When testing modules that interact (work_unit, review_queue, pipeline_integration, source_chunker), the test fixture must:

1. Create a `tmp_data_dir` with the full directory structure
2. Monkeypatch ALL module-level path constants (PERSIST_DIR, DB_PATH, JSON_PATH, WORK_QUEUE_DIR, etc.)
3. Create a real SQLite database with the expected schema
4. Populate with minimal representative data

**Common failure:** Monkeypatching `module.PERSIST_DIR` but forgetting `module.DB_PATH` — tests silently touch the real database.

**Pattern from conftest.py:**

```python
@pytest.fixture
def tmp_data_dir(tmp_path):
    (tmp_path / "memory").mkdir()
    (tmp_path / "memory" / "work_queue").mkdir()
    (tmp_path / "db").mkdir()
    return tmp_path
```

Then each test file's fixture monkeypatches its specific module's paths to point at `tmp_data_dir`.

---

## 17. Prompt Patterns (Bryan's Prompt Toolkit)

These are the prompts Bryan uses to drive development. Each prompt triggers a specific mode of work. Claude should recognize these patterns and respond with the appropriate methodology.

### 17A. The Master Prompt Sequence

This is the complete sequence from problem to convergence, with the actual prompts:

```
PHASE: PROBLEM + PLANNING
─────────────────────────
"I want [goal]. [Problem description]. [Constraints]."
  → Claude writes BUILD_PLAN.md

"extract everything about how we planned this into a development patterns file"
  → Claude writes DEVELOPMENT_PATTERNS.md


PHASE: PLAN AUDITING (iterate until clean)
──────────────────────────────────────────
"audit phase N for any and all issues"
  → Focused audit of single phase

"now do it again"
  → Re-audit same scope, find issues introduced by fixes

"now audit the entire plan for issues"
  → Full-plan cross-phase audit

"audit it again for any remaining issues"
  → Re-audit full plan


PHASE: VIABILITY
────────────────
"if i tell you to run the plan exactly as planned, and then after you
 run it, i tell you to audit and fix any issues, over and over, until
 you report there aren't any left to fix, will the plan succeed?"
  → YES/NO + specific numbered caveats + environment verification

"fix these in the plan as much as possible"
  → Apply fixes to plan document


PHASE: EXECUTION + CONVERGENCE (the "big bang" prompt)
──────────────────────────────────────────────────────
"run the build plan exactly as planned, including creating claude code
 tasks planning the entire thing out, including documentation changes,
 code audits, documentation audits, and then after you run it, audit
 and fix any issues in the code, and in the docs, over and over, until
 you report there aren't any left to fix."
  → Execute all checkboxes → audit code → audit docs → fix → repeat → converge


PHASE: MID-EXECUTION CORRECTIONS
─────────────────────────────────
"1. yes  2. skip it. pointless at this point."
  → Approve/reject specific pending decisions (numbered to match Claude's questions)

"run the build plan as you've planned out with your checkboxes.
 continue until done, and all audits pass with no more issues
 found to correct."
  → Resume execution after interruption, same convergence requirement


PHASE: META-AUDIT
─────────────────
"audit your list of checkboxes for following the build plan against
 the build plan and resolve any gaps or errors."
  → Audit the PROGRESS TRACKER against the PLAN. Are checkboxes honest?
    Are deferred items tracked? Are coverage claims verified?


PHASE: POST-EXECUTION
─────────────────────
"update the DEVELOPMENT_PATTERNS.md file with patterns to this point,
 including prompts"
  → Review everything that happened, add new patterns, capture prompts

"how should i have phrased my prompt to encompass that as well?"
  → Bryan asking Claude to help him refine his prompt toolkit
```

### 17B. The "Big Bang" Prompt — How to Get Full Convergence in One Shot

Bryan's most important prompt discovery was that a single well-phrased prompt can drive the entire execution + audit + convergence cycle without further intervention. The key elements:

| Element | Why It Matters | What Happens Without It |
|---------|---------------|------------------------|
| "run the build plan exactly as planned" | Anchors execution to the plan document | Claude improvises or skips steps |
| "including creating claude code tasks" | Forces checkbox → task conversion before execution | Claude uses plan checkboxes only — no real-time progress, no task persistence across sessions |
| "planning the entire thing out" | Tasks must cover the WHOLE plan, not just the next few steps | Partial task creation, ad hoc execution |
| "including documentation changes" | Explicitly scopes in doc updates | Docs are skipped or deferred |
| "code audits, documentation audits" | Both audit types required | Code audit only → stale docs ship |
| "over and over" | Forces iteration, not single-pass | One audit round declared "clean" prematurely |
| "until you report there aren't any left to fix" | Defines termination condition | No clear stopping point → either premature or infinite |

**The prompt Bryan wished he'd used from the start:**

```
"Run the build plan exactly as planned. After execution, perform
independent audit passes on both code and documentation. Fix all
issues found. Repeat audit-fix cycles until two consecutive
independent audit passes find zero issues across all files.
Report the convergence status after each round."
```

The addition of **"two consecutive independent audit passes"** is what prevents premature convergence. Without it, one clean pass after fixing things feels like convergence but isn't — the auditor has anchoring bias from just having fixed the issues.

### 17C. Prompt Refinement as a Pattern

Bryan explicitly asked: "how should i have phrased my prompt to encompass that as well?" This reveals a meta-pattern: **the prompt itself is a tool that gets refined through use.**

The progression:
1. **V1:** "run the build plan" → too vague, no audit requirement
2. **V2:** "run the build plan, audit and fix issues" → better, but no convergence criterion
3. **V3:** "run the build plan, audit and fix, over and over, until no issues" → good, but convergence was premature (single pass)
4. **V4:** "...until two consecutive independent clean passes" → correct termination condition

Each version was discovered by hitting a failure mode:
- V1 → code wasn't audited → V2
- V2 → one audit wasn't enough → V3 ("over and over")
- V3 → single clean pass wasn't true convergence → V4 ("two consecutive independent")

**Lesson:** After every development round, review the prompts used and refine them. The prompt toolkit improves with the same convergence pattern as the code.

### 17D. Numbered Decision Prompts

When Claude presents multiple pending decisions, Bryan responds with numbered answers:

```
Claude: "Two decisions needed:
  1. Execute orphan cleanup? (4,678 relationships)
  2. Investigate lost work forensically?"

Bryan: "1. yes  2. skip it. pointless at this point."
```

This is efficient — no ambiguity about which decision maps to which answer. Claude should present decisions as numbered lists to enable this pattern.

### 17E. The Checkpoint Audit Prompt

Mid-execution, Bryan can request an audit of the execution itself:

```
"audit your list of checkboxes for following the build plan
 against the build plan and resolve any gaps or errors"
```

This is NOT an audit of the code or the plan — it's an audit of **whether the progress tracker is honest.** It caught:
- Coverage claims that were false (82% marked as ≥100%)
- Deferred items not annotated
- Duplicate Definition of Done sections in the plan
- Missing Known Gaps section

**Pattern:** Run this prompt at least once during execution, especially if the plan is large (>50 checkboxes). It prevents the "plan says COMPLETE but it's lying" failure mode.

---

## 18. Maintenance Patterns

### 18A. Periodic Health Checks

Beyond session-start checks, the system needs periodic maintenance:

| Check | Command | Frequency |
|-------|---------|-----------|
| Relationship integrity | `python3 memory/relationship_repair.py --dry-run` | After bulk operations |
| Orphaned entries | `python3 memory/session_check.py` | Every session start |
| Unsurfaced findings | `python3 memory/add_entry.py --check-unsurfaced` | After every import |
| Stage gate status | `python3 memory/mem_pipeline.py status <person>` | Before processing |
| Test suite health | `uv run python -m pytest memory/tests/ -v` | After any code change |

### 18B. Data Cleanup Protocol

When orphaned data accumulates (relationships, stale entries, abandoned work units):

1. **Dry run first** — always show what would be changed before changing it
2. **Backup before cleanup** — `relationships.backup.YYYYMMDD_HHMMSS.json`
3. **Get Bryan's approval** — present the numbers, let him decide
4. **Execute with logging** — record what was removed
5. **Verify after** — re-run the dry run to confirm 0 remaining issues

In our execution: dry run showed 4,678 orphans → Bryan approved → cleanup executed → verification confirmed 0 both-orphaned remaining, 7,208 relationships kept.

### 18C. Test Count as Health Metric

Track the test count across sessions. It should only go up:

```
Session start:  98 tests (Phase 0-5 complete)
After Phase 6 R1:  98 tests (audit found issues, no new tests yet)
After coverage fix:  109 tests (7 new tests to close coverage gaps)
Final:  109 tests, 0 failures, all modules ≥100%
```

A drop in test count is a red flag — it means tests were deleted (why?) or a module was removed (was it intentional?). A plateau during active development means tests aren't keeping pace with code.
