# Careiance / Zephyr Oakhaven — Capabilities Analysis

**Created:** 2026-03-28
**Persona:** Coach & Author in a composite creative ecosystem
**Platforms in use:** Medium, X, Facebook, Instagram, LinkedIn, Ko-Fi, personal website, Buffer
**Current pain:** Manual marketing plans in Google Docs, Buffer doesn't know about all channels, campaign arcs held in memory, no voice consistency enforcement, manual repurposing of long-form into platform-specific posts.

---

## OUTPUT 1: CruxDev Capabilities Analysis

### 1.1 Marketing Plan Generator

**What they need:** Given a product (book launch, coaching program, course), generate a complete marketing plan: audience segments, core messaging, channel strategy, editorial calendar, timeline with milestones, and KPIs.

**Can CruxDev handle this today?** Partially.
- `research_topic` can do market research and audience analysis.
- `generate_content` can produce individual content pieces.
- `generate_gap_analysis` and `generate_gap_build_plan` handle competitive gap-to-plan conversion.
- The convergence engine can iterate on a plan until it meets quality thresholds.

**What's missing:**
- No "marketing plan" schema — CruxDev's plan templates are build plans (code tasks), not marketing plans (audience/messaging/calendar).
- No calendar data structure — CruxDev tracks build plan phases with checklists, not time-bound calendar events across channels.
- No KPI tracking or marketing metrics store.

**New CruxDev capability needed:** `MarketingPlan` — a new plan type alongside `BuildPlan`. Requires:
1. A marketing plan template with sections: Audience, Messaging, Channels, Calendar, Timeline, KPIs
2. A marketing plan validator (like `plan_validator.rs` but for marketing plans)
3. Marketing plan convergence — the engine iterates on the plan until audience/messaging/channel alignment scores converge

**New MCP tool spec:**

```
generate_marketing_plan:
  description: "Generate a complete marketing plan for a product launch, campaign, or ongoing program."
  params:
    product_name: string (required) — Name of the product/program
    product_type: string (required) — book_launch | course_launch | coaching_program | membership | general
    target_audience: string (optional) — Description of target audience (AI will research if omitted)
    channels: string[] (optional) — Which platforms to include (defaults to all known)
    duration_weeks: int (optional) — Campaign duration (default: 12)
    voice_guide: string (optional) — Path to style/voice guide file
    project_dir: string (optional)
  returns:
    plan_file: string — Path to generated marketing plan markdown
    audience_segments: object[] — Identified audience segments with pain points
    messaging_matrix: object — Core message + per-channel adaptations
    calendar: object[] — Week-by-week content calendar with platform assignments
    kpis: object[] — Measurable KPIs with targets
```

---

### 1.2 Editorial Calendar with Cross-Channel Awareness

**What they need:** A single view of what's going out where and when, across Medium, X, Facebook, Instagram, LinkedIn, Ko-Fi, and the personal website. Buffer handles some channels but doesn't see all of them.

**Can CruxDev handle this today?** No.
- CruxDev's growth module tracks content events and has an X posting queue (`x_queue.jsonl`).
- `generate_content` produces drafts, and `list_content_drafts` shows pending drafts.
- But there is no multi-channel awareness. The content pipeline knows about blog posts and X posts. That's it.

**What's missing:**
- Channel registry — a configuration of which platforms a creator uses, with API credentials or manual-post markers.
- Cross-channel content tracker — a JSONL or database tracking "this piece goes to X on Monday, LinkedIn on Wednesday, Instagram on Thursday."
- Conflict detection — "You already have 3 posts scheduled for Tuesday across channels."
- Gap detection — "Your Ko-Fi hasn't had content in 2 weeks."

**New CruxDev capability needed:** `ContentCalendar` module in the growth engine.

**New MCP tool spec:**

