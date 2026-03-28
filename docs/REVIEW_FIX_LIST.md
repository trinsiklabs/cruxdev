# External Review Fix List

Prioritized list of issues identified from external review, cross-referenced against actual codebase state.

**Actual numbers (verified 2026-03-28):**
- Tests: 502 unit + 18 E2E = **520 total** (`cargo test` output)
- MCP tools: **65** (`#[tool]` annotations in `rust/src/server.rs`)
- Audit dimensions: needs canonical count from engine code; currently claimed as 14, 39, 78, and 137 in different places

---

## P0 — Stale Numbers (breaks trust immediately)

### 1. MCP tool count inconsistent across 6 locations

| File | Current | Should be |
|------|---------|-----------|
| `/cruxdev/README.md` line 41 | "52 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/public/llms.txt` line 10 | "63 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/src/pages/engine.astro` meta (line 5) | "61 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/src/pages/engine.astro` heading (line 17) | "63 MCP Tools" | "65 MCP Tools" |
| `/cruxdev-dev/src/pages/docs/index.astro` line 17 | "52 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/src/pages/docs/quickstart.astro` line 54 | "52 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/src/pages/docs/tools/index.astro` line 62 | "61 MCP tools" | "65 MCP tools" |
| `/cruxdev-dev/src/pages/vs/index.astro` line 21 | "'52 tools'" | "'65 tools'" |
| `/cruxdev-dev/src/pages/blog/20260329-crux-ecosystem.md` line 21 | "61 MCP tools" | "65 MCP tools" |

### 2. Test count stale across multiple locations

| File | Current | Should be |
|------|---------|-----------|
| `/cruxdev/README.md` line 40 | "402 tests (384 unit + 18 E2E)" | "520 tests (502 unit + 18 E2E)" |
| `/cruxdev-dev/public/llms.txt` line 9 | "485 tests (467 unit + 18 E2E)" | "520 tests (502 unit + 18 E2E)" |
| `/cruxdev-dev/src/pages/blog/20260329-485-tests.md` | "485 tests: 467 unit + 18 E2E" | Update or mark as historical snapshot |
| `/cruxdev-dev/src/pages/blog/20260329-github-issue-monitoring.md` line 54 | "485 tests" | "520 tests" |
| `/cruxdev-dev/src/pages/blog/20260329-crux-ecosystem.md` line 21 | "485 tests" | "520 tests" |
| `/cruxdev-dev/src/pages/types/composite.astro` line 51 | "485 tests" | "520 tests" |
| `/cruxdev-dev/src/pages/types/open-source.astro` line 53 | "485 tests" | "520 tests" |

### 3. Dimension count contradicts itself across pages

This is the worst inconsistency. Different pages claim different totals:

| File | Claims |
|------|--------|
| `/cruxdev-dev/src/pages/index.astro` meta (line 5) | "137 audit dimensions across 20 dimension sets" |
| `/cruxdev-dev/public/llms.txt` line 11 | "137 audit dimensions across 21 dimension sets" (20 vs 21) |
| `/cruxdev-dev/src/pages/for/software-engineers.astro` line 5 | "39-dimension audit" |
| `/cruxdev-dev/src/pages/for/software-engineers.astro` line 21 | "39 dimensions organized into 10 audit sets" |
| `/cruxdev-dev/src/pages/vs/index.astro` line 13 | "39 dimensions across 10 sets" |
| `/cruxdev-dev/src/pages/vs/*.astro` (6 files) | "39 dimensions" |
| `/cruxdev-dev/src/pages/lp/when-is-code-done.astro` lines 5,41,100 | "14 audit dimensions" / "14 dimensions — 9 for code, 5 for documentation" |
| `/cruxdev-dev/src/pages/lp/one-pass-not-enough.astro` lines 61,127 | "14 dimensions (9 code + 5 documentation)" |
| `/cruxdev-dev/src/pages/lp/ai-doom-loop.astro` line 88 | "14 dimensions" |
| `/cruxdev/README.md` line 5 | "14 quality dimensions (9 code + 5 documentation)" |
| `/cruxdev-dev/src/pages/methodology.astro` | "9 dimensions" (code) + "5 dimensions" (doc) only |

**Fix:** Establish one canonical count. The llms.txt enumerates: 17 form + 9 code + 5 doc + 7 metrics + 9 dashboard + 6 mobile + 6 GEO + 7 MCP server + 7 UI component + 5 color contrast = **78**. If some sets have sub-dimensions totaling 137, document the breakdown. Update ALL pages to use the same number. The old "14" (9+5) pages are especially misleading since forms, metrics, dashboards, mobile, GEO, MCP, UI, and contrast dimensions now exist.

---

## P1 — Missing Citations (credibility risk)

### 4. "~30% miss rate" claim has no source

- **Files:** `/cruxdev/README.md` line 9, `/cruxdev-dev/public/llms.txt` line 5, `/cruxdev-dev/src/pages/methodology.astro` line 25
- **Current:** "Research shows ~30% of issues go undetected in a single pass"
- **Problem:** No citation. What research? This is the core differentiator claim.
- **Fix:** Either cite a specific study (code review defect detection literature, e.g., Fagan 1976, Kemerer & Paulk 2009, or SmartBear's "Best Kept Secrets of Peer Code Review") or reframe as "our internal testing shows" with data.

### 5. Performance and quality benchmarks uncited

- **Files:** Various `/cruxdev-dev/src/pages/vs/*.astro` comparison pages
- **Problem:** Claims about competitor capabilities (e.g., "no convergence detection") are stated as fact without links to competitor docs or release notes that confirm the absence.
- **Fix:** Add footnotes or source links for each competitor capability claim.

---

## P2 — Missing Competitors

### 6. GStack (39K GitHub stars) not covered

- **Current:** No `/cruxdev-dev/src/pages/vs/gstack.astro` exists. Not in `vs/index.astro` competitor list.
- **Fix:** Create `vs/gstack.astro` comparison page. Add GStack to the feature matrix in `vs/index.astro`.

---

## P3 — Misleading Framing

### 7. "Autonomous mode" depends on Claude Code Stop hook

- **Files:** Multiple pages reference autonomous execution without clarifying the dependency.
- **Problem:** CruxDev's autonomous convergence loop requires the host agent (Claude Code) to keep calling `convergence_next_task` without human intervention. This works when Claude Code is in a mode that doesn't stop for confirmation (the "Stop" hook / yolo mode). The site implies CruxDev itself is autonomous, when really the host agent must be configured for it.
- **Fix:** Add a note to `/cruxdev-dev/src/pages/docs/quickstart.astro` and `/cruxdev-dev/src/pages/engine.astro` explaining: "Autonomous mode requires the host agent to be configured for unattended execution (e.g., Claude Code with auto-accept permissions)."

### 8. Model tier routing not enforced in Claude Code

- **Files:** `/cruxdev-dev/public/llms.txt` line 17 ("Model tier routing: micro, fast, local, standard, frontier"), `/cruxdev-dev/src/pages/engine.astro` meta, multiple vs/ pages
- **Problem:** CruxDev defines model tiers but cannot enforce them when running as an MCP server inside Claude Code -- Claude Code picks its own model. The tiers only apply when CruxDev is the orchestrator (via CruxCLI or CruxBot).
- **Fix:** Add caveat to engine.astro and llms.txt: "Model tier routing is available when CruxDev drives the orchestration (CruxCLI, CruxBot). When running as an MCP server inside Claude Code, the host selects the model."

---

## P4 — Tautological Test Problem

### 9. Tests written by same AI that writes code

- **File:** `/cruxdev-dev/src/pages/for/software-engineers.astro` line 15
- **Current:** The page acknowledges this problem ("tests pass -- but they were written by the same AI that wrote the code, so they test what was built rather than what should have been built") but does not explain how CruxDev solves it beyond TDD enforcement.
- **Problem:** TDD enforcement (tests before code) mitigates but does not eliminate tautological tests. The second audit pass can catch tautological tests, but only if the audit prompt specifically checks for them.
- **Fix:** Add explicit mention that the code audit dimensions include "test quality" checking for tautological tests, assertion-free tests, and tests that mirror implementation. If this is not currently an audit dimension, add it. Reference it from the software-engineers page.

---

## P5 — Technical Bugs

### 10. og:image missing from all pages

- **File:** `/cruxdev-dev/src/layouts/Base.astro` (line 23-26 has og:type, og:url, og:title, og:description but no og:image)
- **Problem:** Social shares (X, LinkedIn, Slack, Discord) show no preview image.
- **Fix:** Add `<meta property="og:image" content="https://cruxdev.dev/og-image.png" />` to Base.astro. Create a 1200x630 OG image and place in `/cruxdev-dev/public/og-image.png`.

### 11. RSS date parsing bug -- posts with time component produce "Invalid Date"

- **File:** `/cruxdev-dev/src/pages/blog/rss.xml.ts` line 28
- **Current:** `new Date(post.date + 'T00:00:00').toUTCString()`
- **Problem:** Posts with dates like `"2026-03-28T23:09"` become `"2026-03-28T23:09T00:00:00"` which is invalid ISO 8601. Only posts with date-only format (`"2026-03-27"`) parse correctly.
- **Fix:** Change to:
  ```typescript
  new Date(post.date.includes('T') ? post.date : post.date + 'T00:00:00').toUTCString()
  ```

---

## P6 — Inconsistencies Between README and Website

### 12. README.md vs llms.txt tool count

- `/cruxdev/README.md`: 52 tools
- `/cruxdev-dev/public/llms.txt`: 61 tools (later 63 on llms.txt line 10)
- llms.txt itself is internally inconsistent (61 in key facts vs 63 in line 10)
- **Fix:** All should say 65.

### 13. Binary size inconsistent

- `/cruxdev/README.md` line 42: "5.1MB"
- `/cruxdev-dev/public/llms.txt` line 38: "5.7MB"
- **Fix:** Check actual binary size, update both.

### 14. engine.astro meta says "12 convergence phases" but the phase list shows 11

- **File:** `/cruxdev-dev/src/pages/engine.astro` line 5 and line 13
- **Current:** Meta says "12 convergence phases" but the `pre` block lists: PLANNING, PLAN_AUDITING, DOC_ALIGNMENT, VIABILITY, EXECUTING, CODE_AUDITING, DOC_AUDITING, WEBSITE_CONVERGENCE, E2E_TESTING, PATTERNS_UPDATE, CONVERGED = **11 states** (10 phases + CONVERGED terminal state)
- **Fix:** Reconcile. Count phases vs states consistently.

---

## Summary by priority

| Priority | Count | Impact |
|----------|-------|--------|
| P0 — Stale numbers | 3 issues | Destroys credibility on first visit |
| P1 — Missing citations | 2 issues | Makes core claims unverifiable |
| P2 — Missing competitors | 1 issue | Obvious gap in landscape coverage |
| P3 — Misleading framing | 2 issues | Sets wrong expectations |
| P4 — Tautological tests | 1 issue | Leaves acknowledged weakness unaddressed |
| P5 — Technical bugs | 2 issues | Broken social previews + RSS feeds |
| P6 — Inconsistencies | 3 issues | Erodes trust in attention to detail |
| **Total** | **14 issues** | |
