# Competitors

**Last Updated:** 2026-03-27 (post-session: 22 build plans converged, 52 tools, 419 tests)
**Project:** CruxDev — Autonomous Convergence Framework
**Research Method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md (broad → academic → practitioner → contrarian → primary sources)

## Competitive Position

CruxDev is the only framework that drives AI coding agents through multi-dimensional audit-fix-re-audit loops until two consecutive independent clean passes are achieved. Every competitor is either an execution substrate (runs code but doesn't know when it's done) or an analysis tool (scans code but doesn't drive fixes). CruxDev sits between them.

**Category validation:** Karpathy calls the autonomous edit-evaluate-keep loop "the final boss battle" ([Fortune, 2026-03-17](https://fortune.com/2026/03/17/andrej-karpathy-loop-autonomous-ai-agents-future/)). CodeRabbit predicts multi-agent audit-fix workflows as the defining 2026 trend. MorphLLM benchmark: "Scaffolding matters as much as the model" — same Claude Opus scored differently across architectures.

**Key risk acknowledged:** IEEE-ISTAS 2025 paper found 37.6% increase in critical security vulnerabilities after 5 iterations of LLM refinement ([arxiv.org/abs/2506.11022](https://arxiv.org/abs/2506.11022)). CruxDev's multi-dimensional audit (including explicit security dimension) and two-consecutive-clean-pass criterion address this, but regression detection between passes is a design enhancement to consider.

---

## Official Competitors (Actively Tracked)

### Superpowers
- **URL:** https://github.com/obra/superpowers
- **GitHub:** 109,961 stars (2026-03-24, verified via GitHub API)
- **Latest:** v5.0.5 (2026-03-17)
- **Category:** Direct
- **License:** Open source (Shell)

**Strengths:**
- Massive community — trending #1 on GitHub, accepted into Anthropic Claude Code plugin marketplace
- 7-phase workflow (brainstorm → worktrees → plan → execute → TDD → review → completion)
- Subagent coordination with 2+ hour autonomous execution windows
- Multi-agent support (Claude, Cursor, Codex, OpenCode, Gemini CLI)
- Claims test coverage jumps from 0-30% to 85-95% with framework active

**Weaknesses:**
- No convergence detection — human still chains skills together
- No autonomous termination criterion (no "done" signal)
- No multi-dimensional audit scoring (8 code + 5 doc)
- No safety gates beyond git isolation (no timeout, no rollback counter, no net-negative)
- No session bus / cross-project coordination
- Enforces 85-95% coverage, not 100%
- No autonomous git workflow (commit/push/PR/merge)
- No GitHub issue monitoring or triage
- No form-specific audit dimensions
- No research-converged methodology docs driving audit criteria
- No build artifact freshness detection
- No multi-platform binary distribution
- No competitive feedback loop (manual competitive updates only)
- No TypeScript/Node.js toolchain detection
- No ecosystem-neutral design (tied to one vendor)
- No OpenClaw integration

**Our moat vs them:**
- Convergence engine with two-consecutive-clean-pass criterion
- 9-dimension code audit + 5-dimension doc audit + 9 form + 7 metrics + 9 dashboard dimensions (39 total audit dimensions)
- Autonomous execution without re-prompting
- Safety gates: timeout, 3-failure rollback, net-negative detection, phase-specific max rounds
- Session bus for cross-project coordination
- Evolution pipeline: 5-beat autonomous loop (gather, evaluate, integrate, post, engage)
- 100% coverage enforcement (not 85-95%)
- Autonomous git workflow: commit → push → PR → merge with safety gates (no force push, no git add -A, reject secrets/binaries)
- GitHub issue monitoring with 5-layer prompt injection defense
- Build artifact freshness gate: auto-detects stale binaries/bundles/sites across Rust, Node, Go, Docker
- Multi-platform binaries: macOS ARM + Intel + Linux x86_64 via GitHub Actions
- OpenClaw integration (MCP stdio) — first convergence engine in the 247K-star AI agent ecosystem
- TypeScript toolchain detection: Jest/Vitest, ESLint/Biome, coverage parsing, npm audit, tsconfig strict
- Competitive feedback loop: auto-detects when build plans change competitive position
- Autonomous growth engine: Typefully posting, GitHub Releases, README optimization, metrics tracking
- 10 research-converged methodology docs baked into audit dimensions (form, metrics, dashboard, color/contrast, DRY, research, competitors, growth, logo, MCP server)
- Ecosystem-neutral: works with any MCP client (not locked to one vendor)
- 52 MCP tools, 419 tests, single 5.1MB Rust binary

**Their moat vs us:**
- 110K stars vs our early stage — massive mindshare
- Anthropic marketplace acceptance
- Skills auto-activate by context
- Gap: should-close (marketplace inclusion, context-activated skills)

**Sources:** github.com/obra/superpowers, github.com/obra/superpowers/releases, pasqualepillitteri.it

---

### Backbeat
- **URL:** https://github.com/dean0x/backbeat
- **GitHub:** 3 stars (2026-03-24)
- **Category:** Direct (closest conceptual competitor)
- **License:** Open source (TypeScript)

**Strengths:**
- Implements Karpathy-style loops for production coding
- Two loop types: Retry (run until exit code 0) and Optimize (track best result via eval script)
- Safety limits: default 10 iterations max, 3 consecutive failures stops
- MCP integration; supports Claude, Codex, Gemini
- Conceptually closest to CruxDev's convergence approach

**Weaknesses:**
- 3 stars — essentially unknown
- Single metric optimization only (no multi-dimensional audit)
- No convergence detection (just iteration limits)
- No safety gates beyond iteration count (no timeout, no net-negative, no rollback)
- No session bus, no cross-project coordination
- No doc audit, no doc alignment

**Our moat vs them:**
- Multi-dimensional audit (8+5 dimensions vs single metric)
- Convergence detection (novelty-based termination vs iteration count)
- Full safety gate suite
- Session bus, evolution pipeline, research methodology
- 555 tests, 100% coverage vs early prototype

**Their moat vs us:**
- Simpler mental model — easy to understand retry/optimize
- Gap: intentional (we're more comprehensive by design)

**Sources:** github.com/dean0x/backbeat, earezki.com/ai-news/2026-03-22

---

### DeepSource
- **URL:** https://deepsource.com
- **Pricing:** Free (open source), $24/user/month (Team), Custom (Enterprise)
- **Category:** Adjacent (code quality)
- **License:** Proprietary (free for open source)

**Strengths:**
- 5-dimension review (Security, Reliability, Complexity, Hygiene, Coverage)
- AI Review + Autofix per PR
- Static analysis, SAST, IaC scanning
- Enterprise-grade (SSO, audit logs, self-hosted option)

**Weaknesses:**
- Single-pass scanning — no iterative convergence
- No convergence criterion — scans once per PR and moves on
- SaaS-only for full features
- Doesn't drive development, only reviews output

**Our moat vs them:**
- Iterative convergence loop (audit → fix → re-audit → repeat)
- Controls full lifecycle (plan → execute → audit → converge)
- Runs locally, no SaaS dependency
- 8 code dimensions (vs their 5)

**Their moat vs us:**
- Production-grade UI and dashboard
- Enterprise customers and pricing
- Gap: intentional (different layer)

**Sources:** deepsource.com/pricing, deepsource.com/changelog/2026-02-23

---

### yoyo-evolve
- **URL:** https://github.com/yologdev/yoyo-evolve
- **GitHub:** 669 stars (2026-03-24)
- **Category:** Direct (self-evolving agent)
- **License:** Open source (Rust)

**Strengths:**
- Self-evolved from 200 to 31,000 lines in 24 days
- 1,346 tests (1,264 unit + 82 integration), 700 commits
- Runs autonomously every 8 hours on cron
- Community-driven via GitHub issues with upvote/downvote
- 11 LLM providers, 55 slash commands
- Daily memory synthesis with time-weighted compression
- Proven the autonomous evolution model works in production

**Weaknesses:**
- Rust-only (single language)
- Simple test-pass gate (no multi-dimensional audit)
- No convergence criterion (just "tests pass")
- No safety gates beyond test passage (no timeout, no rollback counter)
- Only evolves itself — not a framework for other projects

**Our moat vs them:**
- Multi-dimensional audit (not just "tests pass")
- Convergence criterion (two consecutive clean passes)
- Multi-project support (session bus)
- Language-agnostic
- Full safety gate suite

**Their moat vs us:**
- 24 days of proven unattended autonomous evolution
- 31K lines from 200 — demonstrable self-improvement
- Gap: should-close (prove our evolution pipeline works unattended)

**Sources:** github.com/yologdev/yoyo-evolve

---

## Watch List

| Name | Stars | Why watching |
|------|-------|-------------|
| Codex CLI | 67,308 | OpenAI's agent — 25hr uninterrupted runs, but no convergence |
| Gemini CLI | ~96K | Fast growth, free tier, ReAct loop |
| OpenHands | 69,670 | Planning Agent (BETA) — plan-then-code but no convergence |
| Cursor Self-Driving | N/A | 1000 commits/hr internally, hierarchical agents — experimental |
| ARIS | 3,755 | Cross-model adversarial review loops — academic focus |
| COCO Framework | N/A | Formal convergence guarantees — academic, validates our approach |
| Karpathy autoresearch | 53,755 | 700 autonomous changes — ML-only but category-defining |

---

## Noted

| Name | Category | Why noted |
|------|----------|-----------|
| Claude Code | Execution substrate | We run on top of it. /loop + auto-accept, no convergence |
| Cursor | IDE | Automations shipped March 2026. Self-driving is internal research |
| Windsurf | IDE | Turbo Mode, 5 parallel agents, Devin merger |
| Cline | VS Code agent | 59K stars, Plan/Act modes, no convergence |
| Roo Code | VS Code agent | 22K stars, fewer agent-thrashing complaints |
| Continue.dev | Assistant | 20K stars, model-agnostic, no convergence |
| Aider | Pair programming | 32K stars, Architect/Editor split, interactive not autonomous |
| Devin | Cloud agent | $20/month (down from $500), dynamic re-planning, no convergence |
| SonarQube | Scanner | Industry standard, complementary not competitive |
| CodeScene | Analysis | CodeHealth metric, complementary |
| Codacy | Review | Multi-engine parallel scan, complementary |
| DSPy | Prompt optimization | 32K stars, different domain |
| Darwin Godel Machine | Research | Self-modifying agent, SWE-bench 20%→50% |
| HubSpot Sidekick | Enterprise | Two-stage review+judge, single-pass only |
| CodeRabbit | Vendor | Predicts multi-agent audit-fix as 2026 trend |
| Qodo | Vendor | 6 specialist agent dimensions, predicts flow-to-fix |

---

## Feature Matrix

| Feature | CruxDev | Superpowers | Backbeat | DeepSource | yoyo-evolve |
|---------|:-------:|:-----------:|:--------:|:----------:|:-----------:|
| Convergence detection (2 clean passes) | ✓ | — | — | — | — |
| Multi-dimensional audit (8+5) | ✓ | — | 1 | 5 | — |
| Autonomous execution | ✓ | Partial | ✓ | — | ✓ |
| Safety gates (timeout/rollback/net-neg) | ✓ | Partial | Partial | — | Partial |
| Cross-project coordination | ✓ | — | — | — | — |
| Self-evolution pipeline | ✓ | — | — | — | ✓ |
| Model tier routing + escalation | ✓ | — | — | — | — |
| Doc alignment gate | ✓ | — | — | — | — |
| Green-field execution | ✓ | ✓ | — | — | — |
| Protected files enforcement | ✓ | — | — | — | — |
| TDD enforcement | ✓ | ✓ | — | — | ✓ |
| Research methodology (5-pass) | ✓ | — | — | — | — |
| Session bus messaging | ✓ | — | — | — | — |
| Open source | ✓ | ✓ | ✓ | Partial | ✓ |

---

## Academic Landscape

| Paper/System | What it does | CruxDev comparison |
|-------------|-------------|-------------------|
| COCO (arxiv 2508.13815) | Formal convergence guarantees via contextual rollback + bidirectional reflection | CruxDev is practical implementation of COCO's theoretical framework |
| IEEE-ISTAS 2025 (arxiv 2506.11022) | 37.6% security degradation after 5 LLM iterations | CruxDev's multi-dimension audit + security dimension addresses this |
| Codified Context (arxiv 2602.20478) | 283 sessions, persistent context infrastructure | CruxDev's CLAUDE.md + convergence state provides similar context persistence |
| Veracode GenAI Report | AI code has 2.74x more vulnerabilities than human code | Validates multi-pass auditing approach |

---

## Key Contrarian Findings

| Risk | Source | CruxDev mitigation | Residual risk |
|------|--------|--------------------|----- |
| Iteration paradox (more passes = worse security) | IEEE-ISTAS 2025 | Multi-dimensional audit with explicit security dimension | Regression detection between passes not yet implemented |
| Correlated errors (same LLM for generate + audit) | COCO paper | Multi-dimension audit uses different prompts per dimension | Cross-model validation (generate with one, audit with another) would be stronger |
| Compounding accuracy (85%/step = 20% over 10 steps) | Industry analysis | Each audit-fix cycle corrects rather than compounds; 3-failure rollback | Keep cycle count low, invest in per-step accuracy |
| Developer trust deficit (46% distrust AI code) | Multiple sources | Convergence is empirically verified (tests pass, two clean passes) | Need better observability — audit trail surfacing |
| Local correctness vs global coherence | Sonar/DORA 2025 | Architecture is one of the 8 audit dimensions | May need architectural regression testing |

---

## Gap Closure Queue

| Gap | Competitor | Classification | Status |
|-----|-----------|---------------|--------|
| Skills auto-activate by context | Superpowers | Should close | Not started |
| Anthropic marketplace inclusion | Superpowers | Should close | Not started |
| Proven unattended evolution run | yoyo-evolve | Should close | Pipeline built, not yet run |
| Cross-model validation (audit with different LLM) | COCO paper | Should close | Dispatch layer supports it, not wired |
| Regression detection between passes | IEEE paper | Should close | Not started |
| Audit trail UI/observability | Developer trust gap | Nice to have | State files exist, no UI |
| 110K stars community | Superpowers | Intentional (build quality, community follows) | N/A |
| Production dashboard | DeepSource | Nice to have | Not started |
