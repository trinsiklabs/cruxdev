# Site Registration, Indexing & AI Discovery -- Comprehensive Reference

**Created:** 2026-03-26
**Methodology:** 5-pass convergence research (broad landscape, authoritative, practitioner, contrarian, primary sources)
**Scope:** How to get a new website registered, indexed, and crawled by search engines AND AI systems.
**Complements:** `SEO_AND_GEO_REFERENCE.md` (optimization), `GROWTH_STRATEGY.md` (distribution)

---

## Table of Contents

1. [Search Engines -- Registration & Indexing](#1-search-engines----registration--indexing)
   1. [Google](#11-google)
   2. [Bing](#12-bing)
   3. [DuckDuckGo](#13-duckduckgo)
   4. [Yandex](#14-yandex)
   5. [Baidu](#15-baidu)
2. [AI Systems -- Discovery & Citation](#2-ai-systems----discovery--citation)
   1. [OpenAI (ChatGPT / SearchGPT)](#21-openai-chatgpt--searchgpt)
   2. [Anthropic (Claude)](#22-anthropic-claude)
   3. [Perplexity](#23-perplexity)
   4. [Google (Gemini / AI Overviews)](#24-google-gemini--ai-overviews)
3. [Protocols & Standards](#3-protocols--standards)
   1. [sitemap.xml](#31-sitemapxml)
   2. [robots.txt](#32-robotstxt)
   3. [IndexNow](#33-indexnow)
   4. [llms.txt](#34-llmstxt)
   5. [AGENTS.md](#35-agentsmd)
   6. [Schema.org Structured Data](#36-schemaorg-structured-data)
4. [Registration Checklist](#4-registration-checklist)
5. [Timeline -- How Long Each Takes](#5-timeline----how-long-each-takes)
6. [Automation -- What the Growth Engine Can Do](#6-automation----what-the-growth-engine-can-do)
7. [Anti-Patterns -- What NOT to Do](#7-anti-patterns----what-not-to-do)
8. [Sources](#8-sources)

---

## 1. Search Engines -- Registration & Indexing

### 1.1 Google

**Registration tool:** [Google Search Console](https://search.google.com/search-console) (GSC)

**Setup steps:**
1. Go to GSC, sign in with Google account, click "Add Property"
2. Choose **Domain property** (recommended) -- covers all subdomains and protocols
3. Verify ownership via one of: DNS record (recommended), HTML file upload, HTML meta tag, Google Analytics, or Google Tag Manager
4. Submit sitemap: GSC left menu > Indexing > Sitemaps > enter sitemap URL > Submit

**URL-level submission:**
- URL Inspection Tool in GSC: paste URL > "Request Indexing"
- Google usually crawls within 24 hours of request

**Google Indexing API (programmatic):**
- Endpoint: `https://indexing.googleapis.com/v3/urlNotifications:publish`
- Content-Type: `application/json`
- Supports: update URL, remove URL, get status, batch (up to 100 calls)
- Prerequisites: enable API, create service account, verify in GSC, get access token
- Default quota: 200 requests (requires approval for more)
- **Important limitation:** Officially intended for JobPosting and BroadcastEvent schema only. Practitioners report it works for other content but Google may restrict this.
- Response: HTTP 200 means Google *may* try to recrawl soon (not guaranteed)

**Key facts:**
- Google will eventually find any site linked from another indexed site
- GSC is not strictly *required* but dramatically speeds up discovery and provides diagnostic data
- Submitting a sitemap only needs to happen once; Google re-fetches it automatically
- Referencing sitemap in robots.txt is a best practice even if also submitted via GSC

**Automatable:** Yes. Sitemap submission is one-time. Indexing API can be called programmatically on deploy. robots.txt sitemap reference is static config.

### 1.2 Bing

**Registration tool:** [Bing Webmaster Tools](https://www.bing.com/webmasters)

**Setup steps:**
1. Go to Bing Webmaster Tools, sign in (Microsoft, Google, or Facebook account)
2. **Fastest method:** Import from Google Search Console (auto-imports sites, verification, sitemaps)
3. **Manual method:** Add site URL, verify via XML file upload to root directory, or CNAME/meta tag
4. Submit sitemap: left nav > Sitemaps > Submit sitemap

**Key facts:**
- Bing also powers Yahoo and partially powers DuckDuckGo
- Bing supports IndexNow protocol for instant URL notification
- Bing Webmaster Tools URL Submission API available for programmatic submission

**Automatable:** Yes. Import from GSC is one-time. IndexNow pings can be automated on every deploy/publish.

### 1.3 DuckDuckGo

**Registration tool:** None. DuckDuckGo has no webmaster tools or URL submission interface.

**How to appear:**
- DuckDuckGo sources results primarily from Bing's index
- Submit your site to Bing Webmaster Tools and DuckDuckGo will find it
- Also uses Apple Maps (for local), Wikipedia, and its own crawler (DuckDuckBot)

**Automatable:** No action needed beyond Bing registration.

### 1.4 Yandex

**Registration tool:** [Yandex Webmaster](https://webmaster.yandex.com)

**Setup steps:**
1. Sign up for Yandex account
2. Add site in Yandex Webmaster
3. Verify ownership (similar methods to Google: DNS, HTML file, meta tag)
4. Submit sitemap

**Key facts:**
- Important for Russian and CIS markets
- Supports IndexNow protocol
- Interface available in English

**Automatable:** Yes. One-time setup; IndexNow handles ongoing notifications.

### 1.5 Baidu

**Registration tool:** [Baidu Webmaster Tools](https://ziyuan.baidu.com)

**Setup steps:**
1. Register on Baidu Webmaster Tools
2. Submit sitemap
3. Monitor index status

**Key facts:**
- **Mandarin-only interface** -- requires Chinese language proficiency
- **Requires mainland China mobile phone number** for registration
- Important only if targeting Chinese market
- Baidu has its own crawling infrastructure separate from others

**Automatable:** Registration is manual and has significant barriers for non-Chinese sites. Skip unless specifically targeting China.

---

## 2. AI Systems -- Discovery & Citation

### 2.1 OpenAI (ChatGPT / SearchGPT)

**Crawler user agents:**

| Bot | Purpose | Training? | Controllable? |
|-----|---------|-----------|---------------|
| `GPTBot` | Crawls content for AI model training | Yes | Block in robots.txt |
| `OAI-SearchBot` | Indexes for SearchGPT/ChatGPT search results | No | Block in robots.txt |
| `ChatGPT-User` | Fetches pages when a user asks ChatGPT to visit a URL | No | Block in robots.txt |

**How to get discovered:**
- Allow `OAI-SearchBot` and `ChatGPT-User` in robots.txt (block `GPTBot` if you don't want training)
- ChatGPT search relies on **Bing's API** -- being indexed in Bing is critical
- Structured content with clear answers helps citation
- No official submission portal exists

**robots.txt example:**
```
User-agent: OAI-SearchBot
Allow: /

User-agent: ChatGPT-User
Allow: /

User-agent: GPTBot
Disallow: /    # optional: block training while allowing search
```

**Automatable:** robots.txt is static config. Being in Bing index is the main lever.

### 2.2 Anthropic (Claude)

**Crawler user agents:**

| Bot | Purpose | Training? | Controllable? |
|-----|---------|-----------|---------------|
| `ClaudeBot` | Crawls content for model training | Yes | Block in robots.txt |
| `Claude-User` | Fetches pages when Claude users share URLs | No | Block in robots.txt |
| `Claude-SearchBot` | Indexes content for search results | No | Block in robots.txt |

**How to get discovered:**
- Allow `Claude-SearchBot` and `Claude-User` in robots.txt
- Anthropic bots honor robots.txt directives and do not bypass CAPTCHAs
- Anthropic does not publish IP ranges (uses service provider IPs)
- No official submission portal exists

**robots.txt example:**
```
User-agent: Claude-SearchBot
Allow: /

User-agent: Claude-User
Allow: /

User-agent: ClaudeBot
Disallow: /    # optional: block training while allowing search
```

**Automatable:** robots.txt is static config.

### 2.3 Perplexity

**Crawler user agents:**

| Bot | Purpose |
|-----|---------|
| `PerplexityBot/1.0` | General web crawling for search index |
| `Perplexity/1.0` | AI assistant browsing (user-initiated) |

**How to get discovered:**
- Allow `PerplexityBot` in robots.txt
- Perplexity values fast, mobile-optimized sites
- Content authority matters -- backlinks help
- No official submission portal

**Controversy:** Perplexity AI has been caught using undeclared crawlers to access content explicitly blocked in robots.txt. This is a known compliance issue in the industry.

**Automatable:** robots.txt is static config. Site speed optimization is a build concern.

### 2.4 Google (Gemini / AI Overviews)

**Crawler user agents:**

| Bot | Purpose | Blocking effect |
|-----|---------|-----------------|
| `Googlebot` | Main search crawling and indexing | Blocks all Google Search |
| `Google-Extended` | Gemini and AI Overviews training | Blocks AI features only, not search ranking |

**How to get discovered:**
- Being indexed by Google (via Googlebot) is the baseline
- Allowing `Google-Extended` permits use in Gemini and AI Overviews
- Blocking `Google-Extended` does NOT affect search rankings

**Automatable:** robots.txt is static config.

---

## 3. Protocols & Standards

### 3.1 sitemap.xml

**What it does:** Machine-readable list of all important URLs on your site. Tells search engines what to crawl.

**Format:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://www.example.com/page.html</loc>
    <lastmod>2026-03-26</lastmod>
  </url>
</urlset>
```

**Required elements:** `<urlset>`, `<url>`, `<loc>` (fully qualified absolute URL)
**Optional elements:** `<lastmod>` (useful, must be accurate), `<changefreq>` (ignored by Google), `<priority>` (ignored by Google)
**Limits:** 50,000 URLs or 50MB uncompressed per file. Use sitemap index for larger sites.
**Encoding:** UTF-8 required. XML entity escaping required.

**Submission methods:**
1. Google Search Console > Sitemaps
2. Bing Webmaster Tools > Sitemaps
3. Reference in robots.txt: `Sitemap: https://example.com/sitemap.xml`
4. Google Search Console API (programmatic)

**Key fact:** Submit once. Search engines re-fetch automatically. Referencing in robots.txt is belt-and-suspenders.

**Automatable:** Yes. Generate sitemap at build time. Reference in robots.txt. Submit to GSC/Bing once. Static site generators (Astro, Next.js, Hugo) all have sitemap plugins.

### 3.2 robots.txt

**What it does:** Tells crawlers which paths they may or may not access. Advisory, not enforced.

**Location:** Must be at site root: `https://example.com/robots.txt`

**Key directives:**
```
User-agent: *
Allow: /

Sitemap: https://example.com/sitemap.xml

# AI Search bots - allow
User-agent: OAI-SearchBot
Allow: /

User-agent: Claude-SearchBot
Allow: /

User-agent: PerplexityBot
Allow: /

# AI Training bots - block (optional)
User-agent: GPTBot
Disallow: /

User-agent: ClaudeBot
Disallow: /

User-agent: Google-Extended
Disallow: /
```

**Critical mistakes to avoid:**
- Leaving `Disallow: /` from development environment
- Blocking AI search bots when you actually want AI visibility
- Putting indexable URLs in robots.txt Disallow (use noindex meta tag instead)
- Overlap between sitemap and disallowed paths

**Automatable:** Yes. Static file, generated or committed in repo.

### 3.3 IndexNow

**What it does:** Instant notification to participating search engines when you add, update, or delete content.

**Supported search engines:** Bing, Yandex, Naver, Seznam, Yep.
**NOT supported:** Google (still not adopted as of March 2026 despite testing since October 2021).

**How it works:**
1. Generate API key (8-128 hex characters)
2. Host key file at site root: `https://example.com/{key}.txt`
3. Ping the API on content change

**Single URL (GET):**
```
GET https://api.indexnow.org/indexnow?url=https://example.com/new-page&key=YOUR_KEY
```

**Bulk URLs (POST):**
```json
POST https://api.indexnow.org/indexnow
Content-Type: application/json; charset=utf-8

{
  "host": "example.com",
  "key": "YOUR_KEY",
  "urlList": [
    "https://example.com/page1",
    "https://example.com/page2"
  ]
}
```

**Limits:** Up to 10,000 URLs per batch submission.

**Response codes:**
| Code | Meaning |
|------|---------|
| 200 | OK -- received |
| 202 | Accepted, pending key validation |
| 400 | Bad request |
| 403 | Invalid/missing key |
| 422 | URL/host mismatch |
| 429 | Rate limited |

**Key fact:** Submitting to any participating engine auto-shares with all others.

**Automatable:** Yes. Ideal for CI/CD pipeline integration. Ping on every deploy. Many platforms (Wix, Shopify, WordPress via Yoast/RankMath) have built-in support.

### 3.4 llms.txt

**What it does:** Proposed standard for giving AI systems a plain-text, Markdown-formatted summary of your site's content and structure. Like a sitemap but for LLMs.

**Location:** `https://example.com/llms.txt`

**Specification (from llmstxt.org):**

**Required:**
- H1 heading with the project/site name

**Recommended:**
- Blockquote with brief summary
- Markdown sections with detailed info
- H2-delimited "file lists" sections with URLs: `- [Page Name](url): Description`

**Optional:**
- An "Optional" section for secondary info that can be skipped for shorter context
- `.md` versions of pages at same URL + `.md` extension

**Example:**
```markdown
# Crux

> Crux is the AI operating system for software projects. It provides memory, context, and autonomous workflows for AI-assisted development.

## Docs

- [Getting Started](https://crux.dev/docs/getting-started): Installation and first project setup
- [Architecture](https://crux.dev/docs/architecture): How Crux works under the hood
- [API Reference](https://crux.dev/docs/api): Complete API documentation

## Optional

- [Changelog](https://crux.dev/changelog): Version history
- [Blog](https://crux.dev/blog): Technical articles
```

**Adoption status (March 2026):** Moderate. Some major publishers and organizations have adopted it. However:
- No clear evidence that major AI companies follow llms.txt rules
- Google has explicitly said it does not support llms.txt
- Compliance across the industry is inconsistent
- Still worth implementing as a low-cost, high-optionality bet

**Directories:** llmstxt.site, directory.llmstxt.cloud

**Automatable:** Yes. Generate at build time from site structure. Static file.

### 3.5 AGENTS.md

**What it does:** A standard file giving AI coding agents project-specific guidance. "A README for agents."

**Origin:** Created by OpenAI (August 2025), now stewarded by the Agentic AI Foundation (AAIF) under the Linux Foundation (December 2025).

**Adoption:** 60,000+ open source projects. Supported by Amp, Codex, Cursor, Devin, Factory, Gemini CLI, GitHub Copilot, Jules, VS Code.

**Relevance to website discovery:** Indirect. Primarily for code repositories, not websites. However, for developer tool products, having AGENTS.md in your open-source repo increases AI agent awareness of your project.

**Automatable:** Yes. Static Markdown file in repo root.

### 3.6 Schema.org Structured Data

**What it does:** Machine-readable metadata that tells search engines and AI what your content *means*, not just what it says.

**Format:** JSON-LD (recommended), embedded in HTML `<head>`.

**Key schema types for developer tools:**
- `Organization` -- company info
- `SoftwareApplication` -- the product itself
- `Article` / `BlogPosting` -- blog content
- `FAQPage` -- FAQ content (high value for AI citation)
- `HowTo` -- tutorial content
- `BreadcrumbList` -- navigation structure

**Benefits:**
- Rich snippets in search results (star ratings, FAQs, breadcrumbs)
- 20-40% CTR improvement from rich results
- Helps AI systems extract structured answers
- Indirect ranking benefit (better content comprehension)

**Automatable:** Yes. Generate JSON-LD at build time. Most frameworks have plugins.

---

## 4. Registration Checklist

Ordered by priority. Do these when launching a new site.

### Day 0: Before Launch

- [ ] Remove all `noindex` meta tags and `Disallow: /` from robots.txt
- [ ] Ensure site loads over HTTPS
- [ ] Ensure mobile-responsive design
- [ ] Verify no JavaScript-only rendering blocking crawlers (use SSR/SSG)

### Day 0: At Launch

- [ ] **robots.txt** -- Deploy with:
  - Allow all search engine bots
  - Allow AI search bots (OAI-SearchBot, Claude-SearchBot, PerplexityBot)
  - Optionally block AI training bots (GPTBot, ClaudeBot, Google-Extended)
  - Sitemap reference: `Sitemap: https://yourdomain.com/sitemap.xml`
- [ ] **sitemap.xml** -- Generate and deploy at site root
- [ ] **llms.txt** -- Create and deploy at site root
- [ ] **Schema.org** -- Add JSON-LD structured data to all pages (Organization, SoftwareApplication, Article, FAQPage as applicable)

### Day 1: Search Engine Registration

- [ ] **Google Search Console** -- Add property (Domain type), verify via DNS, submit sitemap
- [ ] **Bing Webmaster Tools** -- Import from GSC (or add manually), verify, submit sitemap
- [ ] **IndexNow** -- Generate API key, host key file, set up automated ping on deploy
- [ ] **Yandex Webmaster** -- Add site, verify, submit sitemap (if targeting Russian/CIS markets)

### Day 1: Request Indexing

- [ ] **GSC URL Inspection Tool** -- Request indexing for homepage and top 5-10 pages
- [ ] **Google Indexing API** -- Set up service account for programmatic submission (optional, for high-frequency publishing)

### Week 1: Verify & Monitor

- [ ] Check GSC Page Indexing Report for errors
- [ ] Check Bing Webmaster Tools for crawl issues
- [ ] Verify robots.txt is correctly served (use GSC robots.txt tester)
- [ ] Verify sitemap is being fetched (check GSC Sitemaps report)
- [ ] Test site with `site:yourdomain.com` in Google and Bing

### Week 2-4: Content & Links

- [ ] Publish initial content (blog posts, docs, landing pages)
- [ ] Build initial backlinks (GitHub, Product Hunt, social profiles, directories)
- [ ] Set up Google Analytics 4 (GA4) for traffic monitoring
- [ ] Monitor Core Web Vitals in GSC

### Month 2-3: AI Visibility

- [ ] Check if site appears in ChatGPT, Perplexity, Claude responses
- [ ] Optimize content structure for AI citation (answer-first format, FAQ sections)
- [ ] Add statistics with source citations every 150-200 words in key content
- [ ] Expand key pages to 1500+ words (doubles AI citation probability)

---

## 5. Timeline -- How Long Each Takes

| Action | Time to Effect |
|--------|---------------|
| Google indexing (with GSC + sitemap) | 4-14 days for new site |
| Google indexing (with Indexing API) | 24-48 hours |
| Google indexing (without any submission) | 2-6 weeks, possibly months |
| Bing indexing (with Webmaster Tools) | 1-7 days |
| Bing indexing (with IndexNow) | Minutes to hours |
| DuckDuckGo (via Bing) | 1-7 days after Bing indexes |
| Yandex indexing | 1-2 weeks |
| AI citation in ChatGPT/Perplexity/Claude | 2-3 months for initial mentions |
| Significant AI visibility | 6 months of consistent content |
| Individual page indexing (established site) | 24-72 hours |
| Rich snippets appearing after schema added | 1-4 weeks |

**Factors that speed up indexing:**
- High-quality, unique content
- Clean site structure (every page within 3-4 clicks of homepage)
- Backlinks from already-indexed sites
- Frequent content updates
- Fast page load times
- Mobile-first design

**Factors that slow down indexing:**
- Brand new domain with no backlinks
- JavaScript-only rendering without SSR
- Thin or duplicate content
- Crawl errors (404s, 500s)
- robots.txt blocking crawlers
- noindex tags left from development

---

## 6. Automation -- What the Growth Engine Can Do

### Fully Automatable (do on every deploy)

| Action | Implementation | Growth Engine? |
|--------|---------------|----------------|
| Generate sitemap.xml | Build-time plugin (Astro, Next.js, Hugo all support) | Yes |
| Generate robots.txt | Static file or build-time generation | Yes |
| Generate llms.txt | Build-time from site structure | Yes |
| Generate schema.org JSON-LD | Build-time per-page templates | Yes |
| Ping IndexNow | HTTP POST in CI/CD pipeline after deploy | Yes |
| Ping Google (via sitemap in robots.txt) | Automatic once configured | Yes |

### One-Time Setup (manual, then done)

| Action | Notes |
|--------|-------|
| Google Search Console registration | Manual: verify domain ownership |
| Bing Webmaster Tools registration | Manual: import from GSC |
| Yandex Webmaster registration | Manual: if targeting market |
| IndexNow key generation + hosting | Manual: generate once, host file |
| Google Indexing API service account | Manual: GCP console setup |
| GA4 setup | Manual: create property, add tracking |

### Automatable with CI/CD Integration

| Action | Trigger | Implementation |
|--------|---------|---------------|
| IndexNow ping | On deploy | `curl -X POST https://api.indexnow.org/indexnow -H "Content-Type: application/json" -d '{"host":"example.com","key":"KEY","urlList":["urls"]}'` |
| Google Indexing API | On new page publish | Service account + API call |
| Sitemap regeneration | On build | Build plugin |
| Schema validation | On build/PR | Automated testing |

### Sample CI/CD IndexNow Script

```bash
#!/bin/bash
# Post-deploy IndexNow notification
SITE_HOST="example.com"
INDEXNOW_KEY="your-api-key-here"
CHANGED_URLS='["https://example.com/new-page","https://example.com/updated-page"]'

curl -s -X POST "https://api.indexnow.org/indexnow" \
  -H "Content-Type: application/json; charset=utf-8" \
  -d "{\"host\":\"${SITE_HOST}\",\"key\":\"${INDEXNOW_KEY}\",\"urlList\":${CHANGED_URLS}}"
```

---

## 7. Anti-Patterns -- What NOT to Do

### Paid Submission Services
- **"Submit your site to 500 search engines for $99"** -- These are scams. There are only ~5 search engines that matter. Registration is free on all of them.
- Sites like SubmitExpress and similar services provide zero value.

### Link Farms and PBNs
- Buying backlinks from private blog networks (PBNs) can get your site penalized
- Google's algorithms detect and penalize artificial link patterns

### Keyword Stuffing
- Cramming keywords into meta tags, hidden text, or unnatural density hurts rather than helps

### Duplicate Content Syndication
- Publishing identical content across multiple domains dilutes authority and may trigger duplicate content filters

### Blocking AI Crawlers Accidentally
- Many sites block `GPTBot`, `ClaudeBot`, etc. in robots.txt without realizing this makes them invisible to AI search
- Separate training bots from search bots in your robots.txt

### Relying Solely on Manual Submission
- Submitting URLs without fixing underlying technical issues (broken links, slow load, bad structure) will not help
- Manual submission speeds up discovery but does not replace good technical SEO

### Over-Indexing
- Submitting thousands of low-quality pages dilutes crawl budget
- Only index pages that provide value to searchers

### Ignoring Mobile
- Google uses mobile-first indexing. Desktop-only sites will be crawled and ranked based on their mobile version.

### Ignoring Rendering
- Single Page Applications (SPAs) that rely solely on client-side JavaScript will not be properly indexed
- Use SSR (Server-Side Rendering) or SSG (Static Site Generation) for all important pages

---

## 8. Sources

### Primary Documentation
- [Google Search Console](https://search.google.com/search-console/about)
- [Google: Build and Submit a Sitemap](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)
- [Google: Create and Submit robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/create-robots-txt)
- [Google Indexing API](https://developers.google.com/search/apis/indexing-api/v3/using-api)
- [Bing Webmaster Tools](https://www.bing.com/webmasters)
- [IndexNow Documentation](https://www.indexnow.org/documentation)
- [IndexNow FAQ](https://www.indexnow.org/faq)
- [llms.txt Specification](https://llmstxt.org/)
- [AGENTS.md](https://agents.md/)
- [OpenAI Crawlers Overview](https://platform.openai.com/docs/bots)
- [Anthropic Crawler Policy](https://support.claude.com/en/articles/8896518-does-anthropic-crawl-data-from-the-web-and-how-can-site-owners-block-the-crawler)
- [Schema.org Getting Started](https://schema.org/docs/gs.html)
- [Google: Intro to Structured Data](https://developers.google.com/search/docs/appearance/structured-data/intro-structured-data)

### Guides and Analysis
- [How to Get Indexed and Cited in ChatGPT and AI Search](https://prerender.io/blog/how-to-get-indexed-on-ai-platforms/)
- [Agentic AI Optimization (AAIO) in 2026](https://shortlist.io/blog/agentic-ai-optimization/)
- [How to Make Your Site More Crawlable in 2026](https://www.straightnorth.com/blog/how-to-make-your-website-more-crawlable-for-search-engines-and-ai/)
- [The Complete Guide to AI Crawler Management in 2026](https://dev.to/william_geo/the-complete-guide-to-ai-crawler-management-in-2026-6ai)
- [GEO: The 2026 Guide to AI Search Visibility](https://llmrefs.com/generative-engine-optimization)
- [GEO & AEO: How to Get Cited by AI Search in 2026](https://www.thevccorner.com/p/geo-aeo-how-to-rank-when-ai-answers)
- [Anthropic's Three-Bot Framework](https://almcorp.com/blog/anthropic-claude-bots-robots-txt-strategy/)
- [AI Crawlers Ignoring robots.txt](https://www.tomshardware.com/tech-industry/artificial-intelligence/several-ai-companies-said-to-be-ignoring-robots-dot-txt-exclusion-scraping-content-without-permission-report)
- [New AI Web Standards and Scraping Trends in 2026](https://dev.to/astro-official/new-ai-web-standards-and-scraping-trends-in-2026-rethinking-robotstxt-3730)
- [The State of llms.txt in 2026](https://www.aeo.press/ai/the-state-of-llms-txt-in-2026)
- [Kinsta: How to Submit Your Website to Search Engines](https://kinsta.com/blog/submit-website-to-search-engines/)
- [How to Get Found by Claude, Perplexity & ChatGPT](https://www.amivisibleonai.com/blog/complete-guide-ai-seo-2025)
- [Linux Foundation: Agentic AI Foundation Announcement](https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation)
