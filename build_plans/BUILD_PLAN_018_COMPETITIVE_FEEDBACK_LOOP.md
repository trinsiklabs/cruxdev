# BUILD_PLAN_018: Competitive Feedback Loop — Build Plan → Analysis → Website

**Status:** NOT STARTED
**Priority:** High
**Depends on:** BUILD_PLAN_015 (competitors pipeline), BUILD_PLAN_016 (git automation)

## Context

When a build plan ships a new capability (e.g. git automation, form audit, issue monitoring), it may change the competitive landscape — new differentiators, closed gaps, new moats. Today this requires manual recognition and manual updates to COMPETITORS.md and the website. The loop doesn't close.

This plan creates a systematic, code-driven process that:
1. Detects whether a completed build plan changes the competitive position
2. Triggers competitive analysis refresh when it does
3. Updates COMPETITORS.md with new differentiators/gaps
4. Regenerates vs/ comparison pages
5. Flows changes through to the project website
6. Posts about the competitive shift (changelog, social)

Not every build plan is competitively relevant — a bug fix or internal refactor isn't. The system must distinguish.

## Phase 1: Competitive Impact Detection

### 1.1 Build plan metadata
- [ ] 1.1.1 Add `competitive_impact` field to build plan template in `create_plan_template` MCP tool
  - Values: `none`, `differentiator`, `gap_closure`, `parity`
  - `differentiator`: new capability no competitor has
  - `gap_closure`: closes a gap from COMPETITORS.md gap analysis
  - `parity`: achieves parity with a competitor on a feature
  - `none`: internal change, no competitive relevance
- [ ] 1.1.2 Parse `competitive_impact` from plan markdown in plan validator

### 1.2 Impact classifier (code-first, LLM-minimal)
- [ ] 1.2.1 New module: `rust/src/competitors/impact.rs`
- [ ] 1.2.2 `classify_impact(plan_content, gap_analysis) -> CompetitiveImpact`
  - Check plan title/content against existing gap names
  - If plan closes a gap → `gap_closure` + gap name
  - If plan adds capability not in any competitor's feature list → `differentiator`
  - If plan matches a competitor feature → `parity`
  - Otherwise → `none`
- [ ] 1.2.3 Code-level heuristics: keyword match against feature matrix features
- [ ] 1.2.4 LLM fallback only for ambiguous cases

### 1.3 Tests
- [ ] 1.3.1 Test: plan that closes "git automation" gap → `gap_closure`
- [ ] 1.3.2 Test: plan that adds unique feature → `differentiator`
- [ ] 1.3.3 Test: internal refactor plan → `none`

## Phase 2: Competitive Analysis Refresh

### 2.1 Targeted update (not full re-research)
- [ ] 2.1.1 `refresh_our_features(project_dir)` — rescan project to update our feature list
- [ ] 2.1.2 `refresh_gap_analysis(our_features, competitors)` — re-run gap analysis with updated features
- [ ] 2.1.3 `update_moat_section(plan_name, impact_type)` — add new moat entry to "Our moat vs them"
- [ ] 2.1.4 Mark closed gaps as `status: closed` with plan reference

### 2.2 COMPETITORS.md update
- [ ] 2.2.1 Regenerate the "Our moat vs them" section for each competitor
- [ ] 2.2.2 Update gap analysis section (mark closed gaps, update feature matrix)
- [ ] 2.2.3 Update "Last Updated" date
- [ ] 2.2.4 Atomic write (write-then-rename)

### 2.3 Tests
- [ ] 2.3.1 Test: gap marked closed after plan convergence
- [ ] 2.3.2 Test: moat section updated with new differentiator
- [ ] 2.3.3 Test: feature matrix reflects new capability

## Phase 3: Website Page Regeneration

### 3.1 Comparison page updates
- [ ] 3.1.1 Regenerate vs/ pages for affected competitors using `generate_comparison_page()`
- [ ] 3.1.2 Only regenerate pages where the feature matrix changed
- [ ] 3.1.3 Write updated pages to website repo (configurable path)

### 3.2 Cross-project sync
- [ ] 3.2.1 `sync_to_website(source_files, website_dir)` — copy updated docs/pages to website repo
- [ ] 3.2.2 Detect website repo path from project config or convention
- [ ] 3.2.3 Use session bus to notify website project of updates

### 3.3 Website build trigger
- [ ] 3.3.1 After sync, trigger website build if build command configured
- [ ] 3.3.2 Verify build succeeds before considering sync complete

## Phase 4: MCP Tools + Pipeline Integration

### 4.1 New MCP tool
- [ ] 4.1.1 `check_competitive_impact(plan_file)` — analyze a plan for competitive impact
- [ ] 4.1.2 Returns: impact type, affected gaps, affected competitors, recommended actions

### 4.2 Convergence lifecycle hook
- [ ] 4.2.1 After CONVERGED state, check competitive impact
- [ ] 4.2.2 If impact != none:
  - Refresh gap analysis
  - Update COMPETITORS.md
  - Regenerate affected vs/ pages
  - Notify website project via session bus
  - Generate competitive-shift changelog entry
  - Generate social post about competitive advantage

### 4.3 Evolution integration
- [ ] 4.3.1 INTEGRATE beat checks competitive impact of completed plans
- [ ] 4.3.2 POST beat includes competitive context in changelog/social when relevant

## Phase 5: Tests

- [ ] 5.1 Unit: impact classifier detects each type correctly
- [ ] 5.2 Unit: gap closure updates feature matrix
- [ ] 5.3 Unit: comparison page regeneration only for affected competitors
- [ ] 5.4 E2E: full flow — plan convergence → impact detection → COMPETITORS.md update → vs/ page regeneration
- [ ] 5.5 E2E: `check_competitive_impact` MCP tool round-trip

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo test --test mcp_e2e -- --nocapture
cd rust && cargo clippy -- -D warnings
```

## The Complete Feedback Loop

```
Build Plan Converged
    │
    ▼
check_competitive_impact(plan_file)
    │
    ├── none → done (no competitive change)
    │
    ├── differentiator / gap_closure / parity
    │       │
    │       ▼
    │   refresh_gap_analysis()
    │       │
    │       ▼
    │   update COMPETITORS.md
    │       │
    │       ▼
    │   regenerate affected vs/ pages
    │       │
    │       ▼
    │   sync_to_website()
    │       │
    │       ▼
    │   session_bus.notify("website", "competitive_update")
    │       │
    │       ▼
    │   generate changelog + social post
    │       │
    │       ▼
    │   git_commit → push → PR (via BP016 tools)
    │
    ▼
Website reflects new competitive position
```

## Key Design Decisions

- **Code-first classification** — keyword matching against feature matrix, not LLM
- **Targeted refresh** — only re-run gap analysis with updated features, don't re-research competitors
- **Selective regeneration** — only update vs/ pages for competitors where the feature matrix changed
- **Session bus for cross-project** — website project notified via existing bus infrastructure
- **Build plan metadata** — `competitive_impact` field makes intent explicit and machine-readable
