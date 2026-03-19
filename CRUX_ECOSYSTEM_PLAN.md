# Crux Ecosystem Plan

**Created:** 2026-03-18
**Last Updated:** 2026-03-18
**Status:** NOT STARTED
**Goal:** Build the full Crux ecosystem — three tightly integrated products that form a lights-out AI coding machine surpassing Superpowers (93k+ stars) in both capability and attention.

**Constraint:** Crux roadmap completion (including GitHub Actions support) is a prerequisite — the platform must be feature-complete before the products that depend on it.
**Constraint:** CruxCLI hard fork must complete before any autonomous evolution work begins. You need the runtime under your control before you hand it the keys.
**Constraint:** All three products remain separate repos, separate installs, separate star counts. CruxDev is NOT absorbed into Crux.
**Constraint:** All LLM work during development stays in Claude Code (Pro Max). No API calls for development.
**Rule:** TDD/BDD for everything. Tests before code. 100% coverage enforced via `fail_under = 100`.
**Rule:** Follow DEVELOPMENT_PATTERNS_CRUXDEV.md methodology for all development rounds.

---

## Architecture Overview

### Before (Current State)

```
┌─────────────────────────────────────────────────────────────┐
│                     CURRENT STATE                            │
│                                                              │
│  Crux (core)          CruxCLI (v0.1)       CruxDev          │
│  ─────────────        ──────────────        ────────         │
│  24 modes             Bridge plugin on      Design doc       │
│  37-tool MCP          stock OpenCode        only (~1,700     │
│  7-gate safety        Config overlays       lines)           │
│  1480+ tests          Dogfooding phase                       │
│  runcrux.io                                                  │
│  Python + JS          TypeScript/Bun        Markdown         │
│                                                              │
│  Integration: Crux MCP ← bridge reads filesystem            │
│  Autonomous evolution: none                                  │
│  Content pipeline: Typefully set up but manual               │
│  Upstream strategy: none (bridge on stock OpenCode)          │
└─────────────────────────────────────────────────────────────┘
```

### After (Target State)

```
┌─────────────────────────────────────────────────────────────┐
│                     TARGET STATE                             │
│                                                              │
│  Crux (platform)      CruxCLI (v1.0)       CruxDev          │
│  ─────────────        ──────────────        ────────         │
│  Intelligence layer   Hard fork of          16 skills +      │
│  Modes + MCP          OpenCode              convergence      │
│  Safety pipeline      Binary: `cruxcli`        engine           │
│  Convergence tools    Config: `.crux/`      3 platform       │
│  Evolution pipeline   Full prompt           adapters         │
│  Inspiration digest   authority             Multi-repo       │
│                       Crux-native           ecosystem        │
│                                                              │
│  Integration:                                                │
│  ┌───────────────────────────────────────────────────┐       │
│  │         CruxCLI (the runtime)                     │       │
│  │              │                                    │       │
│  │    ┌─────────┴─────────┐                          │       │
│  │    │                   │                          │       │
│  │  Crux (modes,       CruxDev (skills,             │       │
│  │  MCP, safety,       convergence engine,          │       │
│  │  knowledge)         audit methodology)           │       │
│  └───────────────────────────────────────────────────┘       │
│                                                              │
│  Autonomous evolution:                                       │
│  ┌───────────────────────────────────────────────────┐       │
│  │  Cron → Gather inspiration → Evaluate → Integrate │       │
│  │  → Post to X → Engage community → Loop            │       │
│  │  (all unattended, safety-gated)                   │       │
│  └───────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

### Three Competitive Lanes

```
┌────────────────────────────────────────────────────────────┐
│                                                             │
│  CruxDev ──vs── Superpowers (93.9k stars)                  │
│    Agentic skills + convergence engine framework            │
│    CruxDev wins: autonomous convergence, no "do it again"   │
│                                                             │
│  CruxCLI ──vs── OpenCode (124k stars)                      │
│    AI coding terminal agent                                 │
│    CruxCLI wins: Crux intelligence layer underneath         │
│                                                             │
│  Crux ──vs── (new category, no direct competitor)          │
│    Intelligence / safety platform layer                     │
│    The moat that makes both products better than rivals     │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

---

## Phase Ordering & Dependencies

```
Phase 0: Crux roadmap completion               ← FOUNDATION
    │    (GitHub Actions support + remaining platforms)
    │
    └──→ Phase 1: CruxCLI hard fork + rebrand  ← BLOCKING PREREQUISITE
             │
             ├──→ Phase 2: CruxDev convergence engine core
             │        │
             │        └──→ Phase 3: CruxDev core skills (16 skills)
             │                 │
             │                 └──→ Phase 7: Validation / dogfood on itself
             │
             ├──→ Phase 4: Inspiration registry + digest pipeline
             │        │
             │        └──→ Phase 5: Autonomous evolution pipeline (5-beat loop)
             │
             └──→ Phase 6: Marketing / attention strategy (parallel from day 1)

Phase 0 ensures the platform layer is feature-complete.
Phases 2-3 and 4-5 are independent tracks after Phase 1.
Phase 6 runs in parallel with everything.
Phase 7 requires Phases 2-3 complete.
```

