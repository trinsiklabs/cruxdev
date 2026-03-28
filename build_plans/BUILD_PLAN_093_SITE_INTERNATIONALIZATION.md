# BUILD_PLAN_093: cruxdev.dev Internationalization

**Status:** NOT STARTED
**Priority:** High (global market access)
**Depends on:** I18N_PATTERNS.md (researching)

## Context

CruxDev is a global tool but the website is English-only. Translating the site opens access to developers worldwide. Priority languages based on developer population: Spanish, Portuguese, Japanese, Chinese (Simplified), German, French, Korean, Russian.

## Phase 1: i18n Infrastructure

- [ ] 1.1 Research Astro i18n options (astro-i18n, paraglide, manual content collections)
- [ ] 1.2 Choose URL strategy: subdirectory (/es/, /ja/, /zh/) recommended for SEO
- [ ] 1.3 Set up string extraction for UI elements
- [ ] 1.4 Create translation file structure (JSON per locale)
- [ ] 1.5 Add locale switcher to nav (desktop + mobile hamburger)

## Phase 2: Priority Content Translation

- [ ] 2.1 Homepage — hero, stats, problem/solution, CTA
- [ ] 2.2 Quick start guide
- [ ] 2.3 Engine page
- [ ] 2.4 Methodology page
- [ ] 2.5 Top 3 vs/ pages (Claude Code, Codex, Cursor)

## Phase 3: Translation Pipeline

- [ ] 3.1 AI-assisted first pass (DeepL or Claude for translation)
- [ ] 3.2 Native speaker review (community or paid)
- [ ] 3.3 Continuous localization: new pages auto-translate when published
- [ ] 3.4 Wire into BIP pipeline: blog posts auto-translate to priority languages

## Phase 4: SEO per Locale

- [ ] 4.1 hreflang tags on all pages
- [ ] 4.2 Per-language sitemap
- [ ] 4.3 Google Search Console verification per locale
- [ ] 4.4 Structured data in each language

## Phase 5: Priority Languages (by developer population)

| Language | Code | Developer Population | Priority |
|---|---|---|---|
| Spanish | es | 500M+ speakers, Latin America + Spain | P0 |
| Portuguese | pt-BR | Brazil (large dev community) | P0 |
| Japanese | ja | 3rd largest economy, strong dev culture | P1 |
| Chinese (Simplified) | zh-CN | Largest developer population | P1 |
| German | de | Strong European dev market | P2 |
| French | fr | France + Africa + Canada | P2 |
| Korean | ko | Strong tech industry | P3 |
| Russian | ru | Large dev community | P3 |

## Phase 6: Content Generation

- [ ] 6.1 Blog post: "CruxDev Goes Global — Available in 8 Languages"
- [ ] 6.2 X posts in each language announcing availability
