---
title: Conversion Funnel Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Conversion Funnel Specification

> Defines each conversion path on the website: stages, pages, expected drop-off, and optimization targets.

## 1. Primary Funnel: [e.g., "Demo Signup"]

### 1.1 Funnel Definition

| Property | Value |
|---|---|
| Goal | [e.g., "Visitor completes demo request form"] |
| Entry pages | [e.g., "Homepage, Features, Blog posts"] |
| Conversion page | [e.g., "/demo/"] |
| Thank-you page | [e.g., "/demo/thank-you/"] |
| Target conversion rate | [e.g., "3% of qualified visitors"] |

### 1.2 Funnel Stages

| Stage | Page(s) | Expected % Remaining | Key Metric |
|---|---|---|---|
| 1. Entry | [Homepage, blog post, ad landing page] | 100% | [Sessions] |
| 2. Engagement | [Features, product pages] | [e.g., 60%] | [Scroll depth, time on page] |
| 3. Evaluation | [Pricing, case studies] | [e.g., 35%] | [Pricing page views] |
| 4. Intent | [Demo page / CTA click] | [e.g., 15%] | [CTA clicks] |
| 5. Conversion | [Form submission] | [e.g., 3-5%] | [Form completions] |

### 1.3 CTA Placement

| Page | CTA Text | CTA Type | Position |
|---|---|---|---|
| Homepage hero | [e.g., "Request a Demo"] | Primary button | Above the fold |
| Homepage bottom | [e.g., "See It in Action"] | Primary button | End of page |
| Features page | [e.g., "Try It Free"] | Primary button | After feature sections |
| Pricing page | [e.g., "Start Free Trial"] | Primary button | Each pricing tier |
| Blog posts | [e.g., "Get Started"] | Inline CTA / banner | Mid-article + end |
| Navigation | [e.g., "Get Demo"] | Nav button | Persistent header |

### 1.4 Optimization Levers

| Lever | Current | Target | Test Plan |
|---|---|---|---|
| [Headline copy] | [Current version] | [+20% engagement] | [A/B test 3 variants] |
| [Form fields] | [e.g., 5 fields] | [e.g., 3 fields] | [Remove company + phone] |
| [Social proof placement] | [Below fold] | [Above fold] | [Move logos to hero] |
| [Page load speed] | [Current LCP] | [<2.5s] | [Image optimization] |

---

## 2. Secondary Funnel: [e.g., "Newsletter Signup"]

### 2.1 Funnel Definition

| Property | Value |
|---|---|
| Goal | [e.g., "Visitor subscribes to email newsletter"] |
| Entry pages | [e.g., "Blog posts, resource pages"] |
| Conversion element | [e.g., "Inline form, exit-intent popup, footer form"] |
| Target conversion rate | [e.g., "2% of blog visitors"] |

### 2.2 Funnel Stages

| Stage | Element | Expected % Remaining | Key Metric |
|---|---|---|---|
| 1. Entry | [Blog post via search] | 100% | [Blog sessions] |
| 2. Engagement | [Reads >50% of article] | [e.g., 40%] | [Scroll depth] |
| 3. Exposure | [Sees signup form/popup] | [e.g., 30%] | [Form impressions] |
| 4. Conversion | [Submits email] | [e.g., 2%] | [Subscriptions] |

### 2.3 Form Specification

| Element | Specification |
|---|---|
| Fields | [e.g., "Email only — no name, no company"] |
| Placement | [e.g., "Inline after paragraph 3, bottom of post, exit-intent popup"] |
| Incentive | [e.g., "Free PDF guide" or "Weekly insights, no spam"] |
| Double opt-in | [Yes/No — required for GDPR] |
| Welcome email | [Automated: subject, content, CTA] |

---

## 3. Tertiary Funnel: [e.g., "Contact Sales"]

[Repeat the structure from sections 1 or 2.]

---

## 4. Micro-Conversions

| Micro-Conversion | Why It Matters | Tracking Event | Target |
|---|---|---|---|
| [Watched demo video] | [Indicates high intent] | [video_play, video_complete] | [20% of homepage visitors] |
| [Downloaded resource] | [Lead capture opportunity] | [file_download] | [5% of resource page visitors] |
| [Clicked pricing toggle] | [Active evaluation] | [pricing_toggle_click] | [60% of pricing visitors] |
| [Visited 3+ pages] | [Engaged visitor] | [page_view count >= 3] | [30% of all sessions] |
| [Shared on social] | [Brand amplification] | [social_share_click] | [1% of blog readers] |

---

## 5. Funnel Monitoring

### 5.1 Dashboard Requirements

| Metric | Visualization | Refresh Rate |
|---|---|---|
| Funnel stage drop-off | Funnel chart | Daily |
| Conversion rate trend | Line chart (weekly) | Weekly |
| CTA click-through rate | Bar chart per page | Weekly |
| Form abandonment rate | Single metric + trend | Daily |
| Top entry pages for converters | Table | Weekly |

### 5.2 Alert Thresholds

| Condition | Threshold | Action |
|---|---|---|
| [Conversion rate drops below X%] | [e.g., <1.5%] | [Investigate — check for broken forms, site errors] |
| [Form abandonment exceeds X%] | [e.g., >80%] | [Review form UX, check for errors] |
| [Zero conversions in 24 hours] | [0 for 24h] | [Immediate check — form broken?] |

---

## 6. Related Documents

- [Website Strategy](WEBSITE_STRATEGY.md)
- [User Journeys](USER_JOURNEYS.md)
- [Analytics Plan](ANALYTICS_PLAN.md)
- [Homepage Spec](../pages/HOMEPAGE_SPEC.md)
- [Landing Page Spec](../pages/LANDING_PAGE_SPEC.md)
