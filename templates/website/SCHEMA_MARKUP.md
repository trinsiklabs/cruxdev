---
title: Schema Markup / Structured Data Plan
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# Schema Markup Plan

> Structured data implementation plan: which schema types on which pages, for which rich results.

## 1. Schema Types by Page

| Page / Page Type | Schema Type(s) | Rich Result Target | Priority | Status |
|---|---|---|---|---|
| Homepage | Organization, WebSite (with SearchAction if site search exists) | Sitelinks search box, knowledge panel | P0 | [Done/To-do] |
| All pages | BreadcrumbList | Breadcrumb trail in SERP | P0 | [Status] |
| Blog posts | Article or BlogPosting | Article rich result, author info | P0 | [Status] |
| FAQ sections | FAQPage | FAQ rich result (expandable answers) | P1 | [Status] |
| How-to guides | HowTo | How-to rich result (steps) | P1 | [Status] |
| Pricing page | Product + Offer (if applicable) | Pricing info in SERP | P2 | [Status] |
| About page | Organization (detailed) | Knowledge panel data | P1 | [Status] |
| Contact page | ContactPage, LocalBusiness (if applicable) | Contact info in knowledge panel | P2 | [Status] |
| Documentation | TechArticle | — | P2 | [Status] |

---

## 2. Schema Definitions

### 2.1 Organization (Homepage)

```json
{
  "@context": "https://schema.org",
  "@type": "Organization",
  "name": "[Company Name]",
  "url": "https://[example.com]",
  "logo": "https://[example.com]/logo.png",
  "description": "[Company description]",
  "foundingDate": "[YYYY]",
  "founder": {
    "@type": "Person",
    "name": "[Founder Name]"
  },
  "sameAs": [
    "https://twitter.com/[handle]",
    "https://linkedin.com/company/[handle]",
    "https://github.com/[handle]"
  ],
  "contactPoint": {
    "@type": "ContactPoint",
    "contactType": "customer support",
    "email": "[support@example.com]"
  }
}
```

### 2.2 WebSite with SearchAction (Homepage)

```json
{
  "@context": "https://schema.org",
  "@type": "WebSite",
  "name": "[Site Name]",
  "url": "https://[example.com]",
  "potentialAction": {
    "@type": "SearchAction",
    "target": "https://[example.com]/search?q={search_term_string}",
    "query-input": "required name=search_term_string"
  }
}
```

[Only include SearchAction if the site has search functionality.]

### 2.3 BreadcrumbList (All Pages)

```json
{
  "@context": "https://schema.org",
  "@type": "BreadcrumbList",
  "itemListElement": [
    {
      "@type": "ListItem",
      "position": 1,
      "name": "Home",
      "item": "https://[example.com]/"
    },
    {
      "@type": "ListItem",
      "position": 2,
      "name": "[Section Name]",
      "item": "https://[example.com]/[section]/"
    },
    {
      "@type": "ListItem",
      "position": 3,
      "name": "[Page Title]",
      "item": "https://[example.com]/[section]/[page]/"
    }
  ]
}
```

### 2.4 Article / BlogPosting (Blog Posts)

```json
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "[Post Title]",
  "description": "[Post excerpt / meta description]",
  "image": "https://[example.com]/images/[featured-image].webp",
  "author": {
    "@type": "Person",
    "name": "[Author Name]",
    "url": "https://[example.com]/about/[author-slug]/"
  },
  "publisher": {
    "@type": "Organization",
    "name": "[Company Name]",
    "logo": {
      "@type": "ImageObject",
      "url": "https://[example.com]/logo.png"
    }
  },
  "datePublished": "[YYYY-MM-DD]",
  "dateModified": "[YYYY-MM-DD]",
  "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "https://[example.com]/blog/[post-slug]/"
  }
}
```

### 2.5 FAQPage (FAQ Sections)

```json
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "[Question text]",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "[Answer text]"
      }
    }
  ]
}
```

---

## 3. Implementation Method

| Method | Description | Used For |
|---|---|---|
| JSON-LD in `<head>` | Preferred — script tag in page head | All schema types |
| Auto-generated | Template/framework generates from content data | Blog posts, breadcrumbs |
| Manual | Hand-coded for specific pages | Homepage, about page |

**Framework/tool:** [e.g., "Astro schema integration / Next.js structured data / custom component"]

---

## 4. Validation

- [ ] All schema tested in [Google Rich Results Test](https://search.google.com/test/rich-results)
- [ ] No errors in Google Search Console "Enhancements" section
- [ ] Schema matches visible page content (no misleading markup)
- [ ] Schema validates against Schema.org vocabulary

---

## 5. Related Documents

- [SEO Strategy](SEO_STRATEGY.md)
- [Technical SEO Audit](TECHNICAL_SEO_AUDIT.md)
- [On-Page SEO Checklist](ONPAGE_SEO_CHECKLIST.md)
