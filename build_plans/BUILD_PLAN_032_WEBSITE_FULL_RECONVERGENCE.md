# BUILD_PLAN_032: Website Full Reconvergence to Current CruxDev Standards

**Status:** CONVERGED
**Priority:** Critical
**Depends on:** BUILD_PLAN_028 (content convergence process), BUILD_PLAN_031 (integration competitive analysis)

## Context

The website has drifted from the codebase again. Multiple build plans shipped features, methodology docs, process changes, and integrations without corresponding website updates. The convergence process (BP028 §10.2.1) requires every page read, every claim verified, and no stale content. This plan catches up.

## Audit Findings

| # | Gap | Severity |
|---|-----|----------|
| 1 | No "Integrations" in main nav — buried in docs | CRITICAL |
| 2 | No /integrations index page | CRITICAL |
| 3 | Competitors section doesn't mention integration ecosystems | HIGH |
| 4 | vs/superpowers doesn't mention their integrations (Claude Code, Cursor, Codex, OpenCode, Gemini CLI) | HIGH |
| 5 | vs/ pages don't compare integration depth | HIGH |
| 6 | No Color/Contrast Patterns doc page (doc exists, page doesn't) | HIGH |
| 7 | Test count stale (402→417 after toolchain module) | MEDIUM |
| 8 | Tool count may need verification (52 confirmed) | MEDIUM |
| 9 | llms.txt doesn't mention OpenClaw integration or TypeScript toolchain | MEDIUM |
| 10 | install.sh link on install page — verify it actually serves | MEDIUM |

## Phase 1: Add Integrations to Main Nav

- [ ] 1.1 Add "Integrations" link to Base.astro nav (between Docs and Compare)
- [ ] 1.2 Create `/integrations/index.astro` — lists all current integrations with status
- [ ] 1.3 Move OpenClaw page from /docs/openclaw to /integrations/openclaw (or keep in docs and link from both)
- [ ] 1.4 Add integration cards: Claude Code, OpenClaw, GitHub, Git (all working today)

## Phase 2: Update Competitor Comparison Pages

- [ ] 2.1 vs/superpowers: add integration comparison row (their integrations vs ours)
- [ ] 2.2 vs/yoyo-evolve: add integration comparison
- [ ] 2.3 vs/backbeat: add integration comparison
- [ ] 2.4 vs/deepsource: add integration comparison
- [ ] 2.5 Update feature matrices with integration data
- [ ] 2.6 COMPETITORS.md: add integration tracking per competitor

## Phase 3: Create Missing Doc Pages

- [ ] 3.1 Create /docs/color-contrast page from COLOR_CONTRAST_PATTERNS.md
- [ ] 3.2 Add to docs index under Design Patterns section
- [ ] 3.3 Add link to /docs/growth-strategy from GROWTH_STRATEGY.md (if not exists)

## Phase 4: Update Metrics Everywhere

- [ ] 4.1 Grep for "402" and "393" — update to current test count (417)
- [ ] 4.2 Verify 52 tools is still correct
- [ ] 4.3 Update llms.txt with OpenClaw integration, TypeScript toolchain, multi-platform binaries
- [ ] 4.4 Update homepage hero numbers
- [ ] 4.5 Update footer numbers
- [ ] 4.6 Update engine page if needed

## Phase 5: Full Content Convergence (BP028 §10.2.1)

- [ ] 5.1 Read EVERY page in full
- [ ] 5.2 Verify every technical claim against codebase (ground truth)
- [ ] 5.3 Verify every dimension list matches router.rs
- [ ] 5.4 Verify every number matches test output
- [ ] 5.5 Verify zero hardcoded dark-only color classes (`grep text-dev- src/pages/`)
- [ ] 5.6 Verify "Works today" vs "Roadmap" clearly separated on integration pages
- [ ] 5.7 Two consecutive clean passes

## Phase 6: Build + Deploy + Verify

- [ ] 6.1 npm run build (0 errors)
- [ ] 6.2 Deploy to vh1.trinsik.io
- [ ] 6.3 Verify install.sh serves: `curl -fsSL https://cruxdev.dev/install.sh | head -5`
- [ ] 6.4 Verify no 404s on new pages
- [ ] 6.5 Rebuild release binary if source changed

## Verification

```bash
cd cruxdev-dev && npm run build
curl -fsSL https://cruxdev.dev/install.sh | head -3
curl -s -o /dev/null -w "%{http_code}" https://cruxdev.dev/integrations/
grep -r "text-dev-\|bg-dev-\|text-white\|border-dev-" src/pages/ --include="*.astro"
# Must return 0 results
```
