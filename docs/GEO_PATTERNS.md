# GEO Patterns — Generative Engine Optimization

**Version:** 1.0
**Created:** 2026-03-28
**Scope:** Universal — applies to ANY web property managed by CruxDev. Covers optimization for AI-generated search responses across ChatGPT, Perplexity, Google AI Overviews, Claude, Gemini, and Copilot.
**Derived from:** Princeton GEO research (KDD 2024), Search Engine Land methodology, SurferSEO citation analysis, Profound AI platform studies, and production deployment experience.

This document parallels `DEVELOPMENT_PATTERNS_CRUXDEV.md` (for code), `WEBSITE_PLANNING.md` (for websites), `COMPETITORS_PATTERN.md` (for competitive analysis), and `RESEARCH_PATTERNS.md` (for research). It defines HOW to optimize any web property for AI search visibility to convergence.

---

## Table of Contents

1. [What GEO Is](#1-what-geo-is)
2. [How AI Search Differs from Traditional SEO](#2-how-ai-search-differs-from-traditional-seo)
3. [The Citation Gap — Most AI Sources Are NOT in Google Top 10](#3-the-citation-gap)
4. [The llms.txt Standard](#4-the-llmstxt-standard)
5. [Structured Data That AI Systems Read](#5-structured-data-that-ai-systems-read)
6. [Content Patterns That Get Cited](#6-content-patterns-that-get-cited)
7. [Bing Index Importance](#7-bing-index-importance)
8. [AI Crawler Access Control](#8-ai-crawler-access-control)
9. [Monitoring AI Citations](#9-monitoring-ai-citations)
10. [Target Query Strategy](#10-target-query-strategy)
11. [Anti-Patterns](#11-anti-patterns)
12. [GEO Audit Framework](#12-geo-audit-framework)
13. [Platform-Specific Optimization](#13-platform-specific-optimization)
14. [Implementation Playbook](#14-implementation-playbook)
15. [Metrics and KPIs](#15-metrics-and-kpis)

---

## 1. What GEO Is

Generative Engine Optimization (GEO) is the practice of optimizing content to appear as sources and citations in AI-generated responses. It is SEO for AI — targeting platforms where users increasingly go for answers: ChatGPT (800M weekly users), Google AI Overviews (2B+ monthly users), Perplexity, Claude, Gemini, and Microsoft Copilot.

### The Shift in Numbers

- Gartner predicts a 25% decline in traditional search volume by 2026.
- AI-referred sessions jumped 527% year-over-year in H1 2025.
- AI engines typically cite 2-7 domains per response (vs. traditional 10 blue links).
- OpenAI's Operator (launched January 2026) browses, compares, and completes tasks on behalf of users — not just answering questions but taking actions.

### Why GEO Matters Now

In 2026, GEO is no longer optional. Most enterprise marketing teams have a GEO initiative. Most SMB marketing teams have not started — which represents a significant first-mover opportunity. The businesses implementing GEO now are capturing citation share while competition remains relatively low.

### GEO vs. SEO — Complementary, Not Replacement

GEO does not replace SEO. It extends it. Strong SEO fundamentals (backlinks, technical health, content authority) remain the foundation — 40.58% of AI citations still come from top-10 SERP results. GEO adds a new optimization layer on top of SEO:

| Dimension | SEO | GEO |
|-----------|-----|-----|
| Target | Search engine rankings | AI-generated citations |
| Metric | Position, CTR, traffic | Citation frequency, mention share |
| Content goal | Rank for keywords | Be the answer AI quotes |
| Technical focus | Crawlability, speed, mobile | Schema, llms.txt, crawler access |
| Authority signal | Backlinks, domain authority | Entity consistency, earned media |
| Update cycle | Quarterly refreshes | Monthly or faster |

---

## 2. How AI Search Differs from Traditional SEO

Understanding the mechanics of AI search is essential to optimizing for it. AI search engines do not return a list of links — they synthesize answers from multiple sources, decide what to cite, and present a single narrative response.

### 2.1 Citation-Based, Not Ranking-Based

Traditional search: 10 blue links ranked by relevance signals.
AI search: A synthesized answer that cites 2-7 sources inline.

The fundamental difference: **being cited is the new ranking**. Your content does not need to be "first" — it needs to be quotable, factual, and authoritative enough that the AI chooses to reference it.

### 2.2 Query Decomposition

When a user asks an AI search engine a question, the system does not search for that exact question. It decomposes the query into multiple sub-queries — between 2.9 and 10.7 sub-queries on average — and retrieves results for each one independently. 95% of these fan-out queries have zero traditional search volume.

This means optimizing for long-tail conversational queries is critical. The AI is not searching for "best CRM software" — it is searching for "CRM feature comparison enterprise," "CRM pricing models 2026," "CRM integration capabilities," and similar decomposed sub-queries.

### 2.3 Source Authority Over Domain Authority

AI systems evaluate source authority differently from traditional search:

- **Entity recognition** — Is this source a known, consistent entity across the web?
- **Factual density** — Does this content contain verifiable claims with data?
- **Citation chains** — Does this content cite its own sources? Content that cites authoritative references is itself treated as more authoritative.
- **Recency** — AI systems have a strong recency bias. Content older than 3 months sees significantly fewer citations.
- **Earned media** — Third-party coverage, reviews, and industry mentions outperform brand-owned content for AI citation likelihood.

### 2.4 Structured Data as a First-Class Signal

AI systems rely on structured data (JSON-LD, schema.org markup) to understand context, relationships, and meaning. This is not a nice-to-have — it is a primary signal. Schema markup can boost AI citation chances by over 36%.

### 2.5 The Content Window Problem

LLMs have finite context windows. They cannot process an entire website. They need:

1. Clear signals about what content matters most (llms.txt).
2. Clean, parseable content structure (headings, lists, tables).
3. Concise, information-dense paragraphs — not sprawling prose.
4. Content that loads without JavaScript execution (AI crawlers do not run JS).

---

## 3. The Citation Gap

### The 60%+ Finding

Research analyzing 863,000 search engine results pages revealed a fundamental shift in how AI-powered search systems select sources:

**67.82% of AI Overview citations do NOT rank in Google's Top 10.**

This finding has been replicated and extended:

| Platform | % Citations from Google Top 10 | Source |
|----------|-------------------------------|--------|
| Google AI Overviews (mid-2025) | 76% | Ahrefs |
| Google AI Overviews (Feb 2026, post-Gemini 3) | 38% | Ahrefs |
| ChatGPT | ~10% URL overlap with Google top 10 | Seer Interactive |
| Perplexity | ~12% overlap | Multiple studies |
| GPT-4o | 4% domain overlap with Google | arXiv study |

### Why the Gap Exists

The citation gap is driven by query decomposition. When Google's Gemini 3 upgraded in January 2026, it expanded the query fan-out system to pull from a dramatically wider source pool. The AI does not search the same queries humans type — it breaks them into sub-queries that surface entirely different result sets.

### What This Means for Strategy

1. **Google ranking is not enough.** Being #1 on Google does not guarantee AI citation.
2. **Non-ranking content can win.** Content that answers specific sub-queries can be cited even without traditional SEO rankings.
3. **Topical depth beats position.** Comprehensive content that covers sub-topics thoroughly gets cited by multiple fan-out queries.
4. **Lower-ranked sites can leapfrog.** The Princeton GEO study found that Cite Sources optimization generated 115.1% visibility improvement for rank-5 websites while decreasing rank-1 visibility by 30.3% — potentially democratizing search results.

---

## 4. The llms.txt Standard

### What It Is

llms.txt is a proposed open standard created by Jeremy Howard of Answer.AI in 2024. It is a plain-text file hosted at a website's root directory that provides a concise, Markdown-formatted map of a site's most important resources — specifically designed for LLM consumption.

### Why It Exists

LLMs face a critical limitation: context windows are too small to handle most websites in their entirety. Converting complex HTML pages with navigation, ads, and JavaScript into LLM-friendly plain text is both difficult and imprecise. llms.txt solves this by providing a curated, pre-processed overview.

### Format Specification

The file uses Markdown (not XML or JSON) because LLMs can read Markdown natively:

```markdown
# Project Name

> A concise one-paragraph description of the project with essential context.

Additional prose providing important details, key features,
or context that an LLM needs to understand this project.

## Documentation

- [Getting Started](https://example.com/docs/getting-started): Quick start guide for new users
- [API Reference](https://example.com/docs/api): Complete API documentation
- [Architecture](https://example.com/docs/architecture): System design and components

## Optional

- [Changelog](https://example.com/changelog): Version history
- [Contributing](https://example.com/contributing): How to contribute
```

### Required Elements

- **H1 heading** — Project or site name (the only mandatory element).

### Recommended Elements

- **Blockquote** — Concise summary with essential context.
- **Prose sections** — Paragraphs and lists providing detailed information.
- **H2-delimited sections** — Containing file lists (markdown hyperlinks with optional descriptions).
- **"Optional" H2 section** — Designates secondary resources that may be skipped when context is limited.

### File List Entry Format

Each entry follows this pattern:
```
- [Display Name](URL): Optional descriptive notes
```

### Associated Files

| File | Purpose |
|------|---------|
| `/llms.txt` | Root-level curated site overview |
| `/llms-full.txt` | Expanded version with complete content (for larger context windows) |
| `page.html.md` | Markdown version of any HTML page |
| `index.html.md` | Markdown version of pages without filenames |

### Placement

The file MUST be placed at the root path of the website: `https://example.com/llms.txt`

### Current Adoption Status

As of early 2026, llms.txt is a proposed standard — not officially adopted by OpenAI, Google, or Anthropic. However:

- Thousands of documentation sites support it, including Anthropic and Cursor.
- Tools like `llms_txt2ctx` can expand llms.txt into full context files.
- The cost of implementation is near-zero and the potential upside is significant.
- It coexists with sitemaps and robots.txt — no conflict.

### Implementation for CruxDev Projects

Every CruxDev-managed website SHOULD include an llms.txt file. Template:

```markdown
# [Project Name]

> [One-sentence description of the project and its primary value proposition.]

[Project Name] is [2-3 sentences of essential context: what it does, who it's for,
what makes it different].

## Documentation

- [Main Documentation](https://example.com/docs): Complete product documentation
- [API Reference](https://example.com/docs/api): REST API endpoints and usage
- [Pricing](https://example.com/pricing): Plans and pricing details

## Content

- [Blog](https://example.com/blog): Technical articles and product updates
- [Comparisons](https://example.com/compare): Product comparison pages
- [Case Studies](https://example.com/case-studies): Customer success stories

## Optional

- [Changelog](https://example.com/changelog): Release history
- [Status Page](https://status.example.com): System uptime
```

---

## 5. Structured Data That AI Systems Read

### Why Structured Data Matters for AI

Structured data provides clear, organized information that helps AI systems accurately interpret and categorize web content. It minimizes ambiguity, improves processing efficiency, and enables AI search engines to display content more effectively in AI-generated answers.

JSON-LD is the format Google recommends and AI systems prefer. It separates structured data from HTML, creating a clean data layer that AI systems can process without confusion from page design or content structure.

### 5.1 SoftwareApplication Schema

For software products, developer tools, and SaaS applications:

```json
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "ProductName",
  "description": "Concise product description with key capabilities",
  "applicationCategory": "DeveloperApplication",
  "operatingSystem": "Cross-platform",
  "url": "https://example.com",
  "author": {
    "@type": "Organization",
    "name": "Company Name",
    "url": "https://example.com"
  },
  "offers": {
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "USD",
    "description": "Free tier available"
  },
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingValue": "4.8",
    "ratingCount": "150"
  },
  "featureList": "Feature 1, Feature 2, Feature 3"
}
```

**AI impact:** Helps AI understand what the software does, how it's categorized, what it costs, and how users rate it. This data directly feeds product recommendation queries.

### 5.2 Article Schema

For blog posts, technical articles, and documentation:

```json
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "Article Title (Under 110 Characters)",
  "description": "Article summary in 150-160 characters",
  "author": {
    "@type": "Person",
    "name": "Author Name",
    "url": "https://example.com/about/author",
    "jobTitle": "Role Title",
    "worksFor": {
      "@type": "Organization",
      "name": "Company Name"
    }
  },
  "publisher": {
    "@type": "Organization",
    "name": "Publisher Name",
    "logo": {
      "@type": "ImageObject",
      "url": "https://example.com/logo.png"
    }
  },
  "datePublished": "2026-03-28",
  "dateModified": "2026-03-28",
  "mainEntityOfPage": "https://example.com/article-url",
  "wordCount": 2500,
  "keywords": "keyword1, keyword2, keyword3"
}
```

**AI impact:** Author credentials and publisher identity feed E-E-A-T signals. dateModified signals freshness. wordCount helps AI assess depth.

### 5.3 FAQPage Schema

For FAQ sections, knowledge bases, and Q&A content:

```json
{
  "@context": "https://schema.org",
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is [Topic]?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Direct, concise answer. Include key facts and data points."
      }
    },
    {
      "@type": "Question",
      "name": "How does [Topic] compare to [Alternative]?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "Comparison answer with specific differentiators."
      }
    }
  ]
}
```

**AI impact:** FAQ schema remains critical for AI search platforms. ChatGPT and Perplexity rely heavily on structured FAQ data for citations. FAQ schema can boost featured snippet and AI citation rates significantly.

### 5.4 HowTo Schema

For tutorials, guides, and step-by-step instructions:

```json
{
  "@context": "https://schema.org",
  "@type": "HowTo",
  "name": "How to [Accomplish Task]",
  "description": "Summary of what this guide covers",
  "totalTime": "PT30M",
  "estimatedCost": {
    "@type": "MonetaryAmount",
    "currency": "USD",
    "value": "0"
  },
  "step": [
    {
      "@type": "HowToStep",
      "name": "Step 1 Title",
      "text": "Detailed step instructions with specific actions.",
      "url": "https://example.com/guide#step-1"
    },
    {
      "@type": "HowToStep",
      "name": "Step 2 Title",
      "text": "Next step with concrete details.",
      "url": "https://example.com/guide#step-2"
    }
  ]
}
```

**AI impact:** Maps directly to step-by-step intents. AI systems extract and cite individual steps, making your content the procedural answer.

### 5.5 Organization Schema

For company/brand identity:

```json
{
  "@context": "https://schema.org",
  "@type": "Organization",
  "name": "Company Name",
  "url": "https://example.com",
  "logo": "https://example.com/logo.png",
  "description": "What the company does in one sentence",
  "foundingDate": "2024",
  "sameAs": [
    "https://twitter.com/company",
    "https://github.com/company",
    "https://linkedin.com/company/company"
  ],
  "contactPoint": {
    "@type": "ContactPoint",
    "contactType": "customer support",
    "url": "https://example.com/support"
  }
}
```

**AI impact:** Establishes entity identity across the web. The `sameAs` array helps AI systems connect your brand across platforms — critical for entity consistency.

### 5.6 Schema Priority for AI Citation

Not all schemas are equal for GEO. Priority order based on AI citation impact:

| Priority | Schema Type | Use Case | AI Citation Impact |
|----------|-------------|----------|-------------------|
| 1 | FAQPage | Q&A content | Highest — direct answer extraction |
| 2 | HowTo | Tutorials, guides | High — step-by-step citation |
| 3 | Article / TechArticle | Blog, docs | High — authority + freshness signals |
| 4 | SoftwareApplication | Product pages | High — product recommendation queries |
| 5 | Organization | Brand identity | Medium — entity disambiguation |
| 6 | Product + Review | E-commerce | Medium — commercial intent queries |
| 7 | BreadcrumbList | Navigation | Low — site structure signal |

### Implementation Rules

1. **Match schema to visible content.** Never add schema for content that does not appear on the page.
2. **Use JSON-LD format.** Not Microdata, not RDFa. JSON-LD is what AI systems parse best.
3. **Test before publishing.** Use Google Rich Results Test to validate.
4. **One primary schema per page.** Multiple schemas are allowed, but each page should have one dominant type.
5. **Keep data fresh.** Update dateModified when content changes.

---

## 6. Content Patterns That Get Cited

The Princeton GEO study (KDD 2024) tested nine optimization strategies on 10,000 queries across diverse domains. These are the empirically validated patterns that increase AI citation probability.

### 6.1 Statistics and Quantitative Data

**Impact:** ~40% visibility improvement.

Content with verifiable statistics gets cited dramatically more than unoptimized content. The mechanism is straightforward: AI systems need factual anchors to ground their responses, and quantitative claims provide those anchors.

**Implementation:**
- Include a relevant statistic every 150-200 words.
- Use specific numbers, not vague qualifiers ("40% improvement" not "significant improvement").
- Cite the source of each statistic.
- Present data in tables when comparing multiple items.

**Example — weak:**
> Our tool significantly improves developer productivity.

**Example — strong:**
> In a 2026 benchmark across 847 development teams, teams using [Tool] shipped 43% more features per sprint with 31% fewer defects (Source: [Study Name]).

### 6.2 Citation and Source Addition

**Impact:** ~30% visibility improvement; 115% for lower-ranked sites.

Content that cites its own authoritative sources is treated as more authoritative by AI systems. This is a citation chain effect — AI systems trust content that demonstrates it has done its homework.

**Implementation:**
- Name specific sources (not "studies show" but "a 2026 Stanford study found").
- Link to primary sources when available.
- Include a references section for data-heavy content.
- Cite industry-recognized sources (Gartner, Forrester, academic papers).

### 6.3 Quotation Addition

**Impact:** ~41% visibility improvement (highest single tactic in the Princeton study).

Direct quotations from recognized experts or authoritative sources provide AI systems with ready-to-cite material. The AI can lift a quote directly, which is lower-risk than paraphrasing.

**Implementation:**
- Include 2-3 expert quotations per long-form piece.
- Attribute quotes to named individuals with credentials.
- Use blockquote formatting for visual distinction.
- Ensure quotes add substantive information, not just endorsement.

### 6.4 Original Research and Proprietary Data

**Impact:** Competitors cannot duplicate. Highest long-term citation value.

Proprietary data, original research, benchmarks, and first-hand findings create content that AI systems must cite because no other source has the same data.

**Implementation:**
- Conduct and publish original surveys, benchmarks, or analyses.
- Even modest original research (N=50 survey, internal benchmark data) noticeably boosts citation potential.
- Present findings with methodology transparency.
- Update research annually to maintain freshness.

**Content types that perform well:**
- Benchmark reports with methodology sections.
- Survey results with sample size and demographics.
- Case studies with specific before/after metrics.
- Industry analysis with proprietary data sets.

### 6.5 Comparison Tables and Data Tables

**Impact:** Pages with comparison tables earn 2.5x more citations than text-only equivalents.

Tables create clean data blocks that AI can extract and cite. They are structured information that maps directly to comparison queries — one of the highest-intent query types.

**Implementation:**
- Use HTML tables (not images of tables).
- Include clear column headers.
- Keep data specific and verifiable.
- Cover the dimensions users actually compare on.

**Example structure:**
```markdown
| Feature | Product A | Product B | Product C |
|---------|-----------|-----------|-----------|
| Price | $29/mo | $49/mo | Free |
| API access | Yes | Yes | Limited |
| Integrations | 150+ | 80+ | 30+ |
| Support | 24/7 | Business hours | Community |
```

### 6.6 Content Length and Depth

Content above approximately 1,900 words significantly outperforms shorter pieces for AI citation. But length without substance is counterproductive — the key is information density at length.

**The sweet spot:**
- 1,900-3,500 words for comprehensive guides.
- 800-1,200 words for focused how-to content.
- FAQ content: 100-200 words per answer.

### 6.7 Clear Structure and Heading Hierarchy

AI systems process content through heading hierarchy. Clean structure allows models to locate, extract, and cite specific sections.

**Implementation:**
- Use H2 for major sections, H3 for subsections.
- Begin sections with direct answers before expanding (BLUF principle — Bottom Line Up Front).
- Keep paragraphs to 3-5 sentences.
- Use bulleted/numbered lists for enumerations.
- Include TL;DR statements under key topics.

### 6.8 Content Freshness

AI systems have strong recency bias. Content older than 3 months sees significantly fewer citations.

**Implementation:**
- Add visible "Last updated: [date]" timestamps.
- Refresh cornerstone content monthly with updated data.
- Update dateModified in Article schema when content changes.
- Remove or update outdated statistics.

### 6.9 What the Princeton Study Found Does NOT Work

**Keyword stuffing:** "Offers little to no improvement on generative engine responses" — actually underperformed baseline measurements in the study. Traditional SEO keyword optimization is ineffective for GEO.

**Stylistic-only changes:** Fluency and readability improvements showed 15-30% gains — meaningful but far less than factual enhancements. Style is a secondary factor.

---

## 7. Bing Index Importance

### ChatGPT Uses Bing

ChatGPT Search uses Bing's index as its backbone. This is not speculation — it is the documented architecture. The implications are direct:

**If Bing does not have your page, ChatGPT does not have your page.**

### The Numbers

- **87% of ChatGPT's citations match Bing's top organic results** (Seer Interactive study).
- Google sees only a 56% match with ChatGPT citations.
- It is unlikely ChatGPT Search will move away from Bing anytime soon — the symbiotic relationship benefits both parties.

### What This Means for Strategy

1. **Bing Webmaster Tools is now mandatory**, not optional. Submit your sitemap, verify indexing status, check for errors.
2. **Bing ranks differently from Google.** Bing places more weight on social signals, exact-match domains, and multimedia content.
3. **Check Bing indexation specifically.** Pages indexed in Google may not be indexed in Bing. Use `site:example.com` on Bing to verify.
4. **Submit new content to Bing IndexNow.** Bing supports the IndexNow protocol for instant indexing notification.

### Bing Optimization Checklist

- [ ] Bing Webmaster Tools account verified
- [ ] XML sitemap submitted to Bing
- [ ] IndexNow protocol implemented
- [ ] Social media profiles linked (Bing weighs social signals)
- [ ] Bing-specific meta tags in place (if applicable)
- [ ] Verify key pages are indexed: `site:yourdomain.com` on Bing
- [ ] Check Bing crawl errors and fix issues

### Other AI Platform Index Sources

| AI Platform | Primary Index Source | Secondary Sources |
|-------------|---------------------|-------------------|
| ChatGPT | Bing | Direct web crawling (GPTBot) |
| Google AI Overviews | Google Search | — |
| Perplexity | Proprietary index | Bing, direct crawling |
| Claude | Direct crawling (ClaudeBot) | — |
| Microsoft Copilot | Bing | — |
| Gemini | Google Search | — |

---

## 8. AI Crawler Access Control

### Known AI Crawler User Agents

To be cited by AI systems, you must allow their crawlers to access your content. Here are the major AI crawlers as of early 2026:

#### OpenAI (ChatGPT)
| User Agent | Purpose |
|-----------|---------|
| `GPTBot` | Training data collection |
| `OAI-SearchBot` | Search functionality |
| `ChatGPT-User` | Real-time user requests |

#### Anthropic (Claude)
| User Agent | Purpose |
|-----------|---------|
| `ClaudeBot` | Training data collection |
| `Claude-User` | Real-time user requests |
| `Claude-SearchBot` | Search result indexing |

#### Perplexity
| User Agent | Purpose |
|-----------|---------|
| `PerplexityBot` | Indexing |
| `Perplexity-User` | Real-time retrieval |

#### Google
| User Agent | Purpose |
|-----------|---------|
| `Google-Extended` | AI/Gemini training |
| `Googlebot` | Standard search indexing |

### robots.txt Configuration for GEO

To allow AI crawlers (recommended for GEO):

```
# Allow AI search crawlers
User-agent: GPTBot
Allow: /

User-agent: OAI-SearchBot
Allow: /

User-agent: ChatGPT-User
Allow: /

User-agent: ClaudeBot
Allow: /

User-agent: Claude-User
Allow: /

User-agent: Claude-SearchBot
Allow: /

User-agent: PerplexityBot
Allow: /

User-agent: Perplexity-User
Allow: /

User-agent: Google-Extended
Allow: /
```

### Granular Control Strategy

You may want to allow search-focused crawlers while restricting training crawlers:

```
# Allow search/citation crawlers
User-agent: OAI-SearchBot
Allow: /

User-agent: ChatGPT-User
Allow: /

User-agent: Claude-SearchBot
Allow: /

User-agent: PerplexityBot
Allow: /

# Block training crawlers (optional — prevents training but not search)
User-agent: GPTBot
Disallow: /

User-agent: ClaudeBot
Disallow: /

User-agent: Google-Extended
Disallow: /
```

### Critical Technical Notes

1. **AI crawlers do NOT execute JavaScript.** If your content is in a SPA or loads dynamically, AI cannot read it. Server-side render critical content.
2. **Tabs, accordions, and dropdowns** that require clicks to reveal content are invisible to AI bots. All citation-worthy content must be in the initial HTML.
3. **CDN and WAF settings** may block AI crawlers. Check your Cloudflare, Akamai, or Fastly configuration.
4. **Check server logs** for AI crawler activity. Look for GPTBot, ClaudeBot, and PerplexityBot in your access logs to verify they can reach your content.

---

## 9. Monitoring AI Citations

### Why Monitoring Matters

Unlike traditional SEO where you can check rankings in Google Search Console, AI citations are ephemeral — they appear in conversation responses, not persistent result pages. Monitoring requires different tools and approaches.

### 9.1 Manual Monitoring Protocol

For early-stage GEO or limited budgets:

1. **Compile a target query list** — 50-100 queries your brand should appear in (see Section 10).
2. **Run queries weekly** across ChatGPT, Perplexity, Google AI Overviews, and Claude.
3. **Record results:** Was your brand/URL mentioned? Which competitors appeared? What was the sentiment?
4. **Track in a spreadsheet or JSONL file:**

```jsonl
{"date": "2026-03-28", "platform": "chatgpt", "query": "best developer tools for CI/CD", "cited": true, "url_cited": "https://example.com/docs", "competitors_cited": ["competitor1.com", "competitor2.com"], "sentiment": "positive"}
{"date": "2026-03-28", "platform": "perplexity", "query": "best developer tools for CI/CD", "cited": false, "url_cited": null, "competitors_cited": ["competitor1.com"], "sentiment": null}
```

### 9.2 Automated Monitoring Tools (2026 Landscape)

| Tool | Platforms Tracked | Key Feature | Pricing |
|------|-------------------|-------------|---------|
| Otterly.ai | ChatGPT, Perplexity, Google AIO, Copilot | Citation tracking + competitive benchmarking | Paid |
| Topify | ChatGPT, Claude, Perplexity, Google AIO | Source-level citation tracking (which specific page) | Paid |
| OpenLens | ChatGPT, Claude, Google AI, Perplexity, DeepSeek | Free tier available | Free/Paid |
| Finseo | ChatGPT, Claude, Perplexity | AI visibility tracking | Paid |
| Siftly | ChatGPT, Perplexity, Google AIO | Brand mention analysis | Paid |
| AIclicks | ChatGPT, Google AIO | Rank tracking for AI search | Paid |

### 9.3 What to Monitor

| Metric | Description | Frequency |
|--------|-------------|-----------|
| Citation frequency | How often your brand/URL appears in AI responses | Weekly |
| Share of voice | Your citations vs. competitors for target queries | Weekly |
| Citation sentiment | Positive, neutral, or negative mentions | Weekly |
| Source pages cited | Which of YOUR pages are being cited | Weekly |
| Competitor citations | Who appears where you do not | Bi-weekly |
| Citation accuracy | Is AI representing your brand/product correctly? | Monthly |
| AI-referred traffic | Traffic from AI platforms (via GA4 referrer data) | Weekly |

### 9.4 GA4 Attribution for AI Traffic

Track AI-referred traffic in Google Analytics 4:

- ChatGPT referrals: `chatgpt.com` in referrer
- Perplexity referrals: `perplexity.ai` in referrer
- Claude referrals: `claude.ai` in referrer
- Google AI Overviews: Harder to isolate — appears as Google organic

Create a custom channel group in GA4 for "AI Search" that captures these referrers.

---

## 10. Target Query Strategy

### 10.1 Query Categories

Organize target queries into three tiers:

#### Tier 1 — Brand Queries (Defensive)
Queries where your brand should always appear. Failure to appear here is a critical issue.

Examples:
- "[Brand name] review"
- "[Brand name] vs [Competitor]"
- "What is [Brand name]?"
- "[Brand name] pricing"
- "[Brand name] alternatives"
- "Is [Brand name] good for [use case]?"

#### Tier 2 — Category Queries (Growth)
Generic queries for your product category. High value, high competition.

Examples:
- "Best [product category] tools 2026"
- "Top [product category] for [audience]"
- "How to [solve problem your product solves]"
- "[Product category] comparison"
- "What [product category] should I use for [use case]?"

#### Tier 3 — Competitive Queries (Offensive)
Queries where competitors currently dominate and you want to capture share.

Examples:
- "[Competitor name] alternatives"
- "[Competitor name] vs [your brand]"
- "Switching from [Competitor] to [alternative]"
- "Problems with [Competitor product]"

### 10.2 Query Research Process

1. **Start with your existing SEO keyword list.** Convert keywords to conversational queries.
2. **Study competitor citations.** What queries trigger competitor mentions in AI responses?
3. **Use AI platforms directly.** Ask ChatGPT/Perplexity conversational questions about your category and note which brands appear.
4. **Map the query decomposition.** For each target query, hypothesize what sub-queries the AI might generate and create content for those sub-queries.
5. **Monitor and iterate.** Track which queries you win and which you miss. Create or improve content for gaps.

### 10.3 Prompt Research Framework

Map 50-100 conversational prompts prioritized by:

1. **Decision-stage intent** — Queries where the user is choosing between options (highest value).
2. **Information-stage intent** — Queries where the user is learning (medium value, builds authority).
3. **Awareness-stage intent** — Queries about the problem space (lower value, broadest reach).

Focus 60% of effort on decision-stage queries, 30% on information-stage, 10% on awareness-stage.

---

## 11. Anti-Patterns

These are practices that either do not work for AI search or actively harm your visibility. Each is backed by research or empirical observation.

### 11.1 Keyword Stuffing

**Status: Actively harmful.**

The Princeton GEO study found keyword stuffing "offers little to no improvement on generative engine responses" and actually underperformed baseline measurements. LLMs do not rely on keyword density — they rely on semantic proximity and entity relationships.

**Why it fails:** AI systems understand meaning, not keyword frequency. Repeating "best CRM software" 47 times does not make you more likely to be cited for CRM queries. It makes your content look low-quality.

### 11.2 Thin Content

**Status: Invisible to AI.**

Surface-level information does not establish authority. AI wants to cite comprehensive sources. Content addressing broad topics without specific, intent-focused information is unlikely to be cited.

**Minimum viable depth:** 1,900+ words for comprehensive guides with statistics every 150-200 words and 2-3 cited sources.

### 11.3 Missing Structured Data

**Status: Competitive disadvantage.**

Without JSON-LD schema markup, AI systems have less context about your content. Competitors with schema will be cited preferentially. Schema markup boosts AI citation chances by over 36%.

### 11.4 Blocking AI Crawlers

**Status: Self-imposed invisibility.**

If your robots.txt blocks GPTBot, ClaudeBot, or PerplexityBot, your content cannot be cited. Many websites still have default-block rules from the early LLM period. Audit your robots.txt.

### 11.5 JavaScript-Dependent Content

**Status: Invisible to AI crawlers.**

AI crawlers do not execute JavaScript. SPAs, dynamically loaded content, React/Vue/Angular apps without SSR — all invisible. Content must be in the initial HTML response.

### 11.6 Outdated Content Without Freshness Signals

**Status: Deprioritized.**

Content without "Last updated" dates, with outdated statistics, or with stale references gets deprioritized by AI systems with recency bias. If you published a "2024 Guide" and never updated it, AI systems are skipping it in favor of "2026" content.

### 11.7 Inconsistent Entity Identity

**Status: Confuses AI entity recognition.**

If your brand is "Acme Corp" on your website, "AcmeCorp" on GitHub, "acme" on Twitter, and "Acme Corporation" in press releases, AI systems cannot reliably connect these as the same entity. Use identical brand naming everywhere.

### 11.8 Claims Without Sources

**Status: Reduced trust, fewer citations.**

Unverifiable claims without named sources lose credibility with AI systems. "Studies show our product is faster" is worthless. "A 2026 benchmark by [Named Lab] found [Product] processes 2.3x more requests per second (Source: [URL])" is citable.

### 11.9 Hidden Content

**Status: Invisible.**

Content behind tabs, accordions, modals, or "click to expand" elements is invisible to AI crawlers. All citation-worthy content must be in the visible DOM on page load.

### 11.10 One-Time Optimization

**Status: Decaying returns.**

GEO is not a one-time project. AI systems value freshness. Content optimized once and abandoned loses citation share over months. Plan for monthly content audits and refreshes.

---

## 12. GEO Audit Framework

### Audit Dimensions

A GEO audit evaluates five dimensions, weighted by impact:

| Dimension | Weight | What It Measures |
|-----------|--------|-----------------|
| Access & Indexability | 20% | Can AI crawlers reach and extract content? |
| Entity Clarity & Consistency | 20% | Is the brand clearly defined across the web? |
| Content Citation Readiness | 30% | Is content structured to be quoted by AI? |
| Trust & E-E-A-T Signals | 20% | Are expertise and credibility visible and verifiable? |
| Measurement & Monitoring | 10% | Is AI visibility tracked over time? |

### Scoring

| Score Range | Rating | Meaning |
|-------------|--------|---------|
| 85-100 | AI-Optimized | Fully optimized, maintaining competitive position |
| 70-84 | AI-Ready | Good foundation, targeted improvements needed |
| 55-69 | Needs Optimization | Significant gaps, not competitive in AI search |
| Below 55 | Not AI-Optimized | Minimal AI visibility, comprehensive work needed |

### Dimension 1: Access & Indexability (20 points)

| # | Check | Points | Pass Criteria |
|---|-------|--------|---------------|
| 1 | robots.txt allows AI crawlers | 4 | GPTBot, ClaudeBot, PerplexityBot allowed |
| 2 | Content renders without JavaScript | 4 | Critical pages SSR or static HTML |
| 3 | llms.txt file present and valid | 3 | Root-level, valid Markdown, current links |
| 4 | XML sitemap submitted to Bing | 3 | Verified in Bing Webmaster Tools |
| 5 | No CDN/WAF blocking AI crawlers | 3 | AI bot user agents not challenged |
| 6 | Markdown versions of key pages available | 3 | .html.md files for top content |

### Dimension 2: Entity Clarity & Consistency (20 points)

| # | Check | Points | Pass Criteria |
|---|-------|--------|---------------|
| 7 | Organization schema on homepage | 4 | Valid JSON-LD with sameAs links |
| 8 | Consistent brand naming across web | 4 | Same name on site, social, directories, press |
| 9 | Comprehensive About/Team pages | 4 | Named team members with credentials |
| 10 | Author bios on content pages | 4 | Named authors with expertise signals |
| 11 | Wikipedia/Wikidata presence (if eligible) | 2 | Entry exists or not applicable |
| 12 | Knowledge panel presence | 2 | Google Knowledge Panel exists or claimed |

### Dimension 3: Content Citation Readiness (30 points)

| # | Check | Points | Pass Criteria |
|---|-------|--------|---------------|
| 13 | Statistics included (1 per 150-200 words) | 5 | Verifiable, sourced statistics throughout |
| 14 | Sources cited by name | 4 | Named sources, not "studies show" |
| 15 | Comparison tables present | 4 | HTML tables on comparison/product pages |
| 16 | FAQ sections with schema | 4 | FAQPage schema on relevant pages |
| 17 | Content depth (1,900+ words on guides) | 3 | Long-form content meets threshold |
| 18 | BLUF structure (answer first) | 3 | Sections lead with direct answers |
| 19 | Heading hierarchy clean (H2/H3) | 3 | No skipped levels, descriptive headings |
| 20 | Freshness signals present | 2 | "Last updated" dates visible |
| 21 | Original research/proprietary data | 2 | At least one piece of original research |

### Dimension 4: Trust & E-E-A-T Signals (20 points)

| # | Check | Points | Pass Criteria |
|---|-------|--------|---------------|
| 22 | Article schema with author/publisher | 4 | Valid JSON-LD on content pages |
| 23 | External citations (earned media) | 4 | Third-party mentions, reviews, coverage |
| 24 | Expert quotations in content | 4 | Named expert quotes with credentials |
| 25 | Credentials and awards displayed | 4 | Visible on site, in schema where applicable |
| 26 | No unverifiable claims | 4 | All claims backed by named sources |

### Dimension 5: Measurement & Monitoring (10 points)

| # | Check | Points | Pass Criteria |
|---|-------|--------|---------------|
| 27 | Target query list defined | 3 | 50+ queries across brand/category/competitive |
| 28 | AI citation tracking active | 3 | Manual or automated monitoring in place |
| 29 | GA4 AI referrer tracking | 2 | Custom channel group for AI search traffic |
| 30 | Competitive benchmarking | 2 | Monthly competitor citation comparison |

### Running a GEO Audit

1. **Score each dimension** using the checklist above.
2. **Calculate weighted total** — raw score is already out of 100.
3. **Identify top 5 gaps** ranked by point value.
4. **Create action plan** — address highest-point gaps first.
5. **Re-audit monthly** — GEO requires ongoing optimization.

---

## 13. Platform-Specific Optimization

Each AI platform has distinct citation preferences. Optimize broadly, but understand the differences.

### 13.1 ChatGPT

**Index source:** Bing (87% citation match with Bing results).
**Content preference:** Encyclopedic, comprehensive content. Wikipedia is the #1 cited source at 47.9% of top-10 citation share.
**Key tactic:** Ensure Bing indexation. Create comprehensive, well-structured content with authoritative tone.

**Top cited source types:**
1. Wikipedia (47.9%)
2. Reddit (11.3%)
3. Forbes
4. G2

### 13.2 Google AI Overviews

**Index source:** Google Search.
**Content preference:** More balanced across source types. Strong preference for existing high-ranking content (but declining — down to 38% from 76% after Gemini 3 upgrade).
**Key tactic:** Maintain strong Google SEO foundation. Add schema markup. Keep content fresh.

**Top cited source types:**
1. Reddit (21.0%)
2. YouTube (18.8%)
3. Quora (14.3%)
4. LinkedIn (13.0%)

### 13.3 Perplexity

**Index source:** Proprietary index + Bing.
**Content preference:** Community-driven content, recency, and specific examples. Heavy Reddit preference.
**Key tactic:** Publish timely content. Ensure presence in community platforms. Reward recency.

**Top cited source types:**
1. Reddit (46.7%)
2. YouTube (13.9%)
3. Gartner (7.0%)

### 13.4 Claude

**Index source:** Direct crawling (ClaudeBot).
**Content preference:** Well-structured, technically accurate content.
**Key tactic:** Allow ClaudeBot and Claude-SearchBot in robots.txt. Provide clean HTML structure.

### 13.5 Cross-Platform Domain Distribution

Commercial domains (.com) dominate at 80.41% of all AI citations, followed by non-profit (.org) at 11.29%. Country-specific domains (.uk, .au, etc.) represent approximately 3.5%. Emerging tech TLDs (.io, .ai) show growing presence.

---

## 14. Implementation Playbook

### Phase 1 — Foundation (Week 1-2)

**Objective:** Ensure AI systems can access and understand your content.

- [ ] Audit robots.txt — allow all AI search crawlers
- [ ] Verify Bing indexation — set up Bing Webmaster Tools
- [ ] Submit sitemap to Bing
- [ ] Implement IndexNow protocol
- [ ] Add Organization schema to homepage
- [ ] Create llms.txt file
- [ ] Verify critical content renders without JavaScript
- [ ] Check CDN/WAF settings for AI crawler access

### Phase 2 — Entity and Authority (Week 2-3)

**Objective:** Establish clear entity identity and authority signals.

- [ ] Standardize brand naming across all platforms
- [ ] Create/update comprehensive About page with team credentials
- [ ] Add author bios to all content pages
- [ ] Implement sameAs links in Organization schema
- [ ] Audit external presence (social, directories, press) for name consistency
- [ ] Pursue Wikipedia/Wikidata entry if eligible

### Phase 3 — Content Optimization (Week 3-6)

**Objective:** Make content citation-ready.

- [ ] Add Article schema to all blog/documentation pages
- [ ] Add FAQPage schema to all FAQ content
- [ ] Add HowTo schema to all tutorial/guide content
- [ ] Audit top 20 pages for statistics density (1 per 150-200 words)
- [ ] Add named source citations throughout content
- [ ] Create comparison tables on relevant pages
- [ ] Restructure content with BLUF principle
- [ ] Add "Last updated" dates to all content
- [ ] Ensure heading hierarchy is clean (H2/H3)
- [ ] Remove hidden content (tabs, accordions) or make it visible

### Phase 4 — Content Creation (Week 4-8)

**Objective:** Create high-citation-potential content.

- [ ] Publish original research or benchmark data
- [ ] Create comprehensive comparison pages with tables
- [ ] Build FAQ pages for top 25 target queries
- [ ] Develop methodology/process documentation
- [ ] Write definitive guides (1,900+ words) for category queries
- [ ] Add expert quotations to existing content

### Phase 5 — Monitoring and Iteration (Ongoing)

**Objective:** Track and improve AI visibility continuously.

- [ ] Define target query list (50-100 queries)
- [ ] Set up AI citation monitoring (manual or automated)
- [ ] Configure GA4 AI referrer tracking
- [ ] Run monthly competitive benchmarking
- [ ] Refresh content monthly with updated data
- [ ] Re-run GEO audit quarterly

---

## 15. Metrics and KPIs

### Primary KPIs

| KPI | Definition | Target | Measurement |
|-----|-----------|--------|-------------|
| Citation Rate | % of target queries where brand is cited | >40% for brand, >15% for category | Weekly query monitoring |
| Share of Voice | Your citations / total citations for target queries | >competitor average | Weekly competitive tracking |
| AI-Referred Traffic | Sessions from AI platform referrers | Month-over-month growth | GA4 custom channel |
| Citation Accuracy | % of AI mentions that correctly describe your brand | >90% | Monthly manual review |

### Secondary KPIs

| KPI | Definition | Target | Measurement |
|-----|-----------|--------|-------------|
| GEO Audit Score | Weighted score across 5 dimensions | >85 (AI-Optimized) | Quarterly audit |
| Schema Coverage | % of eligible pages with valid JSON-LD | 100% | Automated scan |
| Content Freshness | % of pages updated within 90 days | >80% | CMS report |
| Bing Index Coverage | % of sitemap URLs indexed in Bing | >95% | Bing Webmaster Tools |
| Query Coverage | % of target queries with dedicated content | >70% | Manual query-content mapping |

### Reporting Cadence

| Report | Frequency | Audience | Content |
|--------|-----------|----------|---------|
| Citation snapshot | Weekly | Marketing team | Citations, share of voice, new mentions |
| Competitive benchmark | Monthly | Leadership | Market position, competitive movement |
| GEO audit | Quarterly | Technical team | Score, gaps, action items |
| Strategy review | Quarterly | Cross-functional | ROI, strategy adjustments, priorities |

---

## Appendix A: Key Research References

1. **Princeton GEO Study** — "GEO: Generative Engine Optimization." Pranjal Aggarwal et al. KDD 2024. Princeton University, Georgia Tech, Allen Institute for AI, IIT Delhi. Tested 9 optimization strategies across 10,000 queries. Found citation+statistics provide ~40% visibility improvement; keyword stuffing is ineffective.

2. **SurferSEO Citation Analysis** — "67.82% of AIO Citations Don't Rank In Google's Top 10." Analysis of AI Overview citation sources showing majority come from outside traditional top rankings.

3. **Ahrefs AI Overview Study** — Tracked citation source evolution from 76% top-10 (mid-2025) to 38% (Feb 2026) following Google's Gemini 3 upgrade.

4. **Seer Interactive ChatGPT Study** — Found 87% of ChatGPT citations match Bing's top organic results. Only 56% match Google's results.

5. **Profound AI Platform Study** — Analyzed citation patterns across ChatGPT, Google AI Overviews, and Perplexity from August 2024 to June 2025. Documented distinct source preferences per platform.

6. **llms.txt Specification** — Jeremy Howard, Answer.AI, 2024. Proposed standard for LLM-friendly site documentation. https://llmstxt.org/

---

## Appendix B: AI Crawler Reference Table

| Crawler | Organization | Purpose | robots.txt Name |
|---------|-------------|---------|-----------------|
| GPTBot | OpenAI | Training | `GPTBot` |
| OAI-SearchBot | OpenAI | Search | `OAI-SearchBot` |
| ChatGPT-User | OpenAI | User requests | `ChatGPT-User` |
| ClaudeBot | Anthropic | Training | `ClaudeBot` |
| Claude-User | Anthropic | User requests | `Claude-User` |
| Claude-SearchBot | Anthropic | Search indexing | `Claude-SearchBot` |
| PerplexityBot | Perplexity | Indexing | `PerplexityBot` |
| Perplexity-User | Perplexity | User retrieval | `Perplexity-User` |
| Google-Extended | Google | AI/Gemini training | `Google-Extended` |
| Googlebot | Google | Search indexing | `Googlebot` |

---

## Appendix C: GEO Audit Score Sheet Template

```
GEO AUDIT — [Project Name]
Date: YYYY-MM-DD
Auditor: [Name]

DIMENSION 1: Access & Indexability (20 pts)
  [ ] robots.txt allows AI crawlers ............ _/4
  [ ] Content renders without JS ............... _/4
  [ ] llms.txt present and valid ............... _/3
  [ ] XML sitemap in Bing ...................... _/3
  [ ] No CDN/WAF blocking AI crawlers .......... _/3
  [ ] Markdown page versions available ......... _/3
  Subtotal: _/20

DIMENSION 2: Entity Clarity (20 pts)
  [ ] Organization schema on homepage .......... _/4
  [ ] Consistent brand naming .................. _/4
  [ ] Comprehensive About/Team pages ........... _/4
  [ ] Author bios on content ................... _/4
  [ ] Wikipedia/Wikidata presence .............. _/2
  [ ] Knowledge panel presence ................. _/2
  Subtotal: _/20

DIMENSION 3: Content Citation Readiness (30 pts)
  [ ] Statistics density (1/150-200 words) ..... _/5
  [ ] Named source citations ................... _/4
  [ ] Comparison tables ........................ _/4
  [ ] FAQ sections with schema ................. _/4
  [ ] Content depth (1,900+ words) ............. _/3
  [ ] BLUF structure ........................... _/3
  [ ] Clean heading hierarchy .................. _/3
  [ ] Freshness signals ........................ _/2
  [ ] Original research/data ................... _/2
  Subtotal: _/30

DIMENSION 4: Trust & E-E-A-T (20 pts)
  [ ] Article schema with author ............... _/4
  [ ] External citations (earned media) ........ _/4
  [ ] Expert quotations ........................ _/4
  [ ] Credentials/awards displayed ............. _/4
  [ ] No unverifiable claims ................... _/4
  Subtotal: _/20

DIMENSION 5: Measurement (10 pts)
  [ ] Target query list (50+) .................. _/3
  [ ] AI citation tracking active .............. _/3
  [ ] GA4 AI referrer tracking ................. _/2
  [ ] Competitive benchmarking ................. _/2
  Subtotal: _/10

TOTAL: _/100
RATING: [ ] AI-Optimized (85+) [ ] AI-Ready (70-84)
        [ ] Needs Optimization (55-69) [ ] Not AI-Optimized (<55)

TOP 5 GAPS:
1.
2.
3.
4.
5.

ACTION PLAN:
1.
2.
3.
4.
5.
```
