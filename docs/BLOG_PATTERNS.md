# Blog System Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** Smashing Magazine, web.dev, MDN, CSS-Tricks, Ghost, Medium, Dev.to, Hashnode, Astro docs, Hugo docs, Next.js docs, Google Search Central, schema.org, Pagefind docs, Plausible docs
**Last updated:** 2026-03-27

---

## 1. Blog Architectures

### 1.1 Static Site Generation (SSG)

The dominant pattern for developer and content blogs in 2025-2026. Content lives in Git as Markdown/MDX, builds to static HTML at deploy time.

| Framework | Language | Build Speed | Strengths | Best For |
|-----------|----------|-------------|-----------|----------|
| **Astro** | JS/TS | Fast | Islands architecture, zero JS by default, content collections with Zod validation | Content-heavy blogs, marketing sites |
| **Hugo** | Go | Fastest (compiled binary) | Sub-second builds at 10K+ pages, single binary | Large blogs, documentation |
| **Next.js** | JS/TS | Moderate | SSG + SSR + ISR hybrid, React ecosystem | Blogs needing dynamic features |
| **Eleventy** | JS | Fast | Zero-config, template-agnostic | Simple blogs, portfolios |
| **Jekyll** | Ruby | Slow | GitHub Pages native, mature ecosystem | GitHub-hosted blogs |

**When to choose SSG:**
- Content changes infrequently (daily or less)
- No user-generated content on post pages
- Performance and security are priorities
- Content authors are comfortable with Git or a Git-based CMS

**Astro content collections example:**

```typescript
// src/content.config.ts
import { defineCollection, z } from 'astro:content';

const blog = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string().max(70),
    description: z.string().max(160),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    author: z.string(),
    tags: z.array(z.string()).default([]),
    category: z.enum(['engineering', 'product', 'methodology', 'announcement']),
    series: z.object({
      name: z.string(),
      part: z.number(),
    }).optional(),
    heroImage: z.string().optional(),
    draft: z.boolean().default(false),
    readTime: z.number().optional(), // minutes, auto-calculated at build
  }),
});

export const collections = { blog };
```

### 1.2 Headless CMS

Content is managed in a dedicated CMS and consumed via API by the frontend.

| CMS | Type | Pricing | Strengths |
|-----|------|---------|-----------|
| **Sanity** | API-based | Generous free tier | Real-time collaboration, GROQ query language, customizable Studio |
| **Strapi** | Self-hosted | Open source | Full control, REST + GraphQL, custom content types |
| **Contentful** | API-based | Free tier limited | Enterprise-grade, rich ecosystem, CDN-backed |
| **Ghost** | Self-hosted / managed | Open source | Built for publishing, native newsletters, membership |
| **Keystatic** | Git-based | Free | Works with Astro/Next.js, edits commit to Git |

**When to choose headless CMS:**
- Non-technical content editors need a visual interface
- Multiple content contributors with different roles
- Content needs to be published to multiple channels (web, app, email)
- Real-time content updates without redeploy

### 1.3 Database-Backed

Traditional architecture where content lives in a database.

| Platform | Strengths | Weaknesses |
|----------|-----------|------------|
| **WordPress** | 40%+ market share, massive plugin ecosystem | Performance overhead, security surface area |
| **Custom (Node/Python/Rust)** | Full control, optimized for exact needs | Build everything yourself |

**When to choose database-backed:**
- User-generated content (comments, reactions)
- Complex querying (faceted search, personalization)
- High-frequency content updates
- Existing WordPress infrastructure

**Hybrid pattern (2025-2026 trend):** Headless WordPress with a Next.js or Astro frontend. Content in WordPress, presentation decoupled. Gets the editor experience of WordPress with the performance of SSG/SSR.

### 1.4 Architecture Decision Matrix

```
Need non-technical editors?
  YES → Headless CMS or WordPress
    Need real-time updates? → Headless CMS
    Need plugin ecosystem? → WordPress (headless optional)
  NO → Static Site Generator
    Need JS interactivity? → Astro (islands) or Next.js
    Need fastest builds? → Hugo
    Need simplest setup? → Eleventy
```

