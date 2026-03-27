---
title: Website Migration / Redesign Plan
last_updated: [YYYY-MM-DD]
project: [Project Name]
migration_type: [Platform migration | Domain change | URL restructure | Full redesign | CMS change]
old_site: [https://old.example.com]
new_site: [https://example.com]
---

# Website Migration Plan

> Complete plan for migrating or redesigning a website while preserving SEO value, traffic, and user experience.

## 1. Migration Overview

| Property | Value |
|---|---|
| Migration type | [e.g., "Platform migration from WordPress to Astro + domain change"] |
| Old site | [https://old.example.com] |
| New site | [https://example.com] |
| Target date | [YYYY-MM-DD] |
| Migration lead | [Name] |
| Risk level | [Low / Medium / High] |
| Expected traffic impact | [e.g., "10-20% temporary dip in organic traffic, recovery within 2-3 months"] |

---

## 2. Pre-Migration Checklist

### 2.1 Baseline Capture (CRITICAL — Do This FIRST)

- [ ] **Organic traffic baseline** — screenshot and export monthly organic sessions for last 12 months
- [ ] **Keyword rankings** — export all tracked keywords and current positions
- [ ] **Indexed pages** — capture count from Google Search Console
- [ ] **Backlink profile** — export all backlinks and referring domains
- [ ] **Top pages** — export top 50 pages by organic traffic
- [ ] **Conversion rates** — capture current conversion metrics
- [ ] **Core Web Vitals** — capture current CWV scores
- [ ] **Content inventory** — complete CONTENT_INVENTORY.md for old site
- [ ] **URL list** — crawl and export all URLs (Screaming Frog / sitemap)

Store all baseline data in: [Location — e.g., "Google Drive folder: Website Migration Baselines"]

### 2.2 Planning

- [ ] Redirect map complete (see REDIRECT_MAP.md)
- [ ] Content migration plan complete (section 3)
- [ ] New site built and tested on staging
- [ ] Pre-launch checklist completed on staging (PRELAUNCH_CHECKLIST.md)
- [ ] DNS cutover plan documented (LAUNCH_PLAN.md)
- [ ] Rollback plan documented
- [ ] Team briefed on timeline and responsibilities
- [ ] Stakeholder approval obtained

---

## 3. Content Migration

### 3.1 Content Migration Matrix

| Content Type | Count (Old) | Count (New) | Action | Status |
|---|---|---|---|---|
| [Marketing pages] | [Count] | [Count] | [Migrate with updates / Rewrite / Drop] | [Done/In progress/To do] |
| [Blog posts] | [Count] | [Count] | [Migrate all / Migrate top-performing / Drop low-quality] | [Status] |
| [Documentation] | [Count] | [Count] | [Migrate / Rewrite] | [Status] |
| [Images/media] | [Count] | [Count] | [Re-optimize and migrate / Replace] | [Status] |
| [PDFs/downloads] | [Count] | [Count] | [Migrate to same URLs or redirect] | [Status] |

### 3.2 Content Migration Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Blog posts with <100 sessions/year | [Migrate / Drop + redirect] | [e.g., "Drop — redirect to category page"] |
| Pages with outdated content | [Rewrite / Redirect to updated page] | [Rationale] |
| URL structure change | [e.g., "/blog/2024/01/slug → /blog/slug"] | [Simpler URLs, better for SEO] |
| Image handling | [Re-optimize / Keep as-is] | [e.g., "Re-optimize to WebP during migration"] |

### 3.3 Content QA

- [ ] All migrated content reviewed for formatting issues
- [ ] All images present and displaying correctly
- [ ] All internal links updated to new URL structure
- [ ] No old-site URLs remaining in content
- [ ] Meta titles and descriptions migrated or rewritten
- [ ] Schema markup migrated or updated

---

## 4. Technical Migration

### 4.1 Platform Changes

| Component | Old | New | Migration Notes |
|---|---|---|---|
| CMS/Framework | [e.g., WordPress] | [e.g., Astro] | [Content extracted via API/export] |
| Hosting | [e.g., Shared hosting] | [e.g., Vercel] | [New deployment pipeline] |
| CDN | [e.g., None] | [e.g., Cloudflare] | [Configure during migration] |
| Domain | [e.g., old.example.com] | [e.g., example.com] | [DNS cutover required] |
| Analytics | [e.g., UA] | [e.g., GA4] | [New property, historical data kept in UA] |

### 4.2 Technical Checklist

- [ ] New hosting environment configured and tested
- [ ] SSL certificate provisioned for new domain
- [ ] Redirects implemented (see REDIRECT_MAP.md)
- [ ] robots.txt updated for new site
- [ ] XML sitemap generated for new site
- [ ] Canonical tags set on all new pages
- [ ] 404 page configured
- [ ] Analytics installed on new site
- [ ] Search Console property created for new domain (if domain change)
- [ ] CDN configured
- [ ] Security headers configured
- [ ] Email routing configured (if domain change)

---

## 5. SEO Migration

### 5.1 SEO Preservation Checklist

- [ ] **Every high-traffic URL has a 1:1 redirect** to equivalent new URL
- [ ] **Every URL with backlinks has a redirect** to equivalent new URL
- [ ] No redirect chains (max 1 hop: old → new)
- [ ] New title tags and meta descriptions written (or migrated)
- [ ] Structured data migrated to new pages
- [ ] Internal linking structure reviewed and optimized
- [ ] Canonical tags self-referencing on all new URLs
- [ ] Hreflang tags updated (if multi-language)

### 5.2 Search Console Actions

| Action | When | Notes |
|---|---|---|
| Verify new property in GSC | Pre-migration | [New domain/URL prefix property] |
| Submit new sitemap | Day of launch | [new-site.com/sitemap.xml] |
| Use "Change of Address" tool | Day of launch (if domain change) | [Only for domain changes — tells Google about the move] |
| Monitor index coverage | Daily for first week | [Watch for errors and excluded pages] |
| Monitor keyword rankings | Daily for first month | [Compare to baseline] |

---

## 6. Launch Day (Migration-Specific)

[See LAUNCH_PLAN.md for general launch procedure. Additional migration-specific steps:]

### Pre-Cutover
- [ ] Final redirect test on staging (sample 20 URLs)
- [ ] Old site crawled one final time for baseline
- [ ] New site fully tested on staging

### Cutover
- [ ] DNS updated (or hosting target changed)
- [ ] Old site set to redirect to new site (not both serving content)
- [ ] SSL verified on new domain
- [ ] Redirects tested live (10 sample URLs)

### Post-Cutover (Migration-Specific)
- [ ] Submit new sitemap to GSC
- [ ] Submit Change of Address in GSC (if domain change)
- [ ] Crawl old URLs — verify all redirect correctly (no 404s)
- [ ] Verify old site is no longer serving content (only redirects)
- [ ] Monitor GSC for crawl errors — check hourly on day 1

---

## 7. Post-Migration Monitoring

### Week 1

| Metric | Baseline | Current | Change | Acceptable? |
|---|---|---|---|---|
| Organic sessions (daily) | [Baseline] | [Current] | [+/- %] | [Yes if within -20%] |
| Indexed pages | [Baseline] | [Current] | [Change] | [New URLs appearing] |
| Crawl errors | [0] | [Current] | — | [<5 critical] |
| 404 errors (GSC) | [0] | [Current] | — | [Investigate all] |

### Month 1

| Metric | Baseline | Current | Change | Action |
|---|---|---|---|---|
| Organic sessions/month | [Baseline] | [Current] | [+/- %] | [Action if >20% drop] |
| Keywords in top 10 | [Baseline] | [Current] | [Change] | [Action if >10% drop] |
| Referring domains | [Baseline] | [Current] | [Change] | [Should be stable] |
| Conversion rate | [Baseline] | [Current] | [Change] | [Investigate if down] |
| Core Web Vitals | [Baseline] | [Current] | [Change] | [Should improve] |

### Month 3

| Metric | Baseline | Current | Recovery? |
|---|---|---|---|
| Organic sessions | [Baseline] | [Current] | [Full recovery expected by month 3] |
| Keywords | [Baseline] | [Current] | [Should be at or above baseline] |

### Escalation Triggers

| Trigger | Action |
|---|---|
| Organic traffic down >30% after week 2 | Urgent investigation — check redirects, indexing, errors |
| Major keyword drops (top 5 pages) | Check individual page redirects, canonical tags, content |
| Crawl errors >50 in GSC | Audit redirect map, fix broken redirects |
| New site not being indexed after 2 weeks | Check robots.txt, noindex tags, sitemap submission |

---

## 8. Related Documents

- [Redirect Map](../seo/REDIRECT_MAP.md)
- [Content Inventory](../strategy/CONTENT_INVENTORY.md)
- [Pre-Launch Checklist](../launch/PRELAUNCH_CHECKLIST.md)
- [Launch Plan](../launch/LAUNCH_PLAN.md)
- [Technical SEO Audit](../seo/TECHNICAL_SEO_AUDIT.md)
- [Hosting Spec](../technical/HOSTING_SPEC.md)