```
editorial_calendar:
  description: "View, manage, and analyze the cross-channel editorial calendar."
  params:
    action: string (required) — view | add | remove | analyze_gaps | check_conflicts
    channel: string (optional) — medium | x | facebook | instagram | linkedin | kofi | website | all
    date_range: string (optional) — "2026-03-28 to 2026-04-28" (default: next 4 weeks)
    content_id: string (optional) — For add/remove actions
    project_dir: string (optional)
  returns:
    calendar: object[] — Entries with date, channel, content_type, status, title
    gaps: object[] — Channels with no scheduled content in the next N days
    conflicts: object[] — Days with too many posts on one channel or cross-channel overlap
```

---

### 1.3 Campaign Arc Builder

**What they need:** Multi-week campaign arcs where each piece links to the next, themes progress, and narrative threads are tracked across all platforms. Currently held in the creator's memory.

**Can CruxDev handle this today?** The convergence engine manages multi-phase build plans with dependencies. The concept of "phases with linked tasks that track to completion" maps well. But build plans are for code tasks, not narrative arcs.

**What's missing:**
- Campaign arc data structure — a sequence of content pieces with theme tags, narrative thread IDs, and cross-references.
- Progressive theme tracking — "Week 1: Problem awareness. Week 2: Solution introduction. Week 3: Social proof. Week 4: Call to action."
- Cross-piece linking — "This Instagram post references the concept introduced in Tuesday's blog post."
- Arc health metrics — "Your narrative thread on X dropped a beat in week 3."

**New CruxDev capability needed:** `CampaignArc` module. Structurally similar to convergence plans but with narrative metadata.

**New MCP tool spec:**

```
campaign_arc:
  description: "Create, manage, and track multi-week campaign arcs with progressive themes."
  params:
    action: string (required) — create | view | add_piece | track_theme | check_health
    arc_name: string (optional) — Campaign name
    theme: string (optional) — Theme to track
    weeks: int (optional) — Arc duration (default: 4)
    content_piece: object (optional) — { title, channel, theme_tags, references[] }
    project_dir: string (optional)
  returns:
    arc: object — Full arc with weeks, themes, pieces, narrative threads
    health: object — Theme coverage, gap detection, dropped beats
```

---

### 2.1 Long-Form Article Generator with Voice Convergence

**What they need:** Draft articles in a specific voice/style. Audit voice consistency against a style guide. Flag passages that deviate.

**Can CruxDev handle this today?** Partially.
- `generate_content` produces content drafts.
- The convergence engine can iterate until quality thresholds are met.
- GTV (Ground Truth Verification) can verify factual claims in content.

**What's missing:**
- Voice profile — a structured representation of a creator's voice (sentence length distribution, vocabulary preferences, tone markers, rhetorical patterns, forbidden phrases).
- Voice convergence scoring — a dimension in the convergence engine that scores text against the voice profile.
- Passage-level deviation flagging — not just "this article doesn't match" but "paragraphs 3, 7, and 12 deviate because X."

**New CruxDev capability needed:** `VoiceConvergence` module.

**New MCP tool spec:**

```
voice_audit:
  description: "Audit content against a voice/style guide. Flag deviations at passage level."
  params:
    content_file: string (required) — Path to content to audit
    style_guide: string (required) — Path to voice/style guide file
    fix_mode: bool (optional) — If true, suggest rewrites for flagged passages (default: false)
    project_dir: string (optional)
  returns:
    overall_score: float — 0.0-1.0 voice consistency score
    deviations: object[] — { paragraph, sentence, issue, severity, suggested_fix }
    voice_profile_match: object — Per-dimension scores (tone, vocabulary, sentence_structure, rhetorical_patterns)
```

```
generate_article:
  description: "Generate a long-form article draft with voice convergence."
  params:
    topic: string (required)
    style_guide: string (required) — Path to voice/style guide
    word_count: int (optional) — Target word count (default: 1500)
    outline: string (optional) — Path to outline file, or inline outline
    research_sources: string[] (optional) — URLs or file paths for source material
    convergence_target: float (optional) — Voice score threshold (default: 0.85)
    project_dir: string (optional)
  returns:
    draft_file: string — Path to generated draft
    voice_score: float — Voice consistency score
    iteration_count: int — How many convergence rounds were needed
    deviations: object[] — Any remaining deviations below threshold
```

---

