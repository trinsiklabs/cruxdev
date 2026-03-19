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

Read DEVELOPMENT_PATTERNS_CRUXDEV.md at the start of any planning session. It contains the full autonomous lifecycle:
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
├── tests/                  # Test files
├── engine/                 # Convergence engine specs (markdown)
├── skills/                 # Skill definitions (markdown)
├── adapters/               # Platform adapters
│   └── claude-code/        # Claude Code integration
├── templates/              # Reusable templates
├── .claude/
│   └── CLAUDE.md           # This file
├── pyproject.toml          # Python project config
├── CRUXDEV.md              # Bootstrap prompt (<2k tokens)
├── CruxDev.md              # Design document
├── CRUX_ECOSYSTEM_PLAN.md  # Master plan
└── DEVELOPMENT_PATTERNS_CRUXDEV.md  # Methodology
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
| `DEVELOPMENT_PATTERNS_CRUXDEV.md` | Full development methodology — read for any planning work |
| `CRUX_ECOSYSTEM_PLAN.md` | Master plan — what to build, in what order |
| `CruxDev.md` | Design document — architecture, skills, engine specs |
| `E2E_TEST_PATTERNS.md` | E2E test convergence methodology — four convergence loops |
| `DEVELOPMENT_PATTERNS.md` | Original methodology (human-driven) — reference only |
