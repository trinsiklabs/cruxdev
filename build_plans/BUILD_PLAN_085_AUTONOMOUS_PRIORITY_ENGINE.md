# BUILD_PLAN_085: Autonomous Priority Engine

**Status:** CONVERGED
**Priority:** Critical (enables true autonomous mode)

## Context

Autonomous mode needs a brain. Right now the evolution cycle gathers signals and evaluates them, but it doesn't prioritize across work sources or pick the next thing to do. The priority engine looks at ALL available work — unconverged build plans, open GitHub issues, competitive gaps, self-adoption findings, stale patterns docs — scores each item, and returns the highest-priority work item to execute.

This is the missing piece between "run a cycle" and "self-improve continuously."

## Work Sources (Inputs)

The priority engine scans:

1. **Unconverged build plans** — scan `build_plans/` for `Status: NOT STARTED` or `Status: IN PROGRESS`
2. **Open GitHub issues** — `gh issue list --state open`
3. **Competitive gap closure queue** — from COMPETITORS.md gap table (must-close > should-close > nice-to-have)
4. **Self-adoption findings** — run classifier + dimension check, report gaps
5. **Stale patterns docs** — patterns docs not referenced in engine dimensions (BP078 check)
6. **SEO health issues** — from last `check_seo_health` run
7. **PageSpeed regressions** — from last `check_pagespeed` run
8. **Broken links** — from sitemap link check
9. **Content pipeline backlog** — converged plans without blog posts

## Priority Scoring

Each work item gets a score (lower = higher priority):

| Source | Base Score | Modifiers |
|---|---|---|
| GitHub issue (bug label) | 10 | +0 if HIGH severity in body |
| GitHub issue (enhancement) | 30 | -10 if from paying user |
| Competitive gap (must-close) | 20 | -5 per month gap has been open |
| Competitive gap (should-close) | 40 | |
| Build plan (P0) | 15 | -5 if blocked by nothing |
| Build plan (P1) | 25 | |
| Build plan (P2) | 35 | |
| Build plan (P3) | 50 | |
| Self-adoption finding | 20 | -10 if it's a pattern integration gap |
| SEO health failure | 25 | -15 if broken link (visible to users) |
| Content backlog | 45 | -20 if >5 posts behind |

## Phase 1: Priority Scanner (Rust)

- [ ] 1.1 New module: `rust/src/engine/priority.rs`
- [ ] 1.2 `scan_work_sources(project_dir, github_repo) -> Vec<WorkItem>`
- [ ] 1.3 `WorkItem { source, title, score, description, action, metadata }`
- [ ] 1.4 `score_item(item) -> u32` — apply base score + modifiers
- [ ] 1.5 `pick_next(items) -> WorkItem` — lowest score wins
- [ ] 1.6 Scan build plans: parse frontmatter for Status + Priority
- [ ] 1.7 Scan GitHub issues: `gh issue list` (already in evolution module)
- [ ] 1.8 Scan competitive gaps: parse COMPETITORS.md gap closure queue table
- [ ] 1.9 Scan self-adoption: run dimension integration check
- [ ] 1.10 Scan content backlog: converged plans without matching blog posts

## Phase 2: MCP Tool

- [ ] 2.1 `prioritize_work(project_dir, github_repo)` — returns ranked work list
- [ ] 2.2 Returns: top 10 items with scores, source, and suggested action
- [ ] 2.3 Action types: "converge_plan", "fix_issue", "close_gap", "self_adopt", "fix_seo", "generate_content"

## Phase 3: CLI Integration

- [ ] 3.1 `cruxdev prioritize` — prints ranked work list to terminal
- [ ] 3.2 `cruxdev evolve --auto-prioritize` — picks highest-priority item and works on it
- [ ] 3.3 Wire into evolve.sh cron script

## Phase 4: Autonomous Loop Integration

- [ ] 4.1 When Claude Code session starts with "autonomous mode":
  - Call `prioritize_work` → get top item
  - Execute the action (converge plan, fix issue, close gap, etc.)
  - After completion, call `prioritize_work` again
  - Repeat until no items above threshold score remain
- [ ] 4.2 Add to convergence result: `"next_priority": { top work item }`
- [ ] 4.3 The LLM sees the next priority and continues without re-prompting

## Phase 5: Tests

- [ ] 5.1 Test: build plan scanning finds NOT STARTED plans with correct priority
- [ ] 5.2 Test: scoring applies modifiers correctly
- [ ] 5.3 Test: must-close gaps rank higher than should-close
- [ ] 5.4 Test: bug issues rank higher than enhancements
- [ ] 5.5 Test: pick_next returns lowest-scored item
- [ ] 5.6 Test: empty work sources returns nothing (idle state)

## Phase 6: Content Generation

- [ ] 6.1 Blog post: "How CruxDev Picks What to Work on Next"
- [ ] 6.2 X post announcing autonomous prioritization

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
