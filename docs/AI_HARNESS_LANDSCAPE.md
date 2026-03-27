# AI Harness Landscape

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 30+ sources including Anthropic, OpenAI, Martin Fowler, Mitchell Hashimoto, Forrester, LangChain, arXiv papers
**Last updated:** 2026-03-27

## The Canonical Equation

```
Agent = Model + Harness
```

The model is a horse — powerful but undirected. The harness is reins, saddle, and bridle — everything that channels power into productive work. Without the harness, agents thrash, hallucinate, loop, and produce inconsistent quality.

**Origin:** "Harness engineering" coined by Mitchell Hashimoto (HashiCorp founder), late 2025. Formalized by OpenAI (February 2026). Validated by Anthropic, Martin Fowler, and three academic papers.

**The industry consensus forming in Q1 2026:** The model is commodity. The harness is moat. Manus proved this empirically — 5 harness rewrites with identical models, each rewrite dramatically better.

---

## 1. The Three-Layer Stack

| Layer | Purpose | Examples |
|-------|---------|---------|
| **Framework** | Defines agent logic (blueprint) | LangChain, CrewAI |
| **Runtime** | Manages execution, state, persistence | LangGraph, Inngest |
| **Harness** | Complete wrapper with methodology, tools, evaluation | Claude Code, CruxDev, Codex |

The harness is the highest layer. It sits above frameworks and runtimes.

---

## 2. Competitive Landscape

### Runtime Harnesses (run agents safely)

| System | Focus | Convergence? | LLM Minimization? | Self-Improving? |
|--------|-------|-------------|-------------------|-----------------|
| Claude Code | Terminal coding | No | No | No |
| Codex | Enterprise coding | No | No | No |
| DeepAgents | General purpose | No | No | No |
| Manus | Iterative improvement | Partial (rewrites) | No | Manual |
| Agent 365 | Enterprise control | No | No | No |
| Agentforce | CRM agents | No | No | No |

### What Nobody Has

| Capability | CruxDev | Everyone Else |
|-----------|---------|--------------|
| **Convergence detection** | 2 consecutive independent clean passes = done | Run until timeout or human says stop |
| **LLM minimization** | Engine owns loops, counters, termination. LLM is a tool. | LLM drives the loop |
| **Multi-dimensional audit** | 39+ dimensions across 10 dimension sets | Single-pass review or tests-pass gate |
| **Autonomous methodology** | Full lifecycle: plan → audit → execute → converge → update patterns | Runtime wrapping only |
| **Self-improvement** | Self-adoption finds tool/process gaps, fixes them, re-audits | Static harness, manual updates |
| **Universal project types** | 18 types: software, books, podcasts, businesses, etc. | Code only |
| **Domain architecture** | Parent projects with typed sub-projects | Flat project structure |

---

## 3. Where CruxDev Fits

CruxDev is an **autonomous convergence harness** — a harness that doesn't just run agents safely, it drives them to mathematically defined completion across multiple quality dimensions without human intervention.

### The Crux Ecosystem as Harness Stack

| Layer | Component | Role |
|-------|-----------|------|
| **Intelligence Harness** | Crux | Modes, safety gates, corrections, knowledge management |
| **Methodology Harness** | CruxDev | Convergence engine, auditing, templates, dimensions, self-improvement |
| **Runtime Harness** | CruxCLI | Interface where harnessed AI meets the user |

### The Subcategory

> Current harnesses answer: "How do I run an agent safely?"
> CruxDev answers: "How do I make an agent converge on correct output autonomously?"

This is **convergence engineering** — the discipline of driving agents to verified completion. It's a level above harness engineering, which focuses on runtime control.

---

## 4. Key Research Findings

- **Manus:** Rewrote harness 5 times with identical models. Each rewrite = better results. Proves harness > model.
- **Vercel:** Removed 80% of agent tools, got superior results. Less is more in harness design.
- **Hashimoto:** "Anytime you find an agent makes a mistake, engineer a solution so it never makes that mistake again."
- **Fowler:** Published formal article on harness engineering (March 2026).
- **Forrester:** Launching formal "Agent Control Plane" market evaluation.
- **Academic papers:** 3 papers on arXiv (2603.05344, 2603.25723, preprints.org/202603.1756).
- **The term is weeks old.** Dedicated websites already exist (harness-engineering.ai, harnessengineering.academy). SEO opportunity is wide open.

---

## 5. The Timing Window

The category is forming NOW (Q1-Q2 2026):
- Feb 2026: Term coined
- Mar 2026: Fowler article, 3 academic papers, Forrester announcement
- May 2026: Microsoft Agent 365 launches (enterprise harness)
- Q3 2026: Category likely consolidates around 2-3 dominant narratives

CruxDev's window to define "autonomous convergence harness" as a subcategory is approximately 3-6 months. After that, larger players will absorb the terminology.

---

## References

- Hashimoto (harness engineering origin) — Mitchell Hashimoto
- OpenAI (harness engineering for Codex) — openai.com/index/harness-engineering/
- Anthropic (effective harnesses) — anthropic.com/engineering/effective-harnesses-for-long-running-agents
- Martin Fowler — martinfowler.com/articles/exploring-gen-ai/harness-engineering.html
- LangChain (frameworks vs runtimes vs harnesses) — blog.langchain.com
- Forrester (agent control plane) — forrester.com/blogs/announcing-our-evaluation-of-the-agent-control-plane-market/
- Manus (5 harness rewrites) — manus.ai
- arXiv 2603.05344 — Building AI Coding Agents for the Terminal
- arXiv 2603.25723 — Natural-Language Agent Harnesses
- Aakash Gupta — "2025 Was Agents. 2026 Is Agent Harnesses."
