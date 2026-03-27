---
title: SEO Strategy
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# SEO Strategy

> Organic search strategy: goals, keyword themes, content approach, link building, and competitive positioning.

## 1. SEO Goals

| Goal | Metric | Current Baseline | 3-Month Target | 6-Month Target | 12-Month Target |
|---|---|---|---|---|---|
| Organic traffic | Monthly sessions from organic | [Current] | [Target] | [Target] | [Target] |
| Keyword rankings | Keywords in top 10 | [Current] | [Target] | [Target] | [Target] |
| Domain authority | DR/DA score | [Current] | [Target] | [Target] | [Target] |
| Organic conversions | Goal completions from organic | [Current] | [Target] | [Target] | [Target] |
| Indexed pages | Pages in Google index | [Current] | [Target] | [Target] | [Target] |

---

## 2. Keyword Strategy

### 2.1 Keyword Themes

| Theme | Intent | Example Keywords | Target Pages | Search Volume (Monthly) | Competition |
|---|---|---|---|---|---|
| [Brand] | Navigational | [e.g., "product name", "company name"] | [Homepage, About] | [Volume] | [Low] |
| [Product category] | Commercial | [e.g., "deployment tool", "CI/CD platform"] | [Features, Homepage] | [Volume] | [High] |
| [Problem/pain point] | Informational | [e.g., "how to automate deployments"] | [Blog posts, Guides] | [Volume] | [Medium] |
| [Comparison] | Commercial | [e.g., "product vs competitor", "product alternative"] | [Comparison pages] | [Volume] | [Medium] |
| [Tutorial/how-to] | Informational | [e.g., "how to set up CI/CD pipeline"] | [Blog, Docs] | [Volume] | [Medium] |

### 2.2 Keyword Prioritization

| Priority | Criteria | Example |
|---|---|---|
| P0 — Must win | Brand terms + high-intent commercial terms | [Brand name, "buy [product type]"] |
| P1 — Should win | Medium-competition terms aligned to core product | [Product category terms] |
| P2 — Opportunity | Long-tail informational terms for content marketing | [How-to queries, comparison queries] |
| P3 — Future | High-competition terms requiring authority to rank | [Broad category terms] |

See [KEYWORD_MAP.md](KEYWORD_MAP.md) for the complete keyword-to-page mapping.

---

## 3. Content SEO Strategy

### 3.1 Content Types for SEO

| Content Type | SEO Purpose | Volume | Cadence |
|---|---|---|---|
| Pillar pages | Rank for broad, competitive terms | [e.g., 3-5] | [Quarterly update] |
| Blog posts | Capture long-tail traffic, support pillar pages | [e.g., 50+] | [2/week] |
| Comparison pages | Capture commercial "vs" and "alternative" queries | [e.g., 5-10] | [Per competitor] |
| Glossary / definitions | Capture informational "what is" queries | [e.g., 20+] | [As needed] |
| Case studies | Rank for "[product] + [industry]" queries | [e.g., 5-10] | [Monthly] |
| Landing pages | Rank for specific product/feature queries | [e.g., 5-10] | [Per feature] |

### 3.2 Internal Linking Strategy

| Principle | Implementation |
|---|---|
| Pillar-cluster model | Pillar pages link to related blog posts; blog posts link back to pillar |
| Topical relevance | Links between pages on the same topic cluster |
| Conversion path | Informational content links to commercial pages (features, pricing) |
| Anchor text | Descriptive, keyword-relevant anchor text (not "click here") |
| New content | Every new page gets at least 2 internal links from existing pages |
| Orphan prevention | No page exists without at least one internal link pointing to it |

### 3.3 Content Update Strategy

| Schedule | Action |
|---|---|
| Monthly | Review top 20 organic pages — update if traffic declining |
| Quarterly | Refresh published dates on updated content |
| Quarterly | Identify content cannibalization — merge or differentiate competing pages |
| Annually | Full content audit — prune, merge, or redirect underperforming content |

---

## 4. Technical SEO Priorities

