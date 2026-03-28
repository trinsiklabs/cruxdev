# BUILD_PLAN_091: Content Strategy Expansion — Tools, Types, Personas, Competitive Pages

**Status:** IN PROGRESS (345 pages deployed — tools, types, personas, patterns, vs/ all exist. Depth needed.)
**Priority:** High (SEO/GEO growth driver)

## Context

Every surface of CruxDev is a content opportunity. The current site has 152 pages but barely scratches the surface. This plan creates a systematic content expansion across 5 dimensions, each generating pages that drive organic search traffic and AI citations.

## Dimension 1: MCP Tool Pages (61 pages)

Every MCP tool gets its own page at `/docs/tools/<tool-name>/`:
- What it does (in depth, not just the one-line description)
- How it works internally (architecture, data flow)
- Usage examples across multiple project types (not just code)
- Parameters with explanations
- Return values with examples
- Related tools (what to call before/after)
- When NOT to use it

### Examples:

| Tool | Page | Non-Software Example |
|---|---|---|
| `start_convergence` | How convergence works end-to-end | Converging a book manuscript |
| `classify_project` | How CruxDev identifies project types | Classifying a coaching business |
| `generate_content` | Auto-generating blog + X posts | Generating a book launch announcement |
| `check_seo_health` | SEO health checking explained | Checking an author's website |
| `prioritize_work` | How the priority engine ranks work | Prioritizing chapters vs marketing |
| `verify_deployment` | Post-deployment verification | Verifying a course platform launch |

## Dimension 2: Project Type Pages (18 pages)

Every project type gets its own page at `/types/<type>/`:
- What this project type is
- The CruxDev lifecycle for this type (adoption → convergence → deliverables)
- Audit dimensions specific to this type
- Templates available
- Example walkthrough
- Success metrics

### Project Types:

| Type | Lifecycle Focus |
|---|---|
| software-new | Greenfield: plan → scaffold → TDD → converge → deploy |
| software-existing | Adoption: assess → remediate → harden → test → converge |
| book | Outline → draft → voice audit → revision → publish |
| book-series | Series arc → per-book convergence → cross-book consistency |
| podcast | Format → record → edit → publish → grow audience |
| newsletter | Template → write → review → send → analyze |
| youtube | Script → produce → optimize → publish → engage |
| course | Curriculum → modules → record → platform → launch |
| business-new | Validate → plan → MVP → launch → iterate |
| business-existing | Audit → optimize → grow → measure |
| campaign | Strategy → create → execute → measure → iterate |
| open-source | Structure → contribute → community → maintain |
| composite | Multi-type: coordinate sub-projects → unified convergence |
| consulting-client | Discovery → proposal → deliver → handoff |
| research | Question → methodology → gather → analyze → publish |
| financial | Model → validate → stress-test → report |
| legal | Draft → review → compliance → finalize |
| governance | Policy → approve → enforce → audit |

## Dimension 3: Persona Pages (10+ pages)

Every target persona gets a page at `/for/<persona>/`:
- Who they are
- Their pain points without CruxDev
- How CruxDev helps them specifically
- Workflow walkthrough with their project type
- Tools they'd use most
- Example session transcript
- Testimonial format (when available)

### Personas:

| Persona | Pain Point | CruxDev Solution |
|---|---|---|
| Author | "How do I know my manuscript is done?" | Voice consistency audit, chapter convergence |
| Entrepreneur | "I'm building 5 things, nothing is finishing" | Priority engine, convergence across project types |
| Podcast Host | "Episode quality is inconsistent" | Content audit dimensions, episode convergence |
| Course Creator | "My curriculum has gaps I can't see" | Curriculum completeness audit, learner journey |
| Newsletter Writer | "I can't maintain quality at scale" | Content dimensions, voice consistency |
| Open Source Maintainer | "PRs pile up, quality degrades" | Automated convergence on PRs, issue triage |
| Agency/Consultant | "Client deliverables vary in quality" | Per-client convergence, handoff verification |
| Software Team Lead | "AI generates code but we can't verify quality" | 39-dimension audit, two clean passes |
| Solo Developer | "I ship and hope for the best" | Full convergence lifecycle, automated testing |
| Technical Writer | "Documentation drifts from reality" | Doc alignment gate, ground truth verification |

## Dimension 4: Competitive Pages per Vertical (10+ pages)

For each project type, a competitive analysis page at `/vs/<vertical>/`:
- AI tools for authors (vs Sudowrite, Jasper, ProWritingAid, Atticus)
- AI tools for podcasters (vs Descript, Riverside, Podcastle, Opus Clip)
- AI tools for course creators (vs Teachable AI, Kajabi, Thinkific)
- AI tools for newsletters (vs Beehiiv, Substack, ConvertKit AI)
- AI tools for entrepreneurs (vs Notion AI, Taskade, Motion)
- AI tools for agencies (vs Loom AI, Scribe, Process Street)
- AI coding tools (vs Cursor, Claude Code, Codex — already done)
- AI for open source (vs GitHub Copilot, CodeRabbit, Qodo)

Each page: feature matrix, honest gaps, when to choose CruxDev vs alternative.

