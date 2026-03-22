# CruxDev — Origin and Development History

## What CruxDev Is

CruxDev is an autonomous convergence framework for AI-driven software development. It transforms generic LLM coding agents into disciplined, self-correcting developers that drive themselves to convergence without human intervention between "start" and "done."

The core innovation: **the agent drives the audit-fix-re-audit loop autonomously.** No human needs to say "do it again." The convergence engine is built into the framework.

## How It Was Built

CruxDev was synthesized from two sources:

### Source 1: DEVELOPMENT_PATTERNS.md (Battle-Tested Methodology)

A 1,758-line methodology document captured from real development sessions. It documents patterns discovered through iterative collaboration between a developer and Claude Code over multiple rounds of planning, building, auditing, and converging code. Key contributions:

- **The multi-pass audit convergence methodology** — focused audits, full-plan audits, viability assessment, each converging independently through "do it again" loops
- **The two-consecutive-clean-pass rule** — one clean pass is not convergence (anchoring bias)
- **Viability assessment** — checking the actual machine state, not just the plan document
- **Honest checkpoint annotation** — caveats on every checked box, coverage verified empirically
- **Coverage by coincidence detection** — tests that appear to cover code but actually hit different branches
- **The "Big Bang" prompt** — a single prompt that drives entire plan execution + audit + convergence
- **79 issues found across 11 audit passes** — empirical convergence data

This document also captures the PETAL stack variant (DEVELOPMENT_PATTERNS_PETAL.md) with Phoenix/Elixir/Tailwind/Ash/LiveView-specific patterns.

### Source 2: Superpowers Framework (obra/superpowers)

Jesse Vincent's open-source agentic skills framework (93k+ GitHub stars). Key contributions raided for CruxDev:

- **Modular SKILL.md architecture** — composable skills that auto-activate based on context
- **Platform-agnostic design** — shared skill corpus with minimal platform adapters
- **Session-start bootstrap pattern** — <2k token injection that activates the full framework
- **Git worktree isolation** — filesystem persistence and rollback safety
- **Subagent delegation** — fresh context per task, parallel dispatch
- **TDD "Iron Law"** enforcement with persuasion language
- **Systematic debugging** methodology (4-phase root-cause-first)
- **Brainstorming** skill (Socratic questioning before implementation)

### The Synthesis

CruxDev combines these by:

1. Taking the convergence methodology from DEVELOPMENT_PATTERNS and encoding it as composable skills (Superpowers architecture)
2. Adding a **convergence engine** that chains skills into autonomous loops (neither source had this)
3. Adding **prompt patterns as skills** — field-tested prompts that trigger specific agent behaviors
4. Adding **error handling** in the convergence state machine (infrastructure failures, net-negative fix rounds)
5. Making it **stack-agnostic** — methodology applies to any language/framework
6. Supporting **three platforms**: Claude Code (primary), Codex, OpenCode

## Development Timeline

### Session 1: Analysis and Design

1. Read both DEVELOPMENT_PATTERNS docs thoroughly (1,758 + 761 lines)
2. Researched Superpowers GitHub repo — README, skill directory structure, key SKILL.md files (TDD, subagent-driven-development, writing-plans, using-git-worktrees, writing-skills, systematic-debugging, brainstorming)
3. Read academic analysis of Superpowers covering continuous operation mechanisms and cross-platform generalization
4. Designed CruxDev architecture: directory structure, skill taxonomy, convergence engine, platform adapters
5. Wrote initial CruxDev.md (1,181 lines)

### Session 2: First Audit and Fixes

Found and fixed:
- Tailwind 4.x config inconsistency in DEVELOPMENT_PATTERNS_PETAL.md
- 5 CruxDev.md issues: context overflow recovery, net-negative fix rounds, subagent independence, idempotent installation, conflict resolution protocol
- ExMachina incompatibility with Ash resources (anti-pattern documentation)

### Session 3: Reference Cleanup

Removed all project-specific references:
- All "DEVELOPMENT_PATTERNS.md" references → generic language
- All personal name references → "the user" / "the human operator"
- Section 6 rewritten as "Migrating from an Existing Methodology" (generic)
- Redundant migration section merged
- Sections renumbered for clean flow

### Session 4: Superpowers Integration and Improvements

Addressed 10 improvement opportunities:
1. Added `patterns-capture` skill
2. Added `systematic-debugging` skill (adapted from Superpowers)
3. Added `brainstorming` skill (adapted from Superpowers)
4. Expanded conflict resolution protocol (5 prioritized tie-breaking rules)
5. Added token budget field to skill format template
6. Added error handling in convergence state machine (5 error types with recovery)
7. Fleshed out Codex and OpenCode platform adapters
8. Added 7 field-tested prompt patterns with examples
9. Clarified convergence-driving / auditing skill boundary
10. Added diminishing returns policy for low-severity issues

## Files in This Directory

| File | Description |
|---|---|
| `CruxDev.md` | The framework design document — architecture, skills, convergence engine, platform adapters |
| `DEVELOPMENT_PATTERNS.md` | Source methodology document — planning cycles, audit patterns, anti-patterns, session management |
| `DEVELOPMENT_PATTERNS_PETAL.md` | PETAL stack variant — Phoenix/Elixir/Tailwind/Ash/LiveView-specific patterns |
| `ORIGIN.md` | This file — development history and context |

## What's Next

CruxDev.md is a design document. Implementation requires:

1. **Phase 1** (Week 1): Bootstrap file + convergence engine core (master-loop.md, convergence state JSON)
2. **Phase 2** (Week 2): Core skills (convergence-driving, auditing, TDD, planning, executing)
3. **Phase 3** (Week 3): Platform adapters (Claude Code plugin, Codex symlinks, OpenCode JSON)
4. **Phase 4** (Week 4): Supporting skills (viability, honest-tracking, prompt-patterns, debugging, brainstorming)
5. **Phase 5** (Week 5): Validation against a real project, community documentation

The framework is designed to be distributed as a Claude Code plugin, a Codex skill directory, or an OpenCode plugin — identical skill corpus with minimal platform wrappers.
