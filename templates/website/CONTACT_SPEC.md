---
title: Contact / Support Page Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
page_url: /contact/
status: draft | content-ready | designed | built | live
---

# Contact / Support Page Specification

> Specification for the contact page: forms, routing, response expectations, and support channels.

## 1. Page Purpose

| Property | Value |
|---|---|
| Primary purpose | [e.g., "Route inquiries to the right team and set response expectations"] |
| Contact types served | [Sales, support, partnerships, press, careers, general] |
| Primary CTA | [e.g., "Submit contact form"] |
| Response time promise | [e.g., "We'll respond within 24 hours"] |

---

## 2. Contact Channels

| Channel | Details | When to Use | Visibility |
|---|---|---|---|
| Contact form | [On-page form — see section 3] | [Default for all inquiries] | [Prominent] |
| Email | [e.g., hello@example.com] | [Alternative to form] | [Visible] |
| Calendar booking | [e.g., Calendly link for sales calls] | [Sales / demos] | [CTA button] |
| Live chat | [e.g., Intercom / Crisp / none] | [Quick questions] | [Widget, if enabled] |
| Social media | [e.g., @company on X/Twitter] | [Public questions] | [Footer links] |
| Phone | [Number, if applicable] | [Urgent / enterprise] | [Optional] |
| Physical address | [Address, if applicable] | [Legal requirement or trust signal] | [Footer or contact page] |

---

## 3. Contact Form Specification

### 3.1 Form Fields

| Field | Type | Required | Options / Validation |
|---|---|---|---|
| Name | Text | Yes | [Min 2 chars] |
| Email | Email | Yes | [Valid email format] |
| Company | Text | No | [Optional] |
| Inquiry type | Dropdown | Yes | [Sales, Support, Partnership, Press, General] |
| Message | Textarea | Yes | [Min 10 chars, max 5000 chars] |
| How did you hear about us? | Dropdown | No | [Google, Social Media, Referral, Event, Other] |

### 3.2 Form Behavior

| Behavior | Specification |
|---|---|
| Spam protection | [e.g., "Honeypot field + rate limiting" or "reCAPTCHA v3"] |
| Submit button | [e.g., "Send Message" — disabled until required fields valid] |
| Success state | [e.g., "Inline confirmation: 'Thanks! We'll respond within 24 hours.'"] |
| Error handling | [e.g., "Inline field errors, generic error message for server failures"] |
| Form backend | [e.g., "Formspree, Netlify Forms, custom API endpoint, email forwarding"] |

### 3.3 Routing Rules

| Inquiry Type | Routes To | Response SLA |
|---|---|---|
| Sales | [sales@example.com or CRM] | [24 hours] |
| Support | [support@example.com or helpdesk] | [24 hours] |
| Partnership | [partnerships@example.com] | [48 hours] |
| Press | [press@example.com] | [48 hours] |
| General | [hello@example.com] | [48 hours] |

---

## 4. Page Content Blocks

### Block 1: Headline

| Element | Content |
|---|---|
| Headline (H1) | [e.g., "Get in Touch"] |
| Subheadline | [e.g., "Have a question? We'd love to hear from you. Fill out the form or reach us directly."] |

### Block 2: Contact Form

[As specified in section 3]

### Block 3: Alternative Contact Methods

| Element | Content |
|---|---|
| Email | [Display email address with mailto: link] |
| Calendar | [e.g., "Book a call" button linking to scheduling tool] |
| Address | [Physical address if applicable] |
| Map | [Embedded map if physical location is relevant] |

### Block 4: FAQ (Optional)

| Question | Answer |
|---|---|
| [e.g., "What's your response time?"] | [e.g., "We respond to all inquiries within 24 business hours."] |
| [e.g., "Do you offer phone support?"] | [e.g., "Phone support is available for Enterprise customers."] |

---

## 5. SEO Specification

| Element | Value |
|---|---|
| Title tag | [e.g., "Contact Us — [Company Name]"] |
| Meta description | [e.g., "Get in touch with [Company]. Sales inquiries, support, partnerships — we respond within 24 hours."] |
| Schema markup | [ContactPage schema, LocalBusiness if applicable] |

---

## 6. Related Documents

- [Integrations](../technical/INTEGRATIONS.md)
- [Compliance Checklist](../compliance/COMPLIANCE_CHECKLIST.md)
