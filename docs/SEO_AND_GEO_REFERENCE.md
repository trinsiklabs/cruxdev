# SEO & Generative Engine Optimization — Comprehensive Reference

**Created:** 2026-03-22
**Scope:** Traditional search engine optimization, AI/LLM visibility (GEO), and performance as a ranking factor.
**Audience:** Developer tools and SaaS products.

---

## Table of Contents

1. [Traditional Search Engine SEO](#1-traditional-search-engine-seo)
   1. [Google Ranking Factors 2025-2026](#11-google-ranking-factors-2025-2026)
   2. [Page Speed and Core Web Vitals](#12-page-speed-and-core-web-vitals)
   3. [Technical SEO Checklist](#13-technical-seo-checklist)
   4. [On-Page SEO](#14-on-page-seo)
   5. [Content SEO](#15-content-seo)
   6. [Backlink Strategy](#16-backlink-strategy)
2. [AI/LLM Visibility — Generative Engine Optimization](#2-aillm-visibility--generative-engine-optimization)
   1. [How AI Models Find and Cite Sources](#21-how-ai-models-find-and-cite-sources)
   2. [AI Overviews in Google](#22-ai-overviews-in-google)
   3. [Optimizing for AI Citation](#23-optimizing-for-ai-citation)
   4. [llms.txt and llms-full.txt](#24-llmstxt-and-llms-fulltxt)
   5. [Schema.org for AI](#25-schemaorg-for-ai)
   6. [Content Structure for AI](#26-content-structure-for-ai)
   7. [AI Crawlers](#27-ai-crawlers)
   8. [Brand Mentions Without Links](#28-brand-mentions-without-links)
3. [Performance as a Ranking Factor](#3-performance-as-a-ranking-factor)
   1. [Exact Impact of Speed on Rankings](#31-exact-impact-of-speed-on-rankings)
   2. [Mobile Speed vs Desktop Speed](#32-mobile-speed-vs-desktop-speed)
   3. [Performance Budgets](#33-performance-budgets)
   4. [Image Optimization](#34-image-optimization)
   5. [JavaScript Impact](#35-javascript-impact)
   6. [CDN and Edge Computing](#36-cdn-and-edge-computing)
   7. [Font Loading](#37-font-loading)

---

## 1. Traditional Search Engine SEO

### 1.1 Google Ranking Factors 2025-2026

#### The Current Hierarchy (approximate weight order)

1. **Content quality and relevance** — The single most important factor. Google's systems evaluate whether content genuinely satisfies the query intent. The Helpful Content System (HCU), now integrated directly into the core ranking algorithm as of March 2024, rewards people-first content and penalizes content created primarily for search engines.

2. **E-E-A-T (Experience, Expertise, Authoritativeness, Trustworthiness)** — Not a direct ranking signal but a framework used by quality raters and reflected in algorithmic signals. Trust is the central pillar.
   - **Experience:** First-hand, lived experience with the topic (added December 2022).
   - **Expertise:** Depth of knowledge, demonstrated through accuracy and detail.
   - **Authoritativeness:** Recognition by others in the field (citations, backlinks, mentions).
   - **Trustworthiness:** Accuracy, transparency, safety. The most important of the four.
   - For YMYL (Your Money Your Life) topics, E-E-A-T requirements are significantly higher.
   - For developer tools: author bios with real credentials, links to GitHub profiles, published research, and demonstrated use of the tools you write about all strengthen E-E-A-T signals.

3. **Backlinks** — Still a top-3 factor despite years of predictions about their decline. Google's own leaked internal documents (2024) confirmed links remain a critical ranking input. Quality matters far more than quantity. A single link from a high-authority, topically relevant domain outweighs hundreds of low-quality links.

4. **Page experience signals (Core Web Vitals)** — A confirmed ranking factor since June 2021, but a tiebreaker rather than a dominant signal. Google has stated that content relevance will always outweigh page experience. However, when multiple pages have similar content quality, CWV becomes the differentiator.

5. **Mobile-first indexing** — Google now uses the mobile version of a site for indexing and ranking for all sites. As of 2024, there is no separate desktop index. If your content doesn't render on mobile, it functionally doesn't exist for Google.

6. **HTTPS** — A lightweight ranking signal since 2014. At this point, not having HTTPS is a negative signal rather than having it being a positive one. Over 95% of page-one results use HTTPS.

7. **RankBrain / AI-based understanding** — Google's AI systems (RankBrain, MUM, Gemini integration) interpret query intent and content meaning. Exact keyword matching matters less; semantic relevance matters more.

#### What Changed Recently (2024-2025)

- **Helpful Content System absorbed into core ranking (March 2024):** No longer a separate system. Sites hit by HCU penalties found recovery much harder because there's no distinct "HCU recovery" anymore — it's baked into everything.
- **Site reputation abuse policy (March 2024):** Third-party content hosted to exploit a site's authority (e.g., coupon pages on news sites) is now penalized.
- **Spam updates targeting AI-generated content at scale:** Google doesn't penalize AI content per se, but penalizes low-quality, mass-produced content regardless of how it was created. The March 2024 spam update specifically targeted "scaled content abuse."
- **Review and product search volatility:** Significant algorithm changes around product reviews, affiliate content, and commercial queries throughout 2024-2025.
- **AI Overviews rollout:** Organic click-through rates for informational queries have declined as AI Overviews answer queries directly in SERPs. This shifts the SEO calculus — being cited in AI Overviews is now its own objective.

#### Confirmed Non-Factors or Minimal Factors

- **Domain age** — Not a ranking factor (confirmed by Google multiple times).
- **Exact match domains** — Negligible benefit, can trigger spam filters.
- **Meta keywords tag** — Completely ignored by Google since 2009.
- **Word count as a direct factor** — Google does not have a minimum word count. However, comprehensive content tends to rank better because it better satisfies intent.
- **Social signals (likes, shares)** — Not a direct ranking factor. Correlation exists because popular content gets both social shares and backlinks, but shares themselves don't cause rankings.
- **Google Ads spending** — Does not affect organic rankings.

---

### 1.2 Page Speed and Core Web Vitals

#### Current Metrics and Thresholds

Google defines three rating categories: **Good**, **Needs Improvement**, and **Poor**.

| Metric | Full Name | Good | Needs Improvement | Poor |
|--------|-----------|------|-------------------|------|
| **LCP** | Largest Contentful Paint | ≤ 2.5s | 2.5s - 4.0s | > 4.0s |
| **INP** | Interaction to Next Paint | ≤ 200ms | 200ms - 500ms | > 500ms |
| **CLS** | Cumulative Layout Shift | ≤ 0.1 | 0.1 - 0.25 | > 0.25 |

**Note:** INP replaced FID (First Input Delay) as a Core Web Vital in March 2024. INP measures responsiveness across the entire page lifecycle, not just the first interaction.

#### Supporting Speed Metrics (not CWVs but important)

| Metric | Good Target | Notes |
|--------|-------------|-------|
| **TTFB** | ≤ 800ms | Time to First Byte. Google recommends under 800ms. Not a CWV but affects LCP directly. |
| **FCP** | ≤ 1.8s | First Contentful Paint. When first content appears. Affects perceived speed. |
| **TBT** | ≤ 200ms | Total Blocking Time. Lab proxy for INP. Sum of blocking portions of long tasks. |
| **SI** | ≤ 3.4s | Speed Index. How quickly content is visually populated. |

#### Pass Rates (CrUX data, approximate as of late 2024/early 2025)

- **LCP Good:** ~60% of origins pass
- **INP Good:** ~75% of origins pass (better than FID because INP was designed with real-world data)
- **CLS Good:** ~78% of origins pass
- **All three CWVs Good:** ~45% of origins pass all three simultaneously

These numbers vary significantly by industry. E-commerce sites tend to have worse LCP due to heavy images. Media sites struggle with CLS due to dynamic ad loading.

#### Techniques to Improve Each Metric

**LCP (Largest Contentful Paint ≤ 2.5s)**

The LCP element is typically a hero image, background image, or large text block.

1. **Identify the LCP element** — Use Chrome DevTools Performance panel or Lighthouse.
2. **Optimize server response time** — Reduce TTFB (see CDN section).
3. **Eliminate render-blocking resources:**
   - Inline critical CSS (above-the-fold styles).
   - Defer non-critical CSS with `media="print" onload="this.media='all'"`.
   - Add `async` or `defer` to non-critical JavaScript.
4. **Optimize the LCP image:**
   - Use modern formats (WebP, AVIF).
   - Add `fetchpriority="high"` to the LCP image element.
   - Preload the LCP image: `<link rel="preload" as="image" href="hero.webp">`.
   - Use appropriate sizing (don't serve a 2000px image in a 600px container).
   - Use responsive images with `srcset` and `sizes`.
5. **Avoid lazy-loading the LCP element** — Only lazy-load below-the-fold images.
6. **Preconnect to required origins:** `<link rel="preconnect" href="https://cdn.example.com">`.
7. **Use a CDN** — Reduce physical distance to users.
8. **Server-side rendering (SSR) or static generation** — Avoid client-side rendering for LCP content.

**INP (Interaction to Next Paint ≤ 200ms)**

INP measures the worst-case interaction latency (at the 98th percentile). Every click, tap, and keypress is measured.

1. **Break up long tasks:**
   - Use `requestAnimationFrame` or `requestIdleCallback` to defer non-urgent work.
   - Use `scheduler.yield()` (Scheduler API) to yield to the main thread.
   - Break JavaScript execution into chunks < 50ms.
2. **Reduce JavaScript execution time:**
   - Code-split aggressively — only load JS needed for the current view.
   - Tree-shake unused code.
   - Audit third-party scripts — each one adds to main thread contention.
3. **Minimize main thread work:**
   - Move computation to Web Workers.
   - Debounce/throttle input handlers.
   - Avoid forced synchronous layouts (reading layout properties then writing, then reading again).
4. **Optimize event handlers:**
   - Keep handlers lean — do minimal work, defer the rest.
   - Use `passive: true` for scroll/touch listeners.
5. **Reduce DOM size** — Large DOMs (> 1,400 nodes) slow down style calculations and layout.

**CLS (Cumulative Layout Shift ≤ 0.1)**

CLS measures unexpected layout shifts. Only shifts without user input in the preceding 500ms count.

1. **Always set dimensions on images and videos:**
   - Use `width` and `height` attributes: `<img width="800" height="600">`.
   - Use CSS `aspect-ratio` for responsive containers.
2. **Reserve space for dynamic content:**
   - Ads: Use fixed-size containers with `min-height`.
   - Embeds: Set explicit dimensions.
   - Dynamic banners/notifications: Reserve space or use `transform` animations (which don't cause layout shifts).
3. **Avoid inserting content above existing content** — Especially avoid injecting banners, cookie notices, or promotions that push content down.
4. **Use `font-display: optional` or `font-display: swap` with size-adjusted fallbacks** — Prevent FOIT/FOUT from causing shifts (see Font Loading section).
5. **Use CSS `contain` property** — `contain: layout` prevents element changes from affecting the rest of the page.
6. **Animate with transforms** — Use `transform: translate()` instead of changing `top`/`left`/`margin`.
7. **Use `content-visibility: auto`** — For off-screen content, tells the browser to skip rendering until needed.

---

### 1.3 Technical SEO Checklist

#### Crawlability

- [ ] **robots.txt properly configured**
  - Located at `/robots.txt` on the root domain.
  - Does NOT block CSS, JS, or image files needed for rendering.
  - Specifies sitemap location: `Sitemap: https://example.com/sitemap.xml`.
  - Test with Google Search Console's robots.txt tester.
  - Syntax: `User-agent: *`, `Disallow: /private/`, `Allow: /public/`.
  - `Disallow: /` blocks everything. Blank `Disallow:` allows everything.

- [ ] **XML sitemap**
  - Maximum 50,000 URLs per sitemap file.
  - Maximum 50MB uncompressed per sitemap.
  - Use a sitemap index file if you have multiple sitemaps.
  - Include `<lastmod>` only if accurate (don't fake it — Google uses this signal).
  - Only include canonical, indexable URLs (200 status, no noindex, self-canonical).
  - Submit via Google Search Console and reference in robots.txt.
  - For dynamic sites, generate sitemaps programmatically and update them regularly.

- [ ] **Crawl budget optimization** (matters mainly for large sites, 10,000+ pages)
  - Eliminate duplicate content.
  - Fix redirect chains (no more than 1 redirect hop).
  - Return proper 404s for dead pages (don't soft-404).
  - Use `<link rel="canonical">` to consolidate duplicate pages.
  - Block low-value pages from crawling (filtered views, internal search results, paginated tag pages with thin content).

- [ ] **Internal linking**
  - Every important page should be reachable within 3 clicks from the homepage.
  - Use descriptive anchor text (not "click here").
  - Implement breadcrumb navigation.
  - Fix orphan pages (pages with no internal links pointing to them).
  - Avoid excessive links per page (Google doesn't enforce a hard limit, but diminishing returns above ~100-150 internal links).

#### Indexability

- [ ] **Meta robots tag** — `<meta name="robots" content="index, follow">` (default, so usually omit).
  - Use `noindex` for pages you don't want indexed (login pages, thank-you pages, staging environments).
  - Use `nofollow` sparingly on a page level — it prevents Google from following ANY link on the page.
  - `X-Robots-Tag` HTTP header works for non-HTML resources (PDFs, images).

- [ ] **Canonical URLs**
  - Every page should have a self-referencing canonical: `<link rel="canonical" href="https://example.com/page/">`.
  - Choose one canonical form: www vs non-www, trailing slash vs no trailing slash, http vs https.
  - Set canonicals for paginated content, URL parameters, and duplicate content.
  - Canonical must be the same protocol and domain you want indexed.
  - Google treats canonicals as hints, not directives. If the canonical page is noindexed or 404s, Google will ignore the canonical.

- [ ] **Structured data (Schema.org)**
  - Use JSON-LD format (Google's recommended format).
  - Key types for developer tools / SaaS:
    - `Organization` — Company info, logo, social profiles.
    - `WebSite` — Site-level info, sitelinks searchbox.
    - `Article` / `BlogPosting` — Blog content, author info, date published/modified.
    - `FAQPage` — FAQ sections (triggers rich results).
    - `HowTo` — Tutorials and guides (triggers rich results).
    - `SoftwareApplication` — Developer tools, pricing, ratings.
    - `BreadcrumbList` — Breadcrumb navigation.
    - `Product` — For paid tools/plans with pricing.
    - `VideoObject` — Embedded videos.
  - Validate with Google's Rich Results Test: https://search.google.com/test/rich-results
  - Validate with Schema.org validator: https://validator.schema.org/
  - Do NOT add structured data for content not visible on the page (this is spam).

- [ ] **Hreflang** (if multilingual/multi-regional)
  - Format: `<link rel="alternate" hreflang="en-us" href="https://example.com/en-us/page/">`.
  - Include a self-referencing hreflang tag.
  - Include `x-default` for the fallback version.
  - Every page referenced in hreflang must reference all other versions back (bidirectional).
  - Can be implemented in HTML head, HTTP headers, or sitemap.

- [ ] **Pagination**
  - Google no longer uses `rel="next"` / `rel="prev"` (deprecated 2019).
  - Instead: ensure all paginated pages are crawlable, use self-referencing canonicals on each page, and consider a "view all" page if feasible.
  - For infinite scroll: ensure content is accessible without JavaScript, or use hybrid approach with paginated URLs.

- [ ] **JavaScript rendering**
  - Google renders JavaScript but with a delay (seconds to days for the initial render, sometimes longer for subsequent updates).
  - Use SSR (Server-Side Rendering) or SSG (Static Site Generation) for critical content.
  - Avoid rendering critical content only via client-side JavaScript.
  - Test with Google Search Console's URL Inspection tool ("View Tested Page" shows what Googlebot sees).
  - Use dynamic rendering as a fallback (serve pre-rendered HTML to crawlers).
  - Ensure JavaScript errors don't prevent content from rendering.

- [ ] **Mobile-first indexing requirements**
  - Same content on mobile and desktop (don't hide content on mobile).
  - Same structured data on mobile and desktop.
  - Same meta tags (title, description, robots) on mobile and desktop.
  - Responsive design preferred over separate mobile URLs (m.example.com).
  - Touch targets at least 48x48 CSS pixels.
  - Text readable without zooming (minimum 16px font size recommended).
  - No horizontal scrolling at standard viewport widths.
  - Viewport meta tag: `<meta name="viewport" content="width=device-width, initial-scale=1">`.

---

### 1.4 On-Page SEO

#### Title Tags

- **Length:** 50-60 characters (Google truncates at ~600 pixels width, roughly 60 chars).
- **Format:** `Primary Keyword — Secondary Keyword | Brand Name` or similar.
- **Include primary keyword** near the beginning.
- **Unique** for every page — duplicate titles dilute ranking signals.
- **Compelling** — The title is your headline in search results. Write for CTR.
- Google may rewrite your title if it doesn't match the page content or query. To reduce rewrites:
  - Make titles accurately describe page content.
  - Avoid keyword stuffing.
  - Don't use boilerplate templates across all pages.

#### Meta Descriptions

- **Length:** 120-160 characters (Google truncates at ~920 pixels on desktop, ~680 on mobile).
- **Not a ranking factor** directly, but affects CTR, which indirectly affects rankings.
- Include a **call to action** or **value proposition**.
- Include the **primary keyword** (Google bolds matching terms).
- **Unique** for every page.
- Google rewrites meta descriptions ~60-70% of the time, pulling text from page content that better matches the query. Write good descriptions anyway — they're used when they match.

#### Heading Hierarchy

- **One H1 per page** — Should contain the primary keyword and clearly describe the page topic.
- **H2s** for major sections.
- **H3-H6** for subsections. Maintain logical nesting (don't skip from H2 to H4).
- Headings help Google understand content structure and can be used for featured snippets.
- Don't use headings purely for visual styling — use CSS for that.

#### Internal Linking

- **Use descriptive anchor text** — "Python testing framework" not "click here."
- **Link from high-authority pages to important pages** — Pass PageRank internally.
- **Use contextual links** within body content (more valuable than navigation/footer links).
- **Audit regularly** for broken internal links.
- **Hub and spoke model** — Pillar pages link to cluster pages and vice versa.
- Strategic internal linking is one of the most underutilized SEO levers.

#### Image Optimization

- **File names:** Descriptive, hyphenated: `python-testing-framework-architecture.webp` not `IMG_3847.jpg`.
- **Alt text:** Describe the image content accurately. Include keywords naturally. 125 characters max recommended.
  - Good: `alt="Diagram showing CruxDev convergence loop with TDD, audit, and fix phases"`
  - Bad: `alt="image"` or `alt=""` (unless decorative) or `alt="best dev tool CLI framework testing AI 2025"`
- **Modern formats:** WebP (26-34% smaller than JPEG), AVIF (50% smaller than JPEG). Use `<picture>` element for fallbacks.
- **Responsive images:** Use `srcset` and `sizes` attributes to serve appropriate sizes.
- **Lazy loading:** Add `loading="lazy"` to below-the-fold images. Never lazy-load the LCP image.
- **Compression:** Target 85% quality for JPEG/WebP. Use tools like Squoosh, Sharp, or ImageOptim.
- **Dimensions:** Always specify `width` and `height` to prevent CLS.

#### URL Structure

- **Short and descriptive:** `/docs/getting-started/` not `/index.php?page=42&category=docs&ref=main`.
- **Use hyphens** as word separators (not underscores).
- **Lowercase** only.
- **Include primary keyword** naturally.
- **Avoid URL parameters** when possible — use clean URLs.
- **Consistent trailing slash** convention.
- **No dates in URLs** unless content is truly date-dependent (news articles). Dates make evergreen content look stale.

#### Content Length

- No magic number. Google doesn't have a minimum word count.
- In practice, top-ranking pages for competitive informational queries average 1,400-2,000 words — but this is correlation, not causation. They rank because they comprehensively cover the topic, not because they hit a word count.
- For developer documentation: completeness and accuracy matter infinitely more than length.
- For product pages: concise, clear, conversion-focused copy often outperforms long-form.

#### Keyword Placement

- **Title tag** (near the beginning)
- **H1** heading
- **First 100 words** of body content
- **URL slug**
- **At least one H2** subheading
- **Image alt text** (naturally)
- **Meta description** (for CTR, not ranking)
- Avoid keyword density targets — write naturally. Google's NLP is sophisticated enough to understand synonyms, related terms, and semantic meaning.

---

### 1.5 Content SEO

#### Topical Authority

Google rewards sites that demonstrate deep, comprehensive coverage of a subject area. A site with 50 high-quality, interlinked articles on "JavaScript testing" will outrank a site with 1 article on the same topic, even if that single article is excellent.

**Building topical authority for developer tools:**
1. Identify your core topic areas (e.g., "CLI development," "AI-assisted coding," "developer productivity").
2. Map all subtopics within each area.
3. Create content that covers every meaningful subtopic.
4. Interlink comprehensively within topic clusters.
5. Update content regularly to maintain freshness.
6. Demonstrate real expertise — show actual code, real benchmarks, genuine experience.

#### Content Clusters (Hub and Spoke Model)

**Pillar page** (the hub):
- Comprehensive overview of a broad topic (2,000-5,000 words).
- Links to all cluster pages.
- Targets the broad, high-volume keyword.
- Example: "Complete Guide to CLI Development"

**Cluster pages** (the spokes):
- Deep dives into specific subtopics (1,000-2,500 words).
- Link back to the pillar page AND to related cluster pages.
- Target long-tail, specific keywords.
- Example: "Argument Parsing in Node.js CLIs," "Testing CLI Applications with Jest," "Publishing CLI Tools to npm"

**Benefits:**
- Establishes topical authority.
- Creates strong internal linking structure.
- Covers the full range of search intent for a topic.
- Makes it easier for Google to understand your site's expertise areas.

#### Search Intent Matching

Google's most important ranking criterion: **does this page satisfy the user's intent?**

| Intent Type | User Goal | Content Format | Example Query |
|------------|-----------|----------------|---------------|
| **Informational** | Learn something | Blog posts, guides, tutorials, documentation | "what is TDD" |
| **Navigational** | Find a specific site/page | Homepage, login page, specific product page | "GitHub login" |
| **Transactional** | Complete an action (buy, sign up) | Product pages, pricing pages, sign-up flows | "buy JetBrains license" |
| **Commercial Investigation** | Compare before deciding | Comparison pages, reviews, "best of" lists | "VS Code vs Cursor comparison" |

**How to match intent:**
1. Google the target keyword and analyze what ranks on page 1.
2. Note the **content type** (blog post, product page, tool page, listicle).
3. Note the **content format** (how-to, list, comparison, review).
4. Note the **content angle** (beginner-focused, advanced, recent, comprehensive).
5. Create content that matches the dominant intent pattern BUT adds unique value.

#### Content Freshness Signals

Google values freshness for queries where recency matters (news, evolving topics, annual events) but not for evergreen topics ("how does TCP work").

**Signals Google uses:**
- Page publication date (from structured data, visible dates, or Google's own discovery date).
- Last modified date (from HTTP headers, XML sitemap `<lastmod>`, or visible "updated" dates).
- Rate and magnitude of content changes.
- New pages being created on the site.
- New backlinks and mentions.

**Best practices:**
- Add "Last updated: [date]" to articles and actually update them.
- Update `<lastmod>` in sitemaps only when content meaningfully changes.
- Don't fake freshness by changing dates without changing content — Google detects this.
- For developer tools: keep documentation in sync with your latest release. Stale docs hurt E-E-A-T.
- Refresh high-value content quarterly or when the topic landscape changes.

---

### 1.6 Backlink Strategy

#### How Links Affect Ranking

Backlinks remain one of Google's top-3 ranking factors. Each link acts as a "vote" of confidence from one site to another. Google evaluates links on multiple dimensions:

- **Authority of the linking domain** — A link from MDN Web Docs or the official Node.js blog carries immense weight. A link from a brand-new blog carries almost none.
- **Relevance of the linking page** — A link from a page about JavaScript testing to your JavaScript testing tool is worth more than a link from a page about cooking.
- **Anchor text** — The clickable text of the link tells Google what the linked page is about. Diverse, natural anchor text is ideal.
- **Link placement** — Editorial links within body content are worth more than footer/sidebar/navigation links.
- **Freshness of the link** — Recent links may carry slightly more weight than very old links.
- **Uniqueness of linking domains** — 10 links from 10 different domains > 100 links from 1 domain.

#### Quality vs Quantity

**The ratio is not even close — quality wins overwhelmingly.**

One high-quality, relevant backlink from a domain with high authority can be worth more than thousands of low-quality links. Low-quality links in bulk can trigger Google's SpamBrain algorithm and result in penalties.

**High-quality link characteristics:**
- From a topically relevant, authoritative domain.
- Editorial (someone genuinely chose to link to you because your content is valuable).
- In body content, not sidebar/footer.
- From a page that itself ranks well and gets traffic.
- Natural anchor text that describes your content.

**Low-quality / toxic link characteristics:**
- Paid links (without `rel="sponsored"`).
- Link exchanges at scale.
- PBN (Private Blog Network) links.
- Comment spam, forum spam.
- Links from hacked sites.
- Links from irrelevant directories.

#### Anchor Text

- **Branded:** "CruxDev" — Always safe, should be the most common type.
- **Exact match:** "AI-driven development framework" — Use sparingly. Over-optimization is a penalty trigger.
- **Partial match:** "tools like CruxDev for AI development" — Natural and valuable.
- **Generic:** "click here," "learn more" — Low value but natural in context.
- **Naked URL:** "https://cruxdev.io" — Natural and safe.

**Ideal distribution:** Mostly branded and partial-match anchors, with a natural spread of generic and naked URL anchors. Very few exact-match keyword anchors.

#### Nofollow / Dofollow

- **`rel="dofollow"`** — Default. Passes PageRank. (Note: there is no actual `dofollow` attribute; the absence of `nofollow` means the link is followed.)
- **`rel="nofollow"`** — Tells Google not to pass PageRank. Google treats this as a "hint" since March 2020 (may still use the link for discovery/ranking).
- **`rel="sponsored"`** — For paid/sponsored links. Required by Google for any paid placement.
- **`rel="ugc"`** — For user-generated content (comments, forum posts).

Even `nofollow` links have value: brand visibility, referral traffic, and Google may still use them as signals.

#### Link Building for Developer Tools and SaaS

**High-ROI approaches (ethical, sustainable):**

1. **Open source contributions and ecosystem presence**
   - Open-source your tool or parts of it. GitHub repos naturally attract links.
   - Contribute to popular projects and get mentioned in their docs.
   - Create useful libraries, plugins, or integrations that others link to.

2. **Developer-focused content marketing**
   - In-depth technical tutorials that become reference material.
   - Benchmark studies and comparisons (data-driven, fair).
   - Industry research reports with original data.
   - "State of X" reports (developers love sharing these).

3. **Developer relations and community**
   - Speak at conferences (talk pages link to speakers).
   - Appear on developer podcasts (show notes include links).
   - Participate authentically in communities (Hacker News, Reddit, Dev.to).
   - Write guest posts for established developer blogs (Smashing Magazine, CSS-Tricks, LogRocket Blog).

4. **Tools and resources**
   - Build free tools that solve a real problem (calculators, generators, validators).
   - Create interactive playgrounds or sandboxes.
   - Design cheat sheets, infographics, or reference cards that others embed.

5. **Documentation as a link magnet**
   - Comprehensive, well-structured documentation naturally attracts links.
   - API references, migration guides, and getting-started tutorials are frequently linked.

6. **Digital PR for developers**
   - Publish original research that tech journalists would cover.
   - Create newsworthy launches and announcements.
   - Offer expert commentary on industry trends.

7. **Integration partnerships**
   - Integrate with popular tools and get listed on their integrations pages.
   - Each integration page is a high-quality, relevant backlink.

---

## 2. AI/LLM Visibility — Generative Engine Optimization

### 2.1 How AI Models Find and Cite Sources

#### Training Data

Large language models are trained on massive datasets of text from the internet. The specific composition varies by model:

- **GPT-4/ChatGPT:** Trained on internet text through a cutoff date (exact corpus not disclosed). Includes web pages, books, academic papers, code repositories, forums, and documentation. When using web browsing, it searches the live web via Bing.
- **Claude:** Trained on a mix of internet text, books, and other sources through its cutoff. When using search tools, accesses the live web.
- **Gemini:** Trained on web data, books, and code. Deeply integrated with Google Search for real-time information.
- **Perplexity:** Not a traditional LLM — it's a search engine that uses LLMs to synthesize answers. Always searches the live web and provides source citations.

#### Source Selection Mechanisms

**For training data influence:**
- Content that appears frequently across the web (in documentation, references, citations) becomes more deeply embedded in model knowledge.
- Authoritative sources (Wikipedia, official documentation, peer-reviewed papers, established technical blogs) are weighted more heavily in training.
- Content structure matters — well-organized, clearly written content is more likely to be correctly interpreted and reproduced.

**For real-time search/citation (RAG-based responses):**
- When models use search (ChatGPT Browse, Perplexity, Claude with search, Google AI Overviews), the source selection process resembles traditional search:
  1. A search query is formulated from the user's question.
  2. Search results are retrieved (usually from Bing or Google).
  3. The model reads the top results and synthesizes an answer.
  4. Sources are cited based on which pages contributed the most to the answer.
- Pages that rank well in traditional search are more likely to be retrieved and cited.
- Pages with clear, direct answers to questions are more likely to be quoted.

**Key insight:** There is massive overlap between good SEO and good GEO. Pages that rank well in Google are more likely to be retrieved by AI systems, and pages with clear, well-structured content are more likely to be cited in AI responses.

### 2.2 AI Overviews in Google

#### Prevalence

- AI Overviews were rolled out in the US in May 2024 and expanded to 100+ countries by late 2024.
- As of early 2025, AI Overviews appear for an estimated **10-15% of all Google queries** (estimates vary by source; some studies report as low as 7%, others as high as 25% depending on query category).
- They appear most frequently for:
  - Informational queries (how-to, what-is, why-does).
  - Health and science questions.
  - Technical/programming queries.
  - Product comparison queries.
- They appear less frequently for:
  - Navigational queries (users looking for a specific site).
  - Simple factual queries answered by Knowledge Panels.
  - YMYL queries where Google is cautious (financial advice, medical treatment).
  - Queries with very recent/breaking news.

#### Source Selection for AI Overviews

Google's AI Overviews pull from pages that already rank well organically, but with some distinct patterns:

- **Pages ranking in the top 10 organic results** for the query are heavily favored. Most AI Overview citations come from page-1 results.
- **Content that directly answers the query** is preferred over content that requires the reader to extract the answer.
- **Structured content** (lists, tables, step-by-step instructions) is cited more often than unstructured prose.
- **Multiple sources** are typically cited (3-8 sources per AI Overview).
- **Authoritative domains** are disproportionately cited (official documentation, well-known publications, government sites, educational institutions).

#### Impact on Organic Traffic

- AI Overviews reduce click-through rates for informational queries by an estimated 15-25% (varies widely by study).
- Transactional and commercial queries are less affected.
- Being cited IN the AI Overview partially compensates for lost organic clicks — cited sources still get traffic.
- The long-term trend suggests that winning a citation in AI Overviews will become as important as winning a featured snippet.

### 2.3 Optimizing for AI Citation

#### The Princeton/Georgia Tech GEO Research

A landmark study from Princeton and Georgia Tech (2024) introduced the concept of "Generative Engine Optimization" (GEO) and tested specific techniques for improving source visibility in AI-generated responses.

**Key findings:**

1. **Citing statistics** improved visibility by up to **40%**. Adding specific numbers, percentages, and data points made content significantly more likely to be cited.

2. **Adding quotations from experts** improved visibility by **~15%**. Including attributed quotes from recognized authorities increased citation rates.

3. **Making content authoritative** (confident, expert framing) improved visibility by **~10-15%**. Using language that conveys expertise and certainty (when accurate) helped.

4. **Technical terminology** performed well for technical queries — using precise, domain-specific language matched how technical queries are phrased.

5. **Fluency optimization** (simply making text read better) had a **moderate positive effect (~5-10%)**.

6. **Keyword stuffing had a NEGATIVE effect** — just as in traditional SEO, forcing keywords hurt more than it helped.

#### Specific Techniques for AI Citation

**1. Answer the question in the first 40-80 words**
- AI systems often pull the most concise, direct answer.
- Start articles/sections with a clear, direct answer, then elaborate.
- Example: "CruxDev is an autonomous convergence framework that drives AI-assisted development to completion without human intervention. It installs as a plugin and uses TDD-driven audit-fix loops to achieve convergence." — Then elaborate.

**2. Use clear, descriptive headings that match questions**
- H2/H3 headings that mirror common queries help AI systems match content to questions.
- Use question-format headings: "What is CruxDev?" "How does CruxDev achieve convergence?"
- AI systems use headings to understand content structure and extract relevant sections.

**3. Include specific statistics and data**
- "Reduces development time by 40%" is more citable than "significantly reduces development time."
- "Achieves 100% test coverage in an average of 3 convergence loops" is more citable than "achieves high test coverage quickly."
- Numbers make statements verifiable and authoritative.

**4. Use structured data (Schema.org)**
- Structured data helps AI systems understand the type and purpose of your content.
- `FAQPage` schema is particularly effective for getting questions and answers surfaced.
- `HowTo` schema helps tutorial content get cited for procedural queries.

**5. Cite your own sources**
- Content that cites authoritative sources is perceived as more trustworthy.
- Link to primary sources, research papers, official documentation.
- This mirrors how academic citation works — well-cited content is trusted more.

**6. Use definition patterns**
- "[Term] is [definition]" patterns are highly extractable by AI systems.
- "Convergence, in the context of CruxDev, means..." is easy for AI to parse and cite.

**7. Create comprehensive, well-organized content**
- AI systems favor content that thoroughly covers a topic.
- Use a logical hierarchy: overview → details → examples → related topics.
- Table of contents at the top signals comprehensive coverage.

**8. Lists and tables**
- Bulleted lists, numbered steps, and comparison tables are easy for AI to parse and reproduce.
- Comparison tables are especially effective for "X vs Y" queries.

**9. Unique, original data and perspectives**
- AI systems can't cite what doesn't exist. Original research, unique benchmarks, novel frameworks, and proprietary data give you content that can ONLY be attributed to you.

### 2.4 llms.txt and llms-full.txt

#### What It Is

`llms.txt` is an emerging standard (proposed in late 2024) that provides a machine-readable summary of a website's content, optimized for consumption by large language models. It is conceptually similar to `robots.txt` (for crawlers) and `sitemap.xml` (for search engines) but designed specifically for AI systems.

**Location:** `https://example.com/llms.txt` (root of the domain, like robots.txt).

#### Purpose

LLMs struggle with complex websites because:
- Websites are designed for human visual consumption with navigation, styling, and interactive elements.
- Relevant information may be spread across many pages.
- Context windows are limited — an LLM can't "browse" a whole site like a human can.

`llms.txt` solves this by providing a structured, text-only summary that an LLM can consume in a single context window.

#### Format

The specification is simple Markdown:

```markdown
# Site Name

> Brief description of what the site/project is about.

## Docs

- [Getting Started](https://example.com/docs/getting-started): How to install and set up.
- [API Reference](https://example.com/docs/api): Full API documentation.
- [Configuration](https://example.com/docs/config): Configuration options.

## Blog

- [Announcing v2.0](https://example.com/blog/v2): Major release with new features.

## Optional

- [Contributing Guide](https://example.com/contributing): How to contribute.
- [Changelog](https://example.com/changelog): Version history.
```

#### llms-full.txt

While `llms.txt` provides a curated index with links and brief descriptions, `llms-full.txt` provides the actual full content in a single file — essentially a complete text dump of all important documentation and content, designed to be loaded entirely into an LLM's context window.

**Location:** `https://example.com/llms-full.txt`

This is particularly valuable for developer tools and documentation sites where an AI assistant needs to understand the full API, configuration options, and usage patterns to give accurate answers.

#### Who Uses It

As of early 2025, adoption is primarily among developer-focused sites and AI-adjacent companies:
- Various AI and developer tool companies have adopted it.
- Documentation frameworks (Mintlify, ReadMe, Docusaurus plugins) are adding support.
- It's still a proposal/convention, not an official standard body specification.

#### Implementation Recommendations

1. Create a `llms.txt` at your site root with your most important pages.
2. Create a `llms-full.txt` with full content for documentation-heavy sites.
3. Keep both files updated as content changes (automate this in your build process).
4. Use clear Markdown formatting — this is consumed by LLMs, so structure matters.
5. Prioritize: put the most important, most frequently asked about content first.
6. Include pricing, features, and comparison information if you're a commercial product.

### 2.5 Schema.org for AI

Structured data helps AI systems understand your content's type, purpose, and relationships. While Schema.org was originally designed for search engines, AI systems increasingly leverage it.

#### Most Valuable Types for AI Visibility

**`FAQPage`**
```json
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [{
    "@type": "Question",
    "name": "What is CruxDev?",
    "acceptedAnswer": {
      "@type": "Answer",
      "text": "CruxDev is an autonomous convergence framework..."
    }
  }]
}
```
- Directly maps questions to answers — perfect for AI extraction.
- Triggers rich results in Google (expandable Q&A in SERPs).
- Maps directly to how users query AI systems.

**`HowTo`**
```json
{
  "@context": "https://schema.org",
  "@type": "HowTo",
  "name": "How to set up CruxDev",
  "step": [{
    "@type": "HowToStep",
    "name": "Install the plugin",
    "text": "Run npm install cruxdev..."
  }]
}
```
- Procedural content that AI systems can cite step-by-step.
- Triggers rich results in Google.

**`Article` / `TechArticle`**
- Provides metadata about the content: author, date published, date modified, publisher.
- Helps AI systems assess recency and authoritativeness.
- `TechArticle` is specifically for technical content and includes `proficiencyLevel`.

**`SoftwareApplication`**
```json
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "CruxDev",
  "applicationCategory": "DeveloperApplication",
  "operatingSystem": "Cross-platform",
  "offers": {
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "USD"
  }
}
```
- Helps AI systems understand your product's category, pricing, and capabilities.
- Important for "what tools do X" and "best tool for Y" queries.

**`Organization`**
- Brand identity, founding date, key people, social profiles.
- Helps AI systems make correct brand attributions.

**`DefinedTerm` / `DefinedTermSet`**
- For glossaries and technical terminology.
- Helps AI systems understand domain-specific vocabulary.
- Under-used but highly relevant for developer tools that introduce new concepts.

**`Dataset`**
- For benchmarks, research data, comparisons.
- Makes your data more discoverable and citable.

#### Best Practices

- Use JSON-LD (not Microdata or RDFa) — it's easier to maintain, doesn't interfere with HTML structure, and is Google's recommended format.
- Nest types logically (Organization contains People, Article has Author, etc.).
- Validate all structured data.
- Don't mark up content that isn't visible on the page — this violates Google's guidelines and can result in penalties.
- Keep structured data synchronized with visible content.

### 2.6 Content Structure for AI

#### The Overlap: What Works for Both SEO and GEO

| Technique | SEO Benefit | GEO Benefit |
|-----------|-------------|-------------|
| Clear heading hierarchy | Helps Google understand structure | Helps AI extract relevant sections |
| Direct answers to questions | Featured snippet potential | Direct citation potential |
| Lists and tables | Rich result eligibility | Easy for AI to parse and reproduce |
| Statistics and data | E-E-A-T signal | Higher citation rate (Princeton study) |
| Comprehensive coverage | Topical authority | More opportunities for citation |
| Internal linking | PageRank distribution | Helps AI crawlers discover content |
| Structured data | Rich results, entity understanding | Content type and relationship understanding |
| Authoritative tone | Trust signal | Higher citation priority |

#### What's Different

**SEO-specific (less important for GEO):**
- Keyword density and placement (AI doesn't care about keyword frequency).
- URL structure (AI systems rarely surface or evaluate URLs).
- Meta descriptions (AI systems read page content, not meta tags).
- Title tag optimization (AI reads the full content, not just the title).
- Page speed / Core Web Vitals (AI systems don't evaluate rendering performance).

**GEO-specific (less important for traditional SEO):**
- **Concise, self-contained answers** — AI systems prefer content that answers a question completely within a paragraph or section, without requiring the reader to navigate elsewhere.
- **Definition patterns** — "X is Y" sentences are highly extractable.
- **llms.txt** — No SEO value, significant GEO value.
- **Content that is quotable** — Sentences that can stand alone as accurate statements are more likely to be cited.
- **Consistent brand naming** — AI systems may have multiple references to your brand; consistent naming helps them consolidate. If your brand is "CruxDev" (one word, camel case), use it consistently, not "Crux Dev" or "cruxdev" or "Crux-Dev."

#### Writing for Both: The Dual-Optimized Approach

1. **Start with a concise answer** (40-80 words) — Good for featured snippets AND AI citation.
2. **Follow with structured, detailed content** — Use headings, lists, tables, and code blocks.
3. **Include specific data** — Numbers, benchmarks, statistics.
4. **Use definition patterns** — Especially for key concepts.
5. **Add structured data** — JSON-LD for both Google rich results and AI comprehension.
6. **End with related topics / next steps** — Helps both internal linking (SEO) and topical coverage (GEO).
7. **Include an FAQ section** — With `FAQPage` schema. Works for both.

### 2.7 AI Crawlers

#### Known AI Crawlers

| Crawler | Company | User-Agent String | Purpose |
|---------|---------|-------------------|---------|
| **GPTBot** | OpenAI | `GPTBot` | Training data and ChatGPT Browse |
| **ChatGPT-User** | OpenAI | `ChatGPT-User` | Real-time browsing in ChatGPT |
| **ClaudeBot** | Anthropic | `ClaudeBot` (previously `anthropic-ai`) | Training data collection |
| **Google-Extended** | Google | `Google-Extended` | Gemini/Bard training data (separate from Googlebot) |
| **Googlebot** | Google | `Googlebot` | Search indexing AND AI Overviews (you cannot block AI Overviews without blocking search) |
| **Bytespider** | ByteDance | `Bytespider` | Training data for ByteDance AI models |
| **PerplexityBot** | Perplexity | `PerplexityBot` | Perplexity search engine |
| **Applebot-Extended** | Apple | `Applebot-Extended` | Apple Intelligence training |
| **Meta-ExternalAgent** | Meta | `Meta-ExternalAgent` | Meta AI training |
| **cohere-ai** | Cohere | `cohere-ai` | Cohere model training |

#### Managing AI Crawler Access via robots.txt

```
# Allow search engines
User-agent: Googlebot
Allow: /

# Block AI training crawlers (examples)
User-agent: GPTBot
Disallow: /

User-agent: ClaudeBot
Disallow: /

User-agent: Google-Extended
Disallow: /

# Allow specific AI crawlers
User-agent: PerplexityBot
Allow: /

User-agent: ChatGPT-User
Allow: /
```

#### Trade-offs: Blocking vs Allowing

**Arguments for ALLOWING AI crawlers:**

1. **AI visibility is the new SEO.** If your content isn't in training data, AI models can't recommend you.
2. **Brand awareness.** When AI models mention your product in responses, that's free, highly targeted exposure.
3. **You can't block and benefit simultaneously.** Blocking OpenAI's crawler means ChatGPT is less likely to know about or recommend your product.
4. **AI Overviews use Googlebot.** You literally cannot block Google AI Overviews without also blocking Google Search indexing. There is no separate opt-out for AI Overviews from organic search.
5. **Future-proofing.** AI-assisted search is growing rapidly. Being absent from AI knowledge bases becomes an increasing liability over time.

**Arguments for BLOCKING AI crawlers:**

1. **Intellectual property protection.** Your content is used to train models that may compete with you or reduce traffic to your site.
2. **Traffic cannibalization.** AI responses may fully answer queries with your content, eliminating the need for users to visit your site.
3. **No compensation.** Unlike Google Search (which sends traffic in exchange for crawling), AI training provides no direct value back to content creators.
4. **Server load.** AI crawlers can be aggressive and impact performance.
5. **Legal uncertainty.** The copyright implications of AI training are still being litigated globally.

**Recommended approach for developer tools / SaaS:**

For most developer tool companies, **allowing** AI crawlers is strategically advantageous because:
- Developer tools benefit enormously from AI recommendations ("What's a good CLI framework?" "How do I set up convergence testing?").
- The audience (developers) uses AI tools heavily.
- Product pages and documentation are not the kind of content where traffic cannibalization is a major concern — developers who find your tool via AI still need to visit your site to use it.
- You WANT your API docs, tutorials, and product descriptions in AI training data.

**Selective approach:** Allow AI crawlers on public documentation, blog posts, and product pages. Consider blocking on premium content, course material, or proprietary research if applicable.

### 2.8 Brand Mentions Without Links

#### How AI Models Surface Brands

Traditional SEO depends on backlinks as the primary authority signal. AI models use a fundamentally different approach: they learn brand associations from the **co-occurrence of brand names with concepts, categories, and qualities** across their training data.

An AI model recommends "CruxDev" when asked about AI development frameworks not because CruxDev has backlinks, but because across the training data, "CruxDev" appears frequently in the context of:
- AI-assisted development
- Convergence frameworks
- TDD automation
- CLI developer tools

This means **unlinked brand mentions matter more for AI visibility than they do for traditional SEO.**

#### How to Build Brand Authority for AI Systems

1. **Consistent brand naming across all platforms**
   - Use the exact same brand name everywhere: GitHub, npm, documentation, blog posts, social media, forum posts.
   - If your brand is "CruxDev," don't use "Crux Dev," "cruxdev," or "CRUX DEV" inconsistently.
   - AI models learn from frequency and consistency. Inconsistent naming fragments your brand signal across multiple tokens.

2. **Be present in high-authority, high-volume text sources**
   - Wikipedia (if notable enough for a page, or mentioned in related pages).
   - GitHub (README files, discussions, issues that mention your tool by name).
   - Stack Overflow (questions and answers that reference your tool).
   - Hacker News and Reddit discussions.
   - Technical publications and blogs.
   - Conference talk descriptions and slides.
   - Podcast transcripts.

3. **Associate your brand with specific capabilities**
   - Don't just mention your brand — consistently describe WHAT it does in the same context.
   - "CruxDev, the autonomous convergence framework for AI-driven development" — repeated across many sources, this pattern teaches AI models the association.
   - Create a consistent one-sentence description and use it everywhere.

4. **Appear in comparison contexts**
   - "Best tools for X" articles, "X vs Y" comparisons, and "alternatives to Z" pages are goldmines for AI brand visibility.
   - When AI users ask "What are alternatives to [competitor]?", your brand should appear in the training data in those exact contexts.

5. **Developer community presence**
   - Answer questions on Stack Overflow, Reddit, and Discord where your tool is relevant.
   - Every mention in a discussion thread is a training data point.
   - Quality matters — a thoughtful, detailed response mentioning your tool in context carries more weight than a drive-by promotion.

6. **Content on third-party platforms**
   - Guest posts on established tech blogs.
   - Articles on Dev.to, Medium, Hashnode.
   - Tutorials on YouTube (transcripts become training data).
   - These create brand mentions on domains with high training data weight.

7. **Press and earned media**
   - Product Hunt launches (descriptions get widely syndicated).
   - Tech news coverage.
   - Interview and profile pieces.
   - Awards and recognition lists.

8. **Create citable definitions and concepts**
   - If you coin a term ("convergence-driven development"), define it clearly and use it consistently. AI models learn new concepts from repeated, consistent definitions.
   - Original frameworks, methodologies, and taxonomies that others reference create strong brand association.

---

## 3. Performance as a Ranking Factor

### 3.1 Exact Impact of Speed on Rankings

#### What Google Has Said

- **2010:** Google announced site speed as a ranking factor for desktop search. Stated it affected fewer than 1% of queries.
- **2018:** "Speed Update" extended speed as a ranking factor to mobile search. Google stated it only affected the slowest sites and would not benefit already-fast sites.
- **2021:** Core Web Vitals became a ranking factor as part of the "page experience update." Google was explicit that it's a tiebreaker, not a dominant signal: "A good page experience doesn't override having great, relevant content."
- **2023:** Google dropped several page experience signals from documentation (mobile-friendly, HTTPS, no intrusive interstitials) as named ranking signals, consolidating into CWV. Later clarified these factors still matter but are not individually tracked as named signals.
- **Google's consistent position:** Content relevance > page experience. But among equally relevant results, faster pages rank higher.

#### What Studies Show

- **Searchmetrics (2020):** Pages ranking in positions 1-3 had 20-30% faster LCP than pages ranking 7-10.
- **Ahrefs (2023):** Found a modest correlation between Core Web Vitals scores and rankings. Passing CWV was associated with slightly higher rankings, but the effect was small compared to content and backlinks.
- **Backlinko (2024):** Analysis of 11.8M Google search results found that page speed (as measured by time to fully load) had a statistically significant but relatively weak correlation with rankings. Content length, domain authority, and backlinks were stronger predictors.
- **Portent (multiple studies):** Found that each additional second of load time reduces conversions by 4.42%. While not directly about rankings, reduced engagement metrics (higher bounce rate, shorter dwell time) can indirectly signal lower quality to Google.

#### The Practical Truth

Page speed's ranking impact is:
- **Small but real** for organic rankings.
- **Significant for user experience metrics** (bounce rate, conversion rate, engagement), which indirectly affect rankings.
- **A qualifying factor, not a competitive advantage.** Being fast won't make you rank #1, but being slow can keep you from ranking well.
- **A bigger factor in competitive niches** where content quality is comparable across top results.
- **Critical for AI Overviews** — Google selects AIO sources from pages it can efficiently process.

### 3.2 Mobile Speed vs Desktop Speed

#### Which Matters More for Ranking?

**Mobile speed is definitively more important for ranking.** Here's why:

1. **Mobile-first indexing** — Google uses the mobile version of your site for all indexing and ranking. Desktop performance is irrelevant for how Google evaluates your site.

2. **CWV data source** — Core Web Vitals in Google Search Console are reported based on the mobile field data from CrUX (Chrome User Experience Report). Mobile CWV passes/failures are what Google uses for ranking.

3. **User behavior** — Over 60% of Google searches are on mobile devices (varies by industry, higher for consumer, lower for B2B developer tools). Even for developer tools, mobile traffic is growing.

4. **Mobile conditions are harder** — Mobile devices have slower CPUs, less RAM, and often use cellular networks. A page that passes CWV on desktop may fail on mobile.

#### Practical Implications

- **Test on real mobile devices,** not just desktop Chrome DevTools with throttling.
- **Use Lighthouse with mobile preset** (the default).
- **CrUX data** (real-world Chrome user data) is what Google actually uses — lab data is for diagnostics, field data is what counts.
- **Desktop CWV still matters for user experience** — even if Google primarily uses mobile data, desktop users leaving due to poor performance hurts engagement metrics.
- **For developer tools:** Your audience may skew heavily desktop. But Google still ranks based on mobile performance. Optimize for mobile even if 80% of your users are on desktop.

### 3.3 Performance Budgets

#### What Is a Performance Budget?

A performance budget is a set of limits on performance metrics that your site must not exceed. It's a tool for maintaining speed as features are added.

#### How to Set Performance Budgets

**Step 1: Baseline from competitors**
- Measure the performance of the top 3-5 competitors in your space.
- Your budget should aim to be 20% faster than the median competitor.

**Step 2: Set metric-based budgets**

| Metric | Budget Target | Rationale |
|--------|--------------|-----------|
| **LCP** | ≤ 2.0s (mobile) | Below Google's 2.5s "Good" threshold with margin |
| **INP** | ≤ 150ms | Below Google's 200ms "Good" threshold with margin |
| **CLS** | ≤ 0.05 | Below Google's 0.1 "Good" threshold with margin |
| **TTFB** | ≤ 600ms | Below Google's 800ms recommendation with margin |
| **FCP** | ≤ 1.5s | Below Google's 1.8s "Good" threshold |
| **Total page weight** | ≤ 500KB (compressed) | For documentation sites. ≤ 1MB for complex apps |
| **JavaScript** | ≤ 200KB (compressed) | Main thread execution time is the biggest perf killer |
| **Images** | ≤ 300KB total above fold | Largest contributor to LCP |
| **Web fonts** | ≤ 100KB | Maximum 2 font families, subset characters |
| **Third-party scripts** | ≤ 100KB (compressed) | Analytics, chat widgets, etc. |
| **HTTP requests** | ≤ 30 on initial load | Fewer requests = faster load with HTTP/2 |

**Step 3: Enforce budgets in CI/CD**
- Use Lighthouse CI to fail builds that exceed budgets.
- Use bundlesize or size-limit to track JavaScript bundle sizes.
- Use SpeedCurve or Calibre for continuous monitoring.

#### Tools for Performance Budgets

| Tool | Purpose | Integration |
|------|---------|-------------|
| **Lighthouse CI** | Automated Lighthouse runs in CI | GitHub Actions, GitLab CI, Jenkins |
| **size-limit** | JavaScript bundle size monitoring | npm, CI/CD |
| **bundlewatch** | Bundle size change detection | GitHub PR checks |
| **SpeedCurve** | Continuous performance monitoring | Alerting, dashboards |
| **Calibre** | Performance monitoring platform | CI/CD integration |
| **WebPageTest** | Detailed performance testing | API for automation |
| **CrUX Dashboard** | Real-world Chrome user data | Looker Studio |
| **PageSpeed Insights API** | Programmatic Lighthouse + CrUX | API integration |

### 3.4 Image Optimization

#### Format Comparison

| Format | Compression vs JPEG | Browser Support | Best For |
|--------|-------------------|-----------------|----------|
| **JPEG** | Baseline | Universal | Photos (fallback) |
| **WebP** | 26-34% smaller | 97%+ (all modern browsers) | Photos, general use |
| **AVIF** | 50% smaller | ~92% (not in Edge legacy, older Safari) | Photos, maximum compression |
| **PNG** | Larger for photos, good for graphics | Universal | Screenshots, graphics with text, transparency |
| **SVG** | N/A (vector) | Universal | Icons, logos, illustrations |

#### Implementation Strategy

```html
<picture>
  <source srcset="image.avif" type="image/avif">
  <source srcset="image.webp" type="image/webp">
  <img src="image.jpg" alt="Description" width="800" height="600" loading="lazy">
</picture>
```

#### Impact on LCP

Images are the LCP element on approximately **70-80% of web pages.** Image optimization is therefore the single most impactful technique for improving LCP.

**Specific techniques and their impact:**

| Technique | Typical LCP Improvement |
|-----------|------------------------|
| Switch from JPEG to WebP | 200-500ms |
| Switch from JPEG to AVIF | 400-800ms |
| Add `fetchpriority="high"` to LCP image | 100-400ms |
| Preload LCP image | 200-600ms |
| Serve correctly sized images (srcset) | 100-1000ms (depends on over-sizing) |
| Enable CDN for images | 100-500ms (depends on geography) |
| Remove lazy-loading from LCP image | 200-500ms |

#### Responsive Images

```html
<img
  srcset="image-400.webp 400w,
          image-800.webp 800w,
          image-1200.webp 1200w,
          image-1600.webp 1600w"
  sizes="(max-width: 600px) 400px,
         (max-width: 1000px) 800px,
         1200px"
  src="image-800.webp"
  alt="Description"
  width="1200"
  height="800"
  loading="lazy"
>
```

#### Lazy Loading

- Use native `loading="lazy"` — supported by all modern browsers.
- **NEVER lazy-load the LCP image** (above-the-fold hero image).
- **NEVER lazy-load images in the initial viewport.**
- Use `loading="eager"` (the default) for above-the-fold images.
- For below-the-fold images, `loading="lazy"` defers fetching until the image is near the viewport (typically within ~1250-2500px on desktop, ~1250px on mobile).

### 3.5 JavaScript Impact

#### Impact on Crawling

- Googlebot renders JavaScript but with a **delay.** Initial crawling gets the raw HTML. Rendering (executing JS) happens in a second pass that may be delayed by seconds, hours, or even days.
- Content rendered only via client-side JavaScript may not be indexed for days after the page is first crawled.
- Google's rendering budget per site is limited. Sites with complex JavaScript may not have all pages fully rendered.
- Other search engines (Bing, Yandex) have more limited JavaScript rendering capabilities.
- AI crawlers (GPTBot, ClaudeBot) typically do NOT render JavaScript — they process raw HTML only.

#### Impact on Core Web Vitals

**LCP:**
- JavaScript render-blocking delays LCP. Every KB of synchronous JS in `<head>` delays when content appears.
- Client-side rendering (React, Vue, Angular without SSR) means the LCP element doesn't exist until JS executes, downloads data, and renders the component.
- Impact: Heavy JS can add 1-4 seconds to LCP.

**INP:**
- JavaScript is the #1 cause of poor INP scores.
- Long tasks (> 50ms) on the main thread block user interactions.
- Third-party scripts (analytics, chat widgets, A/B testing tools) are the most common culprits.
- Impact: A single unoptimized event handler can push INP above 500ms.

**CLS:**
- JavaScript that dynamically inserts content above the fold causes layout shifts.
- Client-side rendering without placeholder/skeleton screens causes shifts.
- Late-loading ad scripts are a major CLS source.

#### JavaScript Budget

For a well-performing site:
- **< 200KB compressed JavaScript** for initial page load (total, including third-party).
- **< 100KB compressed** for your first-party application code.
- **< 1 second total JavaScript execution time** on a mid-range mobile device.

**What 200KB of compressed JS means:**
- Approximately 700KB-1MB uncompressed.
- Approximately 1-2 seconds of parse/compile/execute time on a Moto G Power (representative mid-range device).

#### Techniques to Reduce JavaScript Impact

1. **Code splitting** — Load only the JS needed for the current route. Dynamic `import()` for below-the-fold features.
2. **Tree shaking** — Ensure your bundler eliminates dead code. Requires ES modules (`import`/`export`).
3. **Audit third-party scripts** — Each third-party script you add has a cost. Common culprits:
   - Google Analytics: ~45KB compressed.
   - Chat widgets (Intercom, Drift): 200-400KB compressed.
   - A/B testing (Optimizely): 60-100KB compressed.
   - Social embeds (Twitter, Instagram): 100-300KB each.
   - Consider: Is each script worth its performance cost?
4. **Defer non-critical JS** — Use `async` or `defer` attributes. `defer` maintains execution order; `async` doesn't.
5. **Use Web Workers** — Move computation off the main thread. Available for data processing, crypto, image manipulation.
6. **Server-Side Rendering (SSR)** — Send fully rendered HTML from the server. JavaScript hydrates interactivity afterward. Critical for LCP and SEO.
7. **Static Site Generation (SSG)** — Pre-render pages at build time. Fastest possible TTFB for static content. Ideal for documentation sites.
8. **Progressive hydration / partial hydration** — Only hydrate interactive components. Frameworks like Astro, Qwik, and Fresh support this natively.
9. **Remove unused polyfills** — Modern browsers don't need polyfills for ES6+ features. Use `browserslist` and `useBuiltIns: 'usage'` to only include needed polyfills.

### 3.6 CDN and Edge Computing

#### Impact on TTFB

TTFB (Time to First Byte) is directly impacted by the physical distance between the user and the server. A CDN eliminates this variable.

**Typical TTFB improvements:**

| Scenario | Typical TTFB |
|----------|-------------|
| No CDN, single origin server (same continent as user) | 200-600ms |
| No CDN, single origin server (cross-continent) | 500-2000ms |
| CDN (static assets only) | 50-200ms for cached assets |
| CDN (full page caching) | 20-100ms for cached pages |
| Edge computing (SSR at the edge) | 50-150ms regardless of geography |

**A CDN can reduce TTFB by 100-1500ms** depending on the user's distance from the origin server.

#### CDN Options for Developer Tools

| CDN | Strengths | Best For |
|-----|-----------|----------|
| **Cloudflare** | Free tier, Workers (edge compute), massive network | General purpose, documentation sites |
| **Vercel Edge Network** | Integrated with Next.js, automatic edge caching | Next.js/React apps |
| **Netlify CDN** | Integrated with Netlify platform, edge functions | Static sites, Jamstack |
| **AWS CloudFront** | Deep AWS integration, Lambda@Edge | AWS-hosted applications |
| **Fastly** | Real-time purging, VCL/Compute@Edge | High-traffic, dynamic content |
| **Bunny CDN** | Low cost, simple, fast | Budget-conscious, smaller sites |

#### Edge Computing Patterns

- **Edge SSR** — Render pages at the CDN edge, close to the user. Frameworks: Next.js (Vercel), Remix (Cloudflare Workers), SvelteKit.
- **Edge API routes** — Run lightweight API endpoints at the edge. Good for authentication, redirects, A/B testing.
- **Edge caching with stale-while-revalidate** — Serve cached content immediately, refresh in the background.
  ```
  Cache-Control: public, s-maxage=3600, stale-while-revalidate=86400
  ```
  This serves cached content for 1 hour, then serves stale content for up to 24 hours while refreshing in the background. Users always get a fast response.

#### Geographic Performance

For a global audience, performance can vary dramatically without a CDN:

- **Same city as server:** TTFB 10-50ms
- **Same country:** TTFB 50-150ms
- **Same continent:** TTFB 100-300ms
- **Cross-continent:** TTFB 200-600ms
- **Opposite side of globe:** TTFB 300-1000ms

A CDN with global PoPs (Points of Presence) normalizes this to 20-100ms regardless of location.

### 3.7 Font Loading

#### Impact on CLS and FCP

Web fonts cause two distinct problems:

1. **FOIT (Flash of Invisible Text):** Browser hides text until the font loads. Delays FCP. If the font takes 2 seconds to load, text is invisible for 2 seconds.

2. **FOUT (Flash of Unstyled Text):** Browser shows fallback font, then swaps to web font when loaded. Causes CLS because the web font typically has different metrics (line height, character width) than the fallback.

#### font-display Strategies

```css
@font-face {
  font-family: 'CustomFont';
  src: url('font.woff2') format('woff2');
  font-display: swap; /* or optional, fallback, block, auto */
}
```

| Value | Behavior | FCP Impact | CLS Impact | Best For |
|-------|----------|-----------|------------|----------|
| `swap` | Show fallback immediately, swap when loaded | None (best) | High (worst) | Body text when you want fast FCP |
| `optional` | Show fallback, only use web font if already cached | None (best) | None (best, if combined with size-adjust) | Best overall for performance |
| `fallback` | 100ms invisible, then fallback, swap within 3s | Minimal | Moderate | Compromise approach |
| `block` | Up to 3s invisible, then fallback | High (worst) | Low | Icon fonts, display text |
| `auto` | Browser decides (usually `block`) | Varies | Varies | Not recommended |

#### Recommended Approach

**For maximum performance (recommended for most sites):**

```css
@font-face {
  font-family: 'CustomFont';
  src: url('font.woff2') format('woff2');
  font-display: optional;
  /* Font only used if already cached; no CLS, no FOIT */
}
```

**For reliable web font usage with minimal CLS:**

```css
/* Define a size-adjusted fallback */
@font-face {
  font-family: 'CustomFont-Fallback';
  src: local('Arial');
  ascent-override: 90%;
  descent-override: 20%;
  line-gap-override: 0%;
  size-adjust: 105%;
}

@font-face {
  font-family: 'CustomFont';
  src: url('font.woff2') format('woff2');
  font-display: swap;
}

body {
  font-family: 'CustomFont', 'CustomFont-Fallback', Arial, sans-serif;
}
```

The size-adjusted fallback ensures the fallback font occupies the same space as the web font, eliminating CLS when the swap occurs.

#### Font Performance Techniques

1. **Use WOFF2 only** — WOFF2 has 97%+ browser support and ~30% smaller than WOFF. No need for WOFF, TTF, or EOT fallbacks in 2025.

2. **Subset fonts** — Remove characters you don't use. A full Google Font can be 100-200KB. A Latin-only subset is often 15-30KB.
   ```css
   @font-face {
     font-family: 'CustomFont';
     src: url('font-latin.woff2') format('woff2');
     unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F;
   }
   ```

3. **Preload critical fonts:**
   ```html
   <link rel="preload" as="font" type="font/woff2" href="font.woff2" crossorigin>
   ```
   Only preload 1-2 fonts that are used above the fold. Over-preloading delays other critical resources.

4. **Self-host fonts** — Avoid Google Fonts CDN. Self-hosting eliminates a DNS lookup, a TLS handshake, and a connection to a third-party origin. It also avoids being blocked by cookie consent banners or privacy-focused browsers.

5. **Limit font families and weights:**
   - Maximum 2 font families (one for headings, one for body — or just one for everything).
   - Maximum 3-4 weights/styles (regular, bold, italic, bold italic).
   - Each additional font file is another HTTP request and more bytes.
   - Variable fonts can replace multiple weight files with a single file.

6. **Variable fonts:**
   - Single file contains all weights/widths/styles.
   - Often smaller than downloading 4+ separate weight files.
   - Supported by all modern browsers.
   ```css
   @font-face {
     font-family: 'CustomFont';
     src: url('font-variable.woff2') format('woff2-variations');
     font-weight: 100 900;
     font-display: optional;
   }
   ```

---

## Quick Reference: Key Thresholds

| Metric | Good | Needs Improvement | Poor |
|--------|------|-------------------|------|
| LCP | ≤ 2.5s | 2.5-4.0s | > 4.0s |
| INP | ≤ 200ms | 200-500ms | > 500ms |
| CLS | ≤ 0.1 | 0.1-0.25 | > 0.25 |
| TTFB | ≤ 800ms | 800-1800ms | > 1800ms |
| FCP | ≤ 1.8s | 1.8-3.0s | > 3.0s |
| TBT | ≤ 200ms | 200-600ms | > 600ms |
| JS Bundle | ≤ 200KB gzip | 200-400KB | > 400KB |
| Total Page Weight | ≤ 500KB | 500KB-1.5MB | > 1.5MB |
| DOM Nodes | ≤ 1,400 | 1,400-3,000 | > 3,000 |
| HTTP Requests | ≤ 30 | 30-60 | > 60 |
| Font Files | ≤ 4 | 4-6 | > 6 |

---

## Quick Reference: SEO vs GEO Comparison

| Dimension | SEO (Traditional) | GEO (AI/LLM) |
|-----------|-------------------|---------------|
| **Primary signal** | Backlinks + content | Content quality + brand mentions |
| **Ranking mechanism** | Algorithmic ranking of pages | Probabilistic selection from training data + RAG |
| **Keyword importance** | High (specific placement matters) | Moderate (semantic understanding, not keyword matching) |
| **Page speed** | Ranking factor (tiebreaker) | Not directly relevant |
| **Structured data** | Rich results + entity understanding | Content type understanding |
| **Content format** | Varies by intent | Concise, self-contained answers preferred |
| **Link building** | Critical | Less important; brand mentions matter more |
| **Freshness** | Important for time-sensitive queries | Training data has lag; RAG-based systems are current |
| **Measurement** | Search Console, rank tracking | No standardized tools yet; emerging analytics |
| **Control** | robots.txt, meta robots, canonicals | llms.txt, robots.txt (AI crawlers), content optimization |
| **Visibility timeline** | Days to months for ranking changes | Months to years for training data; immediate for RAG |
| **ROI visibility** | Clear (traffic, rankings, conversions) | Unclear (hard to attribute AI mentions to outcomes) |

---

## Action Items for Developer Tools / SaaS

### Immediate (Week 1-2)
1. Audit Core Web Vitals with PageSpeed Insights and fix any "Poor" metrics.
2. Add `FAQPage` and `SoftwareApplication` structured data.
3. Create `llms.txt` and `llms-full.txt` files.
4. Ensure all images use WebP/AVIF with proper `width`, `height`, `alt`, and `loading` attributes.
5. Configure AI crawler access in robots.txt (recommend: allow).
6. Self-host fonts with `font-display: optional` or size-adjusted `swap`.

### Short-term (Month 1-2)
7. Implement content cluster strategy around core topics.
8. Restructure documentation to answer questions directly in first 40-80 words.
9. Add statistics, benchmarks, and data points to key content.
10. Set up performance budgets in CI/CD.
11. Audit and reduce JavaScript bundle sizes.
12. Implement CDN with edge caching.

### Ongoing
13. Publish content consistently to build topical authority.
14. Monitor Core Web Vitals in Search Console monthly.
15. Build backlinks through community presence, open source, and content.
16. Update key content quarterly for freshness signals.
17. Track brand mentions across developer communities.
18. Monitor AI citation (manually check ChatGPT, Perplexity, Google AI Overviews for your brand/topic queries).
