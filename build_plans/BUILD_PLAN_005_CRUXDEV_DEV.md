# BUILD_PLAN_005: cruxdev.dev — CruxDev Convergence Framework Website

**Created:** 2026-03-22
**Status:** NOT STARTED
**Goal:** Build the documentation and methodology site for CruxDev — the autonomous convergence framework.
**Methodology:** Follow `docs/WEBSITE_PLANNING.md`. SEO details in `docs/SEO_AND_GEO_REFERENCE.md`.

**Domain:** cruxdev.dev
**Product:** CruxDev — convergence engine + development methodology
**Category:** Developer methodology/framework (like Nx, Turborepo, Playwright pattern)
**Current state:** Engine shipped (314 tests, 100% coverage). MCP server with 10 tools. Slash commands. Model tier system. Documentation-heavy product — the site IS the docs.

---

## Audiences

| Audience | Priority | They want | Entry point |
|----------|----------|-----------|-------------|
| Developers tired of "do it again" loops | Primary | Autonomous convergence they can trust | "Say your goal once. The agent converges." |
| Teams adopting AI coding at scale | Secondary | Methodology, safety, reproducibility | Methodology docs, adoption guide |
| AI framework builders | Tertiary | Architecture patterns, engine design | Architecture docs, GitHub |

---

## Site Map

```
cruxdev.dev/
├── / (homepage)
├── /methodology
│   ├── /methodology/overview (DEVELOPMENT_PATTERNS_CRUXDEV.md as web)
│   ├── /methodology/convergence (two clean passes, independence)
│   ├── /methodology/audit-dimensions (8 code + 5 doc)
│   ├── /methodology/safety-gates (timeout, rollback, net-negative)
│   └── /methodology/patterns (anti-patterns, conventions)
├── /engine
│   ├── /engine/overview (architecture, state machine)
│   ├── /engine/mcp-tools (10 tools reference)
│   ├── /engine/task-router (how tasks are assigned)
│   ├── /engine/model-tiers (micro→fast→standard→frontier)
│   └── /engine/api-reference
├── /docs
│   ├── /docs/quickstart (adopt a project in 10 minutes)
│   ├── /docs/installation
│   ├── /docs/slash-commands (/converge, /plan, /adopt, /status)
│   ├── /docs/configuration
│   └── /docs/troubleshooting
├── /guides
│   ├── /guides/first-convergence (walk through a real convergence)
│   ├── /guides/writing-build-plans
│   ├── /guides/adopting-legacy-projects
│   └── /guides/model-routing
├── /blog
├── /changelog
├── /community
├── /vs
│   ├── /vs/superpowers
│   └── /vs/manual-iteration
├── /llms.txt
└── /llms-full.txt
```

---

## Homepage Sections

1. **Hero**: "The convergence engine for AI coding." / "Say your goal once. The agent plans, audits, fixes, and converges to completion — two independent clean passes — without you saying 'do it again.'" / CTA: "Get Started" + "Read the Methodology"
2. **Problem**: "One clean pass misses ~30% of issues. Anchoring bias means the same agent re-checking its own work finds nothing new. CruxDev catches what you miss."
3. **How it works**: 4 steps — Plan → Audit (8 dimensions) → Fix → Converge (2 clean passes)
4. **Trust**: Test count (314), coverage (100%), engine stats, convergence metric
5. **Methodology preview**: Key principles — TDD, two-pass convergence, independence checking, safety gates
6. **Engine architecture**: Visual diagram — state machine phases, MCP tools, model tier routing
7. **Comparison**: CruxDev vs "do it again" (manual iteration) — table showing what's automated
8. **Final CTA**: "Adopt CruxDev in 10 minutes" → quickstart

---

## SEO Targets

| Keyword | Intent | Content |
|---------|--------|---------|
| "AI code convergence" | Informational | Methodology: convergence overview |
| "autonomous coding agent" | Informational | Blog: what autonomous convergence means |
| "Superpowers alternative" | Commercial | /vs/superpowers comparison |
| "AI code audit framework" | Informational | Engine: audit dimensions page |
| "AI coding methodology" | Informational | Methodology overview |
| "TDD with AI" | Informational | Guide: TDD-first convergence |
| "AI coding quality assurance" | Informational | Blog: multi-pass auditing |
| "MCP tools for coding" | Navigational | Engine: MCP tools reference |

---

## AI Visibility

- `/llms.txt`: CruxDev identity, methodology summary, how to install, what convergence means
- The methodology itself is the SEO and GEO content — it's inherently authoritative
- Schema.org: `SoftwareApplication` on homepage, `HowTo` on guides, `TechArticle` on methodology
- Specific metrics: "314 tests", "100% coverage", "8 code audit dimensions", "two consecutive clean passes"
- FAQ pages for common questions: "What is convergence?", "How does the engine decide when to stop?"

---

## Checklist

### Phase 1: Content & Structure
- [ ] 1.1 Homepage copy (hero, problem, how-it-works, architecture)
- [ ] 1.2 Methodology pages (convert DEVELOPMENT_PATTERNS_CRUXDEV.md to web)
- [ ] 1.3 Engine pages (architecture, MCP tools, task router, model tiers)
- [ ] 1.4 Quickstart guide (adopt → plan → converge)
- [ ] 1.5 Slash command reference (/converge, /plan, /adopt, /status)
- [ ] 1.6 Comparison page (vs Superpowers, vs manual iteration)
- [ ] 1.7 Guide: writing good build plans
- [ ] 1.8 Guide: first convergence walkthrough
- [ ] 1.9 llms.txt and llms-full.txt
- [ ] 1.10 All meta tags and structured data

### Phase 2: Design & Build
- [ ] 2.1 Design system (clean, technical, docs-focused)
- [ ] 2.2 Homepage design
- [ ] 2.3 Docs/methodology layout (three-column Stripe-style if applicable)
- [ ] 2.4 Architecture diagrams (state machine, phase flow, MCP tool map)
- [ ] 2.5 Build site
- [ ] 2.6 Responsive design
- [ ] 2.7 Code block styling with copy buttons

### Phase 3: Technical
- [ ] 3.1 Core Web Vitals green
- [ ] 3.2 SEO: sitemap, structured data, canonicals
- [ ] 3.3 Accessibility: WCAG 2.1 AA
- [ ] 3.4 Security: HTTPS, CSP
- [ ] 3.5 Analytics
- [ ] 3.6 Docs search (Algolia DocSearch or similar)

### Phase 4: Launch
- [ ] 4.1 DNS for cruxdev.dev
- [ ] 4.2 Deploy
- [ ] 4.3 Search Console
- [ ] 4.4 Pre-launch QA
- [ ] 4.5 Launch announcement
- [ ] 4.6 Post-launch verification

**Total: 28 checkboxes**