---

## 2. Content Modeling

### 2.1 Frontmatter Schema

Every blog post needs structured metadata. Use strict schemas with validation.

**Minimal schema (required fields):**

```yaml
---
title: "Autonomous Convergence: How We Eliminated Manual QA Loops"
description: "CruxDev's convergence engine runs audit-fix-re-audit loops without human intervention."
date: "2026-03-15T18:30"  # ISO datetime with timestamp — critical for BIP (multiple posts/day)
tags: ["convergence", "automation", "quality"]
---
```

**Why timestamps, not just dates:** AI-driven Build-in-Public (BIP) pipelines can generate multiple posts per day. Date-only (`2026-03-15`) makes all posts from the same day appear identical in the listing. Always use ISO datetime with at least hour:minute precision (`2026-03-15T18:30`).

**Author field:** Use organization name, not personal names. Depersonalization prevents lock-in to individuals and is better for ecosystem-neutral positioning.

**Timezone handling:** Store dates in ISO 8601 format without timezone offset (`2026-03-28T18:30`). Render with client-side JavaScript using `toLocaleDateString(undefined, ...)` and `toLocaleTimeString(undefined, ...)` — the `undefined` locale parameter tells the browser to use the visitor's local timezone and date format. For SSG sites (Astro, Hugo, Next.js static), render ISO strings into `<time datetime="...">` elements and localize on page load. This avoids hardcoding the server's timezone (common BIP bug: all posts show EST because the CI server is in US-East).

**Full schema (all optional fields):**

```yaml
---
title: "Autonomous Convergence: How We Eliminated Manual QA Loops"
slug: "autonomous-convergence"
description: "CruxDev's convergence engine runs audit-fix-re-audit loops..."
pubDate: 2026-03-15
updatedDate: 2026-03-20
author: "Bryan"
authorImage: "/authors/bryan.webp"
tags: ["convergence", "automation", "quality"]
category: "engineering"
series:
  name: "Building CruxDev"
  part: 3
heroImage: "/blog/convergence-hero.webp"
heroImageAlt: "Diagram showing convergence loop architecture"
ogImage: "/blog/convergence-og.png"
draft: false
featured: false
readTime: 8
canonical: "https://cruxdev.com/blog/autonomous-convergence"
---
```

### 2.2 Categories vs Tags vs Series

| Taxonomy | Hierarchy | Assignment | Purpose | Example |
|----------|-----------|------------|---------|---------|
| **Category** | Hierarchical (parent/child) | Single per post (recommended) | Broad content organization | "Engineering", "Product" |
| **Tag** | Flat | Multiple per post (3-7 ideal) | Specific topic cross-references | "rust", "convergence", "testing" |
| **Series** | Ordered sequence | One series per post | Multi-part content | "Building CruxDev (Part 3 of 5)" |

**Principles:**
- Limit categories to 5-10 total. If you need more, your taxonomy is too granular.
- Tags describe what the post is *about*. Categories describe what *type* of post it is.
- Every post MUST have exactly one category. Tags are optional but recommended.
- Series create a reading order. Include part numbers and total count.

### 2.3 Author Profiles

```typescript
// Author schema (separate collection or data file)
interface Author {
  name: string;
  slug: string;
  avatar: string;         // 200x200 WebP minimum
  bio: string;            // 1-2 sentences
  role: string;           // "Founder", "Engineer"
  social: {
    twitter?: string;
    github?: string;
    linkedin?: string;
    website?: string;
  };
}
```

### 2.4 Related Posts

Algorithm for selecting related posts (in priority order):

1. **Same series** — always show other parts
2. **Tag overlap** — count shared tags, weight by specificity (rare tags score higher)
3. **Same category** — same broad topic area
4. **Recency** — among tied scores, prefer newer posts

