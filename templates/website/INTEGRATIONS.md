---
title: Third-Party Integrations
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Third-Party Integrations

> All external services integrated with the website: analytics, forms, chat, payments, marketing tools.

## 1. Integration Inventory

| # | Service | Category | Purpose | Load Method | Performance Impact | Required |
|---|---|---|---|---|---|---|
| 1 | [e.g., Google Analytics 4] | Analytics | Web analytics and conversion tracking | [Script tag, async, post-consent] | [~30KB JS] | Yes |
| 2 | [e.g., Google Tag Manager] | Tag management | Central tag management | [Script tag, async] | [~80KB JS] | [Yes/No] |
| 3 | [e.g., Google Search Console] | SEO | Search performance monitoring | [Meta tag verification only] | [None] | Yes |
| 4 | [e.g., Formspree / Netlify Forms] | Forms | Contact and signup form backend | [Form action URL] | [None on load] | Yes |
| 5 | [e.g., Mailchimp / ConvertKit] | Email | Newsletter signups and email automation | [Embedded form or API] | [~20KB if embedded] | [Yes/No] |
| 6 | [e.g., Calendly] | Scheduling | Sales call booking | [Embed or link] | [~200KB if embedded — use link instead] | [Yes/No] |
| 7 | [e.g., Intercom / Crisp] | Chat | Live chat widget | [Script tag, lazy load] | [~200KB JS — heavy] | [Yes/No] |
| 8 | [e.g., Hotjar / Clarity] | UX research | Heatmaps and session recording | [Script tag, post-consent] | [~50KB JS] | [No — temporary] |
| 9 | [e.g., Stripe] | Payments | Payment processing (if applicable) | [Script tag on checkout pages only] | [~30KB on relevant pages] | [If applicable] |
| 10 | [e.g., reCAPTCHA v3] | Security | Spam protection for forms | [Script tag on form pages] | [~150KB — consider alternatives] | [Yes/No] |

---

## 2. Integration Details

### 2.1 [Service Name — e.g., Google Analytics 4]

| Property | Value |
|---|---|
| Account/Property ID | [e.g., G-XXXXXXXXXX] |
| Implementation | [e.g., "gtag.js script, loaded via GTM, fires after cookie consent"] |
| Cookie consent dependency | [Yes — only loads after user grants analytics consent] |
| Configuration | [See ANALYTICS_PLAN.md for full event tracking spec] |
| Data retention | [e.g., 14 months] |
| Access | [Who has access to the account] |

### 2.2 [Service Name — e.g., Form Backend]

| Property | Value |
|---|---|
| Service | [e.g., "Formspree"] |
| Plan | [e.g., "Free / Gold"] |
| Endpoint | [e.g., "https://formspree.io/f/XXXXXXX"] |
| Forms using it | [Contact form, newsletter signup, demo request] |
| Notifications | [Email notifications to: [address]] |
| Spam filtering | [Built-in / reCAPTCHA / honeypot] |
| Data export | [CSV export / API / webhook to CRM] |

### 2.3 [Service Name — e.g., Email Marketing]

| Property | Value |
|---|---|
| Service | [e.g., "ConvertKit / Mailchimp"] |
| Integration method | [e.g., "API call on form submit" or "Embedded signup form"] |
| Lists/Segments | [e.g., "Newsletter subscribers, Demo requesters"] |
| Double opt-in | [Yes — required for GDPR] |
| Welcome sequence | [Automated email sequence on signup — see email marketing plan] |

[Add sections for each integration as needed.]

---

## 3. Integration Dependencies

| If This Breaks | Impact | Fallback |
|---|---|---|
| [Analytics service down] | [No data collection — acceptable short-term] | [Site functions normally] |
| [Form backend down] | [Contact forms fail — HIGH impact] | [Display email address as fallback] |
| [CDN down] | [Site unreachable — CRITICAL] | [DNS failover if configured] |
| [Chat widget down] | [No live chat — low impact] | [Site functions normally] |
| [Email service down] | [Newsletter signups fail — medium impact] | [Store locally, sync later] |

---

## 4. Privacy & Consent Classification

| Service | Consent Category | Loads Without Consent? | Data Collected |
|---|---|---|---|
| [Analytics] | Analytics | No — requires consent | Page views, events, IP (anonymized) |
| [Heatmaps] | Analytics | No — requires consent | Mouse movement, clicks, scrolls |
| [Chat widget] | Functional | [Evaluate — may be functional] | Chat messages, device info |
| [Form backend] | Necessary | Yes — essential for site function | Form submissions |
| [reCAPTCHA] | Necessary | Yes — security function | Interaction data for scoring |
| [Marketing pixels] | Marketing | No — requires consent | Browsing behavior for ad targeting |

---

## 5. Integration Checklist

- [ ] All integrations documented in this file
- [ ] Each integration has a clear owner/account holder
- [ ] Cookie consent categories assigned per integration
- [ ] Performance impact assessed and within budget
- [ ] Fallback behavior defined for critical integrations
- [ ] API keys and secrets stored securely (not in repo)
- [ ] Privacy policy updated to reflect all data-collecting integrations
- [ ] GDPR data processing agreements (DPAs) in place with vendors

---

## 6. Related Documents

- [Analytics Plan](../strategy/ANALYTICS_PLAN.md)
- [Hosting Spec](HOSTING_SPEC.md)
- [Performance Budget](PERFORMANCE_BUDGET.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md)
