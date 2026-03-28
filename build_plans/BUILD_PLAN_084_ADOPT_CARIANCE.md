# BUILD_PLAN_084: Adopt Cariance — Book Series + Business + Website + Course

**Status:** CONVERGED
**Priority:** High (adoption test for non-software project types)
**Project:** /Users/user/personal/cariance

## Classification

| Type | Weight | Evidence |
|---|---|---|
| BookSeries | Primary | 3 series (Sacred Threshold Practices, Coming Home to Yourself, Inner Architecture), 10+ books, chapter structures |
| Business | Secondary | 5-phase vision 2025-2030, revenue projections ($2K-$70K/yr range), coaching framework (TRC) |
| Course | Secondary | "Coming Home to Yourself" online course framework, audio scripts, 10 practices |
| Website | Secondary | 242 SEO landing pages planned, hub-and-spoke architecture, landing page structure guide |
| Campaign | Secondary | SEO taxonomy (7 hubs, 700-1000+ pages planned), email newsletter strategy |

**Maturity:** Minimal → Growing (Book 1 near final, Books 2-10 in draft, no website yet, no git)

## Phase 1: Project Structure

- [ ] 1.1 Initialize git repo
- [ ] 1.2 Create docs/ directory — move planning docs there
- [ ] 1.3 Create standardized structure:
  ```
  cariance/
  ├── docs/                    # Planning, strategy, methodology
  │   ├── VISION.md            # From PDF → markdown
  │   ├── VOICE_GUIDE.md       # Consolidated voice/style guide
  │   ├── SEO_TAXONOMY.md      # Master SEO taxonomy
  │   ├── LANDING_PAGE_GUIDE.md
  │   └── COMPETITORS.md       # Other books in this space
  ├── series-1/                # Sacred Threshold Practices
  │   ├── SERIES_PLAN.md
  │   ├── book-1-sacred-pause/
  │   ├── book-2-returning-to-center/
  │   ├── book-3-lift-the-fog/
  │   ├── book-4-threshold-breath/
  │   └── book-5-ten-practices/
  ├── series-2/                # Coming Home to Yourself
  │   ├── SERIES_PLAN.md
  │   ├── book-6-soul-expanding/
  │   ├── book-7-conversation-within/
  │   ├── book-8-what-lives-in-dark/
  │   ├── book-9-body-remembers/
  │   └── book-10-thread-holds/
  ├── series-3/                # Inner Architecture
  │   ├── SERIES_PLAN.md
  │   └── (future books)
  ├── course/                  # Coming Home to Yourself course
  │   ├── COURSE_OUTLINE.md
  │   ├── audio-scripts/
  │   └── practices/
  ├── coaching/                # TRC framework
  │   └── FRAMEWORK.md
  ├── website/                 # SEO landing pages (when ready)
  ├── .cruxdev/                # CruxDev config
  └── README.md
  ```
- [ ] 1.4 Move existing files into structure (preserve content, rename for consistency)
- [ ] 1.5 Create README.md with project overview

## Phase 2: Install CruxDev

- [ ] 2.1 Run `cruxdev install /Users/user/personal/cariance`
- [ ] 2.2 Create .cruxdev/ with convergence config
- [ ] 2.3 Add .mcp.json pointing to cruxdev binary
- [ ] 2.4 Configure project classification (composite: book-series + business + course + website)

## Phase 3: Book Series Convergence Setup

- [ ] 3.1 Create MANUSCRIPT_TRACKING.md — status of each book (draft, revision, final, published)
- [ ] 3.2 Create CHAPTER_TEMPLATE.md — standard chapter structure per voice guide
- [ ] 3.3 Define book-specific audit dimensions: voice_consistency, structure, pacing, celtic_framing, practice_quality, accessibility
- [ ] 3.4 Each book gets its own directory with: draft.md, outline.md, status
- [ ] 3.5 Convert Book 1 final draft from .txt to structured markdown

## Phase 4: Business Convergence Setup

- [ ] 4.1 Convert vision PDF to VISION.md (markdown, trackable)
- [ ] 4.2 Create BUSINESS_PLAN.md from vision phases
- [ ] 4.3 Define gentle business metrics (reader letters, email list size, energy sustainability — NOT aggressive KPIs)
- [ ] 4.4 Revenue tracking structure (respecting the "no pressure" philosophy)

## Phase 5: Course Setup

- [ ] 5.1 Structure course materials into course/ directory
- [ ] 5.2 Create COURSE_OUTLINE.md from existing framework
- [ ] 5.3 Audio scripts organized by module

## Phase 6: Website / SEO Prep

- [ ] 6.1 Consolidate SEO taxonomy (V1 + V2.1 → single MASTER)
- [ ] 6.2 Landing page template from structure guide
- [ ] 6.3 Hub-and-spoke architecture documented in docs/WEBSITE_PLANNING.md
- [ ] 6.4 NOTE: Website build is Phase 2 of the business plan — don't rush this

## Phase 7: Content Generation

- [ ] 7.1 Blog post: "Adopting a Book Series into CruxDev — What We Learned"
- [ ] 7.2 X post announcing non-software project support
- [ ] 7.3 Publish via BIP pipeline

## Key Principles for This Adoption

1. **Respect the project's philosophy.** Cariance is explicitly NOT about aggressive growth, empire building, or exhausting timelines. The convergence engine must support gentle, sustainable creation.
2. **Voice consistency is the #1 audit dimension.** Cariance has an extremely detailed voice guide. Every piece of content must pass voice audit.
3. **Celtic framing as lens, not decoration.** The voice guide is explicit about this — Celtic elements serve the work, they're not aesthetic choices.
4. **Reader-first always.** "The reader should feel SEEN first, HELPED second, and INVITED third."
5. **No personal names in CruxDev docs.** (Per standing rule — the author's name appears in Cariance docs, which is correct for that project.)

## Verification

Run classifier:
```bash
cruxdev classify /Users/user/personal/cariance
```

Expected: primary_type = BookSeries, secondary_types = [Business, Course, Website, Campaign]
