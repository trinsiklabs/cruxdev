---
title: Legal & Compliance Checklist
last_updated: [YYYY-MM-DD]
project: [Project Name]
jurisdictions: [e.g., "US, EU, California"]
---

# Legal & Compliance Checklist

> Cookie consent, privacy, terms, accessibility, GDPR, CCPA — everything needed for a legally compliant website.

## 1. Applicable Regulations

| Regulation | Applicable? | Reason | Key Requirements |
|---|---|---|---|
| GDPR (EU) | [Yes/No] | [e.g., "EU visitors expected, marketing to EU"] | Cookie consent, privacy rights, DPA with processors |
| CCPA/CPRA (California) | [Yes/No] | [e.g., "US visitors, >$25M revenue or >50K consumers/yr"] | Privacy notice, opt-out rights, "Do Not Sell" |
| ePrivacy Directive (EU) | [Yes/No] | [If GDPR applies] | Cookie consent before non-essential cookies |
| ADA (US) | [Yes/No] | [e.g., "US-based business, public-facing website"] | Web accessibility (WCAG 2.1 AA) |
| EAA (EU) | [Yes/No] | [e.g., "EU customers, applicable from June 2025"] | WCAG 2.1 AA compliance |
| CAN-SPAM (US) | [Yes/No] | [If sending marketing emails] | Unsubscribe, physical address, honest headers |
| CASL (Canada) | [Yes/No] | [If marketing to Canadians] | Express consent for commercial emails |
| [Other] | [Yes/No] | [Reason] | [Requirements] |

---

## 2. Privacy

### 2.1 Privacy Policy

- [ ] Privacy policy exists at `/privacy/` or `/privacy-policy/`
- [ ] Linked in footer on every page
- [ ] Written in plain, understandable language
- [ ] Covers all required elements:
  - [ ] What personal data is collected
  - [ ] How data is collected (forms, cookies, analytics)
  - [ ] Why data is collected (legal basis for each)
  - [ ] How data is used
  - [ ] Who data is shared with (third parties, processors)
  - [ ] How long data is retained
  - [ ] User rights (access, deletion, correction, portability)
  - [ ] How to exercise rights (contact info, process)
  - [ ] How to file a complaint with supervisory authority (GDPR)
  - [ ] Cookie policy (can be separate or integrated)
  - [ ] Contact information for data controller
  - [ ] Data Protection Officer contact (if applicable)
  - [ ] Last updated date
- [ ] Reviewed by legal counsel
- [ ] Updated within last 12 months

### 2.2 Data Processing

| Data Collected | Collection Method | Purpose | Legal Basis (GDPR) | Retention Period |
|---|---|---|---|---|
| [Email address] | [Contact form, newsletter signup] | [Respond to inquiry, send newsletter] | [Consent / Legitimate interest] | [Until unsubscribe / 2 years] |
| [Name] | [Contact form] | [Personalize response] | [Consent] | [2 years] |
| [IP address] | [Server logs, analytics] | [Security, analytics] | [Legitimate interest] | [14 months] |
| [Browsing behavior] | [Analytics cookies] | [Site improvement] | [Consent] | [14 months] |

---

## 3. Cookie Consent

### 3.1 Cookie Inventory

| Cookie | Provider | Type | Purpose | Duration | Consent Required? |
|---|---|---|---|---|---|
| [Session cookie] | [First-party] | Necessary | [Session management] | [Session] | No |
| [_ga] | [Google Analytics] | Analytics | [User identification for analytics] | [2 years] | Yes |
| [_gid] | [Google Analytics] | Analytics | [Session identification] | [24 hours] | Yes |
| [_fbp] | [Facebook Pixel] | Marketing | [Ad targeting] | [3 months] | Yes |
| [Consent cookie] | [Consent manager] | Necessary | [Store consent preference] | [1 year] | No |

### 3.2 Cookie Consent Implementation

- [ ] Cookie consent banner displays on first visit
- [ ] Banner clearly explains what cookies are used and why
- [ ] Users can accept all, reject all, or customize
- [ ] Customization allows granular control by category:
  - [ ] Necessary (always on, cannot be disabled)
  - [ ] Analytics (optional — default off for EU)
  - [ ] Marketing (optional — default off for EU)
  - [ ] Functional (optional)
- [ ] No non-essential cookies set before consent
- [ ] Consent choice is stored and respected on return visits
- [ ] Users can change consent at any time (link in footer: "Cookie Settings")
- [ ] Consent records are logged (who, when, what they consented to)
- [ ] Banner does not block content entirely (must be dismissable)

### 3.3 Cookie Consent Tool

| Property | Value |
|---|---|
| Tool | [e.g., "CookieYes / Cookiebot / Osano / Custom implementation"] |
| Configuration | [Auto-blocking enabled, geo-detection for EU/US, etc.] |
| Google Consent Mode | [v2 configured — enables GA4 consent mode] |

---

## 4. Terms of Service

