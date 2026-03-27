# BUILD_PLAN_028: Website Full Content Convergence in WEBSITE_PLANNING.md

**Status:** NOT STARTED
**Priority:** Critical
**Depends on:** BP024 (website patterns), BP027 (website reconvergence failure that exposed this gap)

## Context

BUILD_PLAN_027 was supposed to reconverge cruxdev.dev but failed to read every page's prose content. Python 3.12+ survived as a prerequisite on the quickstart page through an entire "reconvergence." Code audit dimension names were wrong on every landing page, comparison page, and methodology page (said "tests, architecture, style" instead of actual engine dimension names). Dimension count was wrong everywhere (said 8 instead of 9, 13 instead of 14).

This happened because the convergence process for websites treated "website convergence" as structural (missing pages, numbers, dark mode) rather than applying the same 5-dimension doc audit (accuracy, completeness, consistency, clarity, currency) to every page.

**Rule:** Website convergence = doc convergence applied to every page. Every technical claim on every page must be verified against the current codebase. Two consecutive clean passes. No exceptions.

## Phase 1: Add Website Content Convergence Requirements to WEBSITE_PLANNING.md

- [ ] 1.1 New section in Phase 10 (Pre-Launch QA) or new Phase 10.5:
  "Website Content Convergence — every page audited for accuracy against current codebase"
- [ ] 1.2 Requirements:
  - Every page read in full (not just structure/metrics)
  - Every technical claim verified against source code (dimensions, tool counts, technology stack, prerequisites)
  - Every version number, count, or metric verified against test output
  - Every link tested (internal + external)
  - Same 5 doc dimensions applied: accuracy, completeness, consistency, clarity, currency
  - Two consecutive clean passes across ALL pages
- [ ] 1.3 Anti-patterns to document:
  - Structural-only audit (checking page existence, not content)
  - Metric-only update (searching for numbers but not reading prose)
  - Trusting old content from pre-migration eras
  - Assuming landing pages don't need accuracy auditing

## Phase 2: Add to Convergence Engine

- [ ] 2.1 Website convergence phase must include full content audit, not just structure
- [ ] 2.2 Content audit checks:
  - Technology references match current stack (no Python refs in a Rust project)
  - Dimension names match engine constants
  - Tool/test counts match actual values
  - Prerequisites are current
  - FAQ schema content matches page content

## Phase 3: Document the Failure

- [ ] 3.1 Add BP027 failure as a case study in DEVELOPMENT_PATTERNS_CRUXDEV.md anti-patterns
  - What happened: website "reconvergence" that didn't read page content
  - Root cause: structural audit ≠ content convergence
  - Fix: apply doc audit dimensions to every website page

## Verification

- Every page on cruxdev.dev has zero stale claims (verified by full read)
- WEBSITE_PLANNING.md includes content convergence requirements
- DEVELOPMENT_PATTERNS_CRUXDEV.md includes this as an anti-pattern