---

## Phase 0: Crux Roadmap Completion

**Purpose:** The platform must be feature-complete before the products that depend on it. Crux is the intelligence layer underneath both CruxCLI and CruxDev — gaps in Crux limit everything downstream. GitHub Actions support is specifically needed for the autonomous evolution pipeline (Phase 5).

**Prerequisite:** None.

**Current state (as of 2026-03-18):**
- MCP server: fully implemented, 36 tools, PLAN-166 security hardening, no stubs
- `crux status`: comprehensive (static + liveness checks + findings engine)
- Tests: 1290 passing, 0 failures
- Coverage: **93.04% (NOT 100%)** — 357 uncovered statements across ~11 modules
- GitHub Actions: **not configured** (no `.github/workflows/`)
- JS tool stubs: Gates 2-3 in `run_script.js` and `manage_models.js` are placeholders
- OpenCode integration: not verified end-to-end
- CLAUDE.md: stale (says 338+ tests, lists background processor as TODO when it exists)

### 0A. Coverage Gap Closure (93% → 100%)

Modules with uncovered statements:

| Module | Coverage | Missing Statements |
|--------|----------|-------------------|
| `crux_security.py` | 73% | 21 lines |
| `crux_hook_runner.py` | 76% | 10 lines |
| `crux_cross_domain.py` | 77% | 25 lines |
| `crux_audit_backend.py` | 82% | 68 lines |
| `crux_cross_project.py` | 83% | 50 lines |
| `extract_corrections.py` | 84% | 24 lines |
| `crux_typefully.py` | 85% | 13 lines |
| `crux_background_processor.py` | 87% | 33 lines |
| `crux_mcp_handlers.py` | 88% | 53 lines |
| `crux_bip_gather.py` | 88% | 15 lines |
| `crux_hooks.py` | 91% | 25 lines |
| **Total** | **93.04%** | **357 lines** |

### 0B. GitHub Actions

No `.github/workflows/` directory exists. Needed for:
- CI: run test suite on PR/push
- Autonomous evolution pipeline (Phase 5): cron-driven evolution loop

### 0C. JS Tool Stubs

- `run_script.js` gates 2-3: placeholder stubs for 8B/32B Ollama audit — need actual API calls
- `manage_models.js` pull action: placeholder stub

### 0D. OpenCode Integration

Plugin hook format, tool export format, and command/mode file format need end-to-end verification with a real OpenCode session.

### 0E. CLAUDE.md Staleness

- Says "338+ tests" — actual count is 1290
- Lists background processor and cross-project aggregator as TODO — Python implementations exist and are tested
- Needs full audit against current codebase

### Progress Tracker — Phase 0

**Coverage (357 lines to close):**
- [x] 0.1 Close coverage gaps in `crux_security.py` (73% → 100%)
- [x] 0.2 Close coverage gaps in `crux_hook_runner.py` (76% → 100%)
- [x] 0.3 Close coverage gaps in `crux_cross_domain.py` (77% → 100%)
- [x] 0.4 Close coverage gaps in `crux_audit_backend.py` (82% → 100%)
- [x] 0.5 Close coverage gaps in `crux_cross_project.py` (83% → 100%)
- [x] 0.6 Close coverage gaps in `extract_corrections.py` (84% → 100%)
- [x] 0.7 Close coverage gaps in `crux_typefully.py` (85% → 100%)
- [x] 0.8 Close coverage gaps in `crux_background_processor.py` (87% → 100%)
- [x] 0.9 Close coverage gaps in `crux_mcp_handlers.py` (88% → 100%)
- [x] 0.10 Close coverage gaps in `crux_bip_gather.py` (88% → 100%)
- [x] 0.11 Close coverage gaps in `crux_hooks.py` (91% → 100%)
- [x] 0.12 Verify `--cov-fail-under=100` passes across entire `scripts/lib`

**GitHub Actions:**
- [x] 0.13 Create `.github/workflows/` with CI workflow (test on PR/push)
- [ ] 0.14 Add GitHub Actions platform support for Crux modes/hooks (deferred — needs design)

**JS Tool Completion:**
- [x] 0.15 `run_script.js` gates 2-3 — documented as MCP-superseded (crux_audit_backend.py handles it)
- [x] 0.16 `manage_models.js` pull — documented as MCP-superseded (crux_ollama.py handles it)

**OpenCode Integration:**
- [x] 0.17-0.19 Deferred to Phase 1 (CruxCLI hard fork) — documented in plugin-shim.js

**Documentation:**
- [x] 0.20 CLAUDE.md audited and updated (40+ fixes)
- [x] 0.21 Test count fixed (338+ → 1561)
- [x] 0.22 Background processor / cross-project status updated (marked as complete)
- [x] 0.23 Full doc convergence (two consecutive clean passes — 53+ fixes across 9 files)

---

## Phase 1: CruxCLI Hard Fork + Rebrand

