# BUILD_PLAN_080: Google Search Console API Integration

**Status:** CONVERGED
**Priority:** Medium (requires OAuth setup)
**Depends on:** BP079 (SEO monitoring foundation)

## Context

Google Search Console has a REST API that provides the richest SEO data available: exact search queries, click-through rates, average position per keyword, crawl errors, index coverage, and Core Web Vitals. This data is critical for understanding what's working and what isn't, but requires OAuth2 authentication.

## Phase 1: OAuth2 Setup

- [ ] 1.1 Create Google Cloud project for CruxDev
- [ ] 1.2 Enable Search Console API
- [ ] 1.3 Create OAuth2 credentials (desktop app type)
- [ ] 1.4 Complete initial authorization flow (browser-based, one-time)
- [ ] 1.5 Store refresh token securely (env var, not config file)

## Phase 2: Data Collection

- [ ] 2.1 `rust/src/growth/search_console.rs` — API client module
- [ ] 2.2 Query: search analytics (queries, clicks, impressions, CTR, position) for last 28 days
- [ ] 2.3 Query: URL inspection (indexed/not indexed, last crawl, crawl errors)
- [ ] 2.4 Query: sitemaps (submitted, indexed count, errors)
- [ ] 2.5 Store in `.cruxdev/growth/search_console.jsonl`

## Phase 3: Analysis and Feedback

- [ ] 3.1 Identify top-performing queries (drive more content for these)
- [ ] 3.2 Identify queries with high impressions but low CTR (improve titles/descriptions)
- [ ] 3.3 Identify pages not indexed (fix or remove from sitemap)
- [ ] 3.4 Track position changes week-over-week for target keywords
- [ ] 3.5 Generate convergence findings from GSC data

## Phase 4: MCP Tool + Growth Cycle

- [ ] 4.1 MCP tool: `search_console_report(days, project_dir)`
- [ ] 4.2 Wire into weekly growth cycle
- [ ] 4.3 Auto-generate content recommendations from search data

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
