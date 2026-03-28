# BUILD_PLAN_097: Crux Bot — New Project

**Status:** NOT STARTED
**Priority:** Critical (THE product)

## What Is Crux Bot

The CruxDev convergence engine + Anthropic API client + daemon loop = **Crux Bot**.

A single Rust binary that runs continuously, monitors your projects, and drives everything to convergence. No Claude Code session. No MCP client. Just the bot.

## Architecture

```
crux-bot (Rust binary)
├── core/           # The daemon loop (CHECK → PRIORITIZE → CONVERGE → DELIVER)
├── engine/         # Convergence engine (from CruxDev — state machine, dimensions, phases)
├── llm/            # Direct Anthropic API client (not MCP — direct HTTP)
├── tools/          # File system, git, test runner, build commands
├── priority/       # Work source scanning, scoring (from CruxDev)
├── content/        # Blog + X post generation, Typefully (from CruxDev)
├── monitor/        # GitHub issues, webhooks, file watchers
├── stream/         # Terminal viewer SSE (from CruxDev)
├── safety/         # Rate limits, cost tracking, STOP file, max actions
└── config/         # Multi-project config, growth.toml, per-project settings
```

## Relationship to Existing Projects

| Project | Role | Status |
|---|---|---|
| **Crux** | Intelligence layer (modes, memory, safety) | Existing |
| **CruxCLI** | Terminal AI coding agent (TUI) | Existing |
| **CruxDev** | Convergence engine (MCP server) | Existing — engine extracts here |
| **Crux Bot** | Autonomous convergence daemon | **NEW** |
| **CruxVibe** | Commercial vibecoding platform | Planned |

Crux Bot consumes CruxDev's engine as a library. CruxDev remains available as an MCP server for interactive use. Crux Bot is the autonomous/headless mode.

## Phase 1: Project Setup

- [ ] 1.1 Create repo: `/Users/user/personal/crux-bot`
- [ ] 1.2 Cargo workspace with shared engine crate
- [ ] 1.3 Extract convergence engine from CruxDev into shared crate
- [ ] 1.4 CruxDev depends on shared crate (no code duplication)
- [ ] 1.5 Crux Bot depends on shared crate + adds daemon + LLM client

## Phase 2: LLM Client

- [ ] 2.1 Direct Anthropic API client (reqwest + serde)
- [ ] 2.2 Messages API: create message, handle streaming
- [ ] 2.3 Tool use: define tools, parse tool calls, return results
- [ ] 2.4 Rate limiting: respect 429s, exponential backoff
- [ ] 2.5 Cost tracking: per-call, per-convergence, daily budget cap
- [ ] 2.6 Model routing: frontier/standard/fast based on task tier

## Phase 3: Core Loop

- [ ] 3.1 Daemon mode: `crux-bot run --config bot.toml`
- [ ] 3.2 The loop: scan → prioritize → converge → deliver → reflect
- [ ] 3.3 LLM calls for: planning, auditing, fixing, content generation
- [ ] 3.4 File operations: direct read/write/edit (no MCP)
- [ ] 3.5 Git operations: commit/push/PR (already in CruxDev)
- [ ] 3.6 Test execution: run cargo test, npm test, etc.

## Phase 4: Configuration

```toml
# bot.toml
[bot]
name = "crux-bot"
api_key_env = "ANTHROPIC_API_KEY"
model = "claude-sonnet-4-6"
frontier_model = "claude-opus-4-6"
fast_model = "claude-haiku-4-5"
daily_budget_usd = 10.0
max_actions_per_cycle = 20

[[projects]]
name = "cruxdev"
dir = "/Users/user/personal/cruxdev"
repo = "trinsiklabs/cruxdev"
enabled = true

[[projects]]
name = "crux"
dir = "/Users/user/personal/crux"
repo = "trinsiklabs/crux"
enabled = true

[[projects]]
name = "cariance"
dir = "/Users/user/personal/cariance"
repo = ""
enabled = false  # No GitHub repo yet
```

## Phase 5: Safety

- [ ] 5.1 All changes on feature branches (never main directly)
- [ ] 5.2 PRs created, not merged (human approves)
- [ ] 5.3 Daily budget cap (default $10/day)
- [ ] 5.4 Max 20 file modifications per cycle
- [ ] 5.5 STOP file for emergency halt
- [ ] 5.6 Health endpoint for monitoring
- [ ] 5.7 Audit log of every LLM call and file modification

## The Tagline

**Crux Bot: The autonomous convergence agent. Runs continuously. Drives everything to done.**
