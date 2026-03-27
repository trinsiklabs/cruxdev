---
title: Performance Budget
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Performance Budget

> Core Web Vitals targets, page weight limits, and performance enforcement for the website.

## 1. Core Web Vitals Targets

| Metric | Good | Needs Improvement | Poor | Our Target | Measurement Tool |
|---|---|---|---|---|---|
| **LCP** (Largest Contentful Paint) | ≤ 2.5s | 2.5s — 4.0s | > 4.0s | [e.g., **≤ 2.0s**] | PageSpeed Insights, CrUX |
| **INP** (Interaction to Next Paint) | ≤ 200ms | 200ms — 500ms | > 500ms | [e.g., **≤ 150ms**] | PageSpeed Insights, CrUX |
| **CLS** (Cumulative Layout Shift) | ≤ 0.1 | 0.1 — 0.25 | > 0.25 | [e.g., **≤ 0.05**] | PageSpeed Insights, CrUX |

### Target: 100% of pages in "Good" range for all three CWV metrics.

---

## 2. Page Weight Budget

### 2.1 Total Page Weight Limits

| Page Type | Max Total Weight | Max HTML | Max CSS | Max JS | Max Images | Max Fonts |
|---|---|---|---|---|---|---|
| Homepage | [e.g., 800KB] | [50KB] | [30KB] | [100KB] | [500KB] | [120KB] |
| Marketing page | [e.g., 600KB] | [40KB] | [30KB] | [80KB] | [350KB] | [120KB] |
| Blog post | [e.g., 500KB] | [40KB] | [30KB] | [60KB] | [250KB] | [120KB] |
| Landing page | [e.g., 400KB] | [30KB] | [20KB] | [50KB] | [200KB] | [100KB] |
| Documentation page | [e.g., 300KB] | [30KB] | [25KB] | [50KB] | [100KB] | [100KB] |

### 2.2 Request Count Limits

| Page Type | Max Requests | Max Third-Party Requests |
|---|---|---|
| Homepage | [e.g., 30] | [e.g., 5] |
| Marketing page | [e.g., 25] | [e.g., 5] |
| Blog post | [e.g., 20] | [e.g., 5] |
| Landing page | [e.g., 15] | [e.g., 3] |

---

## 3. Loading Performance Targets

| Metric | Target | Notes |
|---|---|---|
| Time to First Byte (TTFB) | [≤ 200ms] | [CDN edge response] |
| First Contentful Paint (FCP) | [≤ 1.0s] | [First visible content] |
| Largest Contentful Paint (LCP) | [≤ 2.0s] | [Main content visible] |
| Time to Interactive (TTI) | [≤ 2.5s] | [Page fully interactive] |
| Speed Index | [≤ 2.0s] | [How quickly content is visually populated] |
| Total Blocking Time (TBT) | [≤ 150ms] | [Main thread blocking from JS] |

---

## 4. Third-Party Script Budget

| Script | Purpose | Size | Load Strategy | Removable? |
|---|---|---|---|---|
| [e.g., Google Analytics / GA4] | Analytics | [~30KB] | [async, after consent] | No |
| [e.g., Google Tag Manager] | Tag management | [~80KB] | [async] | [Evaluate — consider Partytown] |
| [e.g., Hotjar] | Heatmaps | [~50KB] | [async, after consent, sampling] | [Removable after initial analysis] |
| [e.g., Intercom] | Live chat | [~200KB] | [lazy load on click/scroll] | [Evaluate impact] |
| [e.g., YouTube embed] | Video | [~500KB per embed] | [Facade pattern — load on click] | N/A |
| **Total third-party** | — | **[~XXX KB]** | — | — |

### Third-Party Rules

1. Every third-party script must justify its existence against the performance cost
2. No third-party script loads synchronously (all async or deferred)
3. Scripts requiring consent (analytics, marketing) load only after consent granted
4. Heavy embeds (YouTube, social) use facade/placeholder pattern — load on interaction
5. Total third-party JS budget: [e.g., **≤ 200KB**]

---

## 5. Image Performance

| Rule | Specification |
|---|---|
| Format | WebP minimum, AVIF where supported (with fallback) |
| Responsive | All content images use srcset with appropriate breakpoints |
| Lazy loading | All images below the fold use `loading="lazy"` |
| LCP image | Hero/above-fold images use `loading="eager"` and `fetchpriority="high"` |
| Dimensions | All images have explicit width and height attributes (prevents CLS) |
| Maximum single image | [e.g., 200KB after optimization] |
| Build-time optimization | [e.g., "sharp / squoosh / Astro image / Next image optimization"] |

---

## 6. Font Performance

| Rule | Specification |
|---|---|
| Font count | [e.g., Maximum 2 font families] |
| Weights loaded | [e.g., "Regular (400) + Bold (700) only — no other weights"] |
| Format | WOFF2 only (smallest, best browser support) |
| Loading strategy | [e.g., "`font-display: swap`" or "`font-display: optional`"] |
| Preload | [e.g., "Preload primary body font only: `<link rel='preload' as='font'>`"] |
| Subsetting | [e.g., "Latin subset only if applicable"] |
| Total font budget | [e.g., ≤ 120KB total for all font files] |

---

## 7. Monitoring & Enforcement

### 7.1 Automated Checks

| Check | Tool | Frequency | Threshold |
|---|---|---|---|
| Lighthouse CI | [e.g., "GitHub Action on every PR"] | Per PR | [Performance score ≥ 90] |
| Bundle size check | [e.g., "bundlewatch / size-limit"] | Per PR | [Fail if budget exceeded] |
| CrUX data | [Google Search Console / CrUX dashboard] | Monthly | [All CWV in "Good"] |
| PageSpeed Insights | [Manual or API] | Weekly | [Score ≥ 90 mobile, ≥ 95 desktop] |
| WebPageTest | [Manual] | Monthly | [Compare against previous month] |

### 7.2 Performance Regression Protocol

| Trigger | Severity | Action |
|---|---|---|
| LCP > 2.5s on any page | High | [Investigate immediately — block deploy if in PR] |
| CLS > 0.1 on any page | High | [Fix before next deploy] |
| INP > 200ms on any page | Medium | [Fix within 1 sprint] |
| Page weight exceeds budget by <20% | Medium | [Optimize within 1 sprint] |
| Page weight exceeds budget by >20% | High | [Block deploy until resolved] |
| Lighthouse score drops below 80 | High | [Investigate — likely regression] |

---

## 8. Performance Checklist

- [ ] Core Web Vitals targets defined for all page types
- [ ] Page weight budgets set and documented
- [ ] Third-party scripts audited and justified
- [ ] All images optimized (format, compression, responsive, lazy loading)
- [ ] Fonts optimized (subset, WOFF2, display:swap, preload)
- [ ] Above-the-fold content renders without JavaScript
- [ ] No render-blocking CSS or JS in document head (except critical CSS)
- [ ] Lighthouse CI configured in deployment pipeline
- [ ] CrUX monitoring set up (requires real user traffic)
- [ ] Performance monitoring dashboard created

---

## 9. Related Documents

- [Hosting Spec](HOSTING_SPEC.md)
- [Style Guide](../design/STYLE_GUIDE.md) (image and font specifications)
- [Media Assets](../design/MEDIA_ASSETS.md)
- [Pre-Launch Checklist](../launch/PRELAUNCH_CHECKLIST.md)
