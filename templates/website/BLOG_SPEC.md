---
title: Blog / Content Hub Specification
last_updated: [YYYY-MM-DD]
project: [Project Name]
page_url: /blog/
status: draft | content-ready | designed | built | live
---

# Blog / Content Hub Specification

> Specification for the blog index, post template, categories, and content strategy integration.

## 1. Blog Purpose

| Property | Value |
|---|---|
| Primary purpose | [e.g., "SEO traffic acquisition — drive organic visits through keyword-targeted content"] |
| Secondary purpose | [e.g., "Thought leadership, product education, newsletter growth"] |
| Target audience | [e.g., "Technical founders, developers, engineering managers"] |
| Publishing cadence | [e.g., "2 posts/week" or "1 post/week minimum"] |
| Success metric | [e.g., "Organic traffic from blog > 5,000/month within 6 months"] |

---

## 2. Blog Index Page

### Layout

| Element | Specification |
|---|---|
| Headline (H1) | [e.g., "Blog" or "[Brand] Engineering Blog" or "Resources"] |
| Featured post | [Latest or pinned post with large image, title, excerpt] |
| Post grid | [Cards: thumbnail, title, excerpt, date, category, reading time] |
| Pagination | [e.g., "12 posts per page, numbered pagination"] |
| Sidebar (optional) | [Categories, popular posts, newsletter signup, search] |
| Category filter | [Tabs or dropdown: All, Engineering, Product, Company, etc.] |

### Categories / Tags

| Category | Slug | Description | Target Keyword Theme |
|---|---|---|---|
| [e.g., Engineering] | `/blog/category/engineering/` | [Technical how-tos, architecture, best practices] | [deployment, CI/CD, devops] |
| [e.g., Product] | `/blog/category/product/` | [Feature announcements, release notes, tutorials] | [product name + feature keywords] |
| [e.g., Company] | `/blog/category/company/` | [Team updates, milestones, culture] | [brand keywords] |
| [e.g., Industry] | `/blog/category/industry/` | [Trends, analysis, thought leadership] | [industry trend keywords] |

---

## 3. Blog Post Template

### Post Metadata

| Field | Required | Example |
|---|---|---|
| Title | Yes | [e.g., "How to Reduce Deployment Time by 80%"] |
| Slug | Yes | [e.g., "reduce-deployment-time"] |
| Author | Yes | [Name + headshot + bio link] |
| Publish date | Yes | [YYYY-MM-DD] |
| Last updated | Yes | [YYYY-MM-DD — show if different from publish date] |
| Category | Yes | [Primary category] |
| Tags | Optional | [e.g., "deployment, automation, tutorial"] |
| Reading time | Auto-calculated | [e.g., "5 min read"] |
| Featured image | Yes | [1200x630px for social sharing] |
| Meta description | Yes | [Max 160 characters] |
| Target keyword | Yes | [Primary SEO keyword for this post] |

### Post Layout

| Section | Content |
|---|---|
| Hero | [Featured image + Title (H1) + Author + Date + Reading time] |
| Introduction | [Hook paragraph — why this matters to the reader] |
| Body | [Structured with H2/H3 headings, short paragraphs, lists, code blocks, images] |
| Conclusion | [Key takeaway + what to do next] |
| CTA | [Inline CTA related to post topic — demo, trial, guide download] |
| Author bio | [Name, headshot, 2-3 sentence bio, social links] |
| Related posts | [3 related posts from same category or topic] |
| Social sharing | [X/Twitter, LinkedIn, copy link] |
| Comments | [Enabled/Disabled — if enabled, moderated] |

### Post SEO Checklist

- [ ] H1 contains target keyword (naturally)
- [ ] Meta description written and contains keyword
- [ ] URL slug is short and contains keyword
- [ ] Featured image has descriptive alt text
- [ ] At least 2 internal links to other site pages
- [ ] At least 1 internal link FROM another page to this post
- [ ] Headings (H2, H3) use related keywords naturally
- [ ] Content is >1,000 words for SEO posts (quality over quantity)
- [ ] Schema markup: Article or BlogPosting

---

## 4. Content Planning

### Initial Launch Posts

| # | Title | Target Keyword | Category | Author | Status |
|---|---|---|---|---|---|
| 1 | [Title] | [Keyword] | [Category] | [Author] | [Draft/Review/Ready] |
| 2 | [Title] | [Keyword] | [Category] | [Author] | [Status] |
| 3 | [Title] | [Keyword] | [Category] | [Author] | [Status] |
| 4 | [Title] | [Keyword] | [Category] | [Author] | [Status] |
| 5 | [Title] | [Keyword] | [Category] | [Author] | [Status] |

### Content Calendar Template

| Week | Post Title | Keyword | Category | Author | Deadline |
|---|---|---|---|---|---|
| Week 1 | [Title] | [Keyword] | [Category] | [Author] | [Date] |
| Week 2 | [Title] | [Keyword] | [Category] | [Author] | [Date] |

---

## 5. Technical Requirements

| Requirement | Specification |
|---|---|
| RSS feed | [e.g., "/blog/feed.xml" — full content or excerpt] |
| JSON feed | [e.g., "/blog/feed.json" — optional] |
| Search | [Blog-specific search or site-wide] |
| Syntax highlighting | [e.g., "Prism.js or Shiki for code blocks"] |
| Image handling | [e.g., "Responsive images, lazy loading, WebP/AVIF"] |
| Table of contents | [Auto-generated from H2/H3 headings for long posts] |
| Estimated reading time | [Auto-calculated: word count / 200 wpm] |
| Open Graph tags | [Auto-generated from post metadata] |
| Twitter cards | [Summary with large image] |

---

## 6. Related Documents

- [SEO Strategy](../seo/SEO_STRATEGY.md)
- [Keyword Map](../seo/KEYWORD_MAP.md)
- [Content Inventory](../strategy/CONTENT_INVENTORY.md)
- [Analytics Plan](../strategy/ANALYTICS_PLAN.md)
- [Style Guide](../design/STYLE_GUIDE.md)