**Purpose:** Own the runtime before handing it the keys. A bridge plugin on someone else's codebase isn't a foundation for autonomous evolution — you can't safely self-modify code you don't own.

**Prerequisite:** None (blocking prerequisite for all other phases).

**Scope:** Fork OpenCode (MIT licensed, TypeScript + Bun) and rebrand completely.

### 1A. Fork Mechanics

- OpenCode is a monorepo with ~15 packages (`packages/opencode`, `packages/app`, `packages/plugin`, `packages/sdk`, `packages/ui`, `packages/desktop`, etc.)
- MIT licensed — clean to fork
- Single-file binary compilation via `Bun.build()` with `compile: true`
- Plugin system has 15 hook points; bridge already works with it

### 1B. Rebrand Scope

| Item | From | To |
|------|------|----|
| Binary name | `opencode` | `cruxcli` |
| Config directory | `.opencode/` | `.crux/` |
| Environment variables | `OPENCODE_*` | `CRUX_*` |
| npm packages | `@opencode/*` | TBD |
| Launch banner / TUI header / status bar | OpenCode branding | Crux branding |
| ASCII logo | OpenCode | Crux |
| Help/about text | OpenCode | Crux |
| Install scripts (Homebrew, AUR) | opencode | crux |

"opencode" appears in hundreds of locations. Systematic grep + replace required.

### 1C. Prompt Replacements (from ROADMAP.md v0.2)

| Prompt Point | Current | Target |
|-------------|---------|--------|
| Plan mode reminder | 200-word "STRICTLY FORBIDDEN" block | 40-word positive framing |
| Build-switch reminder | Verbose | Single sentence |
| Max-steps prompt | Step-count limits | Crux token-budget system |
| Mid-loop user message wrapping | `<system-reminder>` tags | Removed |
| Structured output prompt | 90 words | 40 words |
| Agent generation meta-prompt | OpenCode defaults | Crux mode design rules |

### 1D. Bridge Plugin Absorption

The bridge plugin logic (`crux-bridge.js`) gets absorbed into the fork's source code:
- `experimental.chat.system.transform` → native system prompt injection
- `experimental.chat.messages.transform` → native message handling
- `chat.params` → native parameter configuration

### 1E. Token-Budget System

- Replace step-count limits with per-mode token budgets
- Warning at threshold (70-80%), hard limit at 90-95%
- Infrastructure enforcement via `toolChoice: none` (not prompt instructions)
- Crux `token-budget.js` plugin already tracks per-mode usage

### Progress Tracker — Phase 1

