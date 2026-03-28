# Contributing to CruxDev

CruxDev is an autonomous convergence harness for anything AI builds. Contributions are welcome.

## Getting Started

```bash
# Clone
git clone https://github.com/trinsiklabs/cruxdev.git
cd cruxdev

# Build
cd rust && cargo build

# Test (485 tests)
cargo test

# Lint (zero warnings required)
cargo clippy -- -D warnings
```

## Development Rules

1. **TDD for everything.** Tests before code. No exceptions.
2. **100% test coverage enforced.** All tests must pass.
3. **Zero clippy warnings.** `cargo clippy -- -D warnings` must be clean.
4. **Atomic writes.** All state file operations use write-then-rename.
5. **No API keys in code.** Secrets come from environment variables.
6. **No personal names in docs.** Use organization names.

## How to Contribute

### Report a Bug
[File an issue](https://github.com/trinsiklabs/cruxdev/issues/new?labels=bug) with reproduction steps.

### Suggest a Feature
[File an issue](https://github.com/trinsiklabs/cruxdev/issues/new?labels=enhancement) describing the use case.

### Improve a Patterns Doc
Each patterns doc has a "Report Improvements" section with a labeled issue link (e.g., `patterns:django`, `patterns:nextjs`). Use it.

### Submit Code

1. Fork the repo
2. Create a feature branch
3. Write tests first
4. Implement the feature
5. Run `cargo test && cargo clippy -- -D warnings`
6. Submit a PR with a clear description

### Submit a Stack Patterns Doc

We have 20 stack-specific development patterns docs. If your stack isn't covered:
1. Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` as the model (17 sections, 1000+ lines)
2. Match its depth (pinned versions, project structure, testing, deployment, anti-patterns)
3. Include a "Report Improvements" section with GitHub issue link
4. Label: `patterns:<stack-name>`

## Project Structure

```
cruxdev/
├── rust/src/           # Rust source (convergence engine, MCP server, 61 tools)
├── docs/               # 30+ patterns docs, methodology, research
├── build_plans/        # 90 build plans (current and historical)
├── templates/          # Project templates (18 types, 228 templates)
├── scripts/            # Automation (evolve.sh, convergence_gate.sh)
└── .claude/            # Claude Code config, skills, commands
```

## Architecture

CruxDev is a single Rust binary that runs as an MCP server. It provides 61 tools for convergence, competitive analysis, growth, SEO, content generation, and project management. The convergence engine drives audit-fix-re-audit loops to mathematical completion (two consecutive independent clean passes across 39+ audit dimensions).

## Code of Conduct

Be respectful. Be constructive. Focus on the work.
