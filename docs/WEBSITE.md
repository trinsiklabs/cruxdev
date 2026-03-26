---
title: "CruxDev Website"
last_updated: "2026-03-26"
---

# CruxDev Website

## Site Info

| Field | Value |
|-------|-------|
| **URL** | https://cruxdev.dev |
| **Repo** | `/Users/user/personal/cruxdev-dev` |
| **Framework** | Astro + Tailwind CSS v4 |
| **Domain** | cruxdev.dev |
| **Hosting** | TBD (not yet deployed to production) |

## Pages

| Route | Purpose | Status |
|-------|---------|--------|
| `/` | Homepage — hero, metrics, methodology, safety gates, model tiers, compare, ecosystem, docs | Live |
| `/methodology` | Convergence methodology — lifecycle, safety gates, audit dimensions, LLM minimization | Live |
| `/engine` | Engine architecture — 39 MCP tools, 12 phases, model routing, input normalization | Live |
| `/docs` | Documentation index — links to all doc sections | Live |
| `/docs/quickstart` | 5-minute getting started guide | Live |
| `/docs/adoption` | 9-phase adoption playbook | Live |
| `/docs/research` | 5-pass research methodology | Live |
| `/docs/competitors` | Competitive analysis methodology | Live |
| `/docs/website-planning` | 12-phase website planning | Live |
| `/docs/engines` | 15 website engines compared | Live |
| `/docs/seo` | SEO/GEO reference | Live |
| `/docs/e2e-testing` | E2E test patterns | Live |
| `/docs/uat-testing` | UAT test patterns | Live |
| `/blog` | Blog (coming soon) | Live |
| `/vs` | Competitor comparison index | Live |
| `/vs/superpowers` | CruxDev vs Superpowers | Live |
| `/vs/backbeat` | CruxDev vs Backbeat | Live |
| `/vs/deepsource` | CruxDev vs DeepSource | Live |
| `/vs/yoyo-evolve` | CruxDev vs yoyo-evolve | Live |
| `/lp/stop-re-prompting` | Landing: Stop re-prompting your AI agent | Live |
| `/lp/when-is-code-done` | Landing: When is AI code done? | Live |
| `/lp/ai-coding-mistakes` | Landing: AI agent keeps making mistakes | Live |
| `/lp/one-pass-not-enough` | Landing: One review pass isn't enough | Live |
| `/lp/ai-doom-loop` | Landing: Breaking the doom loop | Live |

## Metrics (on homepage)

These must be kept current with the actual codebase:

| Metric | Current on Site | Actual | Status |
|--------|----------------|--------|--------|
| Tests passing | 1,075 | 1,075 | Current |
| Test coverage | 100% | 100% | Current |
| MCP tools | 39 | 39 | Current |
| Clean passes | 2 | 2 | Current |

## Comparison Pages

Generated from `docs/COMPETITORS.md` per `WEBSITE_PLANNING.md` §4.5:

| Route | Competitor | Status |
|-------|-----------|--------|
| `/vs/` | Index page | Created |
| `/vs/superpowers` | Superpowers (110K stars, direct) | Created |
| `/vs/backbeat` | Backbeat (3 stars, direct) | Created |
| `/vs/deepsource` | DeepSource (adjacent, code review) | Created |
| `/vs/yoyo-evolve` | yoyo-evolve (669 stars, direct) | Created |

Each page includes: feature comparison table, strengths/weaknesses, FAQ, FAQPage schema.org structured data, CTA.

## Build & Deploy

```bash
cd /Users/user/personal/cruxdev-dev
npm install
npm run dev      # Local dev server at localhost:4321
npm run build    # Production build to ./dist/
npm run preview  # Preview production build
```

## SEO Assets

- `public/robots.txt` — exists
- `public/llms.txt` — exists (AI visibility)
- `public/favicon.svg` — exists

## Relationship to CruxDev

This is the marketing/documentation site for the CruxDev convergence engine. It lives in a separate repo (`cruxdev-dev`) at the same directory level as the main `cruxdev` repo. The WEBSITE_CONVERGENCE phase in the engine auto-detects this site via the presence of this `docs/WEBSITE.md` file and `docs/DEPLOYMENT.md`.