- [ ] Terms of service exist at `/terms/` or `/terms-of-service/`
- [ ] Linked in footer on every page
- [ ] Covers:
  - [ ] Acceptable use
  - [ ] Intellectual property
  - [ ] Limitation of liability
  - [ ] Governing law and jurisdiction
  - [ ] Dispute resolution
  - [ ] Account terms (if applicable)
  - [ ] Termination clause
  - [ ] Changes to terms (notification process)
- [ ] Reviewed by legal counsel
- [ ] Effective date displayed
- [ ] Updated within last 12 months

---

## 5. Accessibility (WCAG 2.1 AA)

### 5.1 Perceivable

- [ ] All images have meaningful alt text
- [ ] Decorative images have empty alt (alt="")
- [ ] Video has captions/subtitles
- [ ] Audio has transcripts
- [ ] Color is not the only means of conveying information
- [ ] Text color contrast ≥ 4.5:1 (normal text), ≥ 3:1 (large text)
- [ ] UI component contrast ≥ 3:1
- [ ] Content reflows at 320px width without horizontal scrolling
- [ ] Text spacing adjustable without loss of content
- [ ] Content does not require specific orientation (landscape/portrait)

### 5.2 Operable

- [ ] All functionality available via keyboard
- [ ] No keyboard traps
- [ ] Focus order is logical and intuitive
- [ ] Focus indicator visible on all interactive elements
- [ ] Skip navigation link present
- [ ] Page titles are descriptive and unique
- [ ] Multiple ways to navigate (nav menu, search, sitemap)
- [ ] No timing-dependent interactions (or adjustable timing)
- [ ] No content flashes more than 3 times per second
- [ ] Target size ≥ 44x44px for touch targets

### 5.3 Understandable

- [ ] Page language declared (`<html lang="en">`)
- [ ] Content uses clear, simple language
- [ ] Navigation is consistent across pages
- [ ] Form inputs have visible labels
- [ ] Error messages are specific and helpful
- [ ] Error suggestions provided where possible
- [ ] Important actions are reversible or confirmed

### 5.4 Robust

- [ ] Valid HTML (no parsing errors that affect AT)
- [ ] ARIA used correctly (or not at all — no ARIA > bad ARIA)
- [ ] Custom components have appropriate roles, states, and properties
- [ ] Status messages announced to screen readers (aria-live)

### 5.5 Accessibility Statement

- [ ] Accessibility statement published (recommended at `/accessibility/`)
- [ ] States compliance target (WCAG 2.1 AA)
- [ ] Lists known limitations (if any)
- [ ] Provides contact for accessibility issues
- [ ] Updated with last audit date

---

## 6. GDPR Specific (If Applicable)

- [ ] Legal basis documented for each data processing activity
- [ ] Data processing records maintained (Article 30)
- [ ] Data Protection Impact Assessment conducted (if high-risk processing)
- [ ] Data Processing Agreements (DPAs) signed with all processors:
  - [ ] Analytics provider
  - [ ] Email marketing provider
  - [ ] Hosting provider
  - [ ] Form submission handler
  - [ ] [Other processors]
- [ ] Process for handling data subject requests (access, delete, correct, port)
- [ ] Response time for data requests: within 30 days
- [ ] Data breach notification process documented (72 hours to supervisory authority)
- [ ] International data transfers documented and lawful (EU-US Data Privacy Framework, SCCs)

---

## 7. CCPA/CPRA Specific (If Applicable)

- [ ] "Do Not Sell or Share My Personal Information" link in footer (if selling data)
- [ ] Privacy policy includes CCPA-required disclosures:
  - [ ] Categories of personal information collected
  - [ ] Business/commercial purpose for collection
  - [ ] Categories of third parties with whom data is shared
  - [ ] Consumer rights under CCPA
- [ ] Process for handling consumer requests (know, delete, opt-out)
- [ ] Response time: 45 days (extendable to 90)
- [ ] No discrimination against consumers who exercise rights

---

## 8. Email Compliance

- [ ] All marketing emails include unsubscribe link
- [ ] Unsubscribe requests honored within 10 days (CAN-SPAM)
- [ ] Physical mailing address included in marketing emails
- [ ] Subject lines are not deceptive
- [ ] Double opt-in implemented (recommended, required in some jurisdictions)
- [ ] Consent records stored for each subscriber
- [ ] Distinction between transactional and marketing emails

---

## 9. Compliance Monitoring

| Task | Frequency | Owner | Last Completed |
|---|---|---|---|
| Cookie consent audit | Quarterly | [Name] | [Date] |
| Privacy policy review | Annually (or when processing changes) | [Name] | [Date] |
| Accessibility scan | Quarterly | [Name] | [Date] |
| Third-party DPA review | Annually | [Name] | [Date] |
| Regulation change monitoring | Quarterly | [Name] | [Date] |
| Data subject request audit | Quarterly | [Name] | [Date] |

---

## 10. Related Documents

- [Analytics Plan](../strategy/ANALYTICS_PLAN.md) (consent integration)
- [Integrations](../technical/INTEGRATIONS.md) (third-party data processors)
- [Pre-Launch Checklist](../launch/PRELAUNCH_CHECKLIST.md)
- [Annual Audit](../operations/ANNUAL_AUDIT.md)
