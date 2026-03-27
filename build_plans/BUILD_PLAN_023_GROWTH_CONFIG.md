# BUILD_PLAN_023: Growth Cycle Configuration System

**Status:** CONVERGED
**Priority:** High
**Depends on:** BUILD_PLAN_022 (growth engine)

## Context

The growth engine has no configuration file. API key locations, repo names, posting rates, channel settings, and project metadata are scattered across env vars, hardcoded values, and memory notes. A single configuration file must be the source of truth for all growth cycle settings.

**Rule:** Never store secrets in config files. Config stores the NAME of the env var or the PATH to the secret, not the secret itself.

## Phase 1: Config File Schema

### 1.1 Create `.cruxdev/growth.toml`
- [ ] 1.1.1 Define TOML schema for growth cycle configuration
- [ ] 1.1.2 New module: `rust/src/growth/config.rs`
- [ ] 1.1.3 Load config at growth cycle start, fail clearly if missing
- [ ] 1.1.4 Serde deserialization with defaults for optional fields

### 1.2 Config sections
```toml
[project]
name = "CruxDev"
repo = "trinsiklabs/cruxdev"
url = "https://cruxdev.dev"
description = "Autonomous convergence framework for AI-driven development"

[typefully]
api_key_env = "TYPEFULLY_API_KEY"  # Name of env var, NOT the key
max_posts_per_day = 3
threadify_releases = true
enabled = true

[github]
# Repos this project monitors for issues
repos = ["trinsiklabs/cruxdev"]
issue_monitoring_enabled = true
issue_dry_run = true  # Must be explicitly set to false for live comments
release_creation_enabled = false  # Not yet wired

[content]
website_repo = ""  # Path to website repo for content sync
blog_dir = ""  # Where to write blog posts
vs_dir = "docs/vs"  # Comparison page output

[metrics]
tracking_enabled = true
metrics_file = ".cruxdev/growth/metrics.jsonl"
collection_interval_minutes = 60

[readme]
auto_optimize = true
test_count_source = "cargo test"
tool_count = 49

[llms_txt]
auto_update = true
capabilities = [
    "Multi-dimensional code audit (8 dimensions)",
    "Convergence engine with 2-consecutive-clean-pass criterion",
    "49 MCP tools",
    "5-layer prompt injection defense",
    "Autonomous git workflow with safety gates",
    "Research-converged methodology docs (forms, metrics, dashboards)",
]
methodology_docs = [
    "FORM_PATTERNS.md",
    "METRICS_PATTERNS.md",
    "DASHBOARD_PATTERNS.md",
    "RESEARCH_PATTERNS.md",
    "COMPETITORS_PATTERN.md",
    "GROWTH_STRATEGY.md",
]

[ecosystem]
# All projects sharing this Typefully account
projects = ["crux", "cruxdev", "cruxcli", "cruxvibe"]
shared_typefully = true  # All projects post via same X account
```

## Phase 2: Config Module Implementation

- [ ] 2.1 `load_config(project_dir) -> GrowthConfig` — load from `.cruxdev/growth.toml`
- [ ] 2.2 `resolve_api_key(env_var_name) -> Option<String>` — read env var by name from config
- [ ] 2.3 `validate_config(config) -> Vec<String>` — check required fields, warn on missing
- [ ] 2.4 `create_default_config(project_dir)` — write default config for new projects
- [ ] 2.5 All growth modules read from config instead of hardcoded values

## Phase 3: Wire Into Growth Cycle

- [ ] 3.1 `run_growth_cycle` loads config first, fails if missing
- [ ] 3.2 Typefully posts use config for rate limit, threadify, enabled flag
- [ ] 3.3 Metrics collection uses config for interval, file path
- [ ] 3.4 README check uses config for test/tool counts
- [ ] 3.5 llms.txt uses config for capabilities and methodology docs

## Phase 4: MCP Tool + Tests

- [ ] 4.1 `growth_config(project_dir)` — MCP tool to read/validate config
- [ ] 4.2 `init_growth_config(project_dir)` — MCP tool to create default config
- [ ] 4.3 Unit tests for config loading, validation, API key resolution
- [ ] 4.4 E2E test: growth cycle with config file

## Phase 5: Gitignore Safety

- [ ] 5.1 Verify `.cruxdev/growth.toml` is safe to commit (no secrets, only env var names)
- [ ] 5.2 Pre-commit safety check: reject if config contains actual API keys (pattern match)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
