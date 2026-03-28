# CruxDev Adoption Process

How to adopt any existing project into the Crux/CruxDev ecosystem. Stack-agnostic, principles-based. Hand this file to any MCP-compatible coding agent and say: "Follow ADOPTION_PROCESS.md. Converge."

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

## Step 2: Install CruxDev (Convergence Engine)

CruxDev provides the autonomous convergence engine — planning, auditing, and convergence loops via MCP.

```python
import sys
sys.path.insert(0, "/Users/user/personal/cruxdev")
from src.install import install

result = install(".")  # Installs into current project
for item in result["items"]:
    print(f"  ✓ {item}")
```

This adds the `cruxdev` MCP server to `.claude/mcp.json` and creates `.cruxdev/` for convergence state. **Restart Claude Code** to activate.

After restart, you have 10 MCP tools including `/converge`, `/plan`, `/adopt`, `/status`.

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
Methodology: /Users/user/personal/cruxdev/docs/DEVELOPMENT_PATTERNS_CRUXDEV.md
Playbook: /Users/user/personal/cruxdev/docs/ADOPTION_PLAYBOOK.md
E2E patterns: /Users/user/personal/cruxdev/docs/E2E_TEST_PATTERNS.md
Website planning: /Users/user/personal/cruxdev/docs/WEBSITE_PLANNING.md
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

## Step 5: Website Planning (If Applicable)

If this project includes a website or web application, apply the website planning methodology before or alongside code convergence.

### When This Applies

- The project has a marketing site, docs site, or landing page
- The project has a web application with user-facing UI
- The project will need a website in the future (plan now, build later)

### What to Do

1. **Read the methodology**: `/Users/user/personal/cruxdev/docs/WEBSITE_PLANNING.md`
2. **Read the SEO/GEO reference**: `/Users/user/personal/cruxdev/docs/SEO_AND_GEO_REFERENCE.md`
3. **Create a website build plan**: Use `/plan "Website for [project name]"` or create manually in `build_plans/BUILD_PLAN_NNN_WEBSITE.md`
4. **Audit existing site** (if one exists) against WEBSITE_PLANNING.md:
   - Phase 5 (SEO): structured data, meta tags, sitemap, llms.txt, robots.txt
   - Phase 8 (Performance): Core Web Vitals, image optimization, JS budget
   - Phase 10 (QA): accessibility (WCAG 2.1 AA), cross-browser, security headers
   - Appendix A (Legal): privacy policy, cookie consent, accessibility statement
5. **Converge the website plan**: `/converge build_plans/BUILD_PLAN_NNN_WEBSITE.md`

### Minimum Website Standards (All CruxDev Projects)

Every project website must have:
- [ ] Unique `<title>` and `<meta description>` on every page
- [ ] Open Graph and Twitter Card meta tags
- [ ] Schema.org structured data (at minimum: `WebSite`, page-appropriate types)
- [ ] `/llms.txt` for AI visibility
- [ ] `/robots.txt` allowing AI crawlers
- [ ] XML sitemap
- [ ] WCAG 2.1 AA accessibility compliance
- [ ] Core Web Vitals: LCP < 2.5s, INP < 200ms, CLS < 0.1
- [ ] HTTPS with security headers (CSP, HSTS)
- [ ] Responsive/mobile-first design
- [ ] Privacy policy (if collecting any data)

---

## Step 6: Run the Adoption Playbook

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

## Step 7: Feed Learnings Back into CruxDev

**Every adoption improves the adoption process.** After completing the playbook, run this feedback loop:

### 7a. Classifier improvement
- Did the classifier detect this project's types correctly?
- If not: add missing signals to `adoption/classify.rs` and add a test
- New file patterns, directory names, or config files that indicate project types

### 7b. Patterns improvement
- Did any patterns doc have gaps exposed by this project?
- Missing audit dimensions? Add them to `engine/router.rs` and wire into convergence
- New project-type-specific dimensions? Document them
- Update `ADOPTION_PROCESS.md` if the process itself had gaps

### 7c. Template improvement
- Were any templates missing for this project type?
- Add new templates to `templates/` directory
- Register them in the template discovery system

### 7d. Non-software adoption path
Not every project has code, tests, or CI. For non-software projects (books, businesses, courses, podcasts):

| Software Step | Non-Software Equivalent |
|---|---|
| Test coverage | Manuscript tracking / content inventory |
| Code audit (8 dimensions) | Content audit (voice, structure, completeness) |
| CI/CD | Publishing pipeline / deployment |
| Architecture | Project structure / information architecture |
| E2E tests | Reader journey / user journey verification |
| Coverage enforcement | Completeness tracking (all chapters, all modules) |

### 7e. Blog post
- Generate a blog post about what was learned during adoption
- Include: what project type, what gaps were found, what was improved
- Publish via BIP pipeline

### 7f. File GitHub issue (if systemic)
If the adoption exposed a pattern that affects ALL future adoptions, file a GitHub issue with the `adoption` label so it enters the priority queue.

---

## Reference Paths

| Resource | Path |
|----------|------|
| Crux (core) | `/Users/user/personal/crux` |
| Crux CLI | `/Users/user/personal/crux/bin/crux` |
| CruxDev | `/Users/user/personal/cruxdev` |
| Methodology | `/Users/user/personal/cruxdev/docs/DEVELOPMENT_PATTERNS_CRUXDEV.md` |
| Adoption Playbook | `/Users/user/personal/cruxdev/docs/ADOPTION_PLAYBOOK.md` |
| E2E Patterns | `/Users/user/personal/cruxdev/docs/E2E_TEST_PATTERNS.md` |
| Website Planning | `/Users/user/personal/cruxdev/docs/WEBSITE_PLANNING.md` |
| SEO/GEO Reference | `/Users/user/personal/cruxdev/docs/SEO_AND_GEO_REFERENCE.md` |