### 2.2 Short-Form Social Post Generator (Platform-Specific)

**What they need:** From one long-form piece, generate X threads, Facebook posts, Instagram captions, LinkedIn posts, TikTok scripts, YouTube descriptions. Each adapted to platform conventions.

**Can CruxDev handle this today?** Partially.
- `generate_content` produces blog posts and X posts from events.
- The X posting engine manages queue and rate limiting.
- But there's no multi-platform content adaptation.

**What's missing:**
- Platform format specifications — character limits, hashtag conventions, link placement, thread structure, optimal post length per platform.
- Content decomposition — intelligently breaking a 2000-word article into platform-appropriate pieces (not just truncation).
- Cross-platform deduplication — ensuring the LinkedIn version isn't identical to the Facebook version.

**New CruxDev capability needed:** `ContentRepurposing` module in the growth engine.

**New MCP tool spec:**

```
repurpose_content:
  description: "Transform long-form content into platform-specific posts."
  params:
    source_file: string (required) — Path to the source article/content
    platforms: string[] (required) — [x_thread, x_single, facebook, instagram, linkedin, tiktok_script, youtube_description, kofi]
    style_guide: string (optional) — Path to voice/style guide
    campaign_arc: string (optional) — Arc name to link these posts to
    schedule: bool (optional) — Auto-schedule via editorial calendar (default: false)
    project_dir: string (optional)
  returns:
    posts: object[] — { platform, content, character_count, hashtags, media_suggestions }
    calendar_entries: object[] — If schedule=true, the calendar entries created
```

---

### 2.3 Carousel Creator

**What they need:** Generate Instagram/LinkedIn carousel content slide-by-slide from a source article. Each slide has a headline, body text, and visual direction.

**Can CruxDev handle this today?** No. CruxDev generates text content only. No slide-based or visual content generation.

**What's missing:**
- Carousel data structure — ordered slides with headline, body, visual direction, CTA placement.
- Content-to-carousel decomposition — intelligently breaking an article into 5-12 swipeable slides with narrative arc.
- Visual direction generation — color scheme, font suggestions, image prompts for each slide.
- Platform-specific sizing — Instagram carousel (1080x1350), LinkedIn carousel (1080x1080 or PDF).

**New CruxDev capability needed:** `CarouselGenerator` — likely a new module in the content pipeline. This is the most "new" capability on the list.

**New MCP tool spec:**

```
generate_carousel:
  description: "Generate carousel slide content from a source article."
  params:
    source_file: string (required) — Path to source article
    platform: string (required) — instagram | linkedin
    slide_count: int (optional) — Target number of slides (default: auto, typically 8-12)
    style_guide: string (optional) — Path to voice/style guide
    brand_colors: string[] (optional) — Hex color codes for visual direction
    project_dir: string (optional)
  returns:
    slides: object[] — { slide_number, headline, body, visual_direction, image_prompt, cta }
    cover_slide: object — First slide (hook)
    closing_slide: object — CTA slide
    aspect_ratio: string — Platform-appropriate dimensions
```

---

## OUTPUT 2: CruxVibe Recipe Opportunities

### Marketing Recipe

**Scope:** Marketing plan generation, editorial calendar, campaign arc management.

**Ash Resources:**
- `CruxVibe.Marketing.Plan` — marketing plan with audience, messaging, channels, timeline
- `CruxVibe.Marketing.CalendarEntry` — scheduled content across channels
- `CruxVibe.Marketing.Campaign` — multi-week campaign arc
- `CruxVibe.Marketing.CampaignPiece` — individual content piece within an arc
- `CruxVibe.Marketing.Theme` — narrative theme that spans pieces
- `CruxVibe.Marketing.Channel` — platform configuration (API keys, posting rules)

**Oban Workers:**
- `CruxVibe.Workers.PublishToChannel` — posts content to a specific channel at scheduled time
- `CruxVibe.Workers.AuditCampaignHealth` — periodic check for dropped themes, schedule gaps
- `CruxVibe.Workers.GenerateMarketingPlan` — async plan generation via LLM

