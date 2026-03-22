# Website Upgrade Directive

**For:** Active CruxDev-managed projects that have a website or web application.
**Action:** Read this, then audit and upgrade your site to meet WEBSITE_PLANNING.md standards.

---

## What to Do

### 1. Read the standards

Read these two documents in full before starting:

```
/Users/user/personal/cruxdev/docs/WEBSITE_PLANNING.md
/Users/user/personal/cruxdev/docs/SEO_AND_GEO_REFERENCE.md
```

### 2. Audit your site against the checklist

Run through WEBSITE_PLANNING.md Phase 10 (Pre-Launch QA) against your current site. For each item, mark pass/fail. Focus on these critical areas:

**SEO & AI Visibility (Phase 5):**
- [ ] Every page has unique `<title>` (50-60 chars) and `<meta description>` (150-160 chars)
- [ ] Open Graph tags on all pages (og:title, og:description, og:image)
- [ ] Schema.org structured data (JSON-LD) on every page type
- [ ] `/llms.txt` exists — describes the product for AI systems
- [ ] `/robots.txt` allows AI crawlers (GPTBot, ClaudeBot, Google-Extended)
- [ ] XML sitemap exists and is submitted to Search Console
- [ ] Internal linking: every page linked from at least one other page
- [ ] Heading hierarchy: one H1 per page, logical H2/H3 structure
- [ ] Answer-first content: lead paragraphs answer the query in 40-80 words

**Performance (Phase 8):**
- [ ] LCP < 2.5s (test on mobile with PageSpeed Insights)
- [ ] INP < 200ms
- [ ] CLS < 0.1
- [ ] Images use WebP/AVIF with `srcset`, `width`/`height` attributes, `loading="lazy"`
- [ ] Total JS < 200KB compressed
- [ ] Hero/LCP image preloaded
- [ ] Font loading uses `font-display: swap` or `optional`

**Accessibility (Phase 8.5):**
- [ ] WCAG 2.1 AA compliance
- [ ] All color combinations meet 4.5:1 contrast ratio
- [ ] All images have alt text
- [ ] Keyboard navigation works for every interactive element
- [ ] Skip navigation link present
- [ ] Lighthouse accessibility score 95+

**Security:**
- [ ] HTTPS with valid certificate
- [ ] HSTS header
- [ ] Content Security Policy header
- [ ] No mixed content warnings

**Legal:**
- [ ] Privacy policy published (if collecting any data)
- [ ] Cookie consent (opt-in, not just notification) if using tracking
- [ ] Accessibility statement

**Responsive:**
- [ ] Mobile-first design verified on real devices
- [ ] Touch targets minimum 44x44px
- [ ] No horizontal scroll on mobile
- [ ] All content readable without zooming (min 16px body text)

### 3. Create a build plan for the gaps

```
/plan "Upgrade [project] website to WEBSITE_PLANNING.md standards"
```

Put it in `build_plans/BUILD_PLAN_NNN_WEBSITE_UPGRADE.md`.

### 4. Converge

```
/converge build_plans/BUILD_PLAN_NNN_WEBSITE_UPGRADE.md
```

### 5. Report

When converged, summarize:
- How many items were already compliant
- How many items needed fixing
- What was added (llms.txt, structured data, etc.)
- Current Lighthouse scores (Performance, Accessibility, Best Practices, SEO)
- Current Core Web Vitals

---

## Priority Order

If you can't do everything at once, this is the priority order:

1. **Accessibility** — legal liability, affects all users
2. **Performance** — affects SEO ranking, user experience, conversion
3. **SEO fundamentals** — title tags, meta descriptions, structured data, sitemap
4. **AI visibility** — llms.txt, answer-first content, Schema.org
5. **Security headers** — CSP, HSTS
6. **Legal** — privacy policy, cookie consent
7. **Responsive polish** — touch targets, mobile testing

---

## Quick Wins (< 30 minutes each)

These can be done immediately without a build plan:

1. Add `llms.txt` to your public directory
2. Add `robots.txt` with AI crawler permissions
3. Add Open Graph meta tags to your layout
4. Add `loading="lazy"` to below-fold images
5. Add `width` and `height` attributes to all images (prevents CLS)
6. Add a skip navigation link
7. Set `font-display: swap` on web fonts