```typescript
function getRelatedPosts(current: Post, allPosts: Post[], limit = 3): Post[] {
  const candidates = allPosts
    .filter(p => p.slug !== current.slug && !p.draft)
    .map(post => {
      let score = 0;
      // Same series: highest priority
      if (current.series && post.series?.name === current.series.name) score += 100;
      // Tag overlap: each shared tag adds points
      const sharedTags = current.tags.filter(t => post.tags.includes(t));
      score += sharedTags.length * 10;
      // Same category
      if (post.category === current.category) score += 5;
      // Recency bonus (0-3 points for posts within 90 days)
      const daysDiff = (Date.now() - post.pubDate.getTime()) / 86400000;
      if (daysDiff < 90) score += 3 - Math.floor(daysDiff / 30);
      return { post, score };
    })
    .sort((a, b) => b.score - a.score || b.post.pubDate.getTime() - a.post.pubDate.getTime());

  return candidates.slice(0, limit).map(c => c.post);
}
```

---

## 3. URL Patterns

### 3.1 URL Structures

| Pattern | Example | Pros | Cons |
|---------|---------|------|------|
| `/blog/slug` | `/blog/autonomous-convergence` | Clean, flexible, no date lock-in | No date context in URL |
| `/blog/YYYY/MM/slug` | `/blog/2026/03/autonomous-convergence` | Date context, natural archiving | URL is long, content feels "old" |
| `/blog/category/slug` | `/blog/engineering/autonomous-convergence` | Category context in URL | Must redirect if category changes |
| `/YYYY/MM/slug` | `/2026/03/autonomous-convergence` | No /blog prefix, feels natural for blog-only sites | Conflicts with other routes |

**Recommendation:** `/blog/slug` for most sites. It is the simplest, most flexible, and does not encode information that might change (dates are permanent but make evergreen content feel stale; categories can change).

### 3.2 URL Best Practices

- **Slugs:** lowercase, hyphen-separated, 3-5 words, keyword-rich
- **No trailing slashes** (pick one convention and enforce it)
- **Redirects:** 301 redirect old URLs when slugs change. Never break links.
- **No file extensions:** `/blog/post` not `/blog/post.html`
- **No IDs in URLs:** `/blog/convergence` not `/blog/42`

---

## 4. RSS / Atom Feeds

### 4.1 Feed Generation

Provide at minimum one RSS 2.0 or Atom feed. Atom is recommended for new implementations (less ambiguity in spec).

```xml
<!-- Atom feed example -->
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>CruxDev Blog</title>
  <subtitle>Autonomous convergence for AI-driven development</subtitle>
  <link href="https://cruxdev.com/blog/feed.xml" rel="self"/>
  <link href="https://cruxdev.com/blog"/>
  <id>https://cruxdev.com/blog</id>
  <updated>2026-03-27T00:00:00Z</updated>
  <author>
    <name>CruxDev Team</name>
  </author>
  <entry>
    <title>Autonomous Convergence</title>
    <link href="https://cruxdev.com/blog/autonomous-convergence"/>
    <id>https://cruxdev.com/blog/autonomous-convergence</id>
    <published>2026-03-15T00:00:00Z</published>
    <updated>2026-03-20T00:00:00Z</updated>
    <summary>CruxDev's convergence engine runs audit-fix-re-audit loops...</summary>
    <content type="html"><![CDATA[<p>Full post HTML here...</p>]]></content>
    <category term="engineering"/>
  </entry>
</feed>
```

### 4.2 Autodiscovery

Add a `<link>` tag in the `<head>` of every page:

```html
<link rel="alternate" type="application/atom+xml"
      title="CruxDev Blog"
      href="/blog/feed.xml" />
```

**Rules:**
- Use only ONE autodiscovery link (pick RSS or Atom, not both)
- Use absolute URLs in all feed content
- Include full post content in `<content>`, not just excerpts
- Validate with the W3C Feed Validation Service (validator.w3.org/feed/)
- Generate feeds at build time, not on request

### 4.3 Additional Feeds

Consider offering per-category or per-tag feeds for power users:

```
/blog/feed.xml              — all posts
/blog/category/engineering/feed.xml  — engineering only
/blog/tag/rust/feed.xml     — posts tagged "rust"
```

---

## 5. Open Graph / Twitter Cards / Structured Data

