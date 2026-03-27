---
title: Technical SEO Audit Checklist
last_updated: [YYYY-MM-DD]
project: [Project Name]
audit_date: [YYYY-MM-DD]
auditor: [Name]
tools_used: [e.g., "Screaming Frog, PageSpeed Insights, GSC, Ahrefs"]
---

# Technical SEO Audit Checklist

> Comprehensive technical SEO audit. Run before launch and quarterly thereafter.

## 1. Crawlability

### Robots.txt
- [ ] robots.txt exists at `[example.com/robots.txt]`
- [ ] Allows crawling of all important pages
- [ ] Blocks admin, staging, duplicate, and non-public content
- [ ] References XML sitemap: `Sitemap: https://example.com/sitemap.xml`
- [ ] No accidental `Disallow: /` blocking the entire site
- [ ] Tested in Google Search Console robots.txt tester

**Current robots.txt:**
```
[Paste current robots.txt here]
```

### XML Sitemap
- [ ] Sitemap exists at `[example.com/sitemap.xml]`
- [ ] Includes all indexable pages
- [ ] Excludes noindex pages, redirects, and error pages
- [ ] Contains `<lastmod>` dates that are accurate
- [ ] Validates (no XML errors)
- [ ] Submitted to Google Search Console
- [ ] Submitted to Bing Webmaster Tools
- [ ] Auto-updates when pages are added/removed
- [ ] Under 50,000 URLs per sitemap file (or uses sitemap index)

### Crawl Budget
- [ ] No excessive URL parameters generating duplicate URLs
- [ ] No infinite crawl traps (calendars, filters, session IDs)
- [ ] Server responds within 200ms (TTFB)
- [ ] No soft 404s (pages returning 200 but showing error content)

---

## 2. Indexing

### Index Coverage (Google Search Console)
- [ ] All important pages indexed (Valid count matches expected page count)
- [ ] Review "Excluded" pages — ensure no important pages are excluded
- [ ] No "Crawled — currently not indexed" for important pages
- [ ] No "Discovered — currently not indexed" for important pages
- [ ] "Duplicate without user-selected canonical" issues resolved
- [ ] "Duplicate, Google chose different canonical" issues reviewed

| Status | Count | Notes |
|---|---|---|
| Valid (Indexed) | [Count] | [Expected: ~X pages] |
| Valid with warnings | [Count] | [Review each] |
| Excluded | [Count] | [Review — ensure intentional] |
| Error | [Count] | [Fix all] |

