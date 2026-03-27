---
title: Analytics & Tracking Plan
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Analytics & Tracking Plan

> Complete specification for website analytics: tools, events, goals, dashboards, and reporting.

## 1. Analytics Stack

| Component | Tool | Account/Property | Notes |
|---|---|---|---|
| Web analytics | [e.g., GA4 / Plausible / Umami / Fathom] | [Property ID / URL] | [Primary analytics tool] |
| Tag management | [e.g., Google Tag Manager / none] | [Container ID] | [If using GTM] |
| Heatmaps | [e.g., Hotjar / Microsoft Clarity / none] | [Site ID] | [Optional — user behavior visualization] |
| A/B testing | [e.g., Google Optimize / Optimizely / none] | [Account ID] | [If running experiments] |
| Search console | [Google Search Console] | [Property URL] | [Search performance data] |
| Uptime monitoring | [e.g., UptimeRobot / Pingdom / none] | [Monitor ID] | [See MAINTENANCE_PLAN.md] |
| Error tracking | [e.g., Sentry / none] | [DSN] | [JavaScript error monitoring] |

### 1.1 Privacy & Compliance

| Setting | Value | Reason |
|---|---|---|
| Cookie consent required | [Yes/No] | [GDPR/CCPA requirement] |
| IP anonymization | [Yes/No] | [Privacy compliance] |
| Data retention period | [e.g., 14 months / 26 months] | [Policy requirement] |
| Cross-domain tracking | [Yes/No — domains if yes] | [e.g., "marketing site + app domain"] |
| Server-side tracking | [Yes/No] | [Ad-blocker resilience] |
| Consent mode | [e.g., "GA4 Consent Mode v2"] | [Granular consent] |

---

## 2. Event Tracking Plan

### 2.1 Automatically Tracked Events

[These are tracked by default in most analytics platforms.]

| Event | Trigger | Parameters |
|---|---|---|
| `page_view` | Every page load | `page_location`, `page_title`, `page_referrer` |
| `scroll` | 90% page scroll | `percent_scrolled` |
| `session_start` | New session begins | `session_id` |
| `first_visit` | First-time visitor | — |

### 2.2 Custom Events

| Event Name | Trigger | Parameters | Priority |
|---|---|---|---|
| `cta_click` | Any CTA button click | `cta_text`, `cta_location`, `cta_destination` | P0 |
| `form_start` | User begins filling a form | `form_name`, `form_location` | P0 |
| `form_submit` | Form successfully submitted | `form_name`, `form_location`, `form_type` | P0 |
| `form_abandon` | User starts but doesn't submit | `form_name`, `last_field_filled`, `time_spent` | P1 |
| `demo_request` | Demo form submitted | `form_source`, `company_size` | P0 |
| `newsletter_signup` | Newsletter form submitted | `signup_location`, `incentive` | P1 |
| `video_play` | Video player started | `video_title`, `video_location` | P1 |
| `video_complete` | Video watched to end | `video_title`, `video_duration` | P1 |
| `file_download` | PDF/resource downloaded | `file_name`, `file_type`, `page_location` | P1 |
| `pricing_view` | Pricing page viewed | `pricing_toggle_state` (monthly/annual) | P0 |
| `pricing_toggle` | Pricing toggle clicked | `from_state`, `to_state` | P1 |
| `external_link_click` | Outbound link clicked | `link_url`, `link_text`, `page_location` | P2 |
| `social_share` | Social share button clicked | `platform`, `content_url` | P2 |
| `search_performed` | Site search executed | `search_term`, `results_count` | P1 |
| `error_page` | 404 or error page shown | `requested_url`, `referrer` | P1 |
| `cookie_consent` | Cookie banner interaction | `consent_type` (accept/reject/customize) | P1 |

### 2.3 Enhanced E-commerce Events (If Applicable)

[Only if the site has transactions, subscriptions, or pricing interactions.]

| Event | Trigger | Parameters |
|---|---|---|
| `view_item` | Product/plan page viewed | `item_name`, `item_category`, `price` |
| `add_to_cart` | Plan selected | `item_name`, `price`, `quantity` |
| `begin_checkout` | Checkout started | `items`, `value`, `currency` |
| `purchase` | Transaction completed | `transaction_id`, `value`, `currency`, `items` |

---

## 3. Goals & Conversions

| Goal Name | Type | Trigger | Value | Priority |
|---|---|---|---|---|
| [Demo Request] | Event | `demo_request` event fired | [e.g., $50 estimated lead value] | P0 |
| [Free Trial Start] | Event | `trial_signup` event fired | [e.g., $20 estimated value] | P0 |
| [Newsletter Signup] | Event | `newsletter_signup` event fired | [e.g., $2 estimated value] | P1 |
| [Contact Form] | Event | `form_submit` where `form_name = contact` | [e.g., $30 estimated value] | P1 |
| [Resource Download] | Event | `file_download` event fired | [e.g., $5 estimated value] | P2 |

---

## 4. Dimensions & Segments

