# BUILD_PLAN_012: CruxDev Website Convergence

**Created:** 2026-03-26
**Status:** CONVERGED (superseded by BP027/032)
**Goal:** Bring cruxdev.dev to current CruxDev standards — all nav pages populated, landing pages created, SEO keyword registry, structured data, deployment ready.

**Go-to-market name:** CruxDev
**Project name:** cruxdev
**Website repo:** /Users/user/personal/cruxdev-dev
**Engine:** Astro + Tailwind CSS v4 (conscious selection — content-first, zero JS by default)
**Domain:** cruxdev.dev
**Logo:** None (flagged as gap — using text branding)

**Rule:** Every nav link must resolve to a real page.
**Rule:** Content sourced from docs/ — not invented. The site reflects the actual project.
**Rule:** Landing pages indexed via sitemap, NOT in nav.

---

## Phase 1: Fix Critical Nav Pages

**Purpose:** Every nav link currently 404s except / and /vs/. Fix this.

- [ ] 1.1 /methodology — render DEVELOPMENT_PATTERNS_CRUXDEV.md as a page (the convergence methodology)
- [ ] 1.2 /engine — render CruxDev.md as a page (engine architecture, phases, safety gates)
- [ ] 1.3 /docs — index page linking to quickstart, adoption, methodology, patterns
- [ ] 1.4 /docs/quickstart — getting started guide (from ADOPTION_PROCESS.md)
- [ ] 1.5 /docs/adoption — adoption playbook (from ADOPTION_PLAYBOOK.md)
- [ ] 1.6 /docs/research — research patterns (from RESEARCH_PATTERNS.md)
- [ ] 1.7 /blog — index page (empty state with "Coming soon" or first post about the project)
- [ ] 1.8 Verify all nav links resolve — zero 404s

---

## Phase 2: Content Pages from Docs

**Purpose:** The docs/ folder has rich content. Surface it on the site.

- [ ] 2.1 /docs/competitors — competitive analysis methodology (from COMPETITORS_PATTERN.md)
- [ ] 2.2 /docs/website-planning — website planning methodology (from WEBSITE_PLANNING.md)
- [ ] 2.3 /docs/seo — SEO/GEO reference (from SEO_AND_GEO_REFERENCE.md)
- [ ] 2.4 /docs/engines — website engine ecosystem (from WEBSITE_ENGINES.md)
- [ ] 2.5 /docs/e2e-testing — E2E test patterns (from E2E_TEST_PATTERNS.md)
- [ ] 2.6 /docs/uat-testing — UAT test patterns (from UAT_TEST_PATTERNS.md)
- [ ] 2.7 Create a markdown rendering layout that auto-converts .md docs to site pages

---

## Phase 3: SEO Keyword Registry + Landing Pages

**Purpose:** Per SEO_AND_GEO_REFERENCE.md §1.6 — research keywords across all 5 tiers and create landing pages.

- [ ] 3.1 Create docs/SEO_KEYWORDS.md with initial keyword registry (all 5 tiers)
- [ ] 3.2 Tier 1 (solution-aware): "AI code convergence tool", "autonomous code review", "AI coding agent framework"
- [ ] 3.3 Tier 2 (problem-aware): "how to stop re-prompting AI coding agent", "AI code review multiple passes"
- [ ] 3.4 Tier 3 (symptom-aware): "AI code keeps introducing bugs", "AI coding agent never finishes"
- [ ] 3.5 Tier 4 (problem-unaware): "how to know when AI code is done", "is one code review pass enough"
- [ ] 3.6 Create /lp/ directory with landing pages for top 5-10 keywords
- [ ] 3.7 Each landing page: answer-first structure, 800+ words, internal links to nav pages, CTA
- [ ] 3.8 Add /lp/* pages to sitemap.xml (NOT to nav)
- [ ] 3.9 Update llms.txt with landing page URLs

---

## Phase 4: Structured Data + SEO Polish

**Purpose:** Maximize search engine and AI visibility.

- [ ] 4.1 FAQPage schema on all /vs/ pages (already done) — verify
- [ ] 4.2 SoftwareApplication schema on homepage (already done) — verify and update
- [ ] 4.3 Article schema on /docs/* pages
- [ ] 4.4 BreadcrumbList schema on all pages
- [ ] 4.5 Meta descriptions on every page (unique, keyword-targeted)
- [ ] 4.6 OG images for social sharing (text-based since no logo)
- [ ] 4.7 Verify robots.txt allows all content pages
- [ ] 4.8 Generate sitemap.xml with all pages including /lp/*

---

## Phase 5: Homepage Refresh

**Purpose:** Homepage metrics are current but content should reflect the full scope.

- [ ] 5.1 Update metrics to current values (verify against codebase)
- [ ] 5.2 Add "Compare" section previewing competitor comparisons
- [ ] 5.3 Add "Documentation" section linking to key docs
- [ ] 5.4 Add ecosystem section (Crux + CruxCLI + CruxDev triangle)
- [ ] 5.5 Verify all CTAs link to real pages

---

## Document Alignment

### Product Docs (this plan must conform to):
- docs/WEBSITE_PLANNING.md — Phase 0 (identity, logo, engine), Phase 12 (operations), §4.5 (comparison pages)
- docs/SEO_AND_GEO_REFERENCE.md — §1.6 (keyword-to-landing-page system), §1.6.4 (indexing without nav)
- docs/WEBSITE_ENGINES.md — engine selection rationale
- docs/WEBSITE.md — site info, metrics, comparison page status
- docs/DEPLOYMENT.md — build/deploy process
- docs/COMPETITORS.md — source data for /vs/ pages
- docs/CruxDev.md — source for /engine page
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — source for /methodology page

---

## Test Commands

```bash
# Build the site (zero errors = pass)
cd /Users/user/personal/cruxdev-dev && npm run build

# Verify no broken internal links
grep -r 'href="/' src/pages/ | grep -v node_modules

# CruxDev tests (must still pass)
cd /Users/user/personal/cruxdev && python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: update docs/WEBSITE.md with current page list and metrics
- [ ] Website convergence: all pages build, all nav links resolve, all metrics current
- [ ] Landing page verification: /lp/* pages in sitemap, not in nav, content accurate
- [ ] Deployment: deploy per docs/DEPLOYMENT.md (when hosting is configured)
- [ ] Patterns update: capture learnings if novel
- [ ] Inbox check: process messages from other sessions

## Convergence Criteria

- All checklist items complete
- Site builds with zero errors
- Every nav link resolves to a real page (zero 404s)
- Metrics match codebase (tests, tools, coverage)
- Landing pages in sitemap but not in nav
- SEO keyword registry created
- Two consecutive clean audit passes
