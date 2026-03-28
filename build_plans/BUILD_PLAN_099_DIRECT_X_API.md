# BUILD_PLAN_099: Direct X/Twitter API — Bypass Typefully for Posting

**Status:** NOT STARTED
**Priority:** High (40 drafts stuck, no way to publish via API)
**Triggered by:** Typefully v2 API only creates drafts — no scheduling or publishing

## Context

Typefully's v2 API creates drafts but cannot schedule or publish them. 40 drafts are stuck. We need direct X API access to post autonomously following the X_POSTING_SCHEDULE_PATTERNS.md (3-3-2 rule).

## Phase 1: X API Setup

- [ ] 1.1 Create X Developer account at developer.x.com
- [ ] 1.2 Create app with OAuth 2.0 (tweet.write, tweet.read, users.read scopes)
- [ ] 1.3 Generate access token + refresh token
- [ ] 1.4 Store in env: X_ACCESS_TOKEN, X_CLIENT_ID, X_CLIENT_SECRET

## Phase 2: Direct Posting

- [ ] 2.1 New Rust module: `growth/x_api.rs`
- [ ] 2.2 POST https://api.x.com/2/tweets with OAuth 2.0 Bearer token
- [ ] 2.3 Thread support (reply_to for multi-tweet threads)
- [ ] 2.4 Media upload support (images for blog post previews)

## Phase 3: Local Scheduling Queue

- [ ] 3.1 `.cruxdev/growth/post_queue.json` — scheduled posts with target times
- [ ] 3.2 Follow X_POSTING_SCHEDULE_PATTERNS.md: 3 posts/day, 3hr spacing, optimal times
- [ ] 3.3 Sentinel watches queue, posts when scheduled time arrives
- [ ] 3.4 Burst protocol: 27 drafts → summary thread + 3-4 highlights/day over 7 days
- [ ] 3.5 Content type variety enforcement (don't post all patterns in a row)

## Phase 4: Typefully Fallback

- [ ] 4.1 Keep Typefully for draft creation (backup/preview)
- [ ] 4.2 Primary: post via X API directly
- [ ] 4.3 Fallback: if X API fails, create Typefully draft for manual posting
