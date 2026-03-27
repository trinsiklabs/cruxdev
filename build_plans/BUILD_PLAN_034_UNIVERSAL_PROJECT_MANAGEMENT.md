# BUILD_PLAN_034: Universal Project Management — Beyond Software

**Status:** NOT STARTED
**Priority:** Strategic (market expansion)
**Depends on:** BUILD_PLAN_030 (TypeScript toolchain — proves multi-language support)

## Context

CruxDev currently manages software projects only. But the convergence model (plan → audit → fix → re-audit → converge) applies to ANY project with deliverables, quality criteria, and iterative improvement. A business plan needs the same two-clean-pass discipline as a codebase. An ebook chapter needs the same multi-dimensional audit (accuracy, completeness, clarity, consistency, currency) as documentation.

An existing template library at `/Users/user/trinsik-collaboration/swarm_sync/templates/` contains **489 templates across 18 categories** — already researched documentation standards for businesses, books, podcasts, newsletters, YouTube, open source, campaigns, legal, financial, and more. CruxDev should leverage these as the quality templates that its convergence engine audits against.

## The Vision

CruxDev manages **composite projects** — a business like CruxVibe that has:
- A closed-source software product (convergence on code)
- Open source tools it supports: crux, cruxdev, cruxcli (convergence on each)
- A marketing website (website convergence)
- A payment system (integration convergence)
- A marketing podcast (episode quality convergence)
- An email newsletter (issue quality convergence)
- An ebook explaining the CruxVibe difference (chapter/book convergence)

Or an **author** who:
- Writes standalone books and series (chapter convergence, series bible consistency)
- Has a website for each series and an author portal (website convergence)
- Runs a YouTube channel on book topics (video planning convergence)
- Has a newsletter for readers (newsletter convergence)

Each sub-project has its own convergence criteria, but they all share the same engine.

## Template Categories Available (from swarm_sync)

| Category | Files | Sub-types | Examples |
|----------|-------|-----------|---------|
| **projects/business** | 28 | — | Business plan, brand guidelines, pricing strategy, OKRs, SOPs |
| **projects/code** | 18 | — | Architecture, API, deployment, security, testing |
| **projects/website** | — | — | Website planning, SEO, deployment |
| **projects/marketing** | — | — | Marketing plans, campaigns |
| **publishing/book** | 8 | — | Book outline, chapter template, series bible, manuscript tracking |
| **publishing/podcast** | 6 | — | Show format, episode plan, guest research, distribution |
| **publishing/newsletter** | 5 | — | Strategy, issue plan, growth plan, analytics |
| **publishing/youtube** | 6 | — | Channel strategy, video SEO, upload schedule, thumbnail guide |
| **publishing/blog** | — | — | Blog content templates |
| **publishing/course** | — | — | Course creation templates |
| **publishing/community** | — | — | Community management |
| **publishing/opensource** | 4 | — | Contributing guide, governance, release process |
| **campaigns** | 25 | — | Campaign briefs, budgets, channels |
| **financial** | 16 | — | Budgets, projections, invoicing |
| **legal** | 18 | — | NDAs, TOS, privacy, contracts |
| **people** | 14 | — | Hiring, onboarding, reviews |
| **governance** | 17 | — | Policies, compliance, risk |
| **communications** | 16 | — | PR, crisis, internal comms |

## Phase 1: Project Type Classification

### 1.1 Extend project classifier
- [ ] 1.1.1 Current `classify_project` in adoption/classify.rs only detects software projects
- [ ] 1.1.2 Add detection for: business (BUSINESS_PLAN.md), book (BOOK_OUTLINE.md, MANUSCRIPT_TRACKING.md), podcast (SHOW_FORMAT.md), newsletter (NEWSLETTER_STRATEGY.md), youtube (CHANNEL_STRATEGY.md)
- [ ] 1.1.3 Add composite project detection — multiple sub-project types in one repo or linked repos
- [ ] 1.1.4 ProjectType enum: Software, Website, Business, Book, BookSeries, Podcast, Newsletter, YouTube, Course, OpenSource, Composite

