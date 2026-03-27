# Campaign Tracking & Analytics: [Campaign Name]

> **Campaign:** [Link to CAMPAIGN_BRIEF.md]
> **Analytics Owner:** [Name]
> **Tracking Verified Date:** [Date — must be before launch]

---

## 1. UTM Convention

### Campaign UTM Parameters

All campaign URLs MUST use these UTM parameters consistently:

| Parameter | Value | Notes |
|---|---|---|
| `utm_campaign` | `[campaign-slug]` | Same for ALL links in this campaign |
| `utm_source` | See per-channel table | The platform/origin |
| `utm_medium` | See per-channel table | The marketing medium |
| `utm_content` | Descriptive slug | Differentiates creatives/placements |
| `utm_term` | Keyword (paid search only) | Auto-populated by Google Ads |

### Per-Channel UTM Values

| Channel | utm_source | utm_medium | utm_content examples |
|---|---|---|---|
| Email - launch | `email` | `email` | `launch-email-1`, `launch-email-cta-btn` |
| Email - nurture | `email` | `email` | `nurture-email-3`, `nurture-email-ps` |
| LinkedIn organic | `linkedin` | `social` | `launch-post-1`, `case-study-share` |
| LinkedIn ads | `linkedin` | `paid-social` | `ad-v1-headline-a`, `ad-v2-image-b` |
| Twitter organic | `twitter` | `social` | `launch-tweet`, `thread-1` |
| Twitter ads | `twitter` | `paid-social` | `promoted-tweet-v1` |
| Facebook/Meta ads | `facebook` | `paid-social` | `ad-set-1-creative-a` |
| Google Search | `google` | `cpc` | `brand-keyword`, `category-keyword` |
| Google Display | `google` | `display` | `banner-300x250-v1` |
| Blog post | `blog` | `content` | `post-title-slug` |
| Partner - [name] | `partner-[name]` | `referral` | `co-blog`, `newsletter-mention` |
| Product Hunt | `producthunt` | `referral` | `launch-day`, `comment` |
| Direct / QR | `qr` | `offline` | `event-card`, `flyer-v1` |

### UTM URL Builder

Base URL: `https://[domain]/[landing-page]`

Template:
```
https://[domain]/[path]?utm_campaign=[campaign-slug]&utm_source=[source]&utm_medium=[medium]&utm_content=[content]
```

Example:
```
https://keyvibe.com/trial?utm_campaign=keyvibe-launch-2026&utm_source=linkedin&utm_medium=paid-social&utm_content=ad-v1-headline-a
```

---

## 2. Tracking Pixel & Tag Setup

### Tags to Install

| Tag/Pixel | Platform | Where to Install | Status | Verified |
|---|---|---|---|---|
| Google Analytics 4 (GA4) | Google | All campaign pages | [ ] Installed | [ ] Verified |
| Google Ads conversion | Google | Thank-you / confirmation page | [ ] Installed | [ ] Verified |
| LinkedIn Insight Tag | LinkedIn | All campaign pages | [ ] Installed | [ ] Verified |
| Meta Pixel | Facebook/Instagram | All campaign pages | [ ] Installed | [ ] Verified |
| Twitter Pixel | Twitter/X | All campaign pages | [ ] Installed | [ ] Verified |
| [Retargeting platform] | [Platform] | All campaign pages | [ ] Installed | [ ] Verified |
| [Email tracking] | [Platform] | Landing page | [ ] Installed | [ ] Verified |

### Conversion Events to Track

| Event Name | Trigger | Platform(s) | Value |
|---|---|---|---|
| `page_view` | Landing page load | GA4, all pixels | - |
| `cta_click` | CTA button click | GA4 | - |
| `form_submit` | Form submission | GA4, all ad pixels | $[estimated value] |
| `trial_start` | Trial activation | GA4, all ad pixels | $[estimated value] |
| `purchase` | Payment completed | GA4, all ad pixels | [Actual value] |

### Tag Manager Configuration

- **Tag Manager:** [Google Tag Manager / other]
- **Container ID:** [GTM-XXXXXX]
- **Workspace:** [Campaign-specific workspace name]
- **Published:** [ ] Yes / [ ] No

---

## 3. Attribution Model

### Model Selection