**External API Integrations:**
- X/Twitter API (OAuth 1.0a) — via existing CruxDev X posting engine pattern
- Facebook Graph API — page posts, Instagram posts (connected to FB Business)
- LinkedIn API — article and post publishing
- Medium API — article publishing
- Ko-Fi API (or webhook-based) — post creation
- Buffer API (optional) — as a fallback/bridge for channels without direct API

**Connection to existing recipes:**
- **Newsletter recipe** — marketing campaigns can include email sequences. `CalendarEntry` can reference `Newsletter.Post` for email components.
- **Membership recipe** — campaign arcs can target different membership tiers. Gated content in campaigns respects `Membership.Access` rules.

---

### Content Repurposing Recipe

**Scope:** One article becomes 6+ platform-specific versions automatically.

**Ash Resources:**
- `CruxVibe.Content.Source` — the original long-form piece (belongs_to Newsletter.Post or standalone)
- `CruxVibe.Content.Derivative` — a platform-adapted version of a source
- `CruxVibe.Content.Platform` — platform with format rules (char limits, hashtag conventions)
- `CruxVibe.Content.DerivativeTemplate` — reusable patterns for platform adaptation

**Oban Workers:**
- `CruxVibe.Workers.RepurposeContent` — takes a Source, generates Derivatives for all configured platforms
- `CruxVibe.Workers.PublishDerivative` — posts a Derivative to its target platform at scheduled time
- `CruxVibe.Workers.VoiceAudit` — checks all derivatives against style guide before publishing

**External API Integrations:**
- Same as Marketing Recipe (X, Facebook, LinkedIn, Medium, Ko-Fi)
- OpenAI/Anthropic API (BYOK) — for content adaptation via LLM

**Connection to existing recipes:**
- **Newsletter recipe** — a `Newsletter.Post` can be a content source. When a newsletter goes out, derivatives are auto-generated for all platforms.
- **Membership recipe** — derivatives can be gated. The Instagram caption might be public, but the full article is members-only.

---

### Carousel Recipe

**Scope:** Slide-by-slide content generation with visual direction from any source content.

**Ash Resources:**
- `CruxVibe.Carousel.Deck` — a carousel with metadata (platform, source, brand)
- `CruxVibe.Carousel.Slide` — individual slide (position, headline, body, visual_direction, image_prompt)
- `CruxVibe.Carousel.BrandKit` — colors, fonts, logo URLs for visual consistency

**Oban Workers:**
- `CruxVibe.Workers.GenerateCarousel` — LLM-powered slide content generation from source
- `CruxVibe.Workers.RenderCarousel` — generates actual images (via Puppeteer/Chrome headless rendering HTML templates, or image generation API)

**External API Integrations:**
- OpenAI/Anthropic API (BYOK) — content decomposition into slides
- Image generation API (DALL-E, Midjourney API, or Flux) — optional, for slide backgrounds
- Instagram Graph API — carousel post publishing
- LinkedIn API — document/carousel publishing

**Connection to existing recipes:**
- **Content Repurposing** — carousels are a derivative type. The repurposing engine can auto-trigger carousel generation for Instagram/LinkedIn.
- **Newsletter** — carousel content can be embedded in email as image links.

---

### Voice Convergence Recipe

**Scope:** Style guide enforcement and voice auditing across all content generated or published by the creator.

**Ash Resources:**
- `CruxVibe.Voice.StyleGuide` — the creator's voice profile (tone, vocabulary, patterns, forbidden phrases)
- `CruxVibe.Voice.Audit` — audit result for a piece of content
- `CruxVibe.Voice.Deviation` — specific passage that deviates from style guide
- `CruxVibe.Voice.VoiceSample` — training samples from the creator's existing writing

**Oban Workers:**
- `CruxVibe.Workers.TrainVoiceProfile` — analyze creator's existing writing to build a voice profile
- `CruxVibe.Workers.AuditVoice` — run voice check on any content before publishing
- `CruxVibe.Workers.FixDeviations` — auto-rewrite flagged passages to match voice

**External API Integrations:**
- OpenAI/Anthropic API (BYOK) — voice analysis and rewriting