### 5.1 Open Graph Meta Tags

```html
<meta property="og:type" content="article" />
<meta property="og:title" content="Autonomous Convergence" />
<meta property="og:description" content="CruxDev's convergence engine runs..." />
<meta property="og:url" content="https://cruxdev.com/blog/autonomous-convergence" />
<meta property="og:image" content="https://cruxdev.com/blog/convergence-og.png" />
<meta property="og:image:width" content="1200" />
<meta property="og:image:height" content="630" />
<meta property="og:site_name" content="CruxDev" />
<meta property="article:published_time" content="2026-03-15T00:00:00Z" />
<meta property="article:modified_time" content="2026-03-20T00:00:00Z" />
<meta property="article:author" content="Bryan" />
<meta property="article:tag" content="convergence" />
<meta property="article:tag" content="automation" />
```

### 5.2 Twitter (X) Cards

```html
<meta name="twitter:card" content="summary_large_image" />
<meta name="twitter:title" content="Autonomous Convergence" />
<meta name="twitter:description" content="CruxDev's convergence engine runs..." />
<meta name="twitter:image" content="https://cruxdev.com/blog/convergence-og.png" />
<meta name="twitter:site" content="@cruxdev" />
```

**Note:** Twitter falls back to OG tags if twitter-specific tags are absent. Define both for control, but OG alone works as a minimum.

### 5.3 OG Image Requirements

- **Dimensions:** 1200x630px (2:1.05 ratio)
- **Format:** PNG for text-heavy, JPEG/WebP for photographic
- **File size:** Under 1MB (under 300KB ideal)
- **Safe zone:** Keep critical text within center 80% (platforms crop edges)
- **Auto-generation:** Use tools like @vercel/og or Satori to generate OG images from templates at build time

### 5.4 Schema.org BlogPosting (JSON-LD)

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "Autonomous Convergence",
  "description": "CruxDev's convergence engine runs audit-fix-re-audit loops...",
  "image": "https://cruxdev.com/blog/convergence-hero.webp",
  "url": "https://cruxdev.com/blog/autonomous-convergence",
  "datePublished": "2026-03-15",
  "dateModified": "2026-03-20",
  "author": {
    "@type": "Person",
    "name": "Bryan",
    "url": "https://cruxdev.com/about"
  },
  "publisher": {
    "@type": "Organization",
    "name": "Trinsik Labs",
    "logo": {
      "@type": "ImageObject",
      "url": "https://cruxdev.com/logo.png"
    }
  },
  "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "https://cruxdev.com/blog/autonomous-convergence"
  },
  "wordCount": 2400,
  "articleSection": "Engineering",
  "keywords": ["convergence", "automation", "quality"]
}
</script>
```

**Validate with:** Google Rich Results Test (search.google.com/test/rich-results)

---

## 6. Search

### 6.1 Client-Side Search (Best for Static Sites)

| Tool | Size | Approach | Best For |
|------|------|----------|----------|
| **Pagefind** | ~15KB initial + chunks on demand | WASM-powered, indexes at build | Static sites, any SSG, best overall |
| **Fuse.js** | ~24KB | Fuzzy matching, full index in browser | Small blogs (<100 posts) |
| **Lunr** | ~8KB | Pre-built index, TF-IDF scoring | Simple search needs |

**Pagefind integration (recommended):**

```bash
# Add to build script (runs after SSG build)
npx pagefind --site dist
```

```html
<!-- Search component -->
<link href="/pagefind/pagefind-ui.css" rel="stylesheet" />
<script src="/pagefind/pagefind-ui.js"></script>
<div id="search"></div>
<script>
  new PagefindUI({ element: "#search", showSubResults: true });