| Model | Description | When to Use |
|---|---|---|
| **Last Click** | 100% credit to last touchpoint | Simple, conservative baseline |
| **First Click** | 100% credit to first touchpoint | Valuing awareness channels |
| **Linear** | Equal credit to all touchpoints | Balanced view |
| **Time Decay** | More credit to recent touchpoints | Longer sales cycles |
| **Data-Driven** | ML-based credit distribution | Sufficient data volume |

**Selected model for this campaign:** [Model name]
**Rationale:** [Why this model fits this campaign]

### Attribution Window

| Conversion Type | Click-Through Window | View-Through Window |
|---|---|---|
| Trial signup | [e.g., 30 days] | [e.g., 7 days] |
| Purchase | [e.g., 60 days] | [e.g., 14 days] |

---

## 4. Dashboard Setup

### Real-Time Dashboard

| Metric | Source | Update Frequency | Alert Threshold |
|---|---|---|---|
| Landing page visitors | GA4 | Real-time | <X/day = alert |
| Trial signups | Product DB | Hourly | <X/day = alert |
| Ad spend | Ad platforms | 4x/day | >$X/day = alert |
| Email sends/opens/clicks | Email platform | Per-send | Open rate <X% = alert |
| Social engagement | Social platforms | Daily | |

### Dashboard Location(s)

| Dashboard | URL | Access | Owner |
|---|---|---|---|
| Campaign overview | [URL] | [Team] | [Name] |
| Paid media details | [URL] | [Team] | [Name] |
| Email performance | [URL] | [Team] | [Name] |

---

## 5. Reporting Cadence

| Report | Frequency | Audience | Owner | Template |
|---|---|---|---|---|
| Daily metrics snapshot | Daily (campaign active) | Campaign team | [Name] | Slack message |
| Weekly performance report | Weekly | Marketing team | [Name] | [PERFORMANCE.md] |
| Mid-campaign review | Once (midpoint) | Leadership | [Name] | Presentation |
| Final campaign report | Once (end) | All stakeholders | [Name] | [POST_MORTEM.md] |

---

## 6. Data Collection Plan

### What Data We Need Post-Campaign

| Data Point | Source | Export Format | Owner | Due Date |
|---|---|---|---|---|
| GA4 campaign report | Google Analytics | CSV/PDF | [Name] | [Date] |
| Ad platform reports | Each platform | CSV | [Name] | [Date] |
| Email performance | Email platform | CSV | [Name] | [Date] |
| CRM lead data | CRM | CSV | [Name] | [Date] |
| Trial/conversion data | Product analytics | CSV | [Name] | [Date] |
| Revenue attribution | Billing system | CSV | [Name] | [Date] |

---

## 7. Privacy & Compliance

### Consent and Tracking

- [ ] Cookie consent banner configured for campaign pages
- [ ] Tracking only fires after consent is granted (where required by law)
- [ ] Privacy policy updated to reflect campaign data collection
- [ ] Email opt-in is double opt-in (where required)
- [ ] Data retention policy applied to campaign data
- [ ] GDPR/CCPA data subject request process documented

### Data Handling

| Data Type | Storage Location | Retention Period | Access Control |
|---|---|---|---|
| Email addresses | [CRM/Email platform] | [X months/years] | [Who has access] |
| Analytics data | [GA4/analytics tool] | [X months] | [Who has access] |
| Ad platform data | [Platform native] | [Platform default] | [Who has access] |

---

## 8. Tracking Verification Checklist

Complete BEFORE campaign launch:

- [ ] All UTM links tested — parameters appear in GA4 real-time view
- [ ] All conversion pixels verified with platform debug tools
- [ ] Form submissions create records in CRM/database
- [ ] Email tracking links resolve correctly
- [ ] Ad platform conversion tracking verified with test conversion
- [ ] Dashboard shows data flowing correctly
- [ ] Attribution model configured in GA4
- [ ] Retargeting audiences populating
- [ ] Cross-domain tracking works (if multiple domains)
- [ ] Bot/spam filtering configured to exclude invalid traffic
- [ ] Mobile tracking verified separately from desktop

---

## Related Documents

- [TESTING.md](TESTING.md) — A/B test measurement
- [PERFORMANCE.md](PERFORMANCE.md) — Where tracking data gets reported
- [LAUNCH_CHECKLIST.md](LAUNCH_CHECKLIST.md) — Tracking verification before go-live
- [CHANNELS/PAID.md](CHANNELS/PAID.md) — Platform-specific tracking requirements
