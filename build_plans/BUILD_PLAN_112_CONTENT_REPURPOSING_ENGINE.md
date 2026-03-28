# BUILD_PLAN_112: Content Repurposing Engine

**Status:** NOT STARTED
**Priority:** Critical (highest-impact capability for creator vertical)
**Created:** 2026-03-28
**Triggered by:** Careiance/Zephyr Oakhaven capabilities wishlist — manually adapting every article for 6+ platforms

## Problem

Every creator with a multi-platform presence faces the same grind: write one long-form piece, then manually rewrite it for X (thread + single), Facebook, Instagram (caption + carousel), LinkedIn, TikTok (script), YouTube (description), Ko-Fi, and email. This takes 2-4 hours per article and is the #1 reason creators either go silent on platforms or post identical content everywhere (which algorithms penalize).

No existing tool does this well. Buffer/Hootsuite let you schedule the same post across platforms but don't adapt the content. AI writing tools generate from scratch but don't maintain voice consistency or understand platform conventions deeply.

## Solution

A `repurpose_content` MCP tool and underlying engine that takes one long-form source and produces platform-optimized derivatives. Each derivative respects the platform's conventions (character limits, hashtag culture, link placement, thread structure) while maintaining the creator's voice.

## Architecture

```
Source Article (markdown file)
    │
    ▼
Content Analyzer
    │ (extract: key points, quotes, stats, narrative arc, CTA)
    ▼
Platform Router
    │
    ├──▶ X Thread (3-8 tweets, hook → value → CTA)
    ├──▶ X Single (standalone tweet, < 280 chars)
    ├──▶ Facebook Post (longer, conversational, question-driven)
    ├──▶ Instagram Caption (emoji-friendly, hashtag block, line breaks)
    ├──▶ LinkedIn Post (professional, insight-driven, 1300 chars)
    ├──▶ TikTok Script (spoken word, 60-90 seconds, hook in first 3 secs)
    ├──▶ YouTube Description (SEO keywords, timestamps, links)
    ├──▶ Ko-Fi Post (community tone, gratitude, behind-the-scenes)
    ├──▶ Medium Article (adapted for Medium's audience, different title/SEO)
    └──▶ Carousel Slides (5-12 slides, visual direction per slide)
          │
          ▼
    Voice Audit (optional, if style guide provided)
          │
          ▼
    Editorial Calendar (optional, auto-schedule across days)
          │
          ▼
    Publishing Queue (ready for cross-platform publishing)
```

## Phase 1: Content Analysis Engine

### 1a. Source content parser
- File: `rust/src/growth/content_analyzer.rs`
- Parse markdown source into structured components:
  - `KeyPoint` { text, importance_score, quotability }
  - `Quote` { text, attribution, platform_suitability }
  - `Statistic` { claim, source, visual_potential }
  - `NarrativeArc` { hook, problem, solution, proof, cta }
  - `MediaSuggestion` { type, description, platforms }
- [ ] Markdown parser extracts headings, paragraphs, lists, blockquotes
- [ ] Key point extraction (top 5-10 takeaways)
- [ ] Quote extraction (pithy, shareable sentences)
- [ ] Statistic extraction (numbers with context)
- [ ] Narrative arc detection
- [ ] Tests for each extraction type

### 1b. Platform format specifications
- File: `rust/src/growth/platform_specs.rs`
- Define per-platform constraints and conventions:

| Platform | Max Length | Hashtags | Links | Format Notes |
|----------|-----------|----------|-------|-------------|
| X Single | 280 chars | 0-2 | Optional | Punchy, no fluff |
| X Thread | 280/tweet, 3-15 tweets | 0-2 on last tweet | Last tweet | Hook first tweet, value middle, CTA last |
| Facebook | 63,206 chars (optimal: 100-250) | 0-3 | Inline | Conversational, question hooks |
| Instagram | 2,200 chars | 20-30 (in comment or end) | In bio only | Line breaks, emojis, story-like |
| LinkedIn | 3,000 chars (optimal: 1,300) | 3-5 | Inline | Professional, insight-led, use line breaks |
| TikTok Script | 60-90 sec spoken | 3-5 in caption | In bio | Spoken word, hook in 3 sec, visual cues |
| YouTube Desc | 5,000 chars | 3-5 | Multiple | SEO front-loaded, timestamps, links section |
| Ko-Fi | No limit | 0-2 | Inline | Community tone, gratitude, BTS |
| Medium | No limit | 3-5 (as tags) | Inline | SEO title, different from source title |

- [ ] PlatformSpec struct with all constraints
- [ ] All 9+ platforms defined
- [ ] Validation: generated content fits platform constraints
- [ ] Tests for constraint checking

## Phase 2: Derivative Generation

### 2a. Per-platform generators
- File: `rust/src/growth/derivative_generators.rs`
- Each generator takes the analyzed source + platform spec and produces a derivative
- Generator strategy per platform:
  - **X Thread:** Extract narrative arc → one point per tweet → hook first → CTA last
  - **X Single:** Best single key point or quote → compress to 280 chars
  - **Facebook:** Conversational rewrite of the core insight → question to drive comments
  - **Instagram:** Visual-first thinking → which key point makes the best image? Caption supports image
  - **LinkedIn:** Professional framing → "Here's what I learned" structure → insight + proof
  - **TikTok Script:** Spoken word adaptation → "Stop scrolling if..." hook → 60-sec value → CTA
  - **YouTube Description:** SEO keywords from topic → structured description → timestamps placeholder
  - **Ko-Fi:** Behind-the-scenes angle → "I just published X, here's the story behind it"
  - **Medium:** Re-angle for Medium's audience → different title, different hook, same core content
