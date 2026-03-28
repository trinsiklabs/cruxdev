# BUILD_PLAN_048: Wire Content Pipeline into Convergence Completion (Full BIP System)

**Status:** CONVERGED
**Priority:** High
**Depends on:** BP044 (content pipeline)

## Context

The content pipeline exists (`content_pipeline.rs`) with 16 event types, `classify_event()`, `generate_blog_post()`, and `generate_x_post()` — all working with tests. This plan wired the full BIP (Build-in-Public) system: content generation → Typefully posting → blog publishing → draft archival.

## What Was Implemented

### Post-Convergence Hook
- `generate_and_publish_convergence_content()` — async function called when convergence completes
- Creates `ContentEvent` from convergence metadata (plan name → event type inference)
- Classifies event, generates blog + X post drafts
- Auto-posts to Typefully if API key set and enabled in growth.toml
- Publishes blog posts to website repo (Astro-compatible frontmatter)
- Archives posted drafts to `.cruxdev/evolution/archive/`

### MCP Tools (3 new, total 55)
- `generate_content` (#53) — manual content generation for any of 16 event types
- `list_content_drafts` (#54) — list pending drafts from posts dir
- `publish_drafts` (#55) — publish all pending drafts to Typefully + blog (dry-run by default)

### Blog Publishing Pipeline
- Reads `content.blog_dir` from growth.toml
- Writes blog posts with Astro-compatible frontmatter (title, date, slug)
- Blog index page (`blog/index.astro`) dynamically lists all .md posts

### Growth Config Updated
- `content.website_repo` → `/Users/user/personal/cruxdev-dev`
- `content.blog_dir` → `/Users/user/personal/cruxdev-dev/src/pages/blog`
- `tool_count` → 55

### End-to-End Flow
```
Build plan converges
  → generate_and_publish_convergence_content()
    → classify_event() (FeatureShipped/GapClosed/BugFix)
    → generate_blog_post() → write to blog dir
    → generate_x_post() → post to Typefully
    → archive posted drafts
  → Return content_drafts in convergence result
```

## Verification

451 tests pass, 0 clippy warnings, 55 MCP tools.