**Critical feedback loop:** Each vertical competitive analysis reveals features we NEED but DON'T HAVE. These become new build plans:
- "AI for authors" competitors have manuscript version tracking → BP for manuscript versioning
- "AI for podcasters" have episode scheduling → BP for content calendar
- "AI for course creators" have learner progress tracking → BP for curriculum analytics
- "AI for newsletters" have A/B subject line testing → BP for content optimization

The competitive analysis ISN'T just marketing — it's product discovery. Every vertical gap is a feature request.

## Dimension 5: SEO/GEO Landing Pages

For each page above, identify 5-10 search queries people ask:
- "How to use AI to write a book"
- "Best AI tool for managing a podcast"
- "How do I know when my code is done"
- "AI quality assurance for startups"
- "Automated code review that actually works"

Each query gets a landing page at `/lp/<slug>/` following the Careiance landing page structure:
1. Name the experience (reader feels seen)
2. Name what's happening (framework enters)
3. Provide substance (practical value)
4. Invite (CTA to try CruxDev)

### Scale Estimate

| Dimension | Pages | Notes |
|---|---|---|
| MCP tool pages | 61 | One per tool |
| Project type pages | 18 | One per type |
| Persona pages | 10 | One per persona |
| Competitive vertical overview | 10 | One per vertical |
| Per-competitor vs pages | 45 | Individual matchups |
| Landing pages (5 per above) | ~500 | SEO/GEO optimized |
| **Total** | **~650** | Phase 1: tools + types + personas (89), Phase 2: vs pages (55) |

## Phase 1: Foundation Pages (tools + types + personas)

- [ ] 1.1 Create page templates for each dimension
- [ ] 1.2 Generate 61 MCP tool pages (can be partially automated from tool descriptions + schemas)
- [ ] 1.3 Generate 18 project type pages (from templates + patterns docs)
- [ ] 1.4 Generate 10 persona pages (research-backed)
- [ ] 1.5 Internal linking between related pages

## Dimension 6: Per-Competitor vs Pages Within Each Vertical

Not just `/vs/ai-for-authors/` — individual vs pages for EVERY competitor in EVERY vertical:

| Vertical | Competitors | vs Pages |
|---|---|---|
| AI for authors | Sudowrite, Jasper, ProWritingAid, Atticus, Scrivener AI, NovelAI | 6 pages |
| AI for podcasters | Descript, Riverside, Podcastle, Opus Clip, Castmagic | 5 pages |
| AI for courses | Teachable AI, Kajabi, Thinkific, Mighty Networks, Podia | 5 pages |
| AI for newsletters | Beehiiv AI, Substack Notes, ConvertKit AI, Mailchimp AI | 4 pages |
| AI for businesses | Notion AI, Taskade, Motion, Monday AI, ClickUp AI | 5 pages |
| AI for agencies | Loom AI, Scribe, Process Street, Trainual AI | 4 pages |
| AI coding (existing) | Claude Code, Codex, Cursor, Superpowers, Devin | 5 (done) |
| AI for open source | GitHub Copilot, CodeRabbit, Qodo, Mutable AI | 4 pages |
| AI for coaching | CoachAccountable, Nudge Coach, Practice | 3 pages |
| AI for content | Jasper, Copy.ai, Writesonic, Surfer SEO AI | 4 pages |

**~45 individual vs pages.** Each one:
1. Forces an honest feature matrix (if we can't fill it → build plan)
2. Captures the "X vs Y" search query (high purchase intent)
3. Gets indexed for GEO (AI search compares tools)
4. Generates backlink opportunities (competitor communities discuss comparisons)
5. Creates accountability — our gaps are public, so we close them faster

**The vs page IS the gap analysis.** Public, indexed, and accountable.

## Phase 2: Competitive Verticals

- [ ] 2.1 Research competitors for each vertical (deep research per RESEARCH_PATTERNS.md)
- [ ] 2.2 Generate vertical overview pages (`/vs/ai-for-authors/`, etc.)
- [ ] 2.3 Generate per-competitor vs pages (`/vs/sudowrite/`, `/vs/descript/`, etc.)
- [ ] 2.4 Feature matrices per competitor — gaps become build plans automatically
- [ ] 2.5 Wire into competitive feedback loop (BP018) — new vs pages trigger gap detection

## Phase 3: SEO/GEO Landing Pages

- [ ] 3.1 Keyword research per vertical
- [ ] 3.2 Generate landing pages following the 4-section structure
- [ ] 3.3 Structured data (FAQPage, HowTo) on each
- [ ] 3.4 Internal linking to tool/type/persona pages

## Phase 4: Content Generation Automation

- [ ] 4.1 Templates in CruxDev for auto-generating tool pages from MCP schema
- [ ] 4.2 Templates for project type pages from patterns docs
- [ ] 4.3 BIP pipeline generates landing pages when new tools/types are added

## Phase 5: Content Generation (Meta)

- [ ] 5.1 Blog post: "600 Pages: How CruxDev's Content Strategy Works"
- [ ] 5.2 X post announcing the expansion

## Verification

- All pages return 200
- Structured data validates
- Internal links resolve
- Sitemap includes all new pages
- llms.txt updated with new capabilities
