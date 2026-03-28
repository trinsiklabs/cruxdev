# BUILD_PLAN_081: GEO (Generative Engine Optimization) — AI Citation Monitoring

**Status:** NOT STARTED
**Priority:** Medium (experimental, high strategic value)
**Depends on:** BP079 (SEO monitoring foundation)

## Context

60% of AI-cited sources are NOT in Google's top 10 — original data and structured content matter more than traditional SEO rank. ChatGPT search uses Bing's index. Perplexity, Claude, and Gemini have their own indexing. Monitoring AI citations is the next frontier of discoverability.

CruxDev is positioning as the "autonomous convergence harness" — a new category. Tracking when AI systems start citing us for this term is a leading indicator of market awareness.

## Phase 1: Target Query Definition

- [ ] 1.1 Define primary queries: "autonomous convergence harness", "AI coding harness", "convergence engine for AI development"
- [ ] 1.2 Define brand queries: "cruxdev", "cruxdev.dev", "trinsik labs cruxdev"
- [ ] 1.3 Define competitive queries: "cursor alternative for code quality", "claude code convergence", "ai code audit tool"
- [ ] 1.4 Define category queries: "harness engineering tools", "AI agent harness comparison"
- [ ] 1.5 Store in growth.toml `[geo]` section

## Phase 2: AI Citation Detection

- [ ] 2.1 Research available APIs: Perplexity API, ChatGPT browsing (via Bing), Google AI Overview
- [ ] 2.2 Implement web search check: search target queries, scan results for cruxdev.dev mentions
- [ ] 2.3 Track citation context: what was said about us, in what position, alongside which competitors
- [ ] 2.4 Store results in `.cruxdev/growth/geo_citations.jsonl`

## Phase 3: llms.txt Optimization

- [ ] 3.1 Auto-update llms.txt when capabilities change (post-convergence hook)
- [ ] 3.2 A/B test different llms.txt formats (structured vs narrative vs hybrid)
- [ ] 3.3 Track correlation between llms.txt changes and AI citation frequency
- [ ] 3.4 Include methodology docs summary in llms.txt for depth

## Phase 4: Structured Data for AI

- [ ] 4.1 Validate JSON-LD on all pages (SoftwareApplication, FAQPage, HowTo schemas)
- [ ] 4.2 Add SoftwareSourceCode schema for open source aspects
- [ ] 4.3 Add ComparisonTable schema for vs/ pages
- [ ] 4.4 Test AI systems' ability to extract our structured data

## Phase 5: Content Strategy from GEO Data

- [ ] 5.1 When AI systems cite competitors but not us for a query → gap detected
- [ ] 5.2 Generate content targeting that query (blog post, comparison page, methodology page)
- [ ] 5.3 Feed into BIP pipeline: "GEO gap detected: [query] cites [competitor] but not CruxDev"
- [ ] 5.4 Track if new content closes the citation gap

## Phase 6: Monitoring Dashboard

- [ ] 6.1 Weekly GEO report: queries checked, citations found, gaps identified
- [ ] 6.2 Trend tracking: citation count over time per query
- [ ] 6.3 Competitive comparison: who gets cited for our target queries
- [ ] 6.4 MCP tool: `geo_report(project_dir)` — returns current GEO status

## Phase 7: Content Generation

- [ ] 7.1 Blog post: "GEO Monitoring — How We Track AI Citations for CruxDev"
- [ ] 7.2 X post announcing GEO monitoring capability
- [ ] 7.3 Methodology doc: `docs/GEO_PATTERNS.md` — patterns for optimizing AI discoverability

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
