# BUILD_PLAN_007: Competitors System

**Created:** 2026-03-22
**Status:** NOT STARTED
**Goal:** Build the competitive intelligence system — discovery, research, tracking, gap closure, website integration, and continuous monitoring. Codified in `docs/COMPETITORS_PATTERN.md`.
**Methodology:** Follow `docs/COMPETITORS_PATTERN.md` for the competitive research process. Follow `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md` for code convergence.

**Rule:** TDD. Tests before code. 100% coverage on engine code.
**Rule:** Every claim verified. Every link tested. No unverified assertions.
**Rule:** Follow COMPETITORS_PATTERN.md — two consecutive clean passes on every research phase.

---

## Phase 1: Competitor Research Engine (CruxDev)

**Purpose:** Build MCP tools and slash commands that automate competitive research, gap analysis, and comparison page generation.

### 1A. MCP Tools

| Tool | What it does |
|------|-------------|
| `discover_competitors(project_description, category)` | Searches for competitors using web search, returns raw list |
| `research_competitor(name, url)` | Deep research on one competitor, returns structured profile |
| `verify_competitor_links(competitor_name)` | Tests all URLs in a competitor profile, returns pass/fail per link |
| `generate_gap_analysis(competitors_file)` | Reads COMPETITORS.md, produces feature matrix + classified gaps |
| `generate_comparison_page(competitor_name)` | Generates a `/vs/<name>` comparison page for the project website |
| `generate_gap_build_plan(gap_name, competitor)` | Creates a build plan to close a specific competitive gap |
| `refresh_competitor(competitor_name)` | Re-researches one competitor, diffs against existing data |

### 1B. Slash Commands

| Command | What it does |
|---------|-------------|
| `/competitors` | Run the full competitive discovery + research process |
| `/competitor-refresh` | Refresh all tracked competitors |
| `/competitor-gaps` | Show current gap analysis with closure queue |

### Checklist — Phase 1

- [ ] 1.1 `src/competitors/discovery.py` — web search integration for competitor discovery
- [ ] 1.2 `src/competitors/research.py` — structured competitor profiling
- [ ] 1.3 `src/competitors/verification.py` — URL testing, claim verification
- [ ] 1.4 `src/competitors/gap_analysis.py` — feature matrix, gap classification
- [ ] 1.5 `src/competitors/comparison_page.py` — generate /vs/ page content
- [ ] 1.6 `src/competitors/build_plan_generator.py` — auto-generate gap closure plans
- [ ] 1.7 MCP tools wired in mcp_server.py
- [ ] 1.8 Slash commands in .claude/commands/
- [ ] 1.9 Tests for discovery (mock web search)
- [ ] 1.10 Tests for research (mock data, structured output)
- [ ] 1.11 Tests for verification (mock HTTP, test link checking)
- [ ] 1.12 Tests for gap analysis (feature matrix generation, classification)
- [ ] 1.13 Tests for comparison page generation
- [ ] 1.14 Tests for build plan generation
- [ ] 1.15 Coverage ≥ 100%

---

## Phase 2: COMPETITORS.md Management

**Purpose:** Automated creation and maintenance of the COMPETITORS.md file per the template in COMPETITORS_PATTERN.md.

### Checklist — Phase 2

- [ ] 2.1 `src/competitors/competitors_doc.py` — read/write COMPETITORS.md
- [ ] 2.2 Create from discovery results (template from COMPETITORS_PATTERN.md)
- [ ] 2.3 Update individual competitor sections
- [ ] 2.4 Update feature matrix
- [ ] 2.5 Update gap closure queue
- [ ] 2.6 Tests for COMPETITORS.md read/write roundtrip
- [ ] 2.7 Tests for incremental updates
- [ ] 2.8 Coverage ≥ 100%

---

## Phase 3: Website Integration

**Purpose:** Generate and maintain comparison pages on the project website.

### Checklist — Phase 3

- [ ] 3.1 Page template for /vs/<competitor> following COMPETITORS_PATTERN.md Section 6.1
- [ ] 3.2 SEO: title tags, meta descriptions, Schema.org FAQPage
- [ ] 3.3 Feature comparison table component
- [ ] 3.4 Auto-generate from COMPETITORS.md data
- [ ] 3.5 Verification: all links on comparison pages tested
- [ ] 3.6 Two consecutive clean passes on generated pages

---

## Phase 4: Continuous Monitoring & Auto-Update

**Purpose:** Keep competitive intelligence current. Detect changes. Auto-generate build plans for new gaps.

### 4A. Monitoring Loop

```
Quarterly:
  1. Re-run discovery (Phase 1 of COMPETITORS_PATTERN.md)
  2. Refresh all official competitors
  3. Verify all links
  4. Re-run gap analysis
  5. For each new "must close" gap:
     - Auto-generate build plan
     - Add to gap closure queue in COMPETITORS.md
  6. Update comparison pages
  7. Converge documentation (two clean passes)
  8. Deploy website updates
```

### 4B. Auto-Spawned Build Plans

When gap analysis identifies a new "must close" gap:

1. Engine generates `build_plans/BUILD_PLAN_NNN_GAP_<competitor>_<feature>.md`
2. Plan follows standard template with:
   - Which competitor has the feature
   - How they implement it
   - Proposed implementation approach
   - Success criteria
3. Plan added to gap closure queue in COMPETITORS.md
4. User notified of new plan (not auto-executed — user approves before convergence)

### Checklist — Phase 4

- [ ] 4.1 `src/competitors/monitor.py` — refresh logic, change detection
- [ ] 4.2 `src/competitors/auto_plan.py` — gap → build plan generation
- [ ] 4.3 Integration with task router (convergence engine can trigger refresh)
- [ ] 4.4 Tests for change detection (mock before/after data)
- [ ] 4.5 Tests for auto plan generation
- [ ] 4.6 Tests for gap classification changes (must-close → should-close promotion)
- [ ] 4.7 Coverage ≥ 100%

---

## Phase 5: Convergence

### Checklist — Phase 5

- [ ] 5.1 Full test suite passes, coverage ≥ 100%
- [ ] 5.2 COMPETITORS_PATTERN.md audited against implementation (two clean passes)
- [ ] 5.3 All MCP tools documented in mcp_server.py with thorough descriptions
- [ ] 5.4 Slash commands documented with protocols
- [ ] 5.5 ADOPTION_PROCESS.md updated to reference competitors system
- [ ] 5.6 SESSION_UPGRADE.md updated with competitor tools
- [ ] 5.7 cruxdev.dev website updated with competitors engine docs
- [ ] 5.8 Website deployed per docs/DEPLOYMENT.md
- [ ] 5.9 Code + doc convergence (two consecutive clean passes)

---

## Progress Tracker

**Phase 1: Research Engine (15 items)** — [ ] 1.1 – 1.15
**Phase 2: COMPETITORS.md Management (8 items)** — [ ] 2.1 – 2.8
**Phase 3: Website Integration (6 items)** — [ ] 3.1 – 3.6
**Phase 4: Continuous Monitoring (7 items)** — [ ] 4.1 – 4.7
**Phase 5: Convergence (9 items)** — [ ] 5.1 – 5.9

**Total: 45 checkboxes**

---

## Test Commands

```bash
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

## Convergence Criteria

- All 45 checkboxes complete
- All tests pass, coverage ≥ 100%
- COMPETITORS_PATTERN.md matches implementation
- Every competitor claim verified, every link tested
- Comparison pages generated and verified
- Documentation converged (two clean passes)
- Website converged (two clean passes)
- Deployed
