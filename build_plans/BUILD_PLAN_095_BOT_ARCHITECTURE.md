# BUILD_PLAN_095: CruxDev Bot Architecture — Always-On Autonomous Agent

**Status:** NOT STARTED
**Priority:** Critical (defines the product's future)

## Context

CruxDev's autonomous mode already has all the pieces of a bot:
- Priority engine (what to work on)
- Convergence loop (how to finish work)
- Content pipeline (how to communicate)
- Issue monitoring (how to receive work)
- Evolution cycle (how to self-improve)
- GitHub integration (how to coordinate)

The missing piece: a persistent core routine that loops forever, taking on ANY task from ANY source. Not a cron job that runs every 4 hours — a daemon that's always running, always watching, always converging.

## The Core Loop

```
while true:
    1. CHECK — scan all work sources (issues, build plans, gaps, inbox, external triggers)
    2. PRIORITIZE — score and rank all available work
    3. CLAIM — pick the top item, mark it in-progress
    4. CONVERGE — drive it to completion (plan → audit → fix → re-audit → converge)
    5. DELIVER — commit, push, blog, X post, close issue, deploy
    6. REFLECT — self-adopt, update patterns, check for regressions
    7. REPORT — sync terminal viewer, log to stream
    8. SLEEP — wait for new signals (webhook, timer, file change)
    goto 1
```

## What This Competes With

| Bot/Platform | What It Does | CruxDev Advantage |
|---|---|---|
| OpenClaw | AI agent marketplace — run specialized bots | CruxDev IS the bot, with convergence |
| Devin | Cloud AI coding agent | CruxDev converges, Devin doesn't know when it's done |
| AutoGPT/BabyAGI | Goal-seeking agents | CruxDev has mathematical termination, they loop forever |
| GitHub Copilot Workspace | PR-level coding agent | CruxDev manages the full lifecycle, not just PRs |
| Sweep AI | Automated bug fixing | CruxDev fixes AND verifies AND documents AND announces |
| Dependabot | Dependency updates | CruxDev convergence-verifies every update |

## Phase 1: Persistent Daemon Mode

- [ ] 1.1 `cruxdev daemon` CLI command — starts the core loop
- [ ] 1.2 File-watch trigger: detect changes in project dir → re-prioritize
- [ ] 1.3 Webhook trigger: GitHub webhook → instant issue pickup (not 4-hour wait)
- [ ] 1.4 Timer trigger: periodic health checks (SEO, deployment, link validation)
- [ ] 1.5 Idle detection: if no work, sleep with exponential backoff (1min → 5min → 15min → 1hr)

## Phase 2: Multi-Project Bot

- [ ] 2.1 Configure multiple projects in a single daemon instance
- [ ] 2.2 Round-robin or priority-based project selection
- [ ] 2.3 Cross-project issue coordination (file issues between projects)
- [ ] 2.4 Shared pattern improvements flow to all projects

## Phase 3: External Task Sources

- [ ] 3.1 GitHub webhook receiver (issue created → immediate pickup)
- [ ] 3.2 Slack integration (message in channel → create issue → converge)
- [ ] 3.3 Email integration (forward email → create issue → converge)
- [ ] 3.4 API endpoint (`POST /api/task` → create build plan → converge)
- [ ] 3.5 Scheduled tasks (cron-like but inside the daemon)

## Phase 4: Capability Plugins

Make CruxDev extensible so it can take on ANY bot's role:

| Capability | Plugin | What It Replaces |
|---|---|---|
| Code review | PR convergence | CodeRabbit, Copilot |
| Dependency updates | Build freshness + convergence | Dependabot, Renovate |
| Security scanning | Security dimension + contrast scanner | Snyk, Sonarqube |
| Documentation | Doc alignment gate | ReadMe, GitBook auto-update |
| Content generation | BIP pipeline | Jasper, Copy.ai |
| SEO monitoring | SEO health + PageSpeed | Ahrefs, Semrush alerts |
| Competitive intelligence | Competitor pipeline | Klue, Crayon |
| Issue triage | Monitor + prompt injection defense | Linear Triage, GitHub Auto-label |
| Release management | Git automation + deploy verification | Release Please, semantic-release |
| Translation | i18n pipeline (BP093) | Crowdin, Lokalise |

## Phase 5: The OpenClaw Play

If CruxDev IS the bot, then:
1. Every MCP tool is a capability
2. Every pattern doc is domain knowledge
3. Every project type is a specialization
4. Every template is a starting point
5. Every audit dimension is a quality gate

OpenClaw lists specialized bots. CruxDev is ONE bot that converges ANY project type. The convergence harness IS the universal bot architecture.

## Phase 6: Bot Marketplace Integration

- [ ] 6.1 Publish CruxDev as an OpenClaw agent
- [ ] 6.2 Define capability manifest (what CruxDev can do)
- [ ] 6.3 Accept tasks via OpenClaw's task protocol
- [ ] 6.4 Return converged results with audit trail

## Key Architectural Decisions

1. **LLM is called, not calling.** The daemon drives the loop. The LLM is a tool for language tasks. This is LLM minimization — the core advantage.
2. **Convergence is the differentiator.** Every other bot runs once. CruxDev runs until done.
3. **Self-improvement is built in.** Every task makes the bot better at the next task.
4. **Observability is native.** The terminal viewer shows what the bot is doing in real-time.
5. **Safety gates prevent runaway.** Max actions per cycle, STOP file, timeout, rollback.

## The Vision

One bot. Any project type. Any vertical. Convergence-guaranteed. Self-improving. Observable. Safe.

That's what CruxDev becomes.
