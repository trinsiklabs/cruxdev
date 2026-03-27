# BUILD_PLAN_015: Rust Migration Completion + GitHub Issue Monitoring

**Status:** CONVERGED
**Priority:** Critical
**Depends on:** BUILD_PLAN_014 (Rust Migration — structure complete, content gaps remain)

## Context

BUILD_PLAN_014 ported the structure of all 39 MCP tools to Rust with 271 unit tests and 13 E2E tests. However, a convergence audit reveals **24 Python modules with no Rust equivalent** and **6 partial ports**. The migration is 58% complete by module count. Critical subsystems — the evolution orchestrator, LLM dispatch, and competitors content generation — are stubs or entirely missing.

Additionally, a HIGH-severity bug was reported by cruxcli via session bus: `setup_competitive_analysis` writes empty COMPETITORS.md (data loss).

This plan closes all gaps in priority order, then adds GitHub issue monitoring as the capstone feature.

## Audit Findings (11 findings, ordered by severity)

| # | Severity | Module | Finding |
|---|----------|--------|---------|
| F1 | HIGH | competitors/runner.rs:113 | `competitors_doc: String::new()` — empty COMPETITORS.md writes |
| F2 | HIGH | competitors/runner.rs:115 | `comparison_pages: HashMap::new()` — no vs/ pages generated |
| F3 | HIGH | evolution/mod.rs:95-112 | 4 stub functions: gather, evaluate, post, engage all return empty |
| F4 | HIGH | evolution/ | Orchestrator (5-beat cycle) not ported from Python |
| F5 | HIGH | dispatch/ | No real LLM providers (Anthropic/Ollama) — only StubProvider |
| F6 | HIGH | server.rs:929 | `research_status()` tool returns instructions only, no session state |
| F7 | MEDIUM | server.rs:945 | `verify_research_sources()` only checks URL scheme, no HTTP verification |
| F8 | MEDIUM | server.rs:1171 | `verify_competitor_links()` returns all links as "unchecked" |
| F9 | MEDIUM | engine/ | Phase-specific convergence types missing (code/doc/e2e/plan) |
| F10 | MEDIUM | graph/ | Dependency graph module not ported |
| F11 | MEDIUM | research/ | Convergence detection, archive, telemetry, verification not ported |

## Phase 1: Competitors Content Generation (F1, F2)
**Closes:** Session bus HIGH-severity bug from cruxcli

### 1.1 Port COMPETITORS.md generation
- [x] 1.1.1 Implement `generate_competitors_doc()` in `rust/src/competitors/runner.rs`
  - Group profiles by category (official, watch, noted)
  - Render each via existing `CompetitorProfile::to_markdown()`
  - Append `GapAnalysisResult::to_markdown()`
- [x] 1.1.2 Replace `competitors_doc: String::new()` with real output in `setup()`

### 1.2 Port comparison page generation
- [x] 1.2.1 Implement `generate_comparison_page()` in `rust/src/competitors/runner.rs`
  - YAML frontmatter (title, description, slug)
  - Feature comparison table (Y/N)
  - Strengths/weaknesses sections
  - Pricing section
- [x] 1.2.2 Generate pages for official + watch tier competitors in `setup()`
- [x] 1.2.3 Replace `comparison_pages: HashMap::new()` with real output

### 1.3 Tests
- [x] 1.3.1 Unit test: `generate_competitors_doc()` produces non-empty markdown with all sections
- [x] 1.3.2 Unit test: `generate_comparison_page()` produces frontmatter + feature table
- [x] 1.3.3 Unit test: `write_results()` creates non-empty files on disk
- [x] 1.3.4 E2E test: `test_setup_competitive_analysis_writes_files` via MCP

### 1.4 Acknowledge session bus messages
- [x] 1.4.1 Acknowledge `c4b42f8b` (HIGH: empties COMPETITORS.md)
- [x] 1.4.2 Acknowledge `fb1cbfac` (MEDIUM: no pages generated)

## Phase 2: Evolution Module Port (F3, F4)
**Closes:** Autonomous loop — the core value proposition

### 2.1 Port gather beat
- [x] 2.1.1 `gather_own_changes()` — git log via `Command`
- [x] 2.1.2 `gather_github_issues()` — `gh issue list --json` via `Command`
- [x] 2.1.3 `gather_ci_status()` — `gh run list --json` via `Command`
- [x] 2.1.4 `gather_inbox()` — read session bus inbox
- [x] 2.1.5 `gather_all()` — orchestrate all sources

### 2.2 Port evaluate beat
- [x] 2.2.1 `evaluate_issue()` — priority scoring, label-based triage
- [x] 2.2.2 `evaluate_inbox_message()` — classify bus messages
- [x] 2.2.3 `learnings_admission_gate()` — filter non-novel items
- [x] 2.2.4 `evaluate_all()` — orchestrate evaluation

### 2.3 Port integrate beat
- [x] 2.3.1 `integrate()` — apply changes through convergence pipeline
- [x] 2.3.2 Protected file enforcement
- [x] 2.3.3 Dry-run mode (default)

### 2.4 Port post beat
- [x] 2.4.1 `generate_changelog_entry()` — from evaluation results
- [x] 2.4.2 `generate_x_post()` — social content from changes
- [x] 2.4.3 `save_post()` — write to `.cruxdev/evolution/posts/`

### 2.5 Port engage beat
- [x] 2.5.1 `triage_issue()` — code-level classification
- [x] 2.5.2 `add_label()` — `gh issue edit` via `Command`
- [x] 2.5.3 `add_comment()` — `gh issue comment` via `Command`
- [x] 2.5.4 Social isolation enforcement (engage never modifies code)

