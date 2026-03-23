# Competitors

**Last Updated:** 2026-03-23
**Project:** CruxDev — Autonomous Convergence Framework

## Competitive Position

CruxDev is the only framework that drives AI coding agents through audit-fix-re-audit loops until two consecutive independent clean passes are achieved. Every competitor is either an execution substrate (runs code but doesn't know when it's done) or an analysis tool (scans code but doesn't drive fixes). CruxDev sits between them.

---

## Official Competitors (Actively Tracked)

### Superpowers
- **URL:** https://github.com/obra/superpowers
- **GitHub:** ~99,200 stars
- **Category:** Direct
- **License:** Open source

**Strengths:**
- Massive community (trending #1 on GitHub)
- Composable skills that auto-activate by context
- Two-stage review (spec compliance + code quality)
- Git worktree isolation per task
- TDD skill that enforces test-first

**Weaknesses:**
- No convergence detection — human chains skills together
- No multi-dimensional audit (8 code + 5 doc)
- No autonomous termination — agent never knows when to stop
- No safety gates beyond git isolation (no timeout, no rollback counter, no net-negative detection)

**Our moat vs them:**
- Convergence engine with two-consecutive-clean-pass criterion
- 8-dimension code audit + 5-dimension doc audit
- Autonomous execution without re-prompting
- Safety gates: timeout, 3-failure rollback, net-negative detection, phase-specific max rounds
- Session bus for cross-project coordination
- Evolution pipeline for autonomous self-improvement

**Their moat vs us:**
- 99K stars vs our early stage — massive mindshare advantage
- Skills auto-activate by context — we require explicit `/converge`
- Gap classification: should-close

### DeepSource
- **URL:** https://deepsource.com
- **Category:** Adjacent (code quality)
- **License:** Proprietary (free for open source)

**Strengths:**
- Five-dimension review (Security, Reliability, Complexity, Hygiene, Coverage)
- Autonomous agents that create PRs
- Autofix AI for one-click remediation
- Report card grades

**Weaknesses:**
- Single-pass scanning — no iterative convergence
- No convergence criterion — scans once and moves on
- SaaS-only — no local/self-hosted option
- Doesn't drive the development process, only reviews output

**Our moat vs them:**
- Iterative convergence loop (audit → fix → re-audit → repeat)
- Two-consecutive-clean-pass termination
- Controls the full lifecycle (plan → execute → audit → converge)
- Runs locally, no SaaS dependency

**Their moat vs us:**
- Production-grade UI and dashboard
- Enterprise customers
- Gap classification: intentional (different layer — we're a framework, they're a scanner)

### yoyo-evolve
- **URL:** https://github.com/yologdev/yoyo-evolve
- **GitHub:** ~560 stars
- **Category:** Direct (self-evolving agent)
- **License:** Open source

**Strengths:**
- Self-evolved from 200 to 18,000+ lines in 19 days
- Runs autonomously on cron (every 8 hours)
- Tests-must-pass gate before commit
- Proven the autonomous evolution model works

**Weaknesses:**
- Rust-only (single language)
- Simple test-pass gate (no multi-dimensional audit)
- No convergence criterion (just "tests pass")
- No safety gates beyond test passage
- Single project, single agent

**Our moat vs them:**
- Multi-dimensional audit (not just "tests pass")
- Convergence criterion (two consecutive clean passes)
- Multi-project support (session bus)
- Language-agnostic
- Full safety gate suite (timeout, rollback, net-negative)

**Their moat vs us:**
- Proven 19-day autonomous run — we haven't run unattended yet
- Gap classification: should-close (prove our evolution pipeline works)

---

## Watch List

| Name | URL | Stars | Why watching |
|------|-----|-------|-------------|
| Codex CLI | github.com/openai/codex | N/A | OpenAI's autonomous agent — could add convergence |
| Gemini CLI | github.com/google-gemini/gemini-cli | ~96K | Fast growth, free tier, could add methodology |
| OpenHands | github.com/OpenHands/OpenHands | ~69K | Strong autonomous agent platform |
| Augment Intent | augmentcode.com | N/A | "Living spec" concept closest to convergence thinking |
| Darwin Godel Machine | sakana.ai/dgm | N/A | Self-modifying agent research — could influence our evolution pipeline |

---

## Noted

| Name | Category | Why noted |
|------|----------|-----------|
| Claude Code | Execution substrate | We run on top of it |
| Cursor | IDE | Potential substrate |
| Windsurf | IDE | Potential substrate |
| Cline | VS Code agent | Potential substrate |
| Roo Code | VS Code agent | Potential substrate |
| Continue.dev | Assistant | Different category |
| Aider | Pair programming | Different model (interactive, not autonomous) |
| SonarQube | Scanner | Complementary, not competitive |
| CodeScene | Analysis | Complementary, not competitive |
| Codacy | Review platform | Complementary, not competitive |
| DSPy | Prompt optimization | Different domain |
| Devin | Cloud agent | Different model (SaaS junior engineer) |
| Jules | Cloud agent | Different model (async GitHub agent) |
| Amazon Q | Cloud agent | Enterprise, AWS lock-in |
| Copilot | IDE | Massive distribution, no convergence |
| SWE-agent | Research | Academic, benchmark-focused |

---

## Feature Matrix

| Feature | CruxDev | Superpowers | DeepSource | yoyo-evolve |
|---------|:-------:|:-----------:|:----------:|:-----------:|
| Convergence detection (2 clean passes) | ✓ | — | — | — |
| Multi-dimensional audit (8+5) | ✓ | — | 5 dims | — |
| Autonomous execution | ✓ | Partial | — | ✓ |
| Safety gates (timeout/rollback/net-neg) | ✓ | Partial | — | Partial |
| Cross-project coordination | ✓ | — | — | — |
| Self-evolution pipeline | ✓ | — | — | ✓ |
| Model tier routing + escalation | ✓ | — | — | — |
| Doc alignment gate | ✓ | — | — | — |
| Green-field execution | ✓ | ✓ | — | — |
| Protected files enforcement | ✓ | — | — | — |
| TDD enforcement | ✓ | ✓ | — | ✓ |
| Open source | ✓ | ✓ | Partial | ✓ |

---

## Gap Closure Queue

| Gap | Competitor | Classification | Status |
|-----|-----------|---------------|--------|
| Skills auto-activate by context | Superpowers | Should close | Not started |
| Production dashboard/UI | DeepSource | Nice to have | Not started |
| Proven unattended evolution run | yoyo-evolve | Should close | Evolution pipeline built, not yet run unattended |
| 99K stars community | Superpowers | Intentional gap (build quality, community follows) | N/A |
