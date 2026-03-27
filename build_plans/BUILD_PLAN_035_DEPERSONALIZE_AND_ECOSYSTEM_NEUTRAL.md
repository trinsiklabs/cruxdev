# BUILD_PLAN_035: Depersonalize Codebase + Ecosystem-Neutral Language

**Status:** NOT STARTED
**Priority:** Critical (blocks public launch)

## Context

Two issues that must be fixed before public distribution:

1. **Personal names in documents.** DEVELOPMENT_PATTERNS_CRUXDEV.md contains "Bryan" 20+ times. Names must never appear in product documentation — the product is not tied to a person.

2. **Ecosystem lock-in language.** The site and docs present Claude Code as the primary/default platform. CruxDev is an MCP server that works with ANY MCP client — Claude Code, OpenClaw, CruxCLI, Cursor (if MCP-enabled), or any future client. No single ecosystem should be elevated above others.

## Rules (Add to convergence process)

### Rule: No personal names in product documentation
- No developer names, owner names, or contributor names in docs, website, or code comments
- Exception: CONTRIBUTORS.md or git commit author (standard open source practice)
- Exception: competitor analysis may reference founders by name when citing their public statements
- Convergence check: `grep -ri "Bryan\|[owner name]" docs/ src/ --include="*.md" --include="*.rs"` must return zero

### Rule: Ecosystem-neutral language
- Never present one MCP client as primary/default
- Always list multiple clients when mentioning integration (e.g., "Claude Code, OpenClaw, CruxCLI, or any MCP-compatible tool")
- Per-client config examples are fine — each in its own labeled section
- No "Install Claude Code" as a prerequisite — prerequisite is "an MCP-compatible tool"
- Each ecosystem's integration details belong in their own page/section, not inline in primary docs

## Phase 1: Fix DEVELOPMENT_PATTERNS_CRUXDEV.md

- [ ] 1.1 Replace "Bryan" with "the user" or "the project owner" throughout
- [ ] 1.2 Rename "§10. Bryan's Collaboration Style" → generic section about user interaction patterns
- [ ] 1.3 Remove any personal preferences that aren't universal patterns
- [ ] 1.4 Keep the patterns themselves — they're good; just depersonalize

## Phase 2: Fix Website — Ecosystem-Neutral

- [ ] 2.1 Install page: "Claude Code" as prerequisite → "An MCP-compatible tool (Claude Code, OpenClaw, CruxCLI, or others)"
- [ ] 2.2 Install page: keep per-client config sections (Claude Code, OpenClaw, Generic) but don't elevate one
- [ ] 2.3 Engine page: "Claude Code, CruxCLI, or any MCP-compatible tool" → standard language
- [ ] 2.4 Landing pages: mentions of "Claude Code" → "your AI coding agent" or "any MCP-compatible tool"
- [ ] 2.5 Integrations page: Claude Code is one integration, not THE integration
- [ ] 2.6 Superpowers comparison: reference Claude Code as a platform both support, not as CruxDev's home

## Phase 3: Fix Docs

- [ ] 3.1 ADOPTION_PROCESS.md: "Hand this file to a Claude Code session" → "Hand this file to any MCP-compatible coding agent"
- [ ] 3.2 GROWTH_RESEARCH.md: Claude Code marketplace references are fine as competitive intelligence (factual)
- [ ] 3.3 CLAUDE.md: This file is specifically for Claude Code — that's correct by design (it's a Claude Code config file)

## Phase 4: Full Codebase Audit

- [ ] 4.1 `grep -ri "bryan" docs/ src/ rust/ --include="*.md" --include="*.rs" --include="*.toml"` → zero results
- [ ] 4.2 Verify Claude Code references are contextual (integration section) not preferential
- [ ] 4.3 Check llms.txt, README.md, growth.toml for name/ecosystem bias
- [ ] 4.4 Check website source: `grep -ri "bryan" src/` → zero
- [ ] 4.5 Two consecutive clean passes on the full audit

## Verification

```bash
# No personal names
grep -ri "bryan" docs/ src/ rust/ build_plans/ --include="*.md" --include="*.rs" | grep -v COMPETITORS | wc -l
# Should be 0

# No Claude Code as sole prerequisite
grep -i "prerequisite.*claude code" docs/ --include="*.md" -l
# Should be 0
```
