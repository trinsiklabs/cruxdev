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
| Tests passing | 543 | 1,075 | STALE |
| Test coverage | 100% | 100% | Current |
| MCP tools | 18 | 39 | STALE |
| Clean passes | 2 | 2 | Current |

## Comparison Pages

The `/vs/` directory exists but is empty. Should be populated from `docs/COMPETITORS.md` using `generate_comparison_page()`. Official competitors to create pages for:
- Superpowers
- Backbeat
- DeepSource
- yoyo-evolve

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