**Connection to existing recipes:**
- **ALL recipes** — voice convergence is a cross-cutting concern. Every piece of content (newsletter, blog, social post, carousel, course description) should pass voice audit before publishing.
- Acts as a quality gate in the content pipeline, similar to how GTV acts as a factual accuracy gate in CruxDev.

---

### Cross-Platform Publishing Recipe

**Scope:** Publish to Medium, X, Facebook, Instagram, LinkedIn, Ko-Fi from one place.

**Ash Resources:**
- `CruxVibe.Publishing.Connection` — OAuth connection to a platform (belongs_to Account)
- `CruxVibe.Publishing.Publication` — a published piece (platform, status, external_id, url)
- `CruxVibe.Publishing.Queue` — scheduled publications with rate limiting
- `CruxVibe.Publishing.Analytics` — per-platform engagement metrics (likes, shares, clicks)

**Oban Workers:**
- `CruxVibe.Workers.PublishToX` — X/Twitter via OAuth 1.0a
- `CruxVibe.Workers.PublishToMedium` — Medium API
- `CruxVibe.Workers.PublishToFacebook` — Facebook Graph API
- `CruxVibe.Workers.PublishToInstagram` — Instagram Graph API (requires FB Business)
- `CruxVibe.Workers.PublishToLinkedIn` — LinkedIn API
- `CruxVibe.Workers.PublishToKoFi` — Ko-Fi API/webhook
- `CruxVibe.Workers.FetchAnalytics` — periodic pull of engagement metrics per platform

**External API Integrations:**
- X API v2 (OAuth 1.0a)
- Facebook Graph API v19+ (pages + Instagram)
- LinkedIn Marketing API
- Medium API
- Ko-Fi API
- Stripe (for Ko-Fi alternative — direct tipping/purchases)

**Connection to existing recipes:**
- **Newsletter** — when a newsletter is sent, also publish to social channels
- **Membership** — gated content awareness (don't publish full content to public channels if it's members-only)
- **Storefront** — product launch posts auto-generated and queued
- **Content Repurposing** — repurposed derivatives flow into the publishing queue

---

## Recipe Dependency Graph

```
Voice Convergence ──────────────────────────────┐
      │                                          │
      │ (quality gate for all content)           │
      ▼                                          ▼
Content Repurposing ──────► Cross-Platform Publishing
      ▲                            ▲
      │                            │
Marketing Recipe                   │
      │                            │
      ├── Editorial Calendar ──────┘
      └── Campaign Arc Builder

Carousel Recipe ◄──── Content Repurposing (carousel as derivative type)
      │
      └──► Cross-Platform Publishing (post carousels to IG/LinkedIn)

Newsletter Recipe ──► Content Repurposing (newsletter → social derivatives)
Membership Recipe ──► Cross-Platform Publishing (gating awareness)
```

---

## Priority Ranking

| Rank | Capability | Impact | Effort | Rationale |
|------|-----------|--------|--------|-----------|
| 1 | Content Repurposing Engine | **Very High** | Medium | Biggest time-saver. Every creator does this manually. One article → 6 posts is the highest-leverage automation. |
| 2 | Marketing Plan Generator | **High** | Medium | Replaces the Google Docs workflow entirely. Generates the thing that drives all other content. |
| 3 | Voice Convergence | **High** | Medium | Quality gate that makes all other automation trustworthy. Without voice consistency, AI content is obviously AI content. |
| 4 | Cross-Platform Publishing | **High** | High | Replaces Buffer. But high effort due to 6+ API integrations. Build after repurposing engine so there's content to publish. |
| 5 | Editorial Calendar | **Medium** | Medium | Valuable but partially handled by the combination of Marketing Plan + Content Repurposing + Publishing queue. |
| 6 | Campaign Arc Builder | **Medium** | Low | Structurally simple — it's metadata on top of the editorial calendar. Build after calendar. |
| 7 | Carousel Creator | **Medium** | High | High effort because of visual rendering. Text content generation is straightforward; image generation is a whole pipeline. |
