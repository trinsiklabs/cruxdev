# CruxDev Adoption Process

How to adopt any existing project into the Crux/CruxDev ecosystem. Stack-agnostic, principles-based. Hand this file to a Claude Code session and say: "Follow ADOPTION_PROCESS.md. Converge."

This is the operational companion to ADOPTION_PLAYBOOK.md. The playbook defines the 9 phases in detail. This document tells the agent how to start — install Crux, install CruxDev, configure enforcement, and begin executing the playbook.

---

## Step 1: Install Crux (Intelligence Layer)

Crux provides modes, MCP tools, safety gates, and session management. Install it non-interactively using `adopt_project()`.

```python
import os
import sys

sys.path.insert(0, "/Users/user/personal/crux")

from scripts.lib.crux_adopt import adopt_project

result = adopt_project(
    project_dir=os.getcwd(),              # Current project directory
    home=os.environ["HOME"],              # User home
    active_mode="build-py",               # Set to stack-appropriate mode (build-py, build-ex, etc.)
    active_tool="claude-code",
    working_on="CruxDev adoption",        # Brief description of current work
    key_decisions=[],                      # Fill in as decisions are made
    pending=[],                            # Fill in with known work items
    knowledge_entries={},                  # Stack-specific knowledge (optional)
)

for item in result.items_setup:
    print(f"  ✓ {item}")
```

This creates:
- `.crux/` directory with session state, knowledge, context
- `.claude/mcp.json` pointing to the Crux MCP server
- `.claude/settings.local.json` with hook configuration
- Git history parsed for project context

**Verify:**
```bash
/Users/user/personal/crux/bin/crux status
ls .crux/
cat .claude/mcp.json
```

If `crux status` reports issues, fix them before proceeding.

### Mode Selection

Choose the mode matching the project's primary language:

| Stack | Mode |
|-------|------|
| Python | `build-py` |
| Elixir/Phoenix | `build-ex` |
| General | `build-py` (default) |

List all available modes: `/Users/user/personal/crux/bin/crux modes`

---

## Step 2: Install CruxDev (Convergence Methodology)

CruxDev provides the autonomous convergence engine — planning, auditing, and convergence loops.

```bash
# Copy CruxDev framework into the project
cp -r /Users/user/personal/cruxdev/.cruxdev . 2>/dev/null || true
```

---

## Step 3: Create CLAUDE.md

Create `.claude/CLAUDE.md` following these principles. Every CLAUDE.md must contain:

### Required Sections

**Identity** — What the project is, who owns it. One paragraph.

**Core Rules** — Non-negotiable constraints:
1. TDD for everything. Tests before code. No exceptions without explicit approval.
2. 100% test coverage enforced via the stack's coverage tool.
3. Drive convergence autonomously. Do not wait for "do it again."
4. Two consecutive independent clean passes = convergence.
5. Verify all status claims empirically. Coverage numbers come from tools, not memory.
6. Stack-specific conventions (e.g., "Use Ash resources, don't bypass with raw Ecto" for Elixir/Ash projects).

**Test Commands** — Exact commands to run tests with and without coverage:

| Stack | Coverage Command | Test-Only Command |
|-------|-----------------|-------------------|
| Python | `python3 -m pytest tests/ --cov=src --cov-report=term-missing --cov-fail-under=100` | `python3 -m pytest tests/` |
| Elixir | `mix coveralls --min-coverage 100` | `mix test` |
| Node | `npx jest --coverage --coverageThreshold='{"global":{"lines":100}}'` | `npx jest` |
| Go | `go test -coverprofile=c.out ./... && go tool cover -func=c.out` | `go test ./...` |

**Methodology** — Reference to patterns and playbook:
```
Read ADOPTION_PROCESS.md for current adoption state.
Methodology: /Users/user/personal/cruxdev/DEVELOPMENT_PATTERNS_CRUXDEV.md
Playbook: /Users/user/personal/cruxdev/ADOPTION_PLAYBOOK.md
E2E patterns: /Users/user/personal/cruxdev/E2E_TEST_PATTERNS.md
```

**Project Structure** — Accurate directory tree with purposes.

**Known Issues** — Prioritized list from the assessment (populated during Phase 2).

---

## Step 4: Configure Coverage Enforcement

Set up the stack's coverage gate to enforce 100%.

| Stack | Configuration |
|-------|--------------|
| **Python** | `pyproject.toml`: `[tool.coverage.report]` → `fail_under = 100` |
| **Elixir** | `coveralls.json`: `"minimum_coverage": 100` |
| **Node** | `jest.config.js`: `coverageThreshold.global.lines = 100` |
| **Go** | CI script: `go test -coverprofile=c.out && coverage=$(go tool cover -func=c.out \| grep total \| awk '{print $3}') && test "$coverage" = "100.0%"` |