### 1.2 Template registry
- [ ] 1.2.1 Import templates from swarm_sync into CruxDev's template system
- [ ] 1.2.2 Map each project type to its required templates
- [ ] 1.2.3 `get_templates` MCP tool returns correct templates for ANY project type, not just software

## Phase 2: Non-Software Convergence Dimensions

### 2.1 Content dimensions (books, blogs, newsletters)
- [ ] 2.1.1 CONTENT_DIMENSIONS: accuracy, completeness, clarity, consistency, engagement, structure, voice, citations

### 2.2 Business dimensions
- [ ] 2.2.1 BUSINESS_DIMENSIONS: viability, market_fit, financial_soundness, legal_compliance, competitive_position, scalability

### 2.3 Media dimensions (podcast, youtube)
- [ ] 2.3.1 MEDIA_DIMENSIONS: content_quality, production_quality, audience_fit, SEO, accessibility, consistency

### 2.4 Composite project dimensions
- [ ] 2.4.1 COMPOSITE_DIMENSIONS: cross_project_consistency, brand_alignment, dependency_health, schedule_alignment

## Phase 3: Composite Project Architecture

### 3.1 Project graph
- [ ] 3.1.1 A composite project is a directed graph of sub-projects
- [ ] 3.1.2 Sub-projects can depend on each other (website depends on product, newsletter depends on book releases)
- [ ] 3.1.3 Changes in one sub-project can trigger convergence in dependents

### 3.2 Configuration
```toml
[composite]
name = "CruxVibe"
type = "business"

[[composite.sub_projects]]
name = "cruxvibe-product"
type = "software"
path = "../cruxvibe"

[[composite.sub_projects]]
name = "cruxvibe-website"
type = "website"
path = "../cruxvibe-site"
depends_on = ["cruxvibe-product"]

[[composite.sub_projects]]
name = "cruxvibe-podcast"
type = "podcast"
path = "../cruxvibe-podcast"

[[composite.sub_projects]]
name = "cruxvibe-newsletter"
type = "newsletter"
path = "../cruxvibe-newsletter"
depends_on = ["cruxvibe-podcast"]

[[composite.sub_projects]]
name = "cruxvibe-ebook"
type = "book"
path = "../cruxvibe-ebook"
```

### 3.3 Cross-project convergence
- [ ] 3.3.1 When product ships a feature → website needs update → newsletter announces
- [ ] 3.3.2 Session bus carries these signals between sub-projects
- [ ] 3.3.3 Convergence on composite = all sub-projects converged + cross-project consistency

## Phase 4: Book/Series Support

### 4.1 Book convergence
- [ ] 4.1.1 Chapter as convergence unit (like a file in code)
- [ ] 4.1.2 Series bible as consistency reference (like architecture docs)
- [ ] 4.1.3 Manuscript tracking as project status
- [ ] 4.1.4 Content dimensions per chapter: accuracy, completeness, voice, pacing, consistency_with_series

### 4.2 Series management
- [ ] 4.2.1 Series bible convergence — all books consistent with bible
- [ ] 4.2.2 Character/timeline tracking across books
- [ ] 4.2.3 Cross-book references verified

## Phase 5: MCP Tools

- [ ] 5.1 `classify_project` — updated to detect all project types
- [ ] 5.2 `get_templates` — returns templates for any project type
- [ ] 5.3 `analyze_composite` — analyze a multi-project business/ecosystem
- [ ] 5.4 `converge_chapter` — convergence on a book chapter
- [ ] 5.5 `converge_episode` — convergence on a podcast episode plan

## Phase 6: Tests

- [ ] 6.1 Project type detection for each type
- [ ] 6.2 Template lookup for each type
- [ ] 6.3 Composite project graph building
- [ ] 6.4 Cross-project dependency detection

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
