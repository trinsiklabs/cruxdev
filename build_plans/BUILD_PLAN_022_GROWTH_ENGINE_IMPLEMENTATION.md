# BUILD_PLAN_022: Autonomous Growth Engine Implementation

**Status:** CONVERGED
**Priority:** Critical
**Depends on:** BUILD_PLAN_019 (strategy), BUILD_PLAN_015 (evolution pipeline), BUILD_PLAN_016 (git automation)

## Context

GROWTH_STRATEGY.md defines the autonomous growth loop. The evolution pipeline (gather/evaluate/post/engage) is built. The git workflow is built. What's missing are the connectors that turn convergence outputs into distribution actions. This plan builds each connector.

## Phase 1: Typefully Integration (Primary posting channel)

- [ ] 1.1 New module: `rust/src/growth/typefully.rs`
- [ ] 1.2 `post_draft(api_key, content, schedule_date, threadify)` — POST to Typefully API
- [ ] 1.3 `compose_release_thread(changelog, version, test_count, tool_count)` — format changelog as X thread
- [ ] 1.4 `compose_tip_post(topic, code_snippet)` — format technical tip
- [ ] 1.5 `compose_build_in_public(plan_name, summary, findings_closed)` — format build update
- [ ] 1.6 API key from env var `TYPEFULLY_API_KEY` (never in config files)
- [ ] 1.7 Rate limit: max 3 posts/day, varied timing
- [ ] 1.8 Tests: compose functions produce valid content, API call mocked

## Phase 2: GitHub Release Creation

- [ ] 2.1 `create_release(repo, tag, title, body)` — `gh release create` via Command
- [ ] 2.2 `generate_release_notes(git_log, plan_name)` — from git diff since last tag
- [ ] 2.3 `get_latest_tag(project_dir)` — read latest semver tag
- [ ] 2.4 Wire into post-convergence: after CONVERGED → create release → post to Typefully
- [ ] 2.5 Tests: release note generation, tag parsing

## Phase 3: README Auto-Optimization

- [ ] 3.1 `optimize_readme(project_dir)` — scan README for outdatable elements
- [ ] 3.2 Update test count badge from `cargo test` output
- [ ] 3.3 Update tool count from MCP server tool listing
- [ ] 3.4 Verify quick-start section exists and is current
- [ ] 3.5 Verify demo section exists (flag if missing)
- [ ] 3.6 Tests: badge update, section detection

## Phase 4: SEO Content Pipeline

- [ ] 4.1 `generate_tutorial(topic, code_examples)` — technical tutorial with working code
- [ ] 4.2 `generate_comparison_post(our_name, competitor, feature_diff)` — from COMPETITORS.md
- [ ] 4.3 `generate_how_we_built(plan_name, convergence_data)` — build-in-public from plan
- [ ] 4.4 Write to website repo (configurable path)
- [ ] 4.5 Tests: content generation format, code block inclusion

## Phase 5: llms.txt Auto-Update

- [ ] 5.1 `update_llms_txt(project_dir, capabilities)` — regenerate llms.txt from current state
- [ ] 5.2 Include: tool count, test count, methodology docs, key features
- [ ] 5.3 Wire into post-convergence: update llms.txt when capabilities change
- [ ] 5.4 Tests: llms.txt format, content freshness

## Phase 6: Metrics Tracking

- [ ] 6.1 `track_growth_metrics(project_dir, repo)` — collect star count, issue count, contributor count via GitHub API
- [ ] 6.2 Store metrics in `.cruxdev/growth/metrics.jsonl` (append-only)
- [ ] 6.3 `growth_report(project_dir)` — summary of trends
- [ ] 6.4 Wire into evolution GATHER beat: collect metrics each cycle
- [ ] 6.5 Tests: metric collection, trend calculation

## Phase 7: MCP Tools + Orchestration

- [ ] 7.1 `post_to_typefully(content, schedule)` — MCP tool for manual posting
- [ ] 7.2 `create_github_release(tag, title)` — MCP tool
- [ ] 7.3 `run_growth_cycle(repo, dry_run)` — full autonomous loop: changelog → release → post → README → llms.txt → metrics
- [ ] 7.4 `growth_status(project_dir)` — current metrics + last cycle results
- [ ] 7.5 All tools dry-run by default

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo test --test mcp_e2e -- --nocapture
cd rust && cargo clippy -- -D warnings
```
