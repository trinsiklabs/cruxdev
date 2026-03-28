# BUILD_PLAN_105: X Posting Engine — Rate-Limited, Queue-Based, Scheduled

**Status:** CONVERGED
**Priority:** Critical (prevents spam, enforces brand discipline)
**Triggered by:** Incident on 2026-03-28 where 56 tweets were posted in rapid succession via ad-hoc scripts, violating the 30-minute max rule. All had to be deleted.

## The Rule

**No direct social posting. Ever.** All posting goes through the engine. The engine enforces rate limits in code. No exceptions.

## Architecture

```
Content Generation → Queue (JSONL) → Scheduler (Rust) → X API (OAuth 1.0a)
                                         ↓
                                    Rate Limiter
                                    (30-min min interval)
                                         ↓
                                    Audit Trail
                                    (posted.jsonl)
```

## Phase 1: Queue Module (Rust)

### 1a. Queue data structure
- File: `rust/src/growth/x_queue.rs`
- Queue stored as `.cruxdev/growth/x_queue.jsonl`
- Each entry: `{ id, content, scheduled_at, status, created_at, posted_at, tweet_id }`
- Status enum: `Pending`, `Scheduled`, `Posted`, `Failed`, `Cancelled`
- Atomic writes (write-then-rename)

### 1b. Queue operations
- `enqueue(content: &str, scheduled_at: Option<DateTime>)` → adds to queue
- `dequeue_next()` → returns next pending item whose scheduled_at <= now
- `cancel(id)` → marks as Cancelled
- `list_pending()` → all Pending/Scheduled items
- `list_posted(limit)` → recent posted items
- `prune_old(days)` → remove old entries

### 1c. Rate limiter
- Hard minimum: 30 minutes between posts
- Check `posted.jsonl` for last post timestamp
- `can_post_now() -> bool` — returns false if < 30 min since last post
- `next_post_time() -> DateTime` — when the next post is allowed
- **This is code, not configuration.** The 30-minute minimum cannot be overridden without changing the source.

### 1d. Tests
- [ ] test_enqueue_creates_entry
- [ ] test_dequeue_respects_schedule
- [ ] test_rate_limiter_blocks_within_30_min
- [ ] test_rate_limiter_allows_after_30_min
- [ ] test_cancel_removes_from_queue
- [ ] test_list_pending_ordered_by_schedule
- [ ] test_atomic_write_on_enqueue
- [ ] test_prune_old_entries

## Phase 2: X API Client (Rust)

### 2a. OAuth 1.0a client
- File: `rust/src/growth/x_client.rs`
- Pure Rust OAuth 1.0a signing (no Python/tweepy dependency)
- Uses `reqwest` + custom HMAC-SHA1 signing
- Reads credentials from environment: X_CLIENT_ID, X_CLIENT_SECRET, X_OAUTH_TOKEN, X_OAUTH_TOKEN_SECRET

### 2b. Tweet operations
- `post_tweet(text: &str) -> Result<TweetResponse>`
- `delete_tweet(id: &str) -> Result<()>`
- `get_rate_limit() -> Result<RateLimitInfo>`
- Response parsing: extract tweet ID, URL
- Error handling: 401 (auth), 403 (permissions), 429 (rate limit with backoff)
- Parse X API rate limit headers: `x-rate-limit-remaining`, `x-rate-limit-reset`
- Track remaining API quota — pause posting if approaching limit

### 2c. Tests
- [ ] test_oauth_signature_generation
- [ ] test_post_tweet_dry_run (mock HTTP)
- [ ] test_error_handling_401
- [ ] test_error_handling_429
- [ ] test_rate_limit_parsing
- [ ] test_x_api_429_backoff

## Phase 3: Scheduler (Rust)

### 3a. Scheduling logic
- File: `rust/src/growth/x_scheduler.rs`
- `schedule_optimal(content: &str)` — picks next available slot per X_POSTING_SCHEDULE_PATTERNS.md
- Optimal times: 7-9 AM, 12-1 PM, 5-7 PM local time
- If queue is empty and slot is available: schedule for next optimal slot
- If queue has items: schedule after last queued item + 30 min minimum

### 3b. Execution loop
- `process_queue()` — called by CruxBot's sentinel/cron
- Checks `can_post_now()`
- If yes: dequeue next, post via X client, update status
- If no: return next_post_time
- On failure: mark as Failed, log error, do not retry immediately

### 3c. Tests
- [ ] test_schedule_optimal_picks_good_time
- [ ] test_process_queue_posts_one
- [ ] test_process_queue_respects_rate_limit
- [ ] test_failure_marks_entry
- [ ] test_empty_queue_noop

## Phase 4: MCP Tools

### 4a. New tools in server.rs
- `queue_x_post(content, scheduled_at?)` — add to queue
- `list_x_queue(status_filter?)` — view queue
- `cancel_x_post(id)` — cancel a queued post
- `x_queue_status()` — next post time, queue depth, rate limit status
- `process_x_queue()` — manually trigger queue processing

### 4b. Integration with convergence
- Post-convergence action: `queue_x_post` (NOT direct posting)
- Content generation → queue → engine posts on schedule
- The convergence engine NEVER calls the X API directly

## Phase 5: CruxBot Integration

### 5a. Sentinel trigger
- Add `x_queue` watcher to sentinel
- Checks if queue has items ready to post
- Triggers full wake if items are ready and rate limit allows

### 5b. Cycle integration
- DELIVER beat checks X queue
- Posts one item per cycle (respecting rate limit)
- Logs to audit trail

## Phase 6: Delete Utility

### 6a. Bulk delete
- `delete_x_posts(tweet_ids: Vec<String>)` — rate-limited bulk delete
- Respects X API delete rate limits (waits between deletes)
- Used for cleanup scenarios like the 2026-03-28 incident

## Verification

```bash
cd rust && cargo test x_queue -- --nocapture
cd rust && cargo test x_client -- --nocapture
cd rust && cargo test x_scheduler -- --nocapture
cd rust && cargo clippy -- -D warnings
```

## The Standard

If content needs to be posted to X, it goes through the queue. The queue enforces the rate limit. The scheduler picks optimal times. The engine posts. No human, no ad-hoc script, no direct API calls. This is infrastructure, not a hack.