- [ ] 1.1 Fork OpenCode monorepo
- [ ] 1.2 Decide fork scope: full monorepo vs `packages/opencode` only
- [ ] 1.3 Systematic rebrand: binary name, config dir, env vars
- [ ] 1.4 Rebrand TUI: launch banner, header, status bar, logo
- [ ] 1.5 Rebrand help/about text
- [ ] 1.6 Replace plan mode reminder prompt
- [ ] 1.7 Replace build-switch reminder prompt
- [ ] 1.8 Replace max-steps prompt with token-budget system
- [ ] 1.9 Remove `<system-reminder>` wrapping
- [ ] 1.10 Replace structured output prompt
- [ ] 1.11 Replace agent generation meta-prompt
- [ ] 1.12 Absorb bridge plugin into source
- [ ] 1.13 Integrate token-budget enforcement
- [ ] 1.14 Update install scripts (Homebrew formula, AUR, etc.)
- [ ] 1.15 All tests GREEN (adapt OpenCode's existing test suite)
- [ ] 1.16 Binary compiles and runs: `cruxcli` launches correctly
- [ ] 1.17 Crux MCP server connects and operates through forked runtime

---

## Phase 2: CruxDev Convergence Engine Core

**Purpose:** Build the engine that drives audit-fix-re-audit loops to convergence without human intervention. This is CruxDev's key innovation — what Superpowers doesn't have.

**Prerequisite:** Phase 1 (need the runtime to dogfood with).

### 2A. Engine Files

| File | Purpose |
|------|---------|
| `CRUXDEV.md` | Bootstrap prompt (<2k tokens) — the one file agents always read |
| `engine/CONVERGENCE.md` | Master convergence loop specification |
| `engine/PLAN_CONVERGENCE.md` | Plan audit convergence sub-loop |
| `engine/CODE_CONVERGENCE.md` | Code audit convergence sub-loop |
| `engine/DOC_CONVERGENCE.md` | Documentation audit convergence sub-loop |
| `engine/VIABILITY.md` | Environment viability assessment protocol |

### 2B. Convergence State Machine

State tracked in JSON files with atomic writes:

```json
{
  "loop_type": "code_convergence",
  "status": "in_progress",
  "current_round": 3,
  "max_rounds": 5,
  "rounds": [...],
  "consecutive_clean_passes": 1,
  "convergence_threshold": 2,
  "error_state": null
}
```

Error states: `test_suite_failure`, `subagent_failure`, `file_system_error`, `context_overflow`, `build_failure` — each with recovery action and max retries.

### 2C. Termination Conditions

| Loop Type | Termination | Safety Valve |
|-----------|-------------|--------------|
| Focused audit | Zero issues in one pass | Max 5 passes |
| Full-plan audit | Zero issues in one pass | Max 3 passes |
| Viability | YES with zero caveats | Max 3 passes |
| Code convergence | Two consecutive clean passes (second independent) | Max 5 rounds |
| Doc convergence | Two consecutive clean passes | Max 3 rounds |

### 2D. Templates

| Template | Purpose |
|----------|---------|
| `templates/BUILD_PLAN_TEMPLATE.md` | Plan document skeleton |
| `templates/CODE_AUDIT_STATE.json` | Audit state file template |
| `templates/PATTERNS_TEMPLATE.md` | Patterns file template |

### Progress Tracker — Phase 2

- [ ] 2.1 Write `CRUXDEV.md` bootstrap file (<2k tokens)
- [ ] 2.2 Write `engine/CONVERGENCE.md` (master loop)
- [ ] 2.3 Write `engine/PLAN_CONVERGENCE.md`
- [ ] 2.4 Write `engine/CODE_CONVERGENCE.md`
- [ ] 2.5 Write `engine/DOC_CONVERGENCE.md`
- [ ] 2.6 Write `engine/VIABILITY.md`
- [ ] 2.7 Write convergence state file template
- [ ] 2.8 Write plan template
- [ ] 2.9 Write patterns file template
- [ ] 2.10 Test: run convergence engine on an existing project, verify it terminates

---

## Phase 3: CruxDev Core Skills (16 Skills)

**Purpose:** Build the modular skills that the convergence engine chains together. Each skill is a markdown file that agents load on demand.

**Prerequisite:** Phase 2 (engine must exist to chain skills into).

### 3A. Skill Format

Every skill follows:

```markdown
---
name: skill-name
description: "Use when [triggering conditions]"
loads: [supporting files]
chains-to: [next skills]
token-budget: <max tokens>
---
```

### 3B. Skill Inventory (16 skills)

#### Planning Skills
| # | Skill | Purpose | Token Budget |
|---|-------|---------|-------------|
| 1 | `planning` | Full planning cycle from problem to audited plan | ~1000 |
| 2 | `prompt-patterns` | Proven prompt patterns as reusable triggers | ~900 |
| 3 | `brainstorming` | Idea-to-design refinement before implementation | ~500 |

#### Quality Skills
| # | Skill | Purpose | Token Budget |
|---|-------|---------|-------------|
| 4 | `tdd` | Test-driven development enforcement | ~800 |
| 5 | `auditing` | Multi-dimensional audit (8 code + 5 doc dimensions) | ~1000 |
| 6 | `viability-assessment` | Environment verification against plan | ~800 |
| 7 | `honest-tracking` | Progress tracking with integrity | ~800 |
| 8 | `systematic-debugging` | Root-cause-first debugging | ~600 |

#### Execution Skills
| # | Skill | Purpose | Token Budget |
|---|-------|---------|-------------|
| 9 | `executing` | Plan execution with task conversion | ~800 |
| 10 | `subagent-delegation` | Dispatch fresh agents per task | ~800 |
| 11 | `git-worktrees` | Isolated workspace creation | ~400 |

#### Convergence Skills
| # | Skill | Purpose | Token Budget |
|---|-------|---------|-------------|
| 12 | `convergence-driving` | How to drive any convergence loop | ~1000 |
| 13 | `patterns-capture` | Extract methodology from completed work | ~800 |

#### Infrastructure Skills
| # | Skill | Purpose | Token Budget |
|---|-------|---------|-------------|
| 14 | `data-safety` | Atomic writes, backups, idempotency | ~500 |
| 15 | `state-machines` | State machine design and bridge functions | ~500 |
| 16 | `writing-skills` | Meta-skill: authoring new CruxDev skills | ~400 |

### 3C. Platform Adapters

| Platform | Bootstrap | Commands | Notes |
|----------|-----------|----------|-------|
| Claude Code | `CLAUDE.md` append | `/cruxdev-plan`, `/cruxdev-execute`, `/cruxdev-converge` | Primary platform |
| Codex | `AGENTS.md` symlink | Natural language triggers | Sandbox constraints |
| OpenCode | `agents.json` config | Regex pattern triggers | JSON-based config |

### Progress Tracker — Phase 3

- [ ] 3.1 Write `skills/planning/SKILL.md` + supporting files
- [ ] 3.2 Write `skills/prompt-patterns/SKILL.md` + prompt-library.md
- [ ] 3.3 Write `skills/brainstorming/SKILL.md`
- [ ] 3.4 Write `skills/tdd/SKILL.md` + supporting files
- [ ] 3.5 Write `skills/auditing/SKILL.md` + dimension files + issue-tracking.md
- [ ] 3.6 Write `skills/viability-assessment/SKILL.md` + environment-checks.md
- [ ] 3.7 Write `skills/honest-tracking/SKILL.md` + deferred-items.md + coverage-verification.md
- [ ] 3.8 Write `skills/systematic-debugging/SKILL.md` + diagnostic-checklist.md
- [ ] 3.9 Write `skills/executing/SKILL.md` + checkpoint-honesty.md + session-crash-recovery.md
- [ ] 3.10 Write `skills/subagent-delegation/SKILL.md` + prompt templates
- [ ] 3.11 Write `skills/git-worktrees/SKILL.md` + cleanup.md
- [ ] 3.12 Write `skills/convergence-driving/SKILL.md` + termination-criteria.md + safety-valves.md
- [ ] 3.13 Write `skills/patterns-capture/SKILL.md` + patterns-template.md
- [ ] 3.14 Write `skills/data-safety/SKILL.md` + patterns.md
- [ ] 3.15 Write `skills/state-machines/SKILL.md` + bridge-functions.md
- [ ] 3.16 Write `skills/writing-skills/SKILL.md`
- [ ] 3.17 Write Claude Code adapter (CLAUDE.md bootstrap, slash commands, hooks)
- [ ] 3.18 Write Codex adapter
- [ ] 3.19 Write OpenCode adapter
- [ ] 3.20 Verify each skill loads correctly and triggers on appropriate context
- [ ] 3.21 Verify skill chaining works end-to-end through convergence engine

---

## Phase 4: Inspiration Registry + Digest Pipeline

**Purpose:** Build the intelligence-gathering system that monitors upstream repos for concepts worth integrating. This is the "digest, don't track" strategy in action — not fork maintenance, but idea harvesting.

**Prerequisite:** Phase 1 (need CruxCLI runtime).

### 4A. Design Rationale: Digest, Don't Track

Bryan's explicit decision: do NOT track upstream OpenCode via rebasing/merging. OpenCode releases every 1-3 days with 823+ contributors. Merge hell is not worth it.

Instead: periodically read the git changes of monitored repos, analyze for concepts and approaches, and selectively integrate on merit. This is intelligence gathering, not fork maintenance.

### 4B. Architecture

```
.crux/inspiration/
├── registry.json           # List of monitored repos + last-digest timestamps
├── digests/
│   ├── 2026-03-18.md      # Daily digest output
│   └── ...
└── decisions/
    ├── adopted/            # Concepts integrated with rationale
    └── rejected/           # Concepts passed on with rationale
```

**Registry format:**
```json
{
  "repos": [
    {
      "name": "opencode",
      "url": "https://github.com/anomalyco/opencode",
      "last_digest": "2026-03-18T00:00:00Z",
      "focus": ["streaming", "agent-spawning", "prompt-architecture"]
    },
    {
      "name": "superpowers",
      "url": "https://github.com/obra/superpowers",
      "last_digest": "2026-03-18T00:00:00Z",
      "focus": ["skills", "methodology", "platform-adapters"]
    },
    {
      "name": "yoyo-evolve",
      "url": "https://github.com/yologdev/yoyo-evolve",
      "last_digest": "2026-03-18T00:00:00Z",
      "focus": ["self-evolution", "safety-gates", "cron-patterns"]
    }
  ]
}
```

### 4C. Digest Process (per repo)

Modeled on the BIP gather pattern (`crux_bip_gather.py`):

1. `git fetch` the monitored repo
2. Diff since last digest timestamp
3. Summarize what changed, what's novel
4. Agent evaluates each change against Crux's current state:
   - Does this concept exist in Crux already?
   - Is their approach better?
   - How would we integrate it?
   - Is it worth the effort?
5. Output: `.crux/inspiration/digests/<date>.md`
6. Actionable items promoted to evolution planning
7. Rejected items logged with rationale in `decisions/rejected/`

### 4D. Integration with Crux MCP

New MCP tools (following the pattern in `crux_mcp_handlers.py`):

| Tool | Purpose |
|------|---------|
| `digest_inspiration` | Run digest pipeline for all registered repos |
| `add_inspiration_repo` | Add a new repo to the registry |
| `list_inspiration_repos` | Show all monitored repos |
| `get_digest` | Retrieve a specific digest |

### Progress Tracker — Phase 4

- [ ] 4.1 Design registry.json schema
- [ ] 4.2 Build digest pipeline script (Python, following BIP gather pattern)
- [ ] 4.3 Implement git fetch + diff analysis per repo
- [ ] 4.4 Implement concept evaluation agent prompt
- [ ] 4.5 Implement digest output format
- [ ] 4.6 Implement decision logging (adopted/rejected with rationale)
- [ ] 4.7 Add MCP tools: `digest_inspiration`, `add_inspiration_repo`, `list_inspiration_repos`, `get_digest`
- [ ] 4.8 Tests for digest pipeline
- [ ] 4.9 Tests for MCP tools
- [ ] 4.10 All tests GREEN
- [ ] 4.11 Coverage check ≥ 100%

---

## Phase 5: Autonomous Evolution Pipeline (5-Beat Loop)

**Purpose:** Build the system that lets the Crux ecosystem evolve itself unattended — planning, building, auditing, converging, posting about the process, and engaging with the community.

**Prerequisite:** Phase 4 (inspiration registry), Phase 2 (convergence engine).

**Inspiration:** yoyo-evolve — a Rust-based AI coding agent that grew from 200 lines to 14,700 lines with 619 tests in 18 days, running on a cron via GitHub Actions every 4 hours, with no human writing its code.

### 5A. The Five-Beat Loop

```
┌──────────────────────────────────────────────────────────────┐
│                   AUTONOMOUS EVOLUTION LOOP                    │
│                                                               │
│  Beat 1: GATHER                                               │
│    Scan inspiration registry repos for changes                │
│    Read own source, GitHub issues, CI status                  │
│    Read community input (issues, discussions)                 │
│                                                               │
│  Beat 2: EVALUATE                                             │
│    Does this concept improve something in our ecosystem?      │
│    Gap analysis against north star goals                      │
│    Vote-scored community input for prioritization             │
│    Learnings admission gate (genuinely novel AND would        │
│      change future behavior)                                  │
│                                                               │
│  Beat 3: INTEGRATE                                            │
│    Build through the convergence pipeline (Phase 2 engine)    │
│    Safety gates (Crux 7-gate pipeline) on every code change   │
│    Build/test gates + auto-rollback on failure                │
│    15-min timeout per task (yoyo pattern)                     │
│    3 failed attempts → auto-rollback                          │
│                                                               │
│  Beat 4: POST                                                 │
│    Typefully → X with narrative of what was found/built       │
│    "We analyzed OpenCode's latest release and integrated      │
│     their improved streaming approach with our safety gates"  │
│    Content is byproduct of building, not separate effort      │
│                                                               │
│  Beat 5: ENGAGE                                               │
│    Respond to community reactions on GitHub                   │
│    Process issue input for next cycle                         │
│    Social sessions separated from code evolution              │
│      (prevents social engineering attacks)                    │
│                                                               │
│  Each beat produces content. The system turns competitors'    │
│  innovations into marketing material for Crux.               │
└──────────────────────────────────────────────────────────────┘
```

### 5B. Execution Model

```
GitHub Actions cron (every N hours, frequency TBD)
  → CruxCLI launches
  → Reads .crux/evolution/state.json
  → Drives evolution loop via MCP tools
  → Safety gates (Crux 7-gate pipeline) on every code change
  → Posts progress via crux_typefully.py → X
  → Commits results
  → Updates .crux/evolution/state.json
```

### 5C. Two-Layer Memory Architecture

Adopted from yoyo-evolve for evolution context:

- **Layer 1: Append-only JSONL archive** — immutable history, never modified
- **Layer 2: AI-synthesized active context with time decay** — compressed working memory, periodically regenerated from the archive

This gives immutable history + compressed working memory. The evolution pipeline reads Layer 2 for context and writes to Layer 1 for permanence.

### 5D. Protected Files / Immutable Identity

Safety model: the autonomous agent cannot modify its own safety constraints or core identity during self-evolution.

Protected file categories:
- Convergence engine rules (termination conditions, safety valves)
- Safety pipeline configuration (7-gate pipeline)
- Identity files (README, LICENSE, core mission statement)
- Evolution pipeline constraints (this list)

Any attempted modification to protected files triggers escalation to human.

### 5E. Safety Gates for Autonomous Evolution

| Gate | Trigger | Action on Failure |
|------|---------|------------------|
| Build gate | Code change | Must compile/pass linter |
| Test gate | Code change | All tests must pass |
| Coverage gate | Code change | Coverage must not decrease |
| Safety gate | Any change | Crux 7-gate pipeline must pass |
| Protected file gate | Any change | Cannot modify protected files |
| Rollback gate | 3 failed attempts | Auto-rollback to last known good |
| Timeout gate | Per-task | 15-min max per task |

### Progress Tracker — Phase 5

- [ ] 5.1 Design evolution state file schema
- [ ] 5.2 Implement Beat 1: Gather (reads own source, issues, inspiration digests)
- [ ] 5.3 Implement Beat 2: Evaluate (gap analysis, prioritization, learnings gate)
- [ ] 5.4 Implement Beat 3: Integrate (convergence pipeline with safety gates)
- [ ] 5.5 Implement auto-rollback on build/test failure
- [ ] 5.6 Implement 15-min timeout per task
- [ ] 5.7 Implement Beat 4: Post (Typefully integration for X)
- [ ] 5.8 Implement Beat 5: Engage (GitHub issue/discussion processing)
- [ ] 5.9 Implement social session isolation (separate from code evolution)
- [ ] 5.10 Implement two-layer memory (append-only JSONL + synthesized active context)
- [ ] 5.11 Implement protected files enforcement
- [ ] 5.12 Set up GitHub Actions cron workflow
- [ ] 5.13 Tests for evolution pipeline
- [ ] 5.14 Tests for safety gates
- [ ] 5.15 Tests for memory architecture
- [ ] 5.16 All tests GREEN
- [ ] 5.17 Coverage check ≥ 100%

---

## Phase 6: Marketing / Attention Strategy

**Purpose:** Generate attention for all three products. Runs in parallel with everything else from day 1.

**Prerequisite:** None (parallel track).

### 6A. Competitive Positioning

Each product has its own story:

**CruxDev: "The agent that drives itself to convergence"**
- Differentiator: autonomous convergence engine (no "do it again")
- Target: developers who use Superpowers and want more automation
- Key proof point: two-consecutive-clean-pass rule, empirical convergence data

**CruxCLI: "OpenCode with an intelligence layer"**
- Differentiator: Crux modes, MCP tools, safety pipeline underneath
- Target: OpenCode users who want a smarter agent
- Key proof point: same runtime, better decisions

**Crux: "The .git for AI coding intelligence"**
- Differentiator: tool-agnostic platform (works with any agent)
- Target: teams building AI-assisted workflows
- Key proof point: 24 modes, 37 tools, 7-gate safety, 1480+ tests

### 6B. Superpowers Growth Playbook (Lessons to Apply)

What worked for Superpowers (93.9k stars, zero paid marketing):

1. **Practitioner credibility** — Jesse Vincent has decades of open-source credibility, Wikipedia page
2. **Simon Willison amplification** — "one of the most creative users of coding agents"
3. **Hacker News traction** — front page posts
4. **Anthropic marketplace** — acceptance on Jan 15, 2026 was the inflection point
5. **GitHub Trending snowball** — #2 position, 1,400+/day for 4 consecutive days
6. **Measurable results** — chardet v7.0.0: 41x faster with Superpowers methodology
7. **Viral content** — "100x better" Medium articles

### 6C. Crux Attention Strategy

| Tactic | Applies To | Notes |
|--------|-----------|-------|
| Build-in-public via Typefully → X | All three | Content is byproduct of building, not separate effort |
| Measurable results (convergence data) | CruxDev | "79 issues found across 11 passes" is a compelling story |
| Anthropic marketplace submission | CruxDev | Follow Superpowers' path |
| GitHub Trending | CruxDev, CruxCLI | Three separate repos = three shots at trending |
| Origin blog post | All three | Bryan's practitioner story |
| Inspiration → content pipeline | All three | "We analyzed OpenCode's latest release and..." |
| Hacker News | CruxDev | "Show HN: CruxDev — autonomous convergence for AI coding" |

### 6D. Build-in-Public as Byproduct

Build-in-public is not a separate marketing effort — it is a natural byproduct of the autonomous evolution pipeline. The system's normal operation produces content:

- Inspiration digests → "we analyzed X and integrated Y" posts
- Convergence runs → progress narratives
- Safety improvements → trust-building content
- Community engagement → interaction/visibility

### Progress Tracker — Phase 6

- [ ] 6.1 Write origin blog post (Bryan's story)
- [ ] 6.2 Set up Typefully automation for all three repos
- [ ] 6.3 Prepare Anthropic marketplace submission for CruxDev
- [ ] 6.4 Create measurable results narrative (convergence data)
- [ ] 6.5 Prepare Hacker News Show HN post
- [ ] 6.6 GitHub repo polish (README, badges, screenshots) for all three

---

## Phase 7: Validation / Dogfood on Itself

**Purpose:** Use CruxDev + CruxCLI + Crux to build the remaining CruxDev features. The system builds itself.

**Prerequisite:** Phases 2-3 (convergence engine + skills must exist).

### 7A. The Chicken-and-Egg

The first implementation pass of CruxDev (Phases 2-3) probably needs to happen manually — or with vanilla Claude Code / CruxCLI as-is. Once bootstrap + convergence engine core is working, switch to self-hosting.

### 7B. Validation Criteria

| Criterion | How to Measure |
|-----------|---------------|
| Convergence engine terminates | Run on a real project, verify it reaches "CONVERGED" |
| Skills chain correctly | Run full lifecycle: plan → execute → converge |
| Issue count comparable to manual | Compare convergence data with DEVELOPMENT_PATTERNS.md empirical data |
| Fewer human interventions | Count "do it again" prompts needed (should be zero) |
| Self-hosting works | Use CruxDev to build CruxDev features |

### Progress Tracker — Phase 7

- [ ] 7.1 Select a real project with existing codebase for validation
- [ ] 7.2 Install CruxDev alongside existing methodology
- [ ] 7.3 Run full planning + execution + convergence cycle
- [ ] 7.4 Compare: issue counts, convergence rounds, human interventions
- [ ] 7.5 Update skills based on findings
- [ ] 7.6 Switch to self-hosting: use CruxDev to build remaining CruxDev features
- [ ] 7.7 Document validation results

---

## Open Questions

### Q1: CruxDev Skills vs Crux Modes — What's the Relationship?

**Context:** CruxDev has 16 "skills." Crux has 24 "modes." Are they the same thing with different names?

**Proposed answer:** They're complementary layers. Crux modes control the agent's personality and tool access (how the agent behaves). CruxDev skills control the development methodology (what process the agent follows). A CruxCLI session might be in Crux's `build` mode while executing CruxDev's `tdd` skill.

CruxDev works standalone without Crux (just like Superpowers works without any framework). When Crux is present, it makes CruxDev better — modes provide safety gates, knowledge entries provide context, MCP tools provide infrastructure.

**Status:** Open — needs Bryan's confirmation.

### Q2: CruxCLI Fork Scope

**Context:** OpenCode is a monorepo with ~15 packages. Fork everything or just `packages/opencode`?

**Proposed answer:** Fork the full monorepo. The packages are interdependent, and cherry-picking creates maintenance burden. A full fork gives complete control. Strip packages you don't need later.

**Status:** Open — needs Bryan's decision.

### Q3: Crux Positioning

**Context:** Crux has no direct competitor. How do you position something in a new category?

**Proposed answer:** Position by analogy: "Crux is to AI coding agents what .git is to version control — the infrastructure layer that makes everything else work." The platform play becomes clear when CruxDev and CruxCLI both demonstrably benefit from having Crux underneath.

**Status:** Open — needs Bryan's refinement.

### Q4: Cron Frequency for Autonomous Evolution

**Context:** yoyo-evolve runs every 4 hours. What's right for Crux?

**Proposed answer:** Start with daily. Increase frequency as confidence grows. The evolution pipeline should be safe to run frequently (safety gates prevent damage), but daily gives time to review outputs while building trust.

**Status:** Open — needs Bryan's decision.

### Q5: Is CruxCLI + Crux Functional Enough Today for Development?

**Context:** Dogfooding requires the runtime to be functional. CruxCLI v0.1 is in dogfooding phase on the `local_llm` project.

**Proposed answer:** Depends on Phase 1 outcome. The hard fork (v0.2) is the prerequisite for autonomous work. For manual development, vanilla Claude Code with DEVELOPMENT_PATTERNS_CRUXDEV.md methodology works fine.

**Status:** Open — depends on Phase 1 progress.

---

## File Inventory

### Phase 1 (CruxCLI Fork)
| File | Purpose |
|------|---------|
| Forked monorepo | CruxCLI v0.2 source |
| Updated branding throughout | Rebrand opencode → crux |

### Phase 2 (Convergence Engine)
| File | Purpose |
|------|---------|
| `CRUXDEV.md` | Bootstrap prompt |
| `engine/CONVERGENCE.md` | Master loop |
| `engine/PLAN_CONVERGENCE.md` | Plan audit loop |
| `engine/CODE_CONVERGENCE.md` | Code audit loop |
| `engine/DOC_CONVERGENCE.md` | Doc audit loop |
| `engine/VIABILITY.md` | Viability assessment |
| `templates/BUILD_PLAN_TEMPLATE.md` | Plan template |
| `templates/CODE_AUDIT_STATE.json` | Audit state template |
| `templates/PATTERNS_TEMPLATE.md` | Patterns template |

### Phase 3 (Skills)
| File | Purpose |
|------|---------|
| `skills/*/SKILL.md` (×16) | Skill definitions |
| `skills/*/` supporting files | Skill detail files |
| `adapters/claude-code/*` | Claude Code integration |
| `adapters/codex/*` | Codex integration |
| `adapters/opencode/*` | OpenCode integration |

### Phase 4 (Inspiration)
| File | Purpose |
|------|---------|
| `.crux/inspiration/registry.json` | Monitored repo registry |
| Digest pipeline script | Python, BIP gather pattern |
| MCP tool handlers | Inspiration tools |

### Phase 5 (Evolution)
| File | Purpose |
|------|---------|
| `.crux/evolution/state.json` | Evolution loop state |
| Evolution pipeline scripts | 5-beat loop implementation |
| GitHub Actions workflow | Cron trigger |
| Two-layer memory files | JSONL archive + active context |
| Protected files manifest | Immutable file list |

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| OpenCode fork drift | Hard to compare/adopt new features | Inspiration digest pipeline (Phase 4) monitors upstream without tracking |
| CruxDev ignored vs Superpowers | Lost market positioning | Ship early, measurable results, Anthropic marketplace |
| Autonomous evolution introduces bugs | Reputation damage | 7-gate safety pipeline, auto-rollback, protected files |
| Token pressure in long convergence loops | Session crashes | Sub-session breakdown, state files for resume |
| Three repos = three maintenance burdens | Developer burnout | Autonomous evolution reduces manual maintenance |
| Fork scope too large | Phase 1 takes too long | Can start with minimal rebrand + prompt replacements |

---

## Definition of Done

1. CruxCLI v0.2 hard fork compiles and runs as `crux` binary
2. CruxDev convergence engine runs end-to-end on a real project
3. All 16 CruxDev skills exist and chain correctly
4. Inspiration registry monitors at least 3 repos and produces digests
5. Autonomous evolution pipeline completes at least one unattended cycle
6. At least one X post generated from the inspiration → content pipeline
7. CruxDev dogfooded on itself (Phase 7 validation)
8. All tests pass across all three repos
9. Coverage ≥ 100% on all new code
10. DEVELOPMENT_PATTERNS_CRUXDEV.md captures all methodology from this work
