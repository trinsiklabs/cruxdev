# Architecture

## Overview

CruxDev is a single Rust binary (5.1MB) that exposes 52 MCP tools via stdio transport. The engine is a deterministic state machine — all loops, counters, timeouts, and termination logic are pure code. The LLM is a tool the engine calls, not the other way around.

## Module Structure

```
rust/src/
├── server.rs           # MCP server — 52 tool implementations
├── main.rs             # CLI entry point (mcp start, status, install)
├── cli/                # Clap CLI argument parsing
├── engine/             # Convergence engine
│   ├── state.rs        # ConvergenceState struct
│   ├── convergence.rs  # Phase transitions, convergence detection
│   ├── router.rs       # Task routing, dimension constants
│   ├── persistence.rs  # State save/load (atomic writes)
│   ├── wal.rs          # Write-ahead log for crash recovery
│   ├── checklist.rs    # Build plan checklist parsing
│   ├── plan_validator.rs # Plan structure validation
│   ├── plan_status.rs  # Plan status tracking
│   ├── index.rs        # Convergence run indexing
│   ├── test_runner.rs  # Generic subprocess test execution
│   ├── toolchain.rs    # Language toolchain detection + parsers
│   ├── form_detect.rs  # HTML/JSX/TSX form detection
│   └── build_freshness.rs # Stale artifact detection
├── adoption/           # Project adoption
│   ├── classify.rs     # 18 project type classifier
│   ├── inventory.rs    # Project material scanning
│   ├── templates.rs    # Template registry (228 filesystem + 20 built-in)
│   └── gaps.rs         # Gap analysis against templates
├── competitors/        # Competitive intelligence
│   ├── research.rs     # CompetitorProfile with integrations
│   ├── gap_analysis.rs # Feature matrix, gap classification
│   ├── runner.rs       # Single-call analysis orchestration
│   ├── discovery.rs    # Search query generation
│   ├── guided_research.rs # 5-pass guided research state machine
│   └── impact.rs       # Competitive impact detection
├── evolution/          # 5-beat autonomous loop
│   ├── mod.rs          # Gather, evaluate, post, engage, orchestrator
│   └── github.rs       # Issue monitoring, sanitization, evaluation
├── domain/             # Domain architecture
│   └── mod.rs          # Domain config, validation, sub-projects
├── growth/             # Autonomous growth engine
│   ├── config.rs       # growth.toml configuration
│   ├── typefully.rs    # X/Twitter posting
│   ├── releases.rs     # GitHub Release creation
│   ├── readme.rs       # README health checking
│   ├── content.rs      # SEO content generation
│   ├── llms_txt.rs     # AI discoverability
│   └── metrics.rs      # Star/fork/issue tracking
├── dispatch/           # LLM dispatch
│   ├── mod.rs          # Trait, schemas, stub provider
│   └── anthropic.rs    # Anthropic API provider
├── git.rs              # Git operations with safety gates
├── graph.rs            # Dependency graph (import analysis)
├── bus/                # Session bus
│   ├── broker.rs       # SQLite message broker
│   └── hook.rs         # Hook definitions
├── research/           # Research engine
│   ├── session.rs      # Session tracking, checkpointing
│   ├── counter.rs      # Adversarial verification
│   ├── quality.rs      # Quality scoring
│   └── convergence.rs  # 5-pass convergence detection
├── improvement/        # 15 analysis modules
├── install.rs          # Project installation
├── status.rs           # Health status reporting
└── normalize.rs        # Input normalization
```

## Convergence Phases

```
PLANNING → PLAN_AUDITING → DOC_ALIGNMENT → VIABILITY →
EXECUTING → CODE_AUDITING → DOC_AUDITING →
WEBSITE_CONVERGENCE → E2E_TESTING → PATTERNS_UPDATE → CONVERGED
```

## Audit Dimensions

- **Code (9):** correctness, completeness, edge_cases, error_handling, security, performance, maintainability, test_coverage, duplication
- **Doc (5):** accuracy, completeness, consistency, clarity, currency
- **Form (9):** layout, labels, validation, errors, accessibility, mobile, cta, trust, performance
- **Metrics (7):** coverage, collection, actionability, thresholds, freshness, anti_gaming, accessibility
- **Dashboard (9):** hierarchy, density, visualization, color, real_time, mobile, accessibility, performance, actionability
- **Content (8):** accuracy, completeness, clarity, consistency, engagement, structure, voice, citations
- **Business (6):** viability, market_fit, financial_soundness, legal_compliance, competitive_position, scalability
- **Media (6):** content_quality, production_quality, audience_fit, seo, accessibility, consistency
- **MCP Server (7):** tool_design, security, testing, error_handling, performance, documentation, skill_sync
- **Skills (7):** design, description, mcp_sync, testing, safety, documentation, distribution
