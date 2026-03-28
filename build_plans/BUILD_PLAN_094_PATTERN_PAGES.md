# BUILD_PLAN_094: Pattern Docs as Public Pages — SEO/GEO Content from Existing Research

**Status:** NOT STARTED
**Priority:** High (zero-effort content — docs already exist)

## Context

We have 35+ research-converged pattern docs. Each one is 200-2900 lines of best practices with cited sources. They're currently internal docs in `docs/`. They should be public pages on the site — each one ranks for its topic and gets cited by AI search.

## The Opportunity

Each pattern doc becomes:
1. A public page at `/patterns/<topic>/` with full content
2. Sources section with all research citations
3. Structured data (HowTo or Article schema)
4. SEO-optimized title and meta description
5. GEO-optimized (AI-consumable condensed best practices)

## Pattern Docs to Publish (35+)

### Methodology Patterns
| Doc | Target Keywords |
|---|---|
| FORM_PATTERNS.md | "form design best practices", "web form usability" |
| METRICS_PATTERNS.md | "software metrics best practices", "observability patterns" |
| DASHBOARD_PATTERNS.md | "dashboard design patterns", "data visualization" |
| BLOG_PATTERNS.md | "blog architecture patterns", "static blog best practices" |
| BLOG_PAGINATION_PATTERNS.md | "blog pagination design", "infinite scroll vs pagination" |
| BLOG_TAGGING_PATTERNS.md | "blog tagging system design", "tag vs category" |
| COLOR_CONTRAST_PATTERNS.md | "WCAG contrast requirements", "Tailwind contrast" |
| DRY_UI_COMPONENT_PATTERNS.md | "DRY component design", "UI component architecture" |
| MOBILE_WEB_PATTERNS.md | "mobile web design patterns", "responsive best practices" |
| POST_DEPLOYMENT_PATTERNS.md | "post-deployment verification", "deployment health checks" |
| KV_CACHE_PATTERNS.md | "LLM KV cache optimization", "token cost reduction" |
| GEO_PATTERNS.md | "generative engine optimization", "AI search optimization" |
| I18N_PATTERNS.md | "website internationalization", "i18n best practices" |
| E2E_TEST_PATTERNS.md | "end-to-end testing patterns", "E2E test strategy" |
| RESEARCH_PATTERNS.md | "research methodology", "iterative research" |
| COMPETITORS_PATTERN.md | "competitive analysis methodology", "gap analysis" |

### Stack Patterns (20)
Each one targets "[stack] best practices 2026":
| Doc | Target |
|---|---|
| DEVELOPMENT_PATTERNS_DJANGO.md | "Django best practices 2026" |
| DEVELOPMENT_PATTERNS_NEXTJS.md | "Next.js best practices 2026" |
| DEVELOPMENT_PATTERNS_RAILS.md | "Rails 8 best practices" |
| ... (18 more) | ... |

### Vertical Patterns (future)
Each one targets "AI for [vertical] best practices"

## Phase 1: Pattern Page Template

- [ ] 1.1 Create Astro layout for pattern pages (table of contents, source citations, last updated)
- [ ] 1.2 Render markdown pattern docs as HTML pages
- [ ] 1.3 Auto-extract sources/references section
- [ ] 1.4 Add structured data (Article with datePublished, author, sourceOrganization)
- [ ] 1.5 Add "Last researched" date and methodology note

## Phase 2: Generate Pages

- [ ] 2.1 Script to convert docs/*.md → src/pages/patterns/*.astro
- [ ] 2.2 Or use Astro content collections to render docs/ directly as pages
- [ ] 2.3 Index page at /patterns/ listing all pattern docs with categories
- [ ] 2.4 Internal linking between related patterns

## Phase 3: SEO/GEO Optimization

- [ ] 3.1 Title tags optimized for target keywords
- [ ] 3.2 Meta descriptions with value proposition
- [ ] 3.3 Schema.org Article/HowTo markup
- [ ] 3.4 Each page includes a "Sources" section with all research citations
- [ ] 3.5 llms.txt updated with pattern page URLs

## Why This Is High Priority

- **Zero research cost** — docs already written and research-converged
- **35+ pages instantly** — just need rendering as HTML
- **High GEO value** — condensed best practices are exactly what AI search cites
- **Long-tail SEO** — each doc targets specific queries
- **Authority building** — 2000-line best practices docs with citations establish expertise
