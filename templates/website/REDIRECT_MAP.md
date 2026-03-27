---
title: URL Redirect Map
last_updated: [YYYY-MM-DD]
project: [Project Name]
context: [e.g., "Website migration from old.example.com to example.com" or "URL restructure"]
---

# URL Redirect Map

> Maps old URLs to new URLs for migrations, redesigns, and URL structure changes. Every valuable old URL must have a redirect.

## 1. Redirect Strategy

| Property | Value |
|---|---|
| Redirect type | [301 (Permanent) — for all SEO-preserving redirects] |
| Implementation | [e.g., "Server-side via _redirects file / nginx config / Cloudflare rules / hosting platform"] |
| Total old URLs | [Count] |
| URLs with 1:1 redirect | [Count] |
| URLs redirecting to nearest equivalent | [Count] |
| URLs being removed (410 Gone) | [Count] |

---

## 2. Redirect Map

### 2.1 One-to-One Redirects (Exact Match)

| Old URL | New URL | Type | Monthly Traffic | Backlinks | Priority | Verified |
|---|---|---|---|---|---|---|
| [/old-path/] | [/new-path/] | 301 | [Sessions] | [Count] | [P0/P1/P2] | [ ] |
| [/old-blog/post-slug/] | [/blog/post-slug/] | 301 | [Sessions] | [Count] | [Priority] | [ ] |
| [/products/feature-a/] | [/product/features/#feature-a] | 301 | [Sessions] | [Count] | [Priority] | [ ] |
| [/team/] | [/about/team/] | 301 | [Sessions] | [Count] | [Priority] | [ ] |

### 2.2 Pattern Redirects (Wildcard/Regex)

| Pattern (Old) | Pattern (New) | Type | Pages Affected | Notes |
|---|---|---|---|---|
| [/blog/yyyy/mm/dd/:slug/] | [/blog/:slug/] | 301 | [Count] | [Removing date from blog URLs] |
| [/category/:cat/] | [/blog/category/:cat/] | 301 | [Count] | [Categories moved under /blog/] |
| [/docs/v1/*] | [/docs/*] | 301 | [Count] | [Removing version prefix] |

### 2.3 Removed Pages (410 Gone or Redirect to Best Match)

| Old URL | Action | Redirect Target (if not 410) | Reason |
|---|---|---|---|
| [/obsolete-page/] | 410 Gone | — | [Content permanently removed, no equivalent] |
| [/old-feature/] | 301 | [/product/] | [Feature page consolidated] |
| [/event-2024/] | 410 Gone | — | [Past event, no ongoing relevance] |

---

## 3. Pre-Migration Baseline

[Capture these metrics BEFORE migration. Compare AFTER migration.]

### 3.1 Traffic Baseline (Top Pages)

| URL | Monthly Organic Sessions | Top Keywords | Keyword Positions |
|---|---|---|---|
| [/top-page-1/] | [Sessions] | [Keywords] | [Positions] |
| [/top-page-2/] | [Sessions] | [Keywords] | [Positions] |
| [/top-page-3/] | [Sessions] | [Keywords] | [Positions] |

### 3.2 Backlink Baseline

| URL | Referring Domains | Total Backlinks |
|---|---|---|
| [/most-linked-page-1/] | [Count] | [Count] |
| [/most-linked-page-2/] | [Count] | [Count] |

### 3.3 Overall Metrics

| Metric | Pre-Migration Value | Date Captured |
|---|---|---|
| Total indexed pages | [Count] | [Date] |
| Total organic sessions/month | [Count] | [Date] |
| Total referring domains | [Count] | [Date] |
| Domain authority/rating | [Score] | [Date] |

---

## 4. Implementation

### 4.1 Redirect File Format

[Example for common hosting platforms:]

**Netlify (_redirects):**
```
/old-path/  /new-path/  301
/blog/:year/:month/:slug  /blog/:slug  301
```

**Vercel (vercel.json):**
```json
{
  "redirects": [
    { "source": "/old-path/", "destination": "/new-path/", "permanent": true },
    { "source": "/blog/:year/:month/:slug", "destination": "/blog/:slug", "permanent": true }
  ]
}
```

**Cloudflare Pages (_redirects):**
```
/old-path/ /new-path/ 301
```

**Nginx:**
```nginx
location = /old-path/ { return 301 /new-path/; }
location ~ ^/blog/\d{4}/\d{2}/(.*)$ { return 301 /blog/$1; }
```

---

## 5. Verification Checklist

### Pre-Launch
- [ ] All high-traffic URLs have redirects mapped
- [ ] All URLs with backlinks have redirects mapped
- [ ] No redirect chains (A → B → C)
- [ ] No redirect loops (A → B → A)
- [ ] Pattern redirects tested with sample URLs
- [ ] 404 page configured for unmapped URLs

### Post-Launch (Day 1)
- [ ] Spot-check 10 redirects manually (browser + curl)
- [ ] Check Google Search Console for new crawl errors
- [ ] Verify no 404 spikes in analytics
- [ ] Monitor real-time traffic — no unexpected drops

### Post-Launch (Week 1)
- [ ] Full crawl of old URLs — verify all return 301 (not 404)
- [ ] Google Search Console: review "Page indexing" for new issues
- [ ] Compare organic traffic to baseline — within expected range
- [ ] Check for redirect chains introduced by migration

### Post-Launch (Month 1)
- [ ] Compare keyword rankings to pre-migration baseline
- [ ] Compare organic traffic to baseline
- [ ] Review GSC "Page indexing" — all new URLs indexed
- [ ] Check that old URLs are being removed from index (replaced by new)
- [ ] Monitor backlink transfer — referring domains still pointing to site

---

## 6. Related Documents

- [Migration Plan](../migration/MIGRATION_PLAN.md)
- [SEO Strategy](SEO_STRATEGY.md)
- [Technical SEO Audit](TECHNICAL_SEO_AUDIT.md)
- [Content Inventory](../strategy/CONTENT_INVENTORY.md)
