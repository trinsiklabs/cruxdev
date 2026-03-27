# BUILD_PLAN_031: Integration Tracking in Competitive Analysis + Website Nav

**Status:** NOT STARTED
**Priority:** High
**Depends on:** BUILD_PLAN_007 (competitors system), BUILD_PLAN_018 (competitive feedback loop)

## Context

Competitor integrations are a major competitive dimension we're not tracking. Superpowers integrates with Claude Code, Cursor, Codex, OpenCode, Gemini CLI. OpenClaw has 13,700+ skills on ClawHub, native MCP, and 23+ messaging channels. We track features and moats but not integration ecosystems — which is often the deciding factor for adoption.

Integrations are also a selling point that belongs in main site navigation, not buried in docs.

## Phase 1: Add Integration Tracking to Competitive Analysis

- [ ] 1.1 Add `integrations` field to CompetitorProfile struct in `research.rs`
  - List of integration names, categories, depth (native/plugin/API/community)
- [ ] 1.2 Add integration comparison to gap analysis
  - Our integrations vs each competitor's integrations
  - Gap classification: must-close (competitor has, we don't), parity, differentiator
- [ ] 1.3 Add to COMPETITORS_PATTERN.md: integration tracking methodology
- [ ] 1.4 Update COMPETITORS.md with integration data for each tracked competitor
- [ ] 1.5 Auto-generate integration gap build plans (existing generate_gap_build_plan tool)

## Phase 2: Add Integration Section to Feature Matrix

- [ ] 2.1 Extend feature matrix in gap_analysis.rs to include integration entries
- [ ] 2.2 Comparison pages (vs/) show integration comparison table
- [ ] 2.3 Our integrations page shows what we integrate with and at what depth

## Phase 3: Website — Integrations in Main Nav

- [ ] 3.1 Add "Integrations" to main nav in Base.astro (between Docs and Compare)
- [ ] 3.2 Create `/integrations` index page listing all integrations
- [ ] 3.3 Each integration gets its own page (like /docs/openclaw)
- [ ] 3.4 Integration pages follow ground truth verification — only claim what's implemented
- [ ] 3.5 Structure per integration page:
  - What is [Platform]
  - How CruxDev integrates (with actual config/code)
  - What you get (verified claims only)
  - Roadmap (clearly separated from current)
  - Get Started

## Phase 4: Current Integrations Inventory

Audit what CruxDev actually integrates with today:

| Platform | Status | How |
|----------|--------|-----|
| Claude Code | Working | MCP server via .mcp.json |
| Any MCP client | Working | stdio transport, 52 tools |
| OpenClaw | Working | MCP stdio in openclaw.json |
| GitHub | Working | gh CLI for issues, PRs, releases |
| Git | Working | git CLI for commit, push, branch |
| Typefully | Broken | API auth issue, needs MCP transport update |

## Phase 5: Tests

- [ ] 5.1 Unit test: CompetitorProfile serializes integrations field
- [ ] 5.2 Unit test: gap analysis includes integration gaps
- [ ] 5.3 Website build succeeds with new pages

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
cd cruxdev-dev && npm run build
```
