# BUILD_PLAN_079: SEO/GEO Monitoring with Automated Feedback Loop

**Status:** CONVERGED
**Priority:** High
**Depends on:** Cloudflare setup (done), search registration (done), growth cycle (done)

## Context

cruxdev.dev is registered with Google Search Console, Bing Webmaster Tools, and IndexNow. But registration without monitoring is blind. We need automated tracking of SEO and GEO (Generative Engine Optimization) metrics, with a feedback loop that detects regressions and drives improvements back into the site.

## Phase 1: SEO Health Check (No API Keys Needed)

### 1a. Automated site health probe
- [ ] New Rust module: `rust/src/growth/seo.rs`
- [ ] `check_seo_health(domain: &str) -> SeoHealthReport`
- [ ] Checks (all via HTTP requests, no auth needed):
  - `robots.txt` accessible and contains sitemap reference
  - `sitemap-index.xml` accessible, valid XML, lists pages
  - `llms.txt` accessible and non-empty
  - Homepage returns 200 with `<title>`, `<meta description>`, `<link rel="canonical">`
  - Key pages return 200: `/engine`, `/methodology`, `/vs/`, `/guides/quick-install`, `/blog/`
  - Security headers present (HSTS, X-Frame-Options, CSP via Permissions-Policy)
  - Cloudflare proxy active (`server: cloudflare` header)
  - HTTPS redirect works (HTTP→HTTPS)
  - No mixed content warnings
- [ ] Store results in `.cruxdev/growth/seo_health.jsonl`
- [ ] Alert on any failure (finding → convergence engine if in active loop)

### 1b. Link checker
- [ ] Crawl all internal links from sitemap
- [ ] Report broken links (404s), redirect chains, missing anchors
- [ ] Store broken link report in health results

### 1c. MCP tool
- [ ] `check_seo_health(domain, project_dir)` — runs full health check, returns report
- [ ] Wire into `run_growth_cycle` as an optional step

## Phase 2: PageSpeed Tracking (Free API, No OAuth)

### 2a. PageSpeed API integration
- [ ] Google PageSpeed Insights API is free, no auth needed for basic usage
- [ ] `check_pagespeed(url: &str) -> PageSpeedReport`
- [ ] Endpoint: `https://www.googleapis.com/pagespeedonline/v5/runPagespeed?url={url}&strategy=mobile`
- [ ] Extract: Performance score, Accessibility score, Best Practices score, SEO score
- [ ] Extract Core Web Vitals: LCP, FID/INP, CLS
- [ ] Run against homepage + 3 key pages (engine, methodology, vs/)

### 2b. Score tracking and regression detection
- [ ] Store scores in `.cruxdev/growth/pagespeed.jsonl` (timestamped)
- [ ] Compare against previous run — flag regressions > 5 points
- [ ] Track trends over time (improving/stable/declining)

### 2c. Post-deploy PageSpeed check
- [ ] Add to deploy.sh: after cache purge, hit PageSpeed API for homepage
- [ ] If performance score drops below 80, log warning
- [ ] MCP tool: `check_pagespeed(urls, project_dir)` — manual trigger

### 2d. Feedback into site
- [ ] PageSpeed recommendations → generate convergence findings
- [ ] "Reduce unused JavaScript" → finding with file paths
- [ ] "Serve images in next-gen formats" → finding for image optimization
- [ ] Feed findings into website convergence phase automatically

## Phase 3: Crawl Index Monitoring (No OAuth, Public APIs)

### 3a. Google indexing check
- [ ] `site:cruxdev.dev` search via programmable search — count indexed pages
- [ ] Track indexed page count over time in growth metrics
- [ ] Alternative: parse sitemap and check each URL with `curl -sI` for `X-Robots-Tag`

### 3b. Bing indexing check
- [ ] Bing URL Inspection API (available via Webmaster Tools API key)
- [ ] Track Bing indexed page count

### 3c. Index coverage report
- [ ] Compare sitemap URLs vs indexed URLs
- [ ] Report: pages in sitemap but not indexed (crawl budget issues)
- [ ] Report: pages indexed but not in sitemap (orphan pages)

## Phase 4: GEO (Generative Engine Optimization) Monitoring

### 4a. AI citation tracking
- [ ] Define target queries: "autonomous convergence harness", "AI coding harness", "convergence engine for AI", "cruxdev", "alternative to cursor for code quality"
- [ ] Periodically check if cruxdev.dev appears in AI-generated responses
- [ ] Method: use web search API to check `site:cruxdev.dev` ranking for target terms
- [ ] Track: which queries mention us, citation position, context

### 4b. llms.txt freshness
- [ ] Auto-update llms.txt when capabilities change (new tools, new patterns docs, new project types)
- [ ] Wire into post-convergence hook: if build plan changes capabilities, regenerate llms.txt
- [ ] Include in SEO health check: verify llms.txt matches current state

### 4c. Structured data validation
- [ ] Check JSON-LD schema on all pages (SoftwareApplication, FAQPage, HowTo)
- [ ] Validate against schema.org specs
- [ ] Report missing or invalid structured data as findings

## Phase 5: Automated Feedback Loop

### 5a. SEO findings → convergence engine
- [ ] When SEO health check finds issues, create ContentEvent with type `IssueResolved` or `BugFix`
- [ ] Feed into convergence as findings if a convergence loop is active
- [ ] If no active loop, write to `.cruxdev/evolution/posts/` as improvement candidates

### 5b. PageSpeed findings → website convergence
- [ ] Map PageSpeed recommendations to website convergence dimensions
- [ ] "Poor LCP" → performance dimension finding
- [ ] "Missing alt text" → accessibility dimension finding
- [ ] "No meta description" → seo dimension finding
- [ ] Auto-generate fix suggestions from PageSpeed API response

### 5c. Competitive SEO comparison
- [ ] Track competitor domains' PageSpeed scores for comparison
- [ ] Compare: cruxdev.dev vs cursor.com vs claude.ai/code vs github.com/openai/codex
- [ ] Report when competitors improve and we don't

### 5d. Weekly SEO report
- [ ] Aggregate all metrics into weekly report
- [ ] Stored in `.cruxdev/growth/seo_weekly/YYYY-WNN.json`
- [ ] Content: health check results, PageSpeed scores, index coverage, GEO citations
- [ ] Auto-generate blog post if significant improvement detected
- [ ] BIP pipeline: "SEO Week N: X pages indexed, PageSpeed Y, Z AI citations"

## Phase 6: Growth Config Integration

- [ ] Add `[seo]` section to `.cruxdev/growth.toml`:
  ```toml
  [seo]
  domain = "cruxdev.dev"
  check_on_deploy = true
  pagespeed_threshold = 80
  target_queries = ["autonomous convergence harness", "AI coding harness", "cruxdev"]
  competitor_domains = ["cursor.com", "deepsource.com"]
  weekly_report = true
  ```
- [ ] Wire all checks into `run_growth_cycle`

## Phase 7: Tests

- [ ] Test: SEO health check returns valid report for a mock site
- [ ] Test: PageSpeed API parsing extracts correct scores
- [ ] Test: regression detection flags score drops > 5 points
- [ ] Test: broken link detection finds 404s
- [ ] Test: llms.txt freshness check detects stale content
- [ ] Test: findings generation from PageSpeed recommendations

## Phase 8: Content Generation

- [ ] Blog post: "How CruxDev Monitors Its Own SEO and GEO Performance"
- [ ] X post announcing automated SEO monitoring
- [ ] Publish via BIP pipeline

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
