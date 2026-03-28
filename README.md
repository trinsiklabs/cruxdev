# CruxDev

**Autonomous convergence framework for AI-driven development.**

CruxDev drives AI coding agents to completion without human re-prompting. It wraps any agent in an audit-fix-re-audit loop, enforcing two consecutive independent clean passes across 137 quality dimensions (21 dimension sets) before declaring work done.

## Why

One clean audit pass catches ~70% of issues. The remaining issues hide behind anchoring bias — the same agent re-checking its own work tends to reproduce the same conclusions. CruxDev's two-pass criterion with structurally independent evaluation addresses this. (Note: the ~30% figure is derived from internal testing on CruxDev's own development — not an externally published benchmark.)

## Install

```bash
# One-liner (macOS / Linux — auto-detects platform)
curl -fsSL https://cruxdev.dev/install.sh | sh
```

Or download from [GitHub Releases](https://github.com/trinsiklabs/cruxdev/releases/latest):
- `cruxdev-aarch64-apple-darwin` (macOS Apple Silicon)
- `cruxdev-x86_64-apple-darwin` (macOS Intel)
- `cruxdev-x86_64-unknown-linux-gnu` (Linux)

No Windows support currently.

## Quick Start

```bash
# Connect to your project (Claude Code)
# Add to .mcp.json:
# { "mcpServers": { "cruxdev": { "command": "cruxdev", "args": ["mcp", "start"] } } }

# Create a build plan
create_plan_template(goal="Add user authentication")

# Converge — engine drives autonomously
start_convergence(plan_file="build_plans/BUILD_PLAN_001.md")
```

The engine handles everything: plan audit, doc alignment, viability check, TDD execution, code audit, doc audit, website convergence, E2E testing, GTV (Ground Truth Verification), and patterns update. Two consecutive clean passes = converged.

## Numbers

- **520 tests** (502 unit + 18 E2E), clippy clean
- **63 MCP tools** — convergence, research, competitors, adoption, git, growth, issue monitoring, GTV scanning
- **137 audit dimensions** across 21 dimension sets
- **55+ pattern documents** (45,000+ lines of stack-specific patterns)
- **Single Rust binary** (5.1MB)

## Safety Gates

- 3 failed attempts → auto-rollback
- 15-minute timeout per task
- 5 rounds max per phase → escalation
- Net-negative detection (findings increasing → escalate)
- Pre-commit: rejects secrets, binaries >1MB, gitignored files
- Pre-push: test gate (all tests must pass)
- Never force push. Never `git add -A`.

## Audit Dimension Sets

**Code (9):** correctness, completeness, edge cases, error handling, security, performance, maintainability, test coverage, duplication

**Documentation (5):** accuracy, completeness, consistency, clarity, currency

**Forms (17):** layout, labels, label positioning, required indicators, input sizing, textarea usage, field grouping, validation, errors, error display, accessibility, mobile, CTA, trust, performance, progressive disclosure, input types

**Metrics (7):** coverage, collection, actionability, thresholds, freshness, anti-gaming, accessibility

**Dashboards (9):** hierarchy, density, visualization, color, real-time, mobile, accessibility, performance, actionability

**GTV (9):** file existence, compilation, test execution, URL accessibility, link integrity, stat accuracy, API connectivity, config validity, claim verification

Plus: MCP Server (7), Skills (7), Content (8), Business (6), Media (6), UI Components (7), Color Contrast (5), Logo (4), Post-Deployment (7), E2E Testing (6), Mobile Web (6), GEO (6), UAT Testing (5), BDD (5), Plan (5)

## How It Works

CruxDev is an **MCP server** — a plugin that runs inside AI coding agents (Claude Code, OpenClaw, or any MCP-compatible tool). The AI agent still writes the code. CruxDev provides the convergence structure: what to audit, how to track progress, when to loop, and when to stop.

The engine is a **deterministic state machine in Rust**. All loops, counters, timeouts, phase transitions, and termination logic are code — not LLM decisions. The LLM is called only for language understanding tasks (evaluating code quality, assessing doc accuracy). This design ensures consistent behavior regardless of which LLM model is connected.

**Note on autonomous mode:** CruxDev's autonomous execution currently works in conjunction with the host agent's infrastructure (e.g., Claude Code's Stop hook). CruxBot (a separate Rust daemon) is being developed for fully independent autonomous operation.

## What CruxDev Is Not

- Not a replacement for Claude Code, Cursor, or Codex — it runs inside them
- Not a visual testing tool — no browser automation or screenshot verification
- Not a process sandbox — no kernel-level isolation (Codex has this)
- Not a multi-agent orchestrator — no parallel subagent dispatch (yet)
- Not an enterprise compliance platform — no SOC2, HIPAA, or SSO (yet)

## Pattern Library

55+ research-converged pattern documents including 18 stack-specific development patterns (Next.js, Rails, Django, FastAPI, SvelteKit, NestJS, Flutter, SwiftUI, Axum, Expo, Blazor, Angular, Vue/Nuxt, Astro, Spring Boot, TALL, KMP, GoTH). All built using the 5-pass research methodology with mandatory adversarial verification.

- [Development Patterns](docs/DEVELOPMENT_PATTERNS_CRUXDEV.md) — Full autonomous lifecycle
- [Research Patterns](docs/RESEARCH_PATTERNS.md) — 5-pass with mandatory contrarian analysis
- [Form Patterns](docs/FORM_PATTERNS.md) — 40+ sources (NNg, Baymard, GOV.UK, WCAG 2.2)
- [BIP Patterns](docs/BIP_PATTERNS.md) — Blog It, Post It — content pipeline
- [GTV Patterns](docs/BIP_PATTERNS.md) — Ground Truth Verification

## Ecosystem

CruxDev is part of the Crux ecosystem:

- **[Crux](https://runcrux.io)** — Intelligence layer
- **[CruxCLI](https://www.cruxcli.io)** — Terminal AI agent
- **[CruxBot](https://cruxbot.io)** — Autonomous daemon (drives CruxDev 24/7)
- **[CruxVibe](https://cruxvibe.io)** — Creator platform (built with CruxDev)

## Links

- **Website:** [cruxdev.dev](https://cruxdev.dev)
- **Patterns:** [cruxdev.dev/patterns](https://cruxdev.dev/patterns)
- **Comparisons:** [cruxdev.dev/vs](https://cruxdev.dev/vs)
- **Blog:** [cruxdev.dev/blog](https://cruxdev.dev/blog)

## Community

- [Report a Bug](https://github.com/trinsiklabs/cruxdev/issues/new)
- [Request a Feature](https://github.com/trinsiklabs/cruxdev/issues/new?labels=enhancement)

## License

MIT — [Trinsik Labs](https://trinsiklabs.com)
