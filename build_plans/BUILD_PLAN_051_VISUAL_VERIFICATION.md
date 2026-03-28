# BUILD_PLAN_051: Self-Testing with Visual Verification

**Status:** NOT STARTED
**Priority:** Should Close (Phased)
**Depends on:** BP047 (harness gap closure)
**Competitor:** Cursor (cloud VMs with browser interaction, screenshots, video)

## Context

For frontend/UI work, test suites aren't enough. Cursor agents start apps, interact visually, take screenshots, and verify changes. CruxDev should not try to match Cursor's full cloud VM approach but should add screenshot-based verification.

## Phase 1: Screenshot Capture After Build

- [ ] 1.1 Detect if project has a web frontend (check for build scripts, dev servers)
- [ ] 1.2 After build completes, start headless browser (playwright/puppeteer via CLI)
- [ ] 1.3 Capture screenshots of key pages (homepage, main routes)
- [ ] 1.4 Store screenshots in `.cruxdev/screenshots/<timestamp>/`
- [ ] 1.5 Include screenshot paths in convergence results

## Phase 2: Visual Diff

- [ ] 2.1 Compare before/after screenshots (pixel diff)
- [ ] 2.2 Flag significant visual changes (> threshold % difference)
- [ ] 2.3 Include visual diff results in audit findings
- [ ] 2.4 New MCP tool: `visual_diff(before_dir, after_dir)` → returns diff percentage per page

## Phase 3: Agent-Driven Browser Interaction (Future)

- [ ] 3.1 Use browser interaction to fill forms, click buttons, verify behavior
- [ ] 3.2 Record interaction screenshots at each step
- [ ] 3.3 This is a large capability — defer to later phase

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
