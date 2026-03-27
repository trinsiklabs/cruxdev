# BUILD_PLAN_042: Self-Adoption Audit Findings

**Status:** NOT STARTED
**Priority:** High
**Depends on:** BP041 (convergence enforcement)

## Context

Running CruxDev's own adoption tools against itself revealed 6 gaps and 2 tool bugs. CruxDev doesn't fully pass its own adoption audit.

## Findings

### Tool Bugs

| # | Tool | Issue |
|---|------|-------|
| F1 | classify_project | False-matches on template files — `templates/book/BOOK_OUTLINE.md` triggers "book" project type. Templates are reference material, not project content. |
| F2 | cruxdev_status | Skills count = 0, templates count = 0. Path resolution looks relative to MCP server cwd, not project root. Skills are at `.claude/skills/`, templates at `templates/`. |
| F3 | analyze_gaps | Doesn't find `CLAUDE.md` at `.claude/CLAUDE.md` — only checks root path |

### Missing Docs

| # | File | Severity | Notes |
|---|------|----------|-------|
| F4 | CHANGELOG.md | HIGH | 41 build plans of history, no changelog |
| F5 | docs/ARCHITECTURE.md | HIGH | Rust MCP server architecture not documented |
| F6 | CONTRIBUTING.md | MEDIUM | Required for open source |
| F7 | SECURITY.md | MEDIUM | Required for open source |

## Existing Code Impact

- classify_project needs to exclude `templates/` directory from classification signals
- cruxdev_status needs to resolve paths from project_dir, not server cwd
- analyze_gaps needs to check `.claude/CLAUDE.md` as an alternative path for CLAUDE.md

## Phase 1: Fix Tool Bugs

- [ ] 1.1 classify_project: exclude `templates/` from signal scanning
- [ ] 1.2 cruxdev_status: resolve skills/templates paths from project_dir parameter
- [ ] 1.3 analyze_gaps: check `.claude/CLAUDE.md` as alternative for CLAUDE.md

## Phase 2: Create Missing Docs

- [ ] 2.1 CHANGELOG.md — generate from git log + build plan history
- [ ] 2.2 docs/ARCHITECTURE.md — document Rust MCP server, engine, modules
- [ ] 2.3 CONTRIBUTING.md — how to contribute, development setup, PR process
- [ ] 2.4 SECURITY.md — security policy, vulnerability reporting

## Phase 3: Re-Run Self-Adoption

- [ ] 3.1 Run classify_project — should return software-existing only (no false book/podcast)
- [ ] 3.2 Run cruxdev_status — should show 13 skills, 228 templates
- [ ] 3.3 Run analyze_gaps — should find 0 critical gaps
- [ ] 3.4 Run convergence_gate — must pass

## Verification

```bash
./scripts/convergence_gate.sh
# Plus: self-adoption shows 0 critical gaps, correct skill/template counts
```
