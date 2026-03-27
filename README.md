# CruxDev

**Autonomous convergence framework for AI-driven development.**

CruxDev drives AI coding agents to completion without human re-prompting. It wraps any agent in an audit-fix-re-audit loop, enforcing two consecutive independent clean passes across 14 quality dimensions (9 code + 5 documentation) before declaring work done.

## Why

One clean audit pass catches ~70% of issues. The other ~30% hide behind anchoring bias — the same agent re-checking its own work finds nothing new. CruxDev's two-pass criterion eliminates this.

## Quick Start

```bash
# Install into your project
install_cruxdev(project_dir=".")

# Create a build plan
create_plan_template(goal="Add user authentication")

# Converge — engine drives autonomously
start_convergence(plan_file="build_plans/BUILD_PLAN_001.md")
```

The engine handles everything: plan audit, doc alignment, TDD execution, code audit (9 dimensions), doc audit (5 dimensions), form audit (9 dimensions if detected), E2E testing, and patterns update. Two consecutive clean passes = converged.

## Numbers

- **402 tests** (384 unit + 18 E2E), 100% coverage
- **52 MCP tools** — convergence, research, competitors, adoption, git, growth, issue monitoring
- **Single Rust binary** (5.1MB)
- **0 clippy warnings**

## Safety Gates

- 3 failed attempts → auto-rollback
- 15-minute timeout per task
- Net-negative detection (findings increasing → escalate)
- Pre-commit: rejects secrets, binaries >1MB, gitignored files
- Pre-push: test gate (all tests must pass)
- Never force push. Never `git add -A`.

## Audit Dimensions

**Code (9):** correctness, completeness, edge cases, error handling, security, performance, maintainability, test coverage, duplication

**Documentation (5):** accuracy, completeness, consistency, clarity, currency

**Forms (9):** layout, labels, validation, errors, accessibility, mobile, CTA, trust, performance

**Metrics (7):** coverage, collection, actionability, thresholds, freshness, anti-gaming, accessibility

**Dashboards (9):** hierarchy, density, visualization, color, real-time, mobile, accessibility, performance, actionability

## Methodology Docs

All research-converged via 5-pass iterative deepening:

- [Form Patterns](docs/FORM_PATTERNS.md) — 40+ sources (NNg, Baymard, GOV.UK, WCAG 2.2)
- [Metrics Patterns](docs/METRICS_PATTERNS.md) — USE, Golden Signals, DORA, SPACE, CHAOSS, FinOps
- [Dashboard Patterns](docs/DASHBOARD_PATTERNS.md) — Tufte, Few, Grafana, Material Design
- [Color & Contrast](docs/COLOR_CONTRAST_PATTERNS.md) — WCAG 2.2, APCA, production hex palettes
- [Research Patterns](docs/RESEARCH_PATTERNS.md) — 5-pass with mandatory contrarian analysis
- [Growth Strategy](docs/GROWTH_STRATEGY.md) — Autonomous AI-driven growth engine
- [Development Patterns](docs/DEVELOPMENT_PATTERNS_CRUXDEV.md) — Full autonomous lifecycle

## Links

- **Website:** [cruxdev.dev](https://cruxdev.dev)
- **Docs:** [cruxdev.dev/docs](https://cruxdev.dev/docs)
- **Comparisons:** [cruxdev.dev/vs](https://cruxdev.dev/vs)

## Community

- [Report a Bug](https://github.com/trinsiklabs/cruxdev/issues/new)
- [Request a Feature](https://github.com/trinsiklabs/cruxdev/issues/new?labels=enhancement)

## License

MIT — [Trinsik Labs](https://trinsiklabs.com)