</script>
```

Pagefind loads nothing until the user types. Index fragments load on demand. Handles 10K+ pages without performance issues.

### 6.2 Server-Side Search (Best for Dynamic Sites)

| Service | Hosting | Pricing | Strengths |
|---------|---------|---------|-----------|
| **Typesense** | Self-hosted or cloud | Open source | Written in C++, sub-50ms, full-text + semantic |
| **Meilisearch** | Self-hosted or cloud | Open source | Written in Rust, typo-tolerant, instant results |
| **Algolia** | Cloud only | Free tier, then per-search pricing | Largest ecosystem, best docs, expensive at scale |

**When to choose server-side:** User-generated content that changes frequently, need for faceted search with real-time indexing, or sites with 50K+ documents.

---

## 7. Performance

### 7.1 Image Optimization

```html
<!-- Modern responsive image pattern -->
<picture>
  <source
    type="image/avif"
    srcset="/blog/hero-400.avif 400w,
            /blog/hero-800.avif 800w,
            /blog/hero-1200.avif 1200w"
    sizes="(max-width: 768px) 100vw, 800px"
  />
  <source
    type="image/webp"
    srcset="/blog/hero-400.webp 400w,
            /blog/hero-800.webp 800w,
            /blog/hero-1200.webp 1200w"
    sizes="(max-width: 768px) 100vw, 800px"
  />
  <img
    src="/blog/hero-800.jpg"
    alt="Convergence loop architecture diagram"
    width="800"
    height="450"
    loading="lazy"
    decoding="async"
  />
</picture>
```

**Rules:**
- Serve AVIF with WebP fallback, JPEG as last resort. Browser support: AVIF 95%, WebP 96%.
- Always specify `width` and `height` to prevent layout shift (CLS).
- Use `loading="lazy"` on all images except the first visible one (hero/above-fold).
- Use `loading="eager"` and `fetchpriority="high"` on hero images.
- Generate multiple sizes: 400w, 800w, 1200w, 1600w.
- Target under 100KB per image at display size.

### 7.2 Code Splitting and Prefetching

```html
<!-- Prefetch next likely navigation -->
<link rel="prefetch" href="/blog/page/2" />
<link rel="prefetch" href="/blog/related-post-slug" />

<!-- Preload critical CSS -->
<link rel="preload" href="/fonts/inter-var.woff2" as="font" type="font/woff2" crossorigin />
```

**Astro-specific:** Islands architecture means zero JS by default. Only interactive components ship JavaScript. This is the ideal blog architecture.

### 7.3 Core Web Vitals Targets

| Metric | Target | Blog Impact |
|--------|--------|-------------|
| **LCP** | < 2.5s | Hero image optimization, font loading |
| **FID/INP** | < 200ms | Minimal JS, no heavy frameworks |
| **CLS** | < 0.1 | Image dimensions, font-display: swap |

---

## 8. Analytics

### 8.1 Privacy-Respecting Options (Recommended)

| Service | Hosting | Script Size | Cookie-Free | Pricing |
|---------|---------|-------------|-------------|---------|
| **Plausible** | Cloud or self-hosted | < 1KB | Yes | $9/mo cloud, free self-hosted |
| **Umami** | Self-hosted | ~2KB | Yes | Free (open source) |
| **Fathom** | Cloud | ~1KB | Yes | $14/mo |

**Recommendation:** Plausible or Umami. Both are GDPR-compliant without cookie banners, lightweight (no performance impact), and provide essential metrics: pageviews, referrers, countries, devices, goals.

### 8.2 GA4

Use only if you need advanced features (user journeys, conversion funnels, audience segments). GA4 requires cookie consent banners in the EU and adds ~45KB to page weight.

### 8.3 Custom Events Worth Tracking

```javascript
// Track meaningful interactions, not vanity metrics
plausible('Newsletter Subscribe', { props: { placement: 'post-footer' } });
plausible('Post Scroll Depth', { props: { depth: '75%', post: slug } });
plausible('Code Block Copy', { props: { language: 'rust', post: slug } });
plausible('Share Click', { props: { platform: 'twitter', post: slug } });
```

---

## 9. Comment Systems

| System | Backend | Auth | Privacy | Best For |
|--------|---------|------|---------|----------|
| **Giscus** | GitHub Discussions | GitHub | Excellent (no tracking) | Developer blogs |
| **Utterances** | GitHub Issues | GitHub | Excellent | Developer blogs (simpler) |
| **Isso** | Self-hosted SQLite | Anonymous or name | Excellent | Non-developer audiences |
| **Cusdis** | Self-hosted | Anonymous | Excellent | Lightweight needs |
| **Disqus** | Cloud | Email/social | Poor (tracking, ads) | **Avoid** |

**Recommendation for developer blogs:** Giscus. Uses GitHub Discussions (threaded, supports reactions), no tracking, no ads, free forever. Requires readers to have GitHub accounts, which is a feature for developer audiences (reduces spam).

```html
<!-- Giscus integration -->
<script src="https://giscus.app/client.js"
  data-repo="your-org/your-repo"
  data-repo-id="R_..."
  data-category="Blog Comments"
  data-category-id="DIC_..."
  data-mapping="pathname"
  data-strict="0"
  data-reactions-enabled="1"
  data-emit-metadata="0"
  data-input-position="top"
  data-theme="preferred_color_scheme"
  data-lang="en"
  data-loading="lazy"
  crossorigin="anonymous"
  async>