| Area | Status | Priority | Notes |
|---|---|---|---|
| HTTPS everywhere | [Done/To-do] | P0 | [See HOSTING_SPEC.md] |
| Mobile-friendly | [Done/To-do] | P0 | [Responsive design] |
| Core Web Vitals | [Done/To-do] | P0 | [See PERFORMANCE_BUDGET.md] |
| XML sitemap | [Done/To-do] | P0 | [Auto-generated, submitted to GSC] |
| Robots.txt | [Done/To-do] | P0 | [Blocks admin, staging, duplicates] |
| Canonical tags | [Done/To-do] | P0 | [Self-referencing on all pages] |
| Structured data | [Done/To-do] | P1 | [See SCHEMA_MARKUP.md] |
| Hreflang | [Done/To-do/N/A] | P2 | [Only if multi-language] |
| Page speed | [Done/To-do] | P0 | [See PERFORMANCE_BUDGET.md] |

See [TECHNICAL_SEO_AUDIT.md](TECHNICAL_SEO_AUDIT.md) for the complete audit checklist.

---

## 5. Link Building Strategy

### 5.1 Link Acquisition Channels

| Channel | Strategy | Effort | Expected Links/Month |
|---|---|---|---|
| Content marketing | Create link-worthy content (original research, tools, guides) | High | [e.g., 5-10] |
| Guest posting | Publish on industry blogs with link back | Medium | [e.g., 2-4] |
| Product directories | Submit to Product Hunt, AlternativeTo, G2, etc. | Low | [e.g., 5-10 one-time] |
| Open source | GitHub projects linking to marketing site | Low | [Ongoing organic] |
| Digital PR | Original research, data studies, expert commentary | High | [Variable] |
| Partner cross-links | Mutual linking with integration partners | Low | [e.g., 2-5] |
| Broken link building | Find broken links on relevant sites, offer replacement | Medium | [e.g., 1-3] |

### 5.2 Link Building Rules

- No paid links (Google penalty risk)
- No link exchange schemes
- No low-quality directory spam
- Focus on relevance over quantity
- All guest posts must provide genuine value to the host audience

---

## 6. Competitive SEO Analysis

| Competitor | Domain Rating | Organic Keywords | Organic Traffic | Top-Ranking Content |
|---|---|---|---|---|
| [Competitor A] | [DR] | [Count] | [Monthly] | [Their best content and what it ranks for] |
| [Competitor B] | [DR] | [Count] | [Monthly] | [Best content] |
| [Competitor C] | [DR] | [Count] | [Monthly] | [Best content] |

### Content Gap Analysis

| Keyword / Topic | Competitor A Ranks? | Competitor B Ranks? | We Rank? | Opportunity |
|---|---|---|---|---|
| [Keyword/topic] | [Yes — position X] | [Yes — position X] | [No] | [High — create content] |
| [Keyword/topic] | [Yes] | [No] | [No] | [Medium] |

---

## 7. Measurement & Reporting

| Report | Frequency | Tool | Key Metrics |
|---|---|---|---|
| Keyword rankings | Weekly | [e.g., Ahrefs / SEMrush / free tools] | Position changes for tracked keywords |
| Organic traffic | Monthly | [GA4 + GSC] | Sessions, users, pages/session from organic |
| Indexed pages | Monthly | [GSC] | Index coverage, errors, warnings |
| Backlink profile | Monthly | [Ahrefs / SEMrush] | New links, lost links, DR trend |
| Content performance | Monthly | [GA4] | Per-page organic traffic, conversions, engagement |

---

## 8. Related Documents

- [Keyword Map](KEYWORD_MAP.md)
- [On-Page SEO Checklist](ONPAGE_SEO_CHECKLIST.md)
- [Technical SEO Audit](TECHNICAL_SEO_AUDIT.md)
- [Schema Markup](SCHEMA_MARKUP.md)
- [Redirect Map](REDIRECT_MAP.md)
- [Website Strategy](../strategy/WEBSITE_STRATEGY.md)
- [Blog Spec](../pages/BLOG_SPEC.md)