### Canonical Tags
- [ ] Every page has a canonical tag
- [ ] Canonical tags are self-referencing (point to the page's own URL)
- [ ] No pages with canonical pointing to a different page (unless intentional)
- [ ] Canonical URL matches the indexed URL format (www vs non-www, trailing slash)
- [ ] HTTP pages canonical to HTTPS version

### Meta Robots
- [ ] No important pages have `noindex` tag
- [ ] Staging/preview environments have `noindex` (prevent indexing)
- [ ] `nofollow` only used on paid links or untrusted user content
- [ ] X-Robots-Tag header not conflicting with meta robots tag

---

## 3. Site Architecture

### URL Structure
- [ ] Clean, descriptive URLs (no IDs, parameters, or gibberish)
- [ ] Consistent URL format (trailing slash convention, lowercase)
- [ ] Maximum 3 levels deep for important content
- [ ] No URL parameters for content filtering (use path-based if needed)

### Internal Linking
- [ ] No orphan pages (pages with zero internal links pointing to them)
- [ ] Important pages are within 3 clicks of homepage
- [ ] Navigation links use descriptive anchor text
- [ ] Breadcrumb navigation implemented
- [ ] No excessive links per page (reasonable limit: <100)

### Redirect Audit
- [ ] No redirect chains (A → B → C — should be A → C)
- [ ] No redirect loops (A → B → A)
- [ ] All redirects are 301 (permanent), not 302 (temporary), unless temporary is intentional
- [ ] Old URLs from previous site versions redirect correctly
- [ ] HTTP to HTTPS redirect in place
- [ ] www to non-www (or vice versa) redirect in place

| Issue | Count | Examples |
|---|---|---|
| Redirect chains | [Count] | [Example URLs] |
| Redirect loops | [Count] | [Example URLs] |
| Broken redirects | [Count] | [Example URLs] |

---

## 4. Page Speed & Core Web Vitals

### Core Web Vitals (Field Data — CrUX)

| Metric | Mobile | Desktop | Status |
|---|---|---|---|
| LCP | [Value] | [Value] | [Good/Needs Improvement/Poor] |
| INP | [Value] | [Value] | [Good/Needs Improvement/Poor] |
| CLS | [Value] | [Value] | [Good/Needs Improvement/Poor] |

### PageSpeed Insights Scores

| Page | Mobile Score | Desktop Score | LCP | TBT | CLS |
|---|---|---|---|---|---|
| Homepage | [Score] | [Score] | [Time] | [Time] | [Score] |
| [Key page 2] | [Score] | [Score] | [Time] | [Time] | [Score] |
| [Key page 3] | [Score] | [Score] | [Time] | [Time] | [Score] |
| [Blog post] | [Score] | [Score] | [Time] | [Time] | [Score] |

### Speed Issues Found

- [ ] No render-blocking JavaScript in head
- [ ] Critical CSS inlined or preloaded
- [ ] Images optimized (WebP/AVIF, compressed, responsive)
- [ ] Fonts optimized (WOFF2, font-display: swap, preloaded)
- [ ] Third-party scripts loaded async/defer
- [ ] Lazy loading implemented for below-fold content
- [ ] Server response time (TTFB) < 200ms

---

## 5. Mobile

- [ ] All pages pass Google's Mobile-Friendly Test
- [ ] Content is identical on mobile and desktop (same content served)
- [ ] Text is readable without zooming (≥16px base font)
- [ ] Tap targets are at least 44x44px with adequate spacing
- [ ] No horizontal scrolling required
- [ ] Mobile navigation is functional and accessible
- [ ] No interstitials or popups blocking content on mobile
- [ ] Viewport meta tag present: `<meta name="viewport" content="width=device-width, initial-scale=1">`

---

## 6. HTTPS & Security

- [ ] All pages served over HTTPS
- [ ] HTTP redirects to HTTPS (301)
- [ ] No mixed content (HTTP resources loaded on HTTPS pages)
- [ ] SSL certificate valid and not expiring soon
- [ ] HSTS header enabled
- [ ] Security headers present (CSP, X-Frame-Options, etc.)

---

## 7. Structured Data

- [ ] Organization schema on homepage
- [ ] BreadcrumbList schema on all pages with breadcrumbs
- [ ] Article/BlogPosting schema on blog posts
- [ ] FAQ schema on FAQ pages/sections
- [ ] Product schema on product pages (if applicable)
- [ ] All structured data validates in Google Rich Results Test
- [ ] No structured data warnings in Google Search Console

---

## 8. International (If Applicable)

- [ ] Hreflang tags correct for each language/region
- [ ] Each hreflang page has a return tag
- [ ] x-default hreflang specified
- [ ] Content is properly translated (not auto-translated)
- [ ] Language selector accessible to crawlers

---

## 9. Content Issues

- [ ] No duplicate title tags across pages
- [ ] No duplicate meta descriptions across pages
- [ ] No thin content pages (<200 words with no other value)
- [ ] No duplicate content issues (same content on multiple URLs)
- [ ] All pages return 200 status (no soft 404s)
- [ ] 404 page exists and is helpful (search box, links to key pages)
- [ ] All images have alt text

| Issue | Count | Pages Affected |
|---|---|---|
| Missing title tags | [Count] | [URLs] |
| Duplicate title tags | [Count] | [URLs] |
| Missing meta descriptions | [Count] | [URLs] |
| Duplicate meta descriptions | [Count] | [URLs] |
| Missing H1 | [Count] | [URLs] |
| Multiple H1s | [Count] | [URLs] |
| Missing alt text | [Count] | [URLs] |
| Broken links (internal) | [Count] | [URLs] |
| Broken links (external) | [Count] | [URLs] |

---

## 10. Audit Summary & Action Items

### Critical (Fix Immediately)

| # | Issue | Pages Affected | Action |
|---|---|---|---|
| 1 | [Issue] | [Count/URLs] | [Fix description] |

### High Priority (Fix Within 1 Week)

| # | Issue | Pages Affected | Action |
|---|---|---|---|
| 1 | [Issue] | [Count/URLs] | [Fix description] |

### Medium Priority (Fix Within 1 Month)

| # | Issue | Pages Affected | Action |
|---|---|---|---|
| 1 | [Issue] | [Count/URLs] | [Fix description] |

### Low Priority (Backlog)

| # | Issue | Pages Affected | Action |
|---|---|---|---|
| 1 | [Issue] | [Count/URLs] | [Fix description] |

---

## 11. Related Documents

- [SEO Strategy](SEO_STRATEGY.md)
- [Performance Budget](../technical/PERFORMANCE_BUDGET.md)
- [Hosting Spec](../technical/HOSTING_SPEC.md)
- [Schema Markup](SCHEMA_MARKUP.md)
- [Redirect Map](REDIRECT_MAP.md)
