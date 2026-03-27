# BUILD_PLAN_014: CruxDev Rust Migration

**Created:** 2026-03-26
**Status:** CONVERGED (superseded by BP015)
**Goal:** Rewrite CruxDev from Python to Rust. Single binary, zero runtime dependencies, matching crux's architecture. Same MCP tools, same convergence engine, same behavior — in Rust.

**Source:** 84 Python modules, 11,007 lines → target ~5,000-7,000 lines of Rust
**Tests:** 1,127 Python tests → equivalent `cargo test` coverage
**Reference:** Crux Rust implementation at /Users/user/personal/crux (3,323 lines, rmcp, tokio, serde)

**Rule:** TDD. Tests before code.
**Rule:** Feature parity — every MCP tool that exists in Python must exist in Rust.
**Rule:** Python stays working until Rust passes all equivalent tests. No big-bang cutover.
**Rule:** Single binary: `cruxdev mcp start` for MCP server, `cruxdev status` for health.

---

## Architecture Decisions

### Stack (matching crux)
- **MCP:** `rmcp` with `server`, `transport-io`, `schemars` features
- **Async:** `tokio` full features
- **CLI:** `clap` with derive
- **Serialization:** `serde` + `serde_json`
- **Schema:** `schemars` for MCP tool parameter schemas
- **File walking:** `walkdir`
- **Regex:** `regex`
- **Error handling:** `anyhow`
- **Time:** `chrono`
- **UUIDs:** `uuid` v4
- **Testing:** `tempfile` for tmp dirs

### Module Mapping (Python → Rust)

| Python Module | Rust Module | Priority | Notes |
|---------------|-------------|----------|-------|
| `src/engine/state.py` | `src/engine/state.rs` | P0 | Enums, structs — direct port |
| `src/engine/convergence.py` | `src/engine/convergence.rs` | P0 | Phase order, checks — pure logic |
| `src/engine/persistence.py` | `src/engine/persistence.rs` | P0 | Atomic write, serde |
| `src/engine/plan_validator.py` | `src/engine/plan_validator.rs` | P0 | Regex checks |
| `src/engine/plan_status.py` | `src/engine/plan_status.rs` | P0 | Status line update |
| `src/engine/checklist_parser.py` | `src/engine/checklist.rs` | P0 | Parse + durable mark |
| `src/engine/task_router.py` | `src/engine/router.rs` | P0 | Phase dispatch, auto-discover docs/website |
| `src/engine/wal.py` | `src/engine/wal.rs` | P0 | Append-only JSONL |
| `src/engine/convergence_index.py` | `src/engine/index.rs` | P0 | Plan→run mapping |
| `src/engine/runner.py` | `src/engine/runner.rs` | P1 | Background convergence |
| `src/mcp_server.py` | `src/server.rs` | P0 | All 39+ MCP tools |
| `src/mcp_normalize.py` | `src/normalize.rs` | P0 | Input coercion |
| `src/bus/broker.py` | `src/bus/broker.rs` | P1 | SQLite → rusqlite |
| `src/bus/hook.py` | `src/bus/hook.rs` | P1 | Rate-limited inbox check |
| `src/competitors/*.py` (9 files) | `src/competitors/*.rs` | P2 | Discovery, research, gaps, comparison |
| `src/research/*.py` (7 files) | `src/research/*.rs` | P2 | Sessions, convergence, quality, counter |
| `src/improvement/*.py` (15 files) | `src/improvement/*.rs` | P3 | Self-improvement modules |
| `src/adoption/*.py` (5 files) | `src/adoption/*.rs` | P2 | Classify, inventory, templates, gaps |
| `src/evolution/*.py` (6 files) | `src/evolution/*.rs` | P3 | Gather, evaluate, post, engage |
| `src/dispatch/*.py` (8 files) | `src/dispatch/*.rs` | P3 | LLM dispatch (may simplify) |
| `src/graph/*.py` (1 file) | `src/graph.rs` | P3 | Dependency graph |
| `src/install.py` | `src/install.rs` | P1 | MCP config + gitignore + hooks |
| `src/status.py` | `src/status.rs` | P1 | Health checks |

---

## Phase 0: Project Scaffolding

**Purpose:** Create the Rust project structure alongside the existing Python.

