---
title: Sitemap & Information Architecture
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Sitemap & Information Architecture

> Complete page hierarchy, navigation structure, and URL scheme for the website.

## 1. Site Hierarchy

```
[example.com]
│
├── / (Homepage)
│
├── /product/
│   ├── /product/features/
│   ├── /product/integrations/
│   └── /product/security/
│
├── /pricing/
│
├── /solutions/
│   ├── /solutions/[use-case-1]/
│   └── /solutions/[use-case-2]/
│
├── /resources/
│   ├── /blog/
│   │   └── /blog/[post-slug]/
│   ├── /docs/
│   │   ├── /docs/getting-started/
│   │   └── /docs/[topic]/
│   ├── /case-studies/
│   │   └── /case-studies/[company-slug]/
│   └── /guides/
│       └── /guides/[guide-slug]/
│
├── /about/
│   ├── /about/team/
│   └── /about/careers/
│
├── /contact/
│
├── /privacy/
├── /terms/
└── /cookies/
```

[Adjust the hierarchy above to match your actual site structure. Delete sections that don't apply. Add sections as needed.]

---

## 2. Page Inventory

| URL Path | Page Title | Purpose | Priority | Status | Spec Document |
|---|---|---|---|---|---|
| `/` | [Homepage] | [Primary entry, value prop, conversion] | P0 | [Planned/Draft/Ready/Live] | [HOMEPAGE_SPEC.md] |
| `/product/` | [Product Overview] | [Feature summary, differentiation] | P0 | [Status] | [PAGE_SPEC: product] |
| `/product/features/` | [Features Detail] | [Deep dive into capabilities] | P0 | [Status] | [PAGE_SPEC: features] |
| `/pricing/` | [Pricing] | [Plans, comparison, conversion] | P0 | [Status] | [PRICING_PAGE_SPEC.md] |
| `/about/` | [About Us] | [Team, mission, credibility] | P1 | [Status] | [ABOUT_PAGE_SPEC.md] |
| `/blog/` | [Blog Index] | [Content hub, SEO traffic driver] | P1 | [Status] | [BLOG_SPEC.md] |
| `/docs/` | [Documentation] | [Developer/user self-service] | P1 | [Status] | [DOCS_SITE_SPEC.md] |
| `/contact/` | [Contact] | [Lead capture, support routing] | P1 | [Status] | [CONTACT_SPEC.md] |
| `/privacy/` | [Privacy Policy] | [Legal compliance] | P1 | [Status] | — |
| `/terms/` | [Terms of Service] | [Legal compliance] | P1 | [Status] | — |

[Add all pages. Every URL on the site must appear in this table.]

---

## 3. Navigation Structure

### 3.1 Primary Navigation (Top Nav)

| Label | Links To | Dropdown Items |
|---|---|---|
| [Product] | `/product/` | [Features, Integrations, Security] |
| [Pricing] | `/pricing/` | — |
| [Resources] | `/resources/` | [Blog, Docs, Case Studies, Guides] |
| [About] | `/about/` | [Team, Careers] |
| [CTA Button: "Get Started"] | `/signup/` or `[app URL]` | — |

### 3.2 Footer Navigation

| Column | Links |
|---|---|
| Product | [Features, Pricing, Integrations, Security, Changelog] |
| Resources | [Blog, Documentation, Case Studies, Guides, API Reference] |
| Company | [About, Team, Careers, Contact, Press] |
| Legal | [Privacy Policy, Terms of Service, Cookie Policy] |

### 3.3 Mobile Navigation

[Describe mobile nav behavior: hamburger menu, bottom nav, etc.]

- Navigation type: [Hamburger menu / slide-out drawer / bottom tabs]
- CTA button placement: [Sticky header / bottom of drawer]
- Search: [Included in mobile nav: yes/no]

---

## 4. URL Scheme

### 4.1 URL Conventions

| Convention | Rule | Example |
|---|---|---|
| Case | All lowercase | `/product/features/` |
| Word separator | Hyphens | `/case-studies/acme-corp/` |
| Trailing slash | [Consistent: always or never] | `/pricing/` or `/pricing` |
| File extensions | None (no .html) | `/about/` not `/about.html` |
| Parameters | Minimal, only for filters/search | `/blog/?category=seo` |
| Depth | Maximum 3 levels | `/resources/guides/getting-started/` |

### 4.2 Subdomain Strategy

| Subdomain | Purpose | Notes |
|---|---|---|
| `www.example.com` | Main marketing site | [Canonical, redirects from apex] |
| `docs.example.com` | Documentation | [If separate from main site] |
| `blog.example.com` | Blog | [Only if CMS requires separate subdomain] |
| `app.example.com` | Application | [Not part of this project — listed for clarity] |
| `status.example.com` | Status page | [Third-party hosted] |

---

## 5. Content Relationships

### 5.1 Internal Linking Strategy

| From Page | Links To | Purpose |
|---|---|---|
| Homepage | [Features, Pricing, Blog highlights] | [Guide visitors to conversion or education] |
| Blog posts | [Product pages, other blog posts, docs] | [SEO link equity, user engagement] |
| Features | [Pricing, case studies, docs] | [Drive conversion after education] |
| Case studies | [Product pages, contact] | [Convert after social proof] |

### 5.2 Breadcrumb Structure

```
Home > [Section] > [Page]
Home > Resources > Blog > [Post Title]
Home > Product > Features
Home > About > Team
```

---

## 6. Search

### 6.1 Site Search

- **Enabled:** [Yes / No / Phase 2]
- **Provider:** [e.g., "Algolia, Pagefind, native CMS search, none"]
- **Scope:** [e.g., "Blog + docs only" or "All pages"]
- **Placement:** [e.g., "Nav bar, search icon in header"]

### 6.2 XML Sitemap

- **Location:** `[example.com/sitemap.xml]`
- **Auto-generated:** [Yes — by CMS / build tool / plugin]
- **Includes:** [All public pages, blog posts, docs pages]
- **Excludes:** [Thank-you pages, admin pages, duplicate pages]
- **Submitted to:** [Google Search Console, Bing Webmaster Tools]

---

## 7. Page Grouping for Analytics

| Page Group | URL Pattern | Purpose |
|---|---|---|
| Homepage | `/` | Top-of-funnel entry |
| Product pages | `/product/*` | Education / evaluation |
| Pricing | `/pricing/` | Conversion decision |
| Blog | `/blog/*` | SEO / content marketing |
| Documentation | `/docs/*` | User enablement |
| Legal | `/privacy/`, `/terms/`, `/cookies/` | Compliance |
| Conversion | `/signup/`, `/demo/`, `/thank-you/` | Conversion tracking |

---

## 8. Related Documents

- [Project Brief](PROJECT_BRIEF.md)
- [Website Strategy](WEBSITE_STRATEGY.md)
- [User Journeys](USER_JOURNEYS.md)
- [Keyword Map](../seo/KEYWORD_MAP.md)
- [Redirect Map](../seo/REDIRECT_MAP.md) (if migration)
