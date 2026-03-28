# BUILD_PLAN_109: Full Site Internationalization — Auto-Detect + Language Switcher + 8 Languages

**Status:** NOT STARTED
**Priority:** High (global reach, GEO advantage, competitive differentiation)
**Reference patterns:** I18N_PATTERNS.md, GEO_PATTERNS.md, NAVBAR_PATTERNS.md

## The Goal

Every page on cruxdev.dev available in 8 languages with:
- Auto-detection of visitor's browser language
- Dropdown language switcher in the nav bar
- Subdirectory URL strategy (`/en/`, `/es/`, `/fr/`, etc.)
- hreflang tags on all pages
- Translated sitemap entries
- SEO meta tags per language

## Languages (Phase 1)

| Code | Language | Market |
|------|----------|--------|
| en | English | Default |
| es | Spanish | Latin America, Spain |
| fr | French | France, Africa, Canada |
| de | German | Germany, Austria, Switzerland |
| pt | Portuguese | Brazil, Portugal |
| ja | Japanese | Japan |
| zh | Chinese (Simplified) | China |
| ko | Korean | South Korea |

## Phase 1: Astro i18n Infrastructure

**Files:**
- `astro.config.mjs` — add i18n configuration
- `src/i18n/` — translation utilities and locale files
- `src/i18n/translations/en.json` — English strings (extracted from pages)
- `src/layouts/Base.astro` — language switcher + hreflang tags + lang attribute

### 1a. Astro i18n config
- [ ] Configure `i18n` in astro.config.mjs with subdirectory routing
- [ ] Set defaultLocale: 'en'
- [ ] Set locales: ['en', 'es', 'fr', 'de', 'pt', 'ja', 'zh', 'ko']
- [ ] Set routing: { prefixDefaultLocale: false } (English stays at /)

### 1b. Translation infrastructure
- [ ] Create `src/i18n/utils.ts` — `t()` function, `getLocale()`, `getTranslations()`
- [ ] Create JSON translation files per locale
- [ ] Extract all hardcoded strings from layouts and key pages
- [ ] Create translation key naming convention: `page.section.element`

### 1c. Language switcher
- [ ] Dropdown in nav bar (desktop: next to theme toggle, mobile: in hamburger menu)
- [ ] Shows current language flag/code
- [ ] Clicking switches to same page in different language
- [ ] Persists preference in localStorage
- [ ] Falls back to browser Accept-Language header on first visit

### 1d. Auto-detection
- [ ] Client-side: check `navigator.language` on first visit
- [ ] If matches a supported locale → redirect (or set preference)
- [ ] If no match → default to English
- [ ] Never override explicit user choice (localStorage takes priority)
- [ ] Script in Base.astro `<head>` for instant redirect (no flash)

### 1e. hreflang tags
- [ ] Every page outputs `<link rel="alternate" hreflang="xx" href="...">` for all languages
- [ ] Include `hreflang="x-default"` pointing to English version
- [ ] Generated automatically from locale config

## Phase 2: Content Translation

### 2a. Layout and navigation strings
- [ ] Base.astro: nav items, footer, CTAs
- [ ] Blog index: "Latest posts", "Tags", etc.
- [ ] Pattern index: "Pattern Library", descriptions
- [ ] Common: "Back", "Next", "Read more", "Get started"

### 2b. Key pages (highest traffic)
- [ ] Homepage (index.astro)
- [ ] Engine page
- [ ] Methodology page
- [ ] /for/ persona pages (10 pages)
- [ ] /vs/ comparison pages (9 pages)
- [ ] /patterns/ index and pattern pages (9 pages)

### 2c. Blog posts
- [ ] Blog posts remain English-only initially (Phase 3 for translation)
- [ ] Blog index shows language-aware UI but links to English posts
- [ ] Future: auto-translate blog posts with LLM + human review flag

### 2d. Translation method
- [ ] LLM-generated translations (Claude) with structured review
- [ ] Each translation goes through GTV: verify links still work, code examples unchanged, numbers correct
- [ ] Native speaker review flag: translations marked as "machine-translated" until reviewed

## Phase 3: SEO Per Language

### 3a. Sitemap
- [ ] Generate per-locale sitemaps (sitemap-en.xml, sitemap-es.xml, etc.)
- [ ] Or single sitemap with xhtml:link alternates per URL
- [ ] Submit each to Google Search Console

### 3b. Meta tags per language
- [ ] Translated title and description per page per locale
- [ ] og:locale tag set correctly
- [ ] Structured data with @language property

### 3c. RSS per language
- [ ] Consider per-language RSS feeds (when blog is translated)
- [ ] Default RSS stays English

## Phase 4: Blog Translation (Deferred)

- [ ] Auto-translate blog posts using LLM
- [ ] GTV verification on translated posts (links, code examples, numbers)
- [ ] "Machine translated" badge until human review
- [ ] Per-language blog index pages

## Research Needed

- [ ] Astro i18n best practices (astro-i18next vs manual vs Astro built-in)
- [ ] Auto-detection UX patterns (redirect vs banner vs cookie)
- [ ] Translation management for 266 pages x 8 languages
- [ ] Cost model for LLM translation at scale

## Verification

```bash
cd /Users/user/personal/cruxdev-dev
npm run build  # must build all locale variants
# Check hreflang on output pages
grep -r "hreflang" dist/index.html | head
# Check locale routing
ls dist/es/ dist/fr/ dist/de/
```

## Anti-Patterns

| Anti-Pattern | Correct Approach |
|---|---|
| Auto-redirect without escape | Always show language switcher, never trap users |
| Translate code examples | Keep code in English, translate surrounding text |
| IP-based language detection | Use browser Accept-Language header instead |
| Separate domains per language | Use subdirectories (cheaper, easier to manage, shares domain authority) |
| Machine translate and ship | Machine translate + GTV verify + mark as machine-translated |