</script>
```

---

## 10. Newsletter Integration

### 10.1 Subscribe Form Patterns

**Inline (end of post) — highest conversion:**

```html
<section class="newsletter-cta" aria-labelledby="newsletter-heading">
  <h2 id="newsletter-heading">Get posts like this in your inbox</h2>
  <p>No spam. Unsubscribe anytime. We send 2-4 posts per month.</p>
  <form action="/api/subscribe" method="POST">
    <label for="email" class="sr-only">Email address</label>
    <input type="email" id="email" name="email"
           placeholder="you@example.com" required
           autocomplete="email" />
    <button type="submit">Subscribe</button>
  </form>
  <p class="privacy-note">
    We respect your privacy. Read our <a href="/privacy">privacy policy</a>.
  </p>
</section>
```

### 10.2 CTA Placement (in order of effectiveness)

1. **End of post** — reader has consumed value, highest intent
2. **Sticky banner / slide-in** — after 50% scroll depth
3. **Sidebar** — visible but low conversion on mobile
4. **Exit intent popup** — effective but annoying; use sparingly

### 10.3 Double Opt-In

Always use double opt-in:
1. User submits email
2. Confirmation email sent with verification link
3. User clicks link to confirm
4. Subscription active

**Why:** Prevents spam signups, improves deliverability, required by GDPR in many jurisdictions, and results in higher engagement rates.

---

## 11. Accessibility (WCAG 2.1 AA)

### 11.1 Skip Links

```html
<a href="#main-content" class="skip-link">Skip to main content</a>
<!-- ... navigation ... -->
<main id="main-content" tabindex="-1">
```

```css
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: var(--color-primary);
  color: white;
  padding: 8px 16px;
  z-index: 100;
  transition: top 0.2s;
}
.skip-link:focus {
  top: 0;
}
```

### 11.2 Heading Hierarchy

- One `<h1>` per page (the blog post title or page title)
- Never skip levels (`<h1>` then `<h3>` — wrong)
- Headings must be meaningful when read in sequence (screen readers navigate by heading)
- Use CSS for visual styling, not heading levels

### 11.3 Images

- Every `<img>` must have an `alt` attribute
- Decorative images: `alt=""`
- Complex images (charts, diagrams): provide long description via `aria-describedby`
- Never use images of text (use real text instead)

### 11.4 Color and Contrast

- Text contrast ratio: minimum 4.5:1 (normal text), 3:1 (large text, 18px+)
- Do not convey information by color alone (use icons, patterns, text labels)
- Test with browser dev tools contrast checker

### 11.5 Focus Management

- All interactive elements must have visible focus indicators
- Focus indicator contrast: minimum 3:1 against adjacent colors
- Tab order must follow visual order
- Dynamic content changes: use `aria-live` regions

### 11.6 Screen Reader Testing

Test with at minimum one of:
- VoiceOver (macOS / iOS) — built-in, free
- NVDA (Windows) — free, open source
- Screen reader should be able to navigate by headings, links, and landmarks

---

## 12. SEO

### 12.1 Canonical URLs

```html
<!-- Every page must have a self-referencing canonical -->
<link rel="canonical" href="https://cruxdev.com/blog/autonomous-convergence" />
```

**Rules:**
- Always absolute URLs
- Always HTTPS
- Self-referencing on every page (including paginated pages)
- If content exists at multiple URLs, canonical points to the preferred one
- Syndicated content: canonical points back to the original

### 12.2 Sitemap Generation

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://cruxdev.com/blog/autonomous-convergence</loc>
    <lastmod>2026-03-20</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
</urlset>
```

