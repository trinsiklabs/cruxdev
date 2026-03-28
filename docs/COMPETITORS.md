# Competitors

**Last Updated:** 2026-03-27 (47 build plans, 52 tools, 451 tests, 228 templates, full harness competitive landscape)
**Project:** CruxDev — Autonomous Convergence Harness
**Research Method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Category:** AI Harness Engineering — Autonomous Convergence subcategory

## Competitive Position

**The industry equation (Q1 2026): Agent = Model + Harness.** The model is commodity. The harness is moat. (Hashimoto, Fowler, Anthropic, OpenAI — all converging on this in Feb-Mar 2026.)

CruxDev is the only **autonomous convergence harness** — a harness that drives agents to mathematically defined completion (two consecutive independent clean passes across 39+ audit dimensions) without human intervention. Every other harness is a runtime wrapper: it runs agents safely but doesn't know when they're done.

| What Others Build | What CruxDev Is |
|------------------|----------------|
| Runtime harness (run safely, observe, rollback) | **Convergence harness** (drive to verified completion) |
| Human says "looks good" | Engine proves "two independent clean passes" |
| LLM drives the loop | **Engine drives the loop**, LLM is a tool it calls |
| Code-only | **Any project type** (18 types, 228 templates) |
| Static harness | **Self-improving** (self-adoption after every build plan) |

**Category validation:** Hashimoto coined "harness engineering" (Feb 2026). Fowler published. Forrester launching market evaluation. 3 academic papers on arXiv. Manus proved harness > model (5 rewrites, identical models, each better). The term is weeks old — the subcategory "autonomous convergence harness" is undefined and ownable.

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
- Software-only — cannot manage books, podcasts, newsletters, businesses
- No domain/sub-project architecture for composite businesses
- No project template library (218 templates across 16 types)
- No skill-server synchronization enforcement

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
- 12 research-converged methodology docs baked into audit dimensions (form, metrics, dashboard, color/contrast, DRY, research, competitors, growth, logo, MCP server, AI skills)
- Ecosystem-neutral: works with any MCP client (not locked to one vendor)
- Universal project management: classifies and manages 18 project types (software, books, podcasts, newsletters, YouTube, businesses, courses, open source, composites)
- Domain architecture: parent projects with typed sub-projects, cross-project convergence, multi-domain membership
- 218 project templates across 16 categories (software, business, book, podcast, newsletter, YouTube, etc.)
- SKILL.md standard: 7 skills migrated, skill-server synchronization enforcement
- Self-improvement cycle: product improves itself through self-adoption after every build plan (unique — no competitor does this)
- Universal content pipeline: 16 event types across all project types → blog posts + X posts
- 14 research-converged methodology docs (form, metrics, dashboard, color, DRY, research, competitors, growth, logo, MCP server, AI skills, blog posts, X posts, website planning)
- 52 MCP tools, 451 tests, single 5.1MB Rust binary

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
| Cody (Sourcegraph) | Assistant | Context-aware, multi-repo search |
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

