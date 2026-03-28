# Visual Verification Patterns

**Gap:** Cursor has cloud VMs where agents start apps, take screenshots, and verify changes visually.
**Approach:** Start with screenshot-based verification, not full cloud VM.

## Phase 1: Screenshot Capture

After build completes:
1. Detect if project has a web frontend (dev server, build output)
2. Start headless browser (Playwright preferred — cross-platform, fast)
3. Capture screenshots of key pages (homepage, main routes)
4. Store in `.cruxdev/screenshots/<timestamp>/`

```bash
npx playwright screenshot --full-page https://localhost:3000 screenshot.png
```

## Phase 2: Visual Diff

Compare before/after screenshots:
1. Pixel diff (ImageMagick `compare` or pixelmatch)
2. Threshold: >5% pixel difference = significant change
3. Include diff images in convergence findings

```bash
compare before.png after.png -compose src diff.png
identify -verbose diff.png | grep "mean:"
```

## Phase 3: Agent-Driven Interaction (Future)

- Fill forms, click buttons, verify behavior
- Record interaction screenshots at each step
- This is a large capability — defer to later

## Integration Points

- Run after website convergence phase
- Run after deploy (post-deployment visual check)
- Compare against baseline screenshots stored in git

## Tool Requirements

- Playwright (preferred) or Puppeteer
- ImageMagick for diff
- Headless Chrome/Firefox

## When to Skip

- No web frontend in project
- CLI-only tools
- API-only services (use API testing instead)
