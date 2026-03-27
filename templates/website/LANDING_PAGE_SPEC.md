---
title: "Landing Page Specification: [Campaign/Product/Feature Name]"
last_updated: [YYYY-MM-DD]
project: [Project Name]
page_url: [/path/]
campaign: [Campaign name if applicable]
status: draft | content-ready | designed | built | live
---

# Landing Page Specification: [Campaign/Product/Feature Name]

> Specification for a focused landing page with a single conversion goal.
> Landing pages differ from site pages: they have ONE goal, minimal navigation, and are often campaign-specific.

## 1. Landing Page Overview

| Property | Value |
|---|---|
| Purpose | [e.g., "Drive demo signups from Q1 2026 ad campaign"] |
| Target audience | [e.g., "CTOs at Series A startups clicking Google Ads for 'deployment automation'"] |
| Traffic source | [e.g., "Google Ads, LinkedIn Ads, email campaign, Product Hunt"] |
| Single conversion goal | [e.g., "Demo request form submission"] |
| Secondary action | [e.g., "Watch product video" — optional, must not distract from primary] |
| Campaign dates | [Start — End, or "Evergreen"] |
| A/B testing | [Yes/No — if yes, see section 5] |

### 1.1 Landing Page Rules

- [ ] ONE primary CTA — no competing actions
- [ ] Minimal or NO site navigation (prevents distraction)
- [ ] Message matches the ad/email/link that brought them here
- [ ] Above-the-fold content answers: What is it? Who is it for? What should I do?
- [ ] Every element on the page serves the conversion goal

---

## 2. Content Blocks

### Block 1: Hero

| Element | Content |
|---|---|
| Headline | [Must match or echo the ad headline. e.g., "Deploy 10x Faster — See How"] |
| Subheadline | [Expand with specifics. e.g., "Join 500+ teams using [Product] to cut deployment time from hours to minutes."] |
| Visual | [Product screenshot, short demo GIF, or hero image] |
| CTA | [e.g., "Request Demo" — form inline or button to scroll to form] |
| Trust signal | [e.g., Logo bar: "Used by teams at [Company], [Company], [Company]"] |

### Block 2: Problem → Solution

| Element | Content |
|---|---|
| Problem statement | [e.g., "Your team wastes 10+ hours/week on manual deployments"] |
| Solution | [e.g., "[Product] automates your entire deploy pipeline in one click"] |
| Layout | [e.g., "Left: problem with pain-point icons, Right: solution with benefit icons"] |

### Block 3: Key Benefits (3 maximum)

| Benefit | Headline | Description | Icon/Visual |
|---|---|---|---|
| 1 | [e.g., "10x Faster Deploys"] | [1-2 sentences] | [Icon] |
| 2 | [e.g., "Zero-Downtime Updates"] | [1-2 sentences] | [Icon] |
| 3 | [e.g., "Built-In Rollback"] | [1-2 sentences] | [Icon] |

### Block 4: Social Proof

| Element | Content |
|---|---|
| Testimonial | [Customer quote specific to the landing page's angle] |
| Metrics | [e.g., "Reduced deploy time by 85% — CTO, Acme Corp"] |
| Trust badges | [Logos, review scores, certifications] |

### Block 5: Conversion Form / CTA

| Element | Specification |
|---|---|
| Form headline | [e.g., "Get a personalized demo"] |
| Fields | [List each field: Name, Email, Company, etc.] |
| Required fields | [Mark which are required vs. optional] |
| Submit button text | [e.g., "Request My Demo" — first person, action-oriented] |
| Privacy note | [e.g., "We'll never share your info. See our privacy policy."] |
| Thank-you behavior | [Redirect to /thank-you/ or inline confirmation message] |

### Block 6: FAQ (Optional)

| Question | Answer |
|---|---|
| [e.g., "How long is the demo?"] | [e.g., "20 minutes — we'll show you [Product] with your use case."] |
| [e.g., "Is there a free trial?"] | [e.g., "Yes — 14 days, no credit card required."] |

### Block 7: Footer (Minimal)

| Element | Content |
|---|---|
| Company name | [Name] |
| Legal links | [Privacy Policy, Terms of Service] |
| Contact | [Email or phone for questions] |
| NO full site navigation | [Deliberate — keeps focus on conversion] |

---

## 3. Message Match Matrix

[Ensure the landing page matches the traffic source.]

| Traffic Source | Ad/Email Headline | Landing Page Headline | Match Quality |
|---|---|---|---|
| [Google Ads — keyword: "deploy automation"] | [e.g., "Automate Your Deploys"] | [e.g., "Deploy 10x Faster — See How"] | [Strong / Weak — adjust if weak] |
| [LinkedIn Ad] | [e.g., "Stop Manual Deployments"] | [Same landing page or variant] | [Check] |
| [Email campaign] | [e.g., "Subject: Your deploys are too slow"] | [Same or variant] | [Check] |

---

## 4. SEO Specification

| Element | Value |
|---|---|
| Title tag | [e.g., "Request a Demo — [Product Name]"] |
| Meta description | [e.g., "See how [Product] automates deployments for engineering teams. Request a 20-minute demo."] |
| Robots | [index,follow — or noindex if campaign-only page] |
| Canonical | [Self-referencing, or main page if variant] |

---

## 5. A/B Test Plan (If Applicable)

| Variant | Element Changed | Hypothesis | Duration | Success Metric |
|---|---|---|---|---|
| Control (A) | [Current version] | — | [2 weeks minimum] | [Conversion rate] |
| Variant B | [e.g., "Different headline"] | [e.g., "Specific number converts better than vague benefit"] | [Same] | [Same] |
| Variant C | [e.g., "Shorter form (email only)"] | [e.g., "Fewer fields reduce abandonment"] | [Same] | [Same] |

---

## 6. Performance Requirements

| Metric | Target |
|---|---|
| LCP | < 2.0s (landing pages must be FAST) |
| Total page weight | < 500KB |
| Time to interactive | < 3s |
| Form submit response | < 1s |

---

## 7. Related Documents

- [Website Strategy](../strategy/WEBSITE_STRATEGY.md)
- [Conversion Funnels](../strategy/CONVERSION_FUNNELS.md)
- [Analytics Plan](../strategy/ANALYTICS_PLAN.md)
- [Copy Brief](../design/copy/[campaign]-landing-copy.md)