- [ ] Generator trait/interface defined
- [ ] X Thread generator + tests
- [ ] X Single generator + tests
- [ ] Facebook generator + tests
- [ ] Instagram caption generator + tests
- [ ] LinkedIn generator + tests
- [ ] TikTok script generator + tests
- [ ] YouTube description generator + tests
- [ ] Ko-Fi post generator + tests
- [ ] Medium article generator + tests

### 2b. Cross-platform deduplication
- File: `rust/src/growth/deduplication.rs`
- Problem: LinkedIn and Facebook posts can't be identical. Instagram and X can't share the same hook.
- Solution: After generating all derivatives, compare pairwise and flag overlaps.
- Overlap detection: Jaccard similarity on key phrases > 0.6 triggers rewrite.
- Rewrite strategy: change the angle (same content, different framing).
- [ ] Similarity scoring between derivatives
- [ ] Overlap detection with configurable threshold
- [ ] Rewrite trigger when overlap exceeds threshold
- [ ] Tests for deduplication

## Phase 3: MCP Tool Integration

### 3a. repurpose_content tool
- File: `rust/src/server.rs` (add tool)
- Params:
  - source_file: string (required) — path to markdown source
  - platforms: string[] (required) — which platforms to generate for
  - style_guide: string (optional) — path to voice/style guide
  - campaign_arc: string (optional) — arc name to link posts to
  - schedule: bool (optional) — auto-add to editorial calendar
  - project_dir: string (optional)
- Flow:
  1. Read and analyze source content
  2. Generate derivatives for each requested platform
  3. Run deduplication check
  4. Run voice audit if style guide provided
  5. Write derivatives to `.cruxdev/growth/derivatives/{source_slug}/`
  6. Optionally add to editorial calendar / publishing queue
  7. Return all derivatives with metadata
- [ ] Tool parameter struct defined
- [ ] Tool handler implemented
- [ ] Integration test: article → X thread + LinkedIn + Instagram
- [ ] Integration test: article → all platforms
- [ ] Error handling for invalid source files

### 3b. list_derivatives tool
- Shows all generated derivatives for a source, with status (draft/scheduled/published)
- [ ] Tool parameter struct
- [ ] Tool handler
- [ ] Tests

## Phase 4: Voice and Calendar Integration

### 4a. Voice audit hook
- If style guide is provided, every derivative passes through voice audit before being marked ready
- Derivatives that fail voice audit are flagged with specific deviations
- [ ] Voice audit integration point
- [ ] Test: derivative with voice deviation is flagged
- [ ] Test: derivative passing voice audit is marked ready

### 4b. Editorial calendar integration
- If `schedule=true`, derivatives are spread across days to avoid channel fatigue:
  - Day 1: Blog post goes live + X thread
  - Day 2: LinkedIn post
  - Day 3: Instagram caption/carousel
  - Day 4: Facebook post
  - Day 5: Ko-Fi behind-the-scenes
  - Day 6-7: Medium article (SEO-optimized for discovery)
- Default spacing is configurable per creator
- Respects existing calendar entries (no double-booking)
- [ ] Calendar integration with smart scheduling
- [ ] Conflict detection with existing entries
- [ ] Tests for scheduling logic

## Phase 5: Tests

- [ ] Unit tests for content analyzer (all extraction types)
- [ ] Unit tests for platform specs (all platforms)
- [ ] Unit tests for each generator (at least 3 test articles per platform)
- [ ] Unit tests for deduplication
- [ ] Integration tests for repurpose_content tool
- [ ] Integration tests with voice audit
- [ ] Integration tests with calendar scheduling
- [ ] End-to-end: article → derivatives → voice audit → calendar → publishing queue
- [ ] Edge cases: very short source, very long source, source with no statistics, source with no quotes

## File Locations

| File | Purpose |
|------|---------|
| `rust/src/growth/content_analyzer.rs` | Source content analysis and extraction |
| `rust/src/growth/platform_specs.rs` | Per-platform format specifications |
| `rust/src/growth/derivative_generators.rs` | Platform-specific content generators |
| `rust/src/growth/deduplication.rs` | Cross-platform overlap detection |
| `rust/src/growth/mod.rs` | Module registration |
| `rust/src/server.rs` | MCP tool handlers |
| `rust/tests/content_repurposing_tests.rs` | All tests |

## Success Criteria

1. A 2000-word article produces platform-appropriate derivatives for all 9 platforms in under 30 seconds.
2. No two derivatives for the same source have >60% phrase overlap.
3. Every derivative fits within its platform's character/format constraints.
4. Voice audit scores >0.85 when a style guide is provided.
5. Auto-scheduling spreads derivatives across 5-7 days with no conflicts.
6. Creators report saving 2+ hours per article compared to manual repurposing.