### 2.6 Port orchestrator
- [x] 2.6.1 `run_cycle()` — 5-beat sequential execution
- [x] 2.6.2 State persistence (append to archive.jsonl, update context.json)
- [x] 2.6.3 Error handling + escalation

### 2.7 Tests
- [x] 2.7.1 Unit tests for each gather function (mock Command output)
- [x] 2.7.2 Unit tests for evaluation logic
- [x] 2.7.3 Unit tests for post generation
- [x] 2.7.4 Integration test: full cycle with test fixtures

## Phase 3: Stub Tool Fixes (F6, F7, F8)
**Closes:** MCP tools that return fake data

### 3.1 Fix research_status tool
- [x] 3.1.1 Persist research sessions to disk (`.cruxdev/research/`)
- [x] 3.1.2 Load session state in `research_status()` — return pass, findings, progress
- [ ] 3.1.3 Test: start research → check status → verify real state

### 3.2 Fix verify_research_sources tool
- [ ] 3.2.1 Add `reqwest` (or lightweight HTTP client) to dependencies
- [ ] 3.2.2 Perform actual HTTP HEAD requests with timeout
- [ ] 3.2.3 Return real status codes
- [ ] 3.2.4 Test: verify known-good and known-bad URLs

### 3.3 Fix verify_competitor_links tool
- [ ] 3.3.1 Reuse HTTP client from 3.2
- [ ] 3.3.2 Return real link verification results
- [ ] 3.3.3 Test: verify links with mixed reachability

## Phase 4: Engine Completeness (F9, F10, F11)
**Closes:** Convergence specialization, dependency analysis, research depth

### 4.1 Phase-specific convergence types
- [ ] 4.1.1 Port code audit dimensions and fix-loop logic
- [ ] 4.1.2 Port doc audit dimensions
- [ ] 4.1.3 Port E2E test execution logic
- [ ] 4.1.4 Port plan validation loop

### 4.2 Dependency graph
- [ ] 4.2.1 Port `DependencyGraph` — build from imports/requires
- [ ] 4.2.2 `impact_set()` — files affected by a change
- [ ] 4.2.3 `audit_context()` — relevant files for an audit

### 4.3 Research convergence
- [ ] 4.3.1 Port 5-pass convergence detection
- [ ] 4.3.2 Port archive persistence (JSONL)
- [ ] 4.3.3 Port verification engine (citation checks)

### 4.4 Tests
- [ ] 4.4.1 Unit tests for each convergence type
- [ ] 4.4.2 Unit tests for dependency graph
- [ ] 4.4.3 Unit tests for research convergence detection

## Phase 5: Dispatch / LLM Integration (F5)
**Closes:** Engine can actually call LLMs

### 5.1 Schema validation
- [ ] 5.1.1 Port `validate_and_retry()` — Pydantic → serde validation
- [ ] 5.1.2 Strict schema enforcement for all LLM outputs

### 5.2 Anthropic provider
- [ ] 5.2.1 Add `reqwest` dependency (if not already from Phase 3)
- [ ] 5.2.2 Implement `AnthropicProvider` — API key from env, streaming, timeout
- [ ] 5.2.3 Model routing: fast (Haiku) / standard (Sonnet) / frontier (Opus)

### 5.3 Tests
- [ ] 5.3.1 Unit tests with StubProvider (existing)
- [ ] 5.3.2 Integration test with real API (gated by env var)

## Phase 6: GitHub Issue Monitoring (New Feature)
**Depends on:** Phases 1-2 (competitors pipeline + evolution module)

### 6.1 Prompt injection defense (5-layer)
- [ ] 6.1.1 `sanitize_issue()` — strip unicode tricks, detect injection patterns
- [ ] 6.1.2 Architectural separation: issue content in separate data section
- [ ] 6.1.3 Schema validation for all LLM-generated responses
- [ ] 6.1.4 Dry-run default: all GitHub actions require explicit `live_mode=true`
- [ ] 6.1.5 Audit trail: log evaluations to `.cruxdev/evolution/issue_audit.jsonl`

### 6.2 Issue evaluation pipeline
- [ ] 6.2.1 Code-level triage (labels → priority, no LLM needed)
- [ ] 6.2.2 Duplicate detection (fuzzy match against recent issues + bus)
- [ ] 6.2.3 Scope routing (cruxdev vs crux vs cruxcli)
- [ ] 6.2.4 Feature request → competitive gap mapping

### 6.3 Response pipeline
- [ ] 6.3.1 Schema-validated responses only (`IssueResponse` struct)
- [ ] 6.3.2 Comment templates (acknowledge, triage, duplicate, wontfix)
- [ ] 6.3.3 Competitive reevaluation trigger on feature requests
- [ ] 6.3.4 Auto-generate BUILD_PLAN from accepted issues

### 6.4 MCP tools
- [ ] 6.4.1 `monitor_issues(repo, dry_run)` — one-shot check + evaluate + respond
- [ ] 6.4.2 `issue_audit_log(limit)` — view recent evaluations

### 6.5 Security tests
- [ ] 6.5.1 Test: injection payload → sanitized, not executed
- [ ] 6.5.2 Test: invalid LLM schema → rejected, escalated
- [ ] 6.5.3 Test: dry_run=true → no `gh` commands executed
- [ ] 6.5.4 Test: rate limit check before polling

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo test --test mcp_e2e -- --nocapture
cd rust && cargo clippy -- -D warnings
```

## Success Criteria

- All 11 audit findings closed
- No stubs or empty returns in production code paths
- E2E tests cover all fixed tools
- GitHub issue monitoring works in dry-run mode
- 0 clippy warnings
- Existing 284 tests still pass (271 unit + 13 E2E)