- [x] 0.1 Create Rust project at `cruxdev/rust/` subdirectory during migration (Python keeps `src/`, Rust uses `rust/src/`). On cutover (Phase 7), move `rust/src/` → `src/` and remove Python.
- [x] 0.2 Cargo.toml with all dependencies (match crux's Cargo.toml)
- [x] 0.3 Module structure: `src/engine/`, `src/bus/`, `src/competitors/`, etc.
- [x] 0.4 `main.rs` with clap CLI: `cruxdev mcp start`, `cruxdev status`
- [x] 0.5 CI: `cargo test` + `cargo clippy` in GitHub Actions
- [x] 0.6 Coexistence: Python MCP server stays active. Rust builds at `rust/target/`. Switch .mcp.json only at Phase 7 cutover.

---

## Phase 1: Core Engine (P0 — must work first)

**Purpose:** Port the convergence state machine. This is the heart of CruxDev.

- [ ] 1.1 `engine/state.rs` — ConvergencePhase enum, ConvergenceState struct, Finding, RoundResult
- [ ] 1.2 `engine/convergence.rs` — PHASE_ORDER, advance_phase, check_convergence, check_max_rounds, check_net_negative, record_round, escalate
- [ ] 1.3 `engine/persistence.rs` — atomic write (tempfile + rename), serde serialize/deserialize
- [ ] 1.4 `engine/wal.rs` — append-only JSONL, fsync
- [ ] 1.5 `engine/checklist.rs` — parse_checklist, mark_complete_in_file
- [ ] 1.6 `engine/plan_validator.rs` — regex-based plan validation
- [ ] 1.7 `engine/plan_status.rs` — read/update **Status:** line
- [ ] 1.8 `engine/index.rs` — convergence_index.json, find_active_run, register_run
- [ ] 1.9 `engine/router.rs` — get_next_task, submit_result, phase dispatch, auto-discover docs/website
- [ ] 1.10 `normalize.rs` — input coercion (to_string_list, to_dict_list, normalize_competitors, etc.)
- [ ] 1.11 Tests for all of the above — `cargo test` must pass
- [ ] 1.12 Feature parity verification: run same scenarios as Python tests

---

## Phase 2: MCP Server

**Purpose:** Port all MCP tools to rmcp. This is the interface that Claude Code / CruxCLI connects to.

- [ ] 2.1 `server.rs` — rmcp ServerHandler with tool_router! macro (matching crux pattern)
- [ ] 2.2 Convergence tools: start_convergence, convergence_next_task, convergence_submit_result, convergence_status, convergence_cancel
- [ ] 2.3 Bootstrap tools: get_methodology, get_adoption_process, install_cruxdev
- [ ] 2.4 Planning tools: create_plan_template, validate_plan_structure
- [ ] 2.5 Session bus tools: session_register, session_list, report_issue, report_improvement, share_pattern, notify_breaking_change, check_inbox, acknowledge_message
- [ ] 2.6 Research tools: research_topic, research_status, verify_research_sources, counter_research
- [ ] 2.7 Guided research tools: research_competitor_start, research_competitor_next_step, research_competitor_submit, research_competitor_list
- [ ] 2.8 Competitor tools: setup_competitive_analysis, discover_competitors, research_competitor, verify_competitor_links, generate_gap_analysis, generate_comparison_page, generate_gap_build_plan
- [ ] 2.9 Adoption tools: classify_project, inventory_project, get_templates, analyze_gaps, gap_status
- [ ] 2.10 Status tool: cruxdev_status
- [ ] 2.11 MCP instructions embedded (bootstrap, convergence flow, security rules, session state)
- [ ] 2.12 `cruxdev mcp start` runs the server via stdio transport
- [ ] 2.13 Tests for each tool — verify JSON input/output matches Python behavior

---

## Phase 3: Session Bus (SQLite → rusqlite)

**Purpose:** Port the cross-project messaging system.

- [ ] 3.1 Add `rusqlite` dependency
- [ ] 3.2 `bus/broker.rs` — Broker struct, register_session, send_message, check_inbox, acknowledge
- [ ] 3.3 `bus/hook.rs` — rate-limited inbox check, notification files
- [ ] 3.4 Notification file read/write/clear
- [ ] 3.5 Tests

---

## Phase 4: Competitors + Research + Adoption

**Purpose:** Port the analysis modules.

- [ ] 4.1 `competitors/discovery.rs` — query generation, response parsing
- [ ] 4.2 `competitors/research.rs` — CompetitorProfile, MoatScore, ThreatAssessment, parse
- [ ] 4.3 `competitors/gap_analysis.rs` — feature matrix, classify_gaps
- [ ] 4.4 `competitors/comparison_page.rs` — /vs/ page generation
- [ ] 4.5 `competitors/build_plan_generator.rs` — gap→plan
- [ ] 4.6 `competitors/competitors_doc.rs` — COMPETITORS.md parse/generate
- [ ] 4.7 `competitors/runner.rs` — single-call setup
- [ ] 4.8 `competitors/guided_research.rs` — 5-pass state machine
- [ ] 4.9 `research/session.rs`, `research/convergence.rs`, `research/quality.rs`, `research/counter.rs`, `research/verify.rs`, `research/telemetry.rs`, `research/archive.rs`
- [ ] 4.10 `adoption/classify.rs`, `adoption/inventory.rs`, `adoption/templates.rs`, `adoption/gaps.rs`, `adoption/normalize.rs`
- [ ] 4.11 Tests for all

---

## Phase 5: Improvement + Evolution + Dispatch

**Purpose:** Port the self-improvement and evolution modules.

- [ ] 5.1 `improvement/` — tech_debt, doc_drift, convergence_tuning, test_quality, benchmark, changelog, dep_updates, ci_optimizer, prompt_ab, meta_analysis, agent_evolution, plan_evolution, dimension_priority, release_notes, website_metrics
- [ ] 5.2 `evolution/` — orchestrator, state, gather, evaluate, post, engage
- [ ] 5.3 `dispatch/` — LLM dispatcher trait, stub/anthropic/ollama providers, schema, validation, subagent, credentials
- [ ] 5.4 `graph.rs` — dependency graph (AST analysis — may use `tree-sitter` instead of Python ast)
- [ ] 5.5 Tests for all

---

## Phase 6: Install + Status + CLI

**Purpose:** Port the installation and health check systems.

- [ ] 6.1 `install.rs` — write .mcp.json, copy slash commands, add gitignore patterns, install pre-commit hook
- [ ] 6.2 `status.rs` — 12 health checks
- [ ] 6.3 `cli/mod.rs` — clap subcommands: `mcp start`, `status`, `install`, `converge`
- [ ] 6.4 Single binary: `cargo build --release` → `target/release/cruxdev`
- [ ] 6.5 Update .mcp.json format: command = `target/release/cruxdev`, args = `["mcp", "start"]`
- [ ] 6.6 Tests

---

## Phase 7: Cutover + Cleanup

**Purpose:** Switch from Python to Rust, remove Python code.

- [ ] 7.1 Feature parity audit: every Python MCP tool works identically in Rust
- [x] 7.2 Update all .mcp.json files across crux/cruxcli/cruxdev to point to Rust binary
- [x] 7.3 Update install.py → install.rs behavior (Rust binary writes MCP config)
- [ ] 7.4 Update docs/DEPLOYMENT.md, docs/WEBSITE.md, docs/CruxDev.md
- [ ] 7.5 Update cruxdev.dev website: engine page, metrics
- [ ] 7.6 Remove Python source (src/*.py) — keep tests as reference until Rust tests are confirmed equivalent
- [ ] 7.7 Remove pyproject.toml, pytest config
- [x] 7.8 Update CLAUDE.md: test command = `cargo test`, no more pytest
- [x] 7.9 Final: `cargo build --release` → single binary, all tests pass

---

## Document Alignment

### Product Docs:
- docs/CruxDev.md — engine architecture (update for Rust)
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — test commands (cargo test)
- .claude/CLAUDE.md — update test commands, project structure

---

## Test Commands

```bash
# Rust tests
cargo test

# Rust tests with output
cargo test -- --nocapture

# Clippy lint
cargo clippy -- -D warnings

# Build release binary
cargo build --release

# Python tests (keep running until Phase 7 cutover)
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: update all docs for Rust
- [ ] Website convergence: update engine page, metrics
- [ ] Deployment: update deploy script for Rust binary
- [ ] Patterns update: capture Rust migration learnings
- [ ] Inbox check: notify other sessions of breaking change

## Convergence Criteria

- All Rust tests pass (`cargo test`)
- Zero clippy warnings
- Feature parity: every Python MCP tool has a Rust equivalent
- Single binary builds and runs
- .mcp.json updated to use Rust binary
- Python code removed
- Documentation updated
- Two consecutive clean audit passes

---

## Risk Mitigation

- **SQLite:** `rusqlite` is mature and well-tested. Direct port.
- **MCP:** `rmcp` is the same crate crux uses. Proven.
- **Regex:** Rust `regex` crate is faster than Python's `re`. Direct port.
- **File I/O:** Rust's `std::fs` with `tempfile` for atomic writes. Same pattern as Python.
- **JSON:** `serde_json` is equivalent to Python's `json`. Direct port.
- **Testing:** Rust's `#[test]` + `tempfile` for tmp dirs. Same pattern as pytest fixtures.
- **AST parsing:** Python uses `ast` module for dependency graph. Rust would use `tree-sitter` or skip this module (it's P3 priority).

## Estimated Size

| Phase | Python Lines | Estimated Rust Lines |
|-------|-------------|---------------------|
| 1: Core Engine | 2,000 | 1,500 |
| 2: MCP Server | 1,440 | 1,200 |
| 3: Session Bus | 530 | 400 |
| 4: Competitors/Research/Adoption | 3,160 | 2,400 |
| 5: Improvement/Evolution/Dispatch | 3,300 | 2,000 |
| 6: Install/Status/CLI | 580 | 400 |
| **Total** | **11,010** | **~7,900** |

Rust is typically 70-80% of Python line count for equivalent functionality due to type inference and pattern matching reducing boilerplate, offset by explicit error handling.