| Feature | CruxDev | Claude Code | Codex | Cursor | Manus | Superpowers | Backbeat | DeepSource | yoyo-evolve | DeepAgents |
|---------|:-------:|:-----------:|:-----:|:------:|:-----:|:-----------:|:--------:|:----------:|:-----------:|:----------:|
| Convergence detection (2 clean passes) | ✓ | — | — | — | — | — | — | — | — | — |
| Multi-dimensional audit (39 dims) | ✓ | — | — | — | — | — | 1 | 5 | — | — |
| LLM minimization (code drives loops) | ✓ | — | — | — | — | — | — | — | — | — |
| Self-improvement (self-adoption) | ✓ | — | — | — | — | — | — | — | — | — |
| Autonomous execution | ✓ | ✓ | ✓ | ✓ | ✓ | Partial | ✓ | — | ✓ | ✓ |
| Multi-agent parallelism | — | ✓ | ✓ | ✓ | ✓ | — | — | — | — | ✓ |
| Safety gates (timeout/rollback/net-neg) | ✓ | ✓ | ✓ | Partial | — | Partial | Partial | — | Partial | — |
| Kernel-level sandboxing | — | — | ✓ | — | — | — | — | — | — | — |
| Cross-project coordination | ✓ | — | — | — | — | — | — | — | — | — |
| Self-evolution pipeline | ✓ | — | — | — | — | — | — | — | ✓ | — |
| KV-cache-aware context engineering | — | — | — | — | ✓ | — | — | — | — | — |
| Durable execution / checkpointing | — | — | — | — | — | — | — | — | — | ✓ |
| Visual verification (screenshots) | — | — | — | ✓ | — | — | — | — | — | — |
| Universal project types (18) | ✓ | — | — | — | — | — | — | — | — | — |
| Doc alignment gate | ✓ | — | — | — | — | — | — | — | — | — |
| TDD enforcement | ✓ | — | — | — | — | ✓ | — | — | ✓ | — |
| Research methodology (5-pass) | ✓ | — | — | — | — | — | — | — | — | — |
| MCP ecosystem (servers/tools) | 52 tools | 50+ curated | 20+ plugins | — | — | — | — | — | — | Hundreds |
| Hook system (lifecycle events) | — | 12 events | — | — | — | — | — | — | — | — |
| Voice input | — | ✓ | — | — | — | — | — | — | — | — |
| Enterprise (HIPAA/SOC2/SSO) | — | ✓ | ✓ | — | — | — | — | ✓ | — | — |
| Open source | ✓ | — | ✓ (CLI) | — | — | ✓ | ✓ | Partial | ✓ | ✓ |

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
| KV-cache-aware context engineering | Manus | Must close | BP047 written, not started |
| Durable execution / checkpointing | DeepAgents | Must close | BP047 written, not started |
| Visual verification (screenshots) | Cursor | Should close (phased) | BP047 written, not started |
| Multi-agent parallel execution | Claude Code, Codex | Should close | Not started |
| Hook system (lifecycle events) | Claude Code | Should close | Not started |
| Kernel-level sandboxing | Codex | Nice to have | Not started (Rust binary could use Seatbelt/Landlock) |
| Skills auto-activate by context | Superpowers | Should close | Not started |
| Proven unattended evolution run | yoyo-evolve | Should close | Pipeline built, not yet run |
| Cross-model validation (audit with different LLM) | COCO paper | Should close | Dispatch layer supports it, not wired |
| Regression detection between passes | IEEE paper | Should close | Not started |
| Post-convergence content pipeline | All (none have it) | Differentiator | Content pipeline built, not wired to convergence |
| Audit trail UI/observability | Developer trust gap | Nice to have | State files exist, no UI |
| 82K+ stars community | Claude Code | Intentional (build quality, community follows) | N/A |
| Enterprise readiness (HIPAA/SOC2/SSO) | Claude Code, Codex | Future | Not started |

---

## Harness Competitors (New Category — Q1 2026)

### Claude Code (Anthropic)
- **URL:** https://code.claude.com
- **GitHub:** ~82K stars (March 2026)
- **Category:** Runtime coding harness (terminal-based)
- **Revenue:** ~$2.5B annualized run rate (all Claude products, early 2026)
- **Users:** 41-68% of developers actively using; 46% "most loved" (Pragmatic Engineer survey, 15K devs)
- **Architecture:** Terminal agent with LLM-driven think-act-observe loop. Auto mode uses classifier model to gate each action. Agent Teams (research preview) enable multi-agent parallel execution in git worktrees. 1M token context window. MCP protocol for integrations (50+ curated, hundreds community).
- **Strengths:** Massive ecosystem and community (82K stars, MCP standard adopted by OpenAI/Google), Agent Teams with git worktree isolation, sophisticated permission model (auto mode classifier + 12-event hook system), 1M token context, enterprise-ready (HIPAA, SOC2, SSO), voice input (20 languages), computer use/remote control, best blind code quality (wins 67% vs Codex 25%)
- **Weaknesses:** No convergence detection (LLM self-assesses "done"), LLM drives the loop (no code-driven orchestration), rate limit instability (March 2026 "war log" of outages), confirmed context degradation in long sessions (instructions evaporate), ecosystem lock-in (Opus restricted from third-party), no self-improvement, no multi-dimensional audit, pricing opacity
- **Our gap:** Multi-agent parallelism, hook system (12 lifecycle events), MCP ecosystem scale, enterprise readiness, voice input
- **Our moat:** Autonomous convergence (mathematical "done" signal), LLM minimization, 39-dimension audit, self-improvement cycle, universal project types (18 vs code-only)
- **Strategic position:** CruxDev is a convergence layer that runs ON TOP of Claude Code. Not competing — augmenting. Claude Code is execution-powerful but convergence-blind.