### 4.1 Custom Dimensions

| Dimension | Scope | Values | Purpose |
|---|---|---|---|
| `visitor_type` | User | [new, returning, subscriber, customer] | Segment by relationship stage |
| `content_type` | Event | [landing, blog, docs, product, legal] | Analyze by content category |
| `traffic_campaign` | Session | [Campaign name from UTM] | Campaign attribution |
| `device_category` | Session | [desktop, mobile, tablet] | Device-specific analysis |

### 4.2 Key Segments

| Segment | Definition | Purpose |
|---|---|---|
| Converters | Users who completed any goal | Analyze conversion paths |
| Blog readers | Users with >1 blog page view | Content marketing effectiveness |
| High-intent visitors | Users who viewed pricing | Sales funnel analysis |
| Organic traffic | Source = google/bing/etc. | SEO performance |
| Returning visitors | Session count > 1 | Engagement / loyalty |

---

## 5. UTM Tagging Convention

### 5.1 UTM Parameter Standards

| Parameter | Convention | Examples |
|---|---|---|
| `utm_source` | Platform name, lowercase | `google`, `twitter`, `linkedin`, `newsletter` |
| `utm_medium` | Channel type, lowercase | `cpc`, `social`, `email`, `referral` |
| `utm_campaign` | Campaign name, kebab-case | `q1-2026-launch`, `blog-promo`, `partner-xyz` |
| `utm_content` | Ad/link variant | `hero-cta`, `sidebar-banner`, `email-header` |
| `utm_term` | Paid keyword (paid search only) | `deployment-tool`, `ci-cd-platform` |

### 5.2 UTM Examples

```
https://example.com/?utm_source=twitter&utm_medium=social&utm_campaign=q1-2026-launch&utm_content=thread-link
https://example.com/blog/post/?utm_source=newsletter&utm_medium=email&utm_campaign=weekly-digest-2026-03
https://example.com/?utm_source=google&utm_medium=cpc&utm_campaign=brand-search&utm_term=example+product
```

---

## 6. Dashboard Specification

### 6.1 Executive Dashboard

| Widget | Metric | Visualization | Time Range |
|---|---|---|---|
| Traffic overview | Sessions, users, page views | Line chart | Last 30 days + YoY |
| Conversion rate | Goal completions / sessions | Single metric + trend | Last 30 days |
| Top pages | Page views by URL | Table, top 10 | Last 30 days |
| Traffic sources | Sessions by source/medium | Pie chart | Last 30 days |
| Core Web Vitals | LCP, INP, CLS | Gauge (green/yellow/red) | Last 28 days |

### 6.2 Content Dashboard

| Widget | Metric | Visualization | Time Range |
|---|---|---|---|
| Blog performance | Page views, avg time, bounce rate per post | Table | Last 30 days |
| Content by type | Sessions by content_type dimension | Bar chart | Last 30 days |
| Search terms | Site search queries | Table | Last 30 days |
| Organic keywords | GSC clicks/impressions by query | Table | Last 30 days |

### 6.3 Conversion Dashboard

| Widget | Metric | Visualization | Time Range |
|---|---|---|---|
| Funnel visualization | Stage-by-stage drop-off | Funnel chart | Last 30 days |
| Goal completions | Conversions by goal | Bar chart | Last 30 days |
| Conversion by source | Goal completions by source/medium | Table | Last 30 days |
| Form analytics | Start, submit, abandon rates | Bar chart per form | Last 30 days |

---

## 7. Implementation Checklist

- [ ] Analytics tool account created and property configured
- [ ] Tracking code installed on all pages (verify with real-time view)
- [ ] Tag Manager container set up (if applicable)
- [ ] All custom events implemented and tested
- [ ] Goals/conversions configured
- [ ] Custom dimensions created
- [ ] UTM tagging guide shared with team
- [ ] Dashboards built and shared
- [ ] Google Search Console verified and linked
- [ ] Cross-domain tracking configured (if applicable)
- [ ] Cookie consent integration tested (analytics only fires after consent)
- [ ] IP filters set up (exclude internal traffic)
- [ ] Data retention settings configured
- [ ] Referral exclusion list set (payment processors, auth providers)
- [ ] Site search tracking enabled
- [ ] 404 error tracking working

---

## 8. Reporting Schedule

| Report | Frequency | Audience | Contents |
|---|---|---|---|
| Weekly summary | Monday AM | [Team] | Traffic, conversions, top content, anomalies |
| Monthly deep-dive | 1st of month | [Stakeholders] | Full dashboard review, trends, recommendations |
| Quarterly review | End of quarter | [Leadership] | KPI progress vs. targets, strategic recommendations |
| Annual audit | End of year | [All] | Year-over-year analysis, next year targets |

---

## 9. Related Documents

- [Website Strategy](WEBSITE_STRATEGY.md)
- [Conversion Funnels](CONVERSION_FUNNELS.md)
- [SEO Strategy](../seo/SEO_STRATEGY.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md)
