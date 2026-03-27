---
title: Pricing Page Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
page_url: /pricing/
status: draft | content-ready | designed | built | live
---

# Pricing Page Specification

> The pricing page is a high-intent page — visitors here are actively evaluating. Its job is to make the purchase decision easy, not to create confusion.

## 1. Page Mission

| Property | Value |
|---|---|
| Primary job | [Make pricing clear and drive plan selection] |
| Primary CTA | [e.g., "Start Free Trial" per tier] |
| Pricing model | [e.g., "Tiered SaaS (Free/Pro/Enterprise)" or "Custom pricing"] |
| Billing options | [Monthly / Annual / Both with toggle] |
| Annual discount | [e.g., "20% off annual" or "2 months free"] |

---

## 2. Content Blocks

### Block 1: Headline

| Element | Content |
|---|---|
| Headline (H1) | [e.g., "Simple, transparent pricing"] |
| Subheadline | [e.g., "Start free. Upgrade when you're ready. No surprises."] |
| Billing toggle | [Monthly / Annual — show savings on annual] |

### Block 2: Pricing Tiers

| Element | Free / Starter | Pro / Growth | Enterprise |
|---|---|---|---|
| Tier name | [e.g., "Free"] | [e.g., "Pro"] | [e.g., "Enterprise"] |
| Price (monthly) | [$0/month] | [$XX/month] | [Custom] |
| Price (annual) | [$0/month] | [$XX/month (billed annually)] | [Custom] |
| Description | [1 sentence: who this is for] | [1 sentence] | [1 sentence] |
| CTA button | [e.g., "Get Started Free"] | [e.g., "Start Pro Trial"] | [e.g., "Contact Sales"] |
| Highlighted | [No] | [Yes — recommended tier] | [No] |
| Key features | [Bullet list — 5-8 items] | [Bullet list — includes all Free + more] | [Bullet list — includes all Pro + more] |

### Block 3: Feature Comparison Table

| Feature | Free | Pro | Enterprise |
|---|---|---|---|
| [Feature 1] | [Limit or check/cross] | [Limit or check] | [check or "Unlimited"] |
| [Feature 2] | [check/cross] | [check] | [check] |
| [Feature 3] | [cross] | [check] | [check] |
| [Feature 4] | [cross] | [cross] | [check] |
| Support level | [Community] | [Email, 24h response] | [Dedicated, 1h response] |
| SLA | [None] | [99.9%] | [99.99%] |

### Block 4: FAQ

| Question | Answer |
|---|---|
| Can I switch plans? | [Yes — upgrade or downgrade anytime...] |
| What payment methods do you accept? | [Credit card, invoice for Enterprise...] |
| Is there a free trial? | [Yes — 14 days on Pro, no credit card...] |
| What happens when my trial ends? | [Your account downgrades to Free...] |
| Do you offer discounts for startups/nonprofits? | [Yes/No — details...] |
| Can I cancel anytime? | [Yes — no cancellation fees...] |

### Block 5: Social Proof

| Element | Content |
|---|---|
| Testimonial | [Customer quote about value/ROI — someone who upgraded] |
| Trust badges | [Payment security, SOC2, money-back guarantee] |

### Block 6: Enterprise CTA

| Element | Content |
|---|---|
| Heading | [e.g., "Need a custom plan?"] |
| Description | [e.g., "For teams with specific requirements, security needs, or volume pricing."] |
| CTA | [e.g., "Talk to Sales"] |

---

## 3. SEO Specification

| Element | Value |
|---|---|
| Title tag | [e.g., "[Product] Pricing — Free, Pro, and Enterprise Plans"] |
| Meta description | [e.g., "See [Product] pricing. Free tier available. Pro starts at $XX/month. Enterprise custom pricing. Start your free trial today."] |
| Target keyword | [e.g., "[product name] pricing"] |
| Schema markup | [Product schema with offers, or FAQ schema for the FAQ section] |

---

## 4. Analytics Events

| Event | Trigger |
|---|---|
| `pricing_view` | Page load |
| `pricing_toggle` | Monthly/Annual toggle clicked |
| `pricing_cta_click` | Any tier CTA clicked (parameter: tier_name) |
| `pricing_faq_click` | FAQ accordion opened (parameter: question) |
| `pricing_comparison_scroll` | User scrolls to comparison table |

---

## 5. Related Documents

- [Website Strategy](../strategy/WEBSITE_STRATEGY.md)
- [Conversion Funnels](../strategy/CONVERSION_FUNNELS.md)
- [Copy Brief: Pricing](../design/copy/pricing-copy.md)