Also update CI configuration to enforce the gate on every PR/push.

---

## Step 5: Run the Adoption Playbook

Infrastructure is installed (Phase 1 of ADOPTION_PLAYBOOK.md). Now execute the remaining phases.

### Create the Build Plan

Create `BUILD_PLAN_001_ADOPTION.md` (numbered, with descriptor — per DEVELOPMENT_PATTERNS_CRUXDEV.md Section 1C).

The plan works through ADOPTION_PLAYBOOK.md phases:

```
Phase 2: Codebase Assessment
  - Architecture inventory (modules, entry points, data stores, integrations, state machines, I/O, test infrastructure)
  - Standards gap analysis (10 dimensions: atomic writes, crash resilience, input validation, path safety, error handling, connection safety, state machines, logging, coverage, docs)
  - Prioritized remediation list (P0 security → P5 style)
  - Viability assessment (tools installed? tests run? CI accessible?)
  - Audit to convergence (two clean passes on 4 assessment dimensions)

Phase 3: Architecture Remediation
  - Brainstorming gate before each major decision
  - Fix structural issues: authoritative data source, state machines, config consolidation, entry points, dependency direction
  - Stabilize public interfaces before hardening
  - Audit to convergence

Phase 4: Code Hardening
  - Atomic writes for all critical file operations
  - Connection/resource safety (context managers, try/finally)
  - Input validation at all external boundaries
  - Error handling (no bare except/rescue, cleanup in finally)
  - Crash resilience (state files, idempotent operations)
  - Security hardening (no hardcoded creds, no path traversal, no injection)
  - Audit to convergence

Phase 5: Test Suite Build-Out
  - Coverage gap closure to 100% (verified with line-missing report)
  - All test categories: unit, integration, edge case, security, crash recovery
  - Coverage-by-coincidence elimination (verify specific lines, not test names)

Phase 6: Documentation Convergence
  - Audit all docs against code (5 dimensions: accuracy, completeness, staleness, phantoms, architecture)
  - Fix CLAUDE.md to match reality
  - Two consecutive clean passes

Phase 7: E2E Test Suite
  - Enumerate user roles and journeys
  - Four convergence loops (plan audit → alignment → execution → docs)
  - CRITICAL journeys first
  - Testing pyramid: only test what lower levels can't catch

Phase 8: Convergence Verification
  - Full codebase audit (8 code + 5 doc dimensions) — ENTIRE codebase, not just changed files
  - Two consecutive independent clean passes
  - Before/after comparison against Phase 1 baseline

Phase 9: Methodology Handoff
  - Create DEVELOPMENT_PATTERNS_<PROJECT>.md (learnings admission gate)
  - Finalize CLAUDE.md
  - Gate all future development through CruxDev methodology
```

### The One Prompt

Once the build plan is written and audited:

```
"Execute the plan. Converge."
```

The agent drives the entire cycle to completion per DEVELOPMENT_PATTERNS_CRUXDEV.md. No further prompting needed.

---

## Stack Adaptation Principles

The playbook is stack-agnostic. Adapt these concepts to your stack:

| Concept | Principle | Examples |
|---------|-----------|---------|
| **Coverage enforcement** | The coverage tool must fail the build below 100% | `fail_under`, `--min-coverage`, `coverageThreshold` |
| **Line-level verification** | Must be able to see which specific lines are uncovered | `term-missing`, `coveralls.detail`, `--coverage-reporters=text` |
| **Atomic writes** | Critical file operations must be crash-safe | Write-then-rename (Python), Ecto transactions (Elixir), fs.rename (Node) |
| **Connection safety** | All connections cleaned up on error | Context managers, `with` blocks, `try/finally`, `defer` |
| **Input validation** | External input validated at system boundary | Framework validators, changeset constraints, Zod schemas, type assertions |
| **State machines** | Explicit states, named transitions, documented terminals | Enums, state machine libraries, typed unions |
| **Test isolation** | Each test owns its own data, runs in any order | tmp_path (Python), sandbox (Ecto), temp dirs |
| **E2E testing** | User journeys tested through the actual interface | Wallaby, Playwright, subprocess-based CLI tests |

---

## Reference Paths

| Resource | Path |
|----------|------|
| Crux (core) | `/Users/user/personal/crux` |
| Crux CLI | `/Users/user/personal/crux/bin/crux` |
| CruxDev | `/Users/user/personal/cruxdev` |
| Methodology | `/Users/user/personal/cruxdev/DEVELOPMENT_PATTERNS_CRUXDEV.md` |
| Adoption Playbook | `/Users/user/personal/cruxdev/ADOPTION_PLAYBOOK.md` |
| E2E Patterns | `/Users/user/personal/cruxdev/E2E_TEST_PATTERNS.md` |
