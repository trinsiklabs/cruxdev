# CruxDev — Claude Code Configuration

## Identity

CruxDev is an autonomous convergence framework for AI-driven development. It installs into any project as a Claude Code plugin and drives the agent to convergence without human intervention.

**Owner:** Bryan (splntrb on GitHub), Trinsik Labs
**Repo:** /Users/user/personal/cruxdev

## Core Rules

1. **TDD for everything.** Tests before code. No exceptions without Bryan's explicit approval.
2. **100% test coverage enforced.** `pytest --cov --cov-fail-under=100`. Verify with `--cov-report=term-missing`, not by assertion.
3. **Drive convergence autonomously.** Do not wait for "do it again." Run audit-fix-re-audit loops to termination conditions defined in DEVELOPMENT_PATTERNS_CRUXDEV.md.
4. **Two consecutive independent clean passes = convergence.** One clean pass is not convergence (anchoring bias).
5. **Verify all status claims empirically.** Coverage numbers come from tools, not memory. Checkbox claims are verified before marking.
6. **Atomic writes for all state files.** Write-then-rename for JSON, JSONL, and any critical data.
7. **LLM MINIMIZATION.** If it can be code, it must be code. The engine owns all loops, counters, timeouts, and termination. The LLM is a tool the engine calls for language understanding tasks only. Enforced by architecture tests in `tests/test_architecture.py` and CI.

## Methodology

Read `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md` at the start of any planning session. It contains the full autonomous lifecycle:
- Brainstorming gate → Plan → Converge plan → Execute with safety gates → Converge code+docs → Update patterns → Report

## Test Commands

```bash
# Run Python tests with coverage enforcement
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100

# Run tests only (no coverage)
python3 -m pytest tests/ -v --tb=short
```

## Project Structure

```
cruxdev/
├── src/                    # Source code (Python)
│   ├── engine/             # Convergence engine (state, loops, routing)
│   ├── dispatch/           # LLM dispatch (providers, schema, validation)
│   ├── graph/              # Dependency graph
│   ├── mcp_server.py       # FastMCP server (10 tools)
│   └── install.py          # Project installation
├── tests/                  # Test files (314 tests, 100% coverage)
├── docs/                   # All documentation
├── build_plans/            # All build plans (current and historical)
├── .claude/
│   ├── CLAUDE.md           # This file
│   └── commands/           # Slash commands (/converge, /plan, /adopt, /status)
└── pyproject.toml          # Python project config
```

## Safety Gates

- **Build/test gate:** All tests must pass before any merge
- **Coverage gate:** Coverage must not decrease, target 100%
- **3 failed attempts → auto-rollback** to last known good state
- **15-min timeout per task** to prevent infinite loops

## Session Protocol

1. Read this file (you're doing it now)
2. Check project state: are there pending tasks? Is a convergence loop in progress?
3. If a convergence state file exists, resume from it
4. Load skills on demand based on current phase — do NOT load all skills at start

## Key Files

| File | Purpose |
|------|---------|
| `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md` | Full development methodology — read for any planning work |
| `docs/ADOPTION_PROCESS.md` | How to adopt a project into CruxDev |
| `docs/ADOPTION_PLAYBOOK.md` | Detailed 9-phase adoption playbook |
| `docs/CruxDev.md` | Design document — architecture, engine specs |
| `docs/SESSION_UPGRADE.md` | Guide for leveling up active sessions |
| `docs/WEBSITE_PLANNING.md` | Universal website planning methodology |
| `docs/COMPETITORS_PATTERN.md` | Competitive intelligence methodology |
| `build_plans/CRUX_ECOSYSTEM_PLAN.md` | Master plan — what to build, in what order |

## Conventions

- **All documentation** lives in `docs/`. Not root.
- **All build plans** live in `build_plans/`. Not root. Named `BUILD_PLAN_NNN_SLUG.md`.
- **Every project with competitors** has `docs/COMPETITORS.md`.
- **Every project with a website** has `docs/DEPLOYMENT.md`.
- These conventions apply to every CruxDev-managed project.