### Codex (OpenAI)
- **URL:** https://github.com/openai/codex
- **GitHub:** ~67K stars (March 2026), 400 contributors, 640 releases
- **Category:** Runtime coding harness (terminal CLI + cloud app)
- **Architecture:** Stateless agent loop (full conversation shipped per request, no server-side state). OS-level sandboxing: Apple Seatbelt on macOS, Landlock LSM + seccomp BPF on Linux (deny-by-default, kernel-enforced). Cloud sandboxes: isolated containers per task, no network. Up to 6 concurrent subagents (explorer/worker/default roles). GPT-5.4 (1M context), GPT-5.3-Codex, Spark (1000+ tok/sec).
- **Strengths:** Best-in-class kernel-level sandboxing (Seatbelt/Landlock/seccomp), fastest inference (Spark 1000+ tok/sec), cloud parallel execution (6 subagents in isolated containers), open source CLI, wins terminal/CLI benchmarks (75.1% vs 65.4%), 640 releases (extreme shipping velocity), 20+ curated plugins (launched March 26, 2026)
- **Weaknesses:** No convergence detection, LLM drives the loop, weak on frontend/React tasks, quadratic prompt growth (stateless = full history per request), cache fragility (tool/model changes invalidate), smaller plugin ecosystem (20 vs hundreds), rate limit frustration (5-hour rolling windows), loses 67% of blind code quality comparisons vs Claude
- **Our gap:** Kernel-level sandboxing, cloud sandbox pattern, parallel subagents, shipping velocity
- **Our moat:** Autonomous convergence, LLM minimization, 39-dimension audit, self-improvement, universal project types
- **Strategic position:** CruxDev augments Codex the same way it augments Claude Code. Codex is execution-fast but convergence-blind.

### Cursor
- **URL:** https://cursor.com
- **Category:** IDE-integrated harness
- **Revenue:** $2B ARR (March 2026), 1M+ daily active users
- **Architecture:** VS Code fork with inline completions (Fusion model), Composer agent mode (8 parallel agents), Cloud Agents (autonomous VMs with browser/terminal)
- **Strengths:** Best developer UX (IDE-native), self-testing agents (start app, interact, screenshot, video), cloud VM isolation, multi-model, tab autocomplete, 30% of their own PRs from cloud agents
- **Weaknesses:** No convergence concept, no safety gates, reliability issues (data loss reports), context inflation (200K advertised, 70-120K usable), code quality at scale, closed source
- **Our gap:** Self-testing with visual verification, IDE integration (but CruxDev plugs INTO Cursor via MCP)
- **Our moat:** Autonomous convergence, LLM minimization, multi-dimensional audit, self-improvement

### Manus
- **URL:** https://manus.im (acquired by Meta, Dec 2025, $2B)
- **Category:** General-purpose task harness
- **Architecture:** Multi-agent (Planner → Executor), context engineering mastery (KV-cache optimization, append-only context, tool masking, error preservation)
- **Strengths:** Best context engineering in the industry, 4x speed improvement through architecture alone, $125M+ ARR, empirically proven harness > model (5 rewrites, same model, each better)
- **Weaknesses:** Acquired by Meta (independence lost), closed source, not code-specific, no convergence guarantee
- **Our gap:** KV-cache-aware context engineering, controlled variation to prevent drift
- **Our moat:** Autonomous convergence, code-specific multi-dimensional audit, open source, self-improvement

### DeepAgents (LangChain)
- **URL:** https://github.com/langchain-ai/deepagents
- **Stars:** 17,796
- **Category:** General-purpose harness built on LangGraph
- **Architecture:** Middleware stack (TodoList + Filesystem + SubAgents), pluggable backends, durable execution via LangGraph checkpointing
- **Strengths:** Best composability (middleware mix-and-match), durable execution (crash recovery), model-agnostic, LangChain ecosystem (hundreds of integrations), open source
- **Weaknesses:** Abstraction tax (3 layers), young (v0.2), no convergence concept, high token cost
- **Our gap:** Durable mid-task checkpointing, pluggable storage backends, ecosystem breadth
- **Our moat:** Autonomous convergence, LLM minimization, safety gates, self-improvement

