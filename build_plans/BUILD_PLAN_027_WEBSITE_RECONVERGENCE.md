# BUILD_PLAN_027: cruxdev.dev Website Reconvergence to Current Standards

**Status:** NOT STARTED
**Priority:** Critical
**Depends on:** BUILD_PLAN_024 (dark mode, quick start/install patterns), BUILD_PLAN_026 (build freshness)

## Context

Gap analysis of cruxdev.dev against current CruxDev methodology docs reveals 73% compliance. 6 critical gaps, 3 high, 1 medium. The site was last converged earlier today but standards evolved faster than the site (14 build plans in one session). This plan brings it to 100%.

## Audit Findings (Prioritized)

| # | Severity | Gap | Standard |
|---|----------|-----|----------|
| F1 | CRITICAL | 3 pattern doc pages referenced but don't exist (form, metrics, dashboard) | WEBSITE_PLANNING Phase 4 |
| F2 | CRITICAL | Metrics stale: 393→402 tests, 50→52 tools in 5 files | GROWTH_STRATEGY §9 |
| F3 | CRITICAL | No dark mode (no toggle, no light palette, no OS detection) | WEBSITE_PLANNING Appendix E |
| F4 | CRITICAL | No dedicated install page (merged into quickstart) | WEBSITE_PLANNING Appendix F |
| F5 | CRITICAL | Quickstart doesn't follow 7-section structure | WEBSITE_PLANNING Appendix G |
| F6 | HIGH | No light mode color palette defined | WEBSITE_PLANNING Appendix E §E.3 |
| F7 | HIGH | No theme toggle UI component | WEBSITE_PLANNING Appendix E §E.4 |
| F8 | HIGH | No FOUC prevention script | WEBSITE_PLANNING Appendix E §E.5 |
| F9 | MEDIUM | No FAQPage schema on comparison pages | WEBSITE_PLANNING Phase 5 |

## Phase 1: Create Missing Pattern Doc Pages (F1)

- [ ] 1.1 Create `src/pages/docs/form-patterns.astro` — render key sections from FORM_PATTERNS.md
- [ ] 1.2 Create `src/pages/docs/metrics-patterns.astro` — render key sections from METRICS_PATTERNS.md
- [ ] 1.3 Create `src/pages/docs/dashboard-patterns.astro` — render key sections from DASHBOARD_PATTERNS.md
- [ ] 1.4 Verify docs index links work (no 404s)

## Phase 2: Update Metrics (F2)

- [ ] 2.1 index.astro: 393→402 tests, 50→52 tools
- [ ] 2.2 Base.astro footer: 393→402, 50→52
- [ ] 2.3 llms.txt: 393→402 tests, 50→52 tools
- [ ] 2.4 engine.astro: 50→52 tools

## Phase 3: Dark Mode (F3, F6, F7, F8)

### 3.1 Color system
- [ ] 3.1.1 Define light mode tokens in global.css (bg: #ffffff, text: #1a1a1a, secondary: #666)
- [ ] 3.1.2 Keep existing dark tokens as `.dark` class overrides
- [ ] 3.1.3 Tailwind dark: variant for component-level overrides

### 3.2 FOUC prevention
- [ ] 3.2.1 Inline blocking script in Base.astro `<head>` BEFORE stylesheets
- [ ] 3.2.2 Read localStorage('theme'), check prefers-color-scheme, apply .dark class

### 3.3 Toggle component
- [ ] 3.3.1 Three-state button in header: system (monitor) / light (sun) / dark (moon)
- [ ] 3.3.2 localStorage persistence
- [ ] 3.3.3 matchMedia listener for OS changes in system mode

### 3.4 Color scheme meta
- [ ] 3.4.1 `<meta name="color-scheme" content="light dark">`
- [ ] 3.4.2 `:root { color-scheme: light dark; }` in CSS

### 3.5 Accessibility
- [ ] 3.5.1 Verify WCAG AA contrast in both modes
- [ ] 3.5.2 No color-only indicators

## Phase 4: Dedicated Install Page (F4)

- [ ] 4.1 Create `src/pages/docs/install.astro`
- [ ] 4.2 Structure per Appendix F:
  1. Prerequisites (Claude Code or MCP-compatible tool)
  2. Install command (one-liner, copy button)
  3. Verification (`cruxdev status` or similar)
  4. Link to quickstart
- [ ] 4.3 Add to docs index and navigation

## Phase 5: Restructure Quickstart (F5)

- [ ] 5.1 Rewrite `src/pages/docs/quickstart.astro` per Appendix G 7-section structure:
  1. One-sentence description
  2. Prerequisites (with versions)
  3. Install (link to install page)
  4. Initialize/Setup (create build plan)
  5. First meaningful action (start convergence)
  6. See the result (expected output shown)
  7. What's next (3-5 links)
- [ ] 5.2 All code blocks copy-paste runnable with file paths
- [ ] 5.3 TTFV target: < 5 minutes

## Phase 6: FAQPage Schema on Comparison Pages (F9)

- [ ] 6.1 Add `<script type="application/ld+json">` FAQPage schema to each vs/ page
- [ ] 6.2 Generate from existing FAQ data already in the page source

## Phase 7: Build + Deploy + Freshness

- [ ] 7.1 `npm run build` — verify 0 errors
- [ ] 7.2 Deploy to vh1.trinsik.io via rsync
- [ ] 7.3 Verify all new pages load (no 404s)
- [ ] 7.4 Run `check_build_freshness` on cruxdev repo — verify binary current

## Verification

```bash
# Website
cd /Users/user/personal/cruxdev-dev && npm run build
# No 404s for new pages
curl -s -o /dev/null -w "%{http_code}" https://cruxdev.dev/docs/form-patterns/
curl -s -o /dev/null -w "%{http_code}" https://cruxdev.dev/docs/install/

# CruxDev engine
cd /Users/user/personal/cruxdev/rust && cargo test && cargo clippy -- -D warnings
```