**Rules:**
- Generate automatically at build time from content index
- Include in robots.txt: `Sitemap: https://cruxdev.com/sitemap.xml`
- Split into indexed sitemaps if >50K URLs
- Include `lastmod` with real modification dates
- Submit to Google Search Console

### 12.3 Meta Tags Checklist

```html
<head>
  <title>Autonomous Convergence | CruxDev Blog</title>
  <meta name="description" content="150-160 char description..." />
  <link rel="canonical" href="https://cruxdev.com/blog/autonomous-convergence" />
  <!-- OG tags (section 5.1) -->
  <!-- Twitter card tags (section 5.2) -->
  <!-- JSON-LD structured data (section 5.4) -->
  <!-- RSS autodiscovery (section 4.2) -->
</head>
```

### 12.4 robots.txt

```
User-agent: *
Allow: /blog/
Disallow: /blog/draft/
Disallow: /api/

Sitemap: https://cruxdev.com/sitemap.xml
```

---

## 13. Anti-Patterns

| Anti-Pattern | Why It Is Wrong | Do Instead |
|---|---|---|
| Client-side rendering for blog content | Poor SEO, slow initial load, no RSS | SSG or SSR |
| No RSS feed | Alienates power users and aggregators | Always provide Atom/RSS |
| Disqus for comments | Tracking, ads, slow, privacy violation | Giscus, Utterances, or Isso |
| Google Analytics without consent banner | GDPR violation | Plausible, Umami, or Fathom |
| No OG image | Ugly link previews reduce click-through | Auto-generate OG images |
| Date in URL for evergreen content | Content feels stale, URLs are long | Use `/blog/slug` |
| No canonical tag | Duplicate content confusion | Self-referencing canonical on every page |
| Skip heading levels for styling | Breaks screen reader navigation | Use CSS for styling, headings for structure |
| Infinite scroll without URL changes | User cannot share position, back button broken | Paginated URLs or load-more with history API |
| Loading all images eagerly | Slow page load, wasted bandwidth | Lazy load below-fold images |
| No alt text on images | Accessibility violation, missed SEO signal | Descriptive alt text on every image |
| Tag proliferation (50+ tags) | Thin tag pages hurt SEO, confusing taxonomy | 5-10 categories, 20-30 tags maximum |
| Newsletter popup on page load | Annoying, no value established yet | Show after scroll depth or at post end |

---

## 14. Audit Dimensions

Use these dimensions to evaluate any blog system:

1. **architecture** — SSG/headless/database choice matches content update frequency and team skills
2. **content-model** — frontmatter schema is strict, validated, and consistent across all posts
3. **taxonomy** — categories are few and broad, tags are specific and controlled, series are ordered
4. **urls** — clean, keyword-rich slugs; no date encoding; 301 redirects for changes
5. **feeds** — valid Atom/RSS with full content, autodiscovery link present
6. **social-meta** — OG tags, Twitter cards, and JSON-LD BlogPosting on every post
7. **search** — functional search with typo tolerance; Pagefind for static, Typesense/Meilisearch for dynamic
8. **performance** — LCP < 2.5s, CLS < 0.1, images optimized (AVIF/WebP, lazy loading, srcset)
9. **analytics** — privacy-respecting analytics installed, custom events for meaningful interactions
10. **comments** — privacy-respecting system or intentionally omitted
11. **newsletter** — subscribe form present, double opt-in, value proposition clear
12. **accessibility** — skip links, heading hierarchy, alt text, focus indicators, contrast ratios
13. **seo** — canonical URLs, sitemap, meta descriptions, structured data validated
