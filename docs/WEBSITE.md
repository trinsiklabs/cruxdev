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

| Route | Purpose |
|-------|---------|
| `/` | Homepage — hero, trust metrics, methodology overview, safety gates, model tiers |
| `/docs/` | Documentation (quickstart, guides) |
| `/methodology/` | Full development methodology |
| `/engine/` | Engine architecture and specs |
| `/guides/` | Usage guides |
| `/blog/` | Blog (empty) |
| `/vs/` | Competitor comparison pages (empty — needs generation from COMPETITORS.md) |

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
