# Job Queue & Scheduling Patterns — JSONL-Based Local Queue for Rust

**Version:** 1.0
**Created:** 2026-03-28
**Scope:** Persistent local job queues with rate limiting, scheduling, and failure handling. Designed for CruxBot's posting queue but applicable to any daemon that needs durable task scheduling without external infrastructure.

**Crates used:** `serde`, `serde_json`, `chrono`, `uuid`, `fs2` (file locking), `anyhow`, `tokio`

---

## Table of Contents

1. [Design Principles](#1-design-principles)
2. [JSONL Queue File Format](#2-jsonl-queue-file-format)
3. [Job Status Lifecycle](#3-job-status-lifecycle)
4. [Core Data Structures](#4-core-data-structures)
5. [Atomic File Operations](#5-atomic-file-operations)
6. [Queue Operations](#6-queue-operations)
7. [Idempotency](#7-idempotency)
8. [Rate Limiting Strategies](#8-rate-limiting-strategies)
9. [Scheduling](#9-scheduling)
10. [Failure Handling](#10-failure-handling)
11. [Concurrency Safety](#11-concurrency-safety)
12. [Monitoring](#12-monitoring)
13. [Complete Queue Implementation](#13-complete-queue-implementation)
14. [Testing Patterns](#14-testing-patterns)

---

## 1. Design Principles

**No external infrastructure.** The queue is a JSONL file on disk. No Redis, no Postgres, no message broker. A single daemon process owns the queue. This is correct for CruxBot's use case: one process, one machine, low throughput (posts per hour, not per second).

**Append-only log with compaction.** New jobs append. Status changes rewrite the file (atomically). Periodic compaction prunes completed/cancelled jobs older than a retention window.

**Crash safety via atomic writes.** Every mutation writes to a temp file then renames. A crash mid-write leaves the previous version intact. No corruption.

**LLM Minimization.** The queue is pure Rust. No LLM calls for scheduling, rate limiting, or retry logic. The LLM produces content; the queue manages when and whether to post it.

---

## 2. JSONL Queue File Format

Each line is a self-contained JSON object. One job per line. The file is human-readable and `grep`-able.

```
{"id":"a1b2c3","kind":"post_typefully","status":"pending","created_at":"2026-03-28T10:00:00Z","scheduled_at":"2026-03-28T14:00:00Z","payload":{"content":"...","thread_id":null},"attempts":0,"max_attempts":3,"idempotency_key":"post-blog-rust-queues-2026-03-28"}
{"id":"d4e5f6","kind":"post_typefully","status":"completed","created_at":"2026-03-28T09:00:00Z","scheduled_at":"2026-03-28T12:00:00Z","completed_at":"2026-03-28T12:01:00Z","payload":{"content":"..."},"attempts":1,"max_attempts":3,"idempotency_key":"post-blog-atomic-ops-2026-03-28"}
```

**Why JSONL over SQLite?** For a posting queue doing < 100 jobs/day, JSONL is simpler to debug, diff, and version. SQLite is better when you need indexed queries over thousands of records. CruxBot already uses JSONL for other state files (SEO health, growth metrics), so this stays consistent.

---

## 3. Job Status Lifecycle

```
                 ┌──────────┐
                 │ Pending  │
                 └────┬─────┘
                      │ scheduled_at reached
                      ▼
                 ┌──────────┐
                 │Scheduled │
                 └────┬─────┘
                      │ rate limit allows + dequeued
                      ▼
                 ┌──────────┐
            ┌───▶│ Running  │◀──── retry
            │    └────┬─────┘
            │         │
            │    ┌────┴────┐
            │    ▼         ▼
       ┌─────────┐  ┌──────────┐
       │Completed│  │  Failed  │──── attempts < max → retry
       └─────────┘  └────┬─────┘
                         │ attempts >= max
                         ▼
                    ┌──────────┐
                    │DeadLetter│
                    └──────────┘

       ┌──────────┐
       │Cancelled │ ◀── explicit cancel (any state except Completed)
       └──────────┘
```

**Status transitions (valid):**

| From | To | Trigger |
|------|----|---------|
| Pending | Scheduled | `scheduled_at` is in the past or now |
| Pending | Cancelled | Explicit cancel |
| Scheduled | Running | Dequeued by processor, rate limit allows |
| Scheduled | Cancelled | Explicit cancel |
| Running | Completed | Success |
| Running | Failed | Error (will retry if attempts < max) |
| Failed | Running | Retry (after backoff) |
| Failed | DeadLetter | Max attempts exceeded |
| Failed | Cancelled | Explicit cancel |

---

## 4. Core Data Structures

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    DeadLetter,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    PostTypefully,
    PostGithub,
    SendNotification,
    RunGrowthCycle,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub kind: JobKind,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub scheduled_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    pub payload: serde_json::Value,
    pub attempts: u32,
    pub max_attempts: u32,
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new(kind: JobKind, payload: serde_json::Value, scheduled_at: DateTime<Utc>) -> Self {
        let id = Uuid::new_v4().to_string()[..8].to_string();
        let idempotency_key = format!(
            "{:?}-{}-{}",
            kind,
            scheduled_at.format("%Y-%m-%d"),
            &id
        );
        Self {
            id,
            kind,
            status: JobStatus::Pending,
            created_at: Utc::now(),
            scheduled_at,
            started_at: None,
            completed_at: None,
            failed_at: None,
            payload,
            attempts: 0,
            max_attempts: 3,
            idempotency_key,
            last_error: None,
            next_retry_at: None,
        }
    }

    /// Whether this job is eligible for processing right now.
    pub fn is_ready(&self, now: DateTime<Utc>) -> bool {
        match self.status {
            JobStatus::Pending => self.scheduled_at <= now,
            JobStatus::Scheduled => true,
            JobStatus::Failed => {
                self.attempts < self.max_attempts
                    && self.next_retry_at.map_or(true, |t| t <= now)
            }
            _ => false,
        }
    }

    /// Whether this job can be cancelled.
    pub fn is_cancellable(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Pending | JobStatus::Scheduled | JobStatus::Failed
        )
    }
}
```

---

## 5. Atomic File Operations

Every write to the queue file must be crash-safe. The pattern: write to a temporary file in the same directory, then atomically rename over the target.

```rust
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Atomically write content to a file using write-then-rename.
/// The temp file is created in the same directory to ensure same-filesystem rename.
pub fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    let dir = path.parent().context("path has no parent directory")?;
    let temp_path = dir.join(format!(
        ".tmp.{}.{}",
        path.file_name().unwrap().to_string_lossy(),
        std::process::id()
    ));

    let mut file = fs::File::create(&temp_path)
        .with_context(|| format!("failed to create temp file: {}", temp_path.display()))?;
    file.write_all(content)?;
    file.sync_all()?; // fsync before rename — ensures data is on disk
    fs::rename(&temp_path, path)
        .with_context(|| format!("failed to rename {} -> {}", temp_path.display(), path.display()))?;
    Ok(())
}

/// Atomically write a Vec<Job> as JSONL.
pub fn write_jobs(path: &Path, jobs: &[Job]) -> Result<()> {
    let mut buf = Vec::new();
    for job in jobs {
        serde_json::to_writer(&mut buf, job)?;
        buf.push(b'\n');
    }
    atomic_write(path, &buf)
}

/// Read all jobs from a JSONL file. Returns empty vec if file doesn't exist.
pub fn read_jobs(path: &Path) -> Result<Vec<Job>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let mut jobs = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let job: Job = serde_json::from_str(line)
            .with_context(|| format!("invalid JSON on line {}: {}", i + 1, line))?;
        jobs.push(job);
    }
    Ok(jobs)
}
```

**Key details:**
- `sync_all()` before rename ensures the data hits disk, not just the OS buffer.
- Temp file uses PID in name to avoid collisions if multiple processes exist (shouldn't happen, but defense in depth).
- Same directory for temp file guarantees same-filesystem rename (atomic on POSIX).

---

## 6. Queue Operations

### Enqueue

```rust
use fs2::FileExt;
use std::fs::OpenOptions;

pub struct JobQueue {
    path: std::path::PathBuf,
    lock_path: std::path::PathBuf,
}

impl JobQueue {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        let path = path.into();
        let lock_path = path.with_extension("lock");
        Self { path, lock_path }
    }

    /// Acquire an exclusive file lock. Returns a guard that releases on drop.
    fn lock(&self) -> Result<fs::File> {
        let lock_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&self.lock_path)?;
        lock_file.lock_exclusive()?; // blocks until acquired
        Ok(lock_file)
    }

    /// Add a job to the queue. Returns the job ID.
    pub fn enqueue(&self, job: Job) -> Result<String> {
        let _lock = self.lock()?;
        let mut jobs = read_jobs(&self.path)?;

        // Idempotency check — reject duplicate keys
        if jobs.iter().any(|j| j.idempotency_key == job.idempotency_key) {
            anyhow::bail!(
                "duplicate idempotency key: {}",
                job.idempotency_key
            );
        }

        let id = job.id.clone();
        jobs.push(job);
        write_jobs(&self.path, &jobs)?;
        Ok(id)
    }

    /// Peek at the next ready job without changing its status.
    pub fn peek(&self) -> Result<Option<Job>> {
        let _lock = self.lock()?;
        let jobs = read_jobs(&self.path)?;
        let now = Utc::now();
        Ok(jobs.into_iter().find(|j| j.is_ready(now)))
    }

    /// Dequeue the next ready job: mark it Running and return it.
    pub fn dequeue(&self) -> Result<Option<Job>> {
        let _lock = self.lock()?;
        let mut jobs = read_jobs(&self.path)?;
        let now = Utc::now();

        // Find first ready job, ordered by scheduled_at
        let idx = jobs
            .iter()
            .enumerate()
            .filter(|(_, j)| j.is_ready(now))
            .min_by_key(|(_, j)| j.scheduled_at)
            .map(|(i, _)| i);

        match idx {
            Some(i) => {
                jobs[i].status = JobStatus::Running;
                jobs[i].started_at = Some(now);
                jobs[i].attempts += 1;
                let job = jobs[i].clone();
                write_jobs(&self.path, &jobs)?;
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    /// Mark a job as completed.
    pub fn complete(&self, job_id: &str) -> Result<()> {
        self.update_status(job_id, |job| {
            job.status = JobStatus::Completed;
            job.completed_at = Some(Utc::now());
        })
    }

    /// Mark a job as failed, with retry scheduling.
    pub fn fail(&self, job_id: &str, error: &str) -> Result<()> {
        self.update_status(job_id, |job| {
            let now = Utc::now();
            job.failed_at = Some(now);
            job.last_error = Some(error.to_string());
            if job.attempts >= job.max_attempts {
                job.status = JobStatus::DeadLetter;
            } else {
                job.status = JobStatus::Failed;
                // Exponential backoff: 1min, 4min, 16min, ...
                let backoff_minutes = 4_i64.pow(job.attempts.saturating_sub(1));
                job.next_retry_at =
                    Some(now + chrono::Duration::minutes(backoff_minutes));
            }
        })
    }

    /// Cancel a job. Returns error if job is not cancellable.
    pub fn cancel(&self, job_id: &str) -> Result<()> {
        self.update_status(job_id, |job| {
            if !job.is_cancellable() {
                // This will be checked after the closure runs
                return;
            }
            job.status = JobStatus::Cancelled;
        })
    }

    /// Remove completed/cancelled/dead-letter jobs older than `retention`.
    pub fn prune(&self, retention: chrono::Duration) -> Result<usize> {
        let _lock = self.lock()?;
        let jobs = read_jobs(&self.path)?;
        let cutoff = Utc::now() - retention;
        let before = jobs.len();
        let kept: Vec<Job> = jobs
            .into_iter()
            .filter(|j| {
                match j.status {
                    JobStatus::Completed | JobStatus::Cancelled | JobStatus::DeadLetter => {
                        // Keep if completed recently
                        j.completed_at.unwrap_or(j.created_at) > cutoff
                    }
                    _ => true, // Always keep active jobs
                }
            })
            .collect();
        let pruned = before - kept.len();
        write_jobs(&self.path, &kept)?;
        Ok(pruned)
    }

    /// Generic status update helper.
    fn update_status(&self, job_id: &str, f: impl FnOnce(&mut Job)) -> Result<()> {
        let _lock = self.lock()?;
        let mut jobs = read_jobs(&self.path)?;
        let job = jobs
            .iter_mut()
            .find(|j| j.id == job_id)
            .context(format!("job not found: {job_id}"))?;
        f(job);
        write_jobs(&self.path, &jobs)?;
        Ok(())
    }
}
```

---

## 7. Idempotency

Double-processing is the most dangerous failure mode for a posting queue. Posting the same content twice looks unprofessional and may trigger platform rate limits or spam detection.

**Three layers of defense:**

### Layer 1: Idempotency key on enqueue

Every job has an `idempotency_key`. The enqueue operation rejects duplicates. The key should encode the *intent*, not just a random ID:

```rust
// Good: encodes what we're posting and when
let key = format!("post-blog-{slug}-{date}");

// Bad: random UUID tells us nothing
let key = Uuid::new_v4().to_string();
```

### Layer 2: Status check before execution

The dequeue operation atomically transitions to `Running`. If the process crashes after dequeue but before completion, the job stays `Running`. On restart, detect stuck jobs:

```rust
impl JobQueue {
    /// Find jobs stuck in Running for longer than `timeout`.
    pub fn recover_stuck(&self, timeout: chrono::Duration) -> Result<Vec<String>> {
        let _lock = self.lock()?;
        let mut jobs = read_jobs(&self.path)?;
        let cutoff = Utc::now() - timeout;
        let mut recovered = Vec::new();

        for job in &mut jobs {
            if job.status == JobStatus::Running {
                if let Some(started) = job.started_at {
                    if started < cutoff {
                        // Reset to Failed so retry logic handles it
                        job.status = JobStatus::Failed;
                        job.last_error = Some("recovered: stuck in running state".into());
                        recovered.push(job.id.clone());
                    }
                }
            }
        }

        if !recovered.is_empty() {
            write_jobs(&self.path, &jobs)?;
        }
        Ok(recovered)
    }
}
```

### Layer 3: Platform-side deduplication

Before posting, check the platform's recent posts for duplicate content. This catches the case where a post succeeded but the completion acknowledgment was lost:

```rust
/// Check if content was already posted (platform-side dedup).
async fn already_posted(client: &HttpClient, content_hash: &str) -> Result<bool> {
    let recent = client.get_recent_posts(10).await?;
    Ok(recent.iter().any(|p| hash_content(&p.content) == content_hash))
}
```

---

## 8. Rate Limiting Strategies

### Strategy 1: Hard Minimum Interval

The simplest and most appropriate for posting queues. Enforces a minimum time gap between executions of the same job kind.

```rust
use std::collections::HashMap;

pub struct MinimumIntervalLimiter {
    /// Minimum gap between jobs of each kind.
    intervals: HashMap<JobKind, chrono::Duration>,
}

impl MinimumIntervalLimiter {
    pub fn new() -> Self {
        let mut intervals = HashMap::new();
        // At most one social post every 30 minutes
        intervals.insert(
            JobKind::PostTypefully,
            chrono::Duration::minutes(30),
        );
        // At most one GitHub post every 60 minutes
        intervals.insert(
            JobKind::PostGithub,
            chrono::Duration::minutes(60),
        );
        Self { intervals }
    }

    /// Check if a job of this kind can run now, given the last completion time.
    pub fn allows(&self, kind: &JobKind, last_completed: Option<DateTime<Utc>>) -> bool {
        let interval = match self.intervals.get(kind) {
            Some(d) => *d,
            None => return true, // No limit configured
        };
        match last_completed {
            None => true, // Never run before
            Some(t) => Utc::now() - t >= interval,
        }
    }

    /// How long until the next job of this kind can run.
    pub fn time_until_allowed(
        &self,
        kind: &JobKind,
        last_completed: Option<DateTime<Utc>>,
    ) -> chrono::Duration {
        let interval = match self.intervals.get(kind) {
            Some(d) => *d,
            None => return chrono::Duration::zero(),
        };
        match last_completed {
            None => chrono::Duration::zero(),
            Some(t) => {
                let elapsed = Utc::now() - t;
                if elapsed >= interval {
                    chrono::Duration::zero()
                } else {
                    interval - elapsed
                }
            }
        }
    }
}
```

### Strategy 2: Token Bucket

For APIs with burst allowances. Allows short bursts but enforces a sustained rate over time.

```rust
pub struct TokenBucket {
    capacity: u32,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: DateTime<Utc>,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_per_minute: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate: refill_per_minute / 60.0,
            last_refill: Utc::now(),
        }
    }

    /// Try to consume one token. Returns true if allowed.
    pub fn try_acquire(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Seconds until the next token is available.
    pub fn time_until_available(&mut self) -> f64 {
        self.refill();
        if self.tokens >= 1.0 {
            0.0
        } else {
            (1.0 - self.tokens) / self.refill_rate
        }
    }

    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).num_milliseconds() as f64 / 1000.0;
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }
}
```

**Token bucket is the right choice when:**
- The API has both a burst limit (e.g., 10 requests) and a sustained limit (e.g., 100/hour)
- You want to allow quick bursts after idle periods

### Strategy 3: Sliding Window Counter

Counts events in a rolling time window. More accurate than fixed windows, simpler than token buckets.

```rust
pub struct SlidingWindowLimiter {
    window: chrono::Duration,
    max_events: u32,
    /// Timestamps of recent events, kept sorted.
    events: Vec<DateTime<Utc>>,
}

impl SlidingWindowLimiter {
    pub fn new(window: chrono::Duration, max_events: u32) -> Self {
        Self {
            window,
            max_events,
            events: Vec::new(),
        }
    }

    /// Record an event and check if it's within limits.
    /// Returns false if the limit would be exceeded (event is NOT recorded).
    pub fn try_record(&mut self) -> bool {
        let now = Utc::now();
        self.prune_old(now);
        if self.events.len() as u32 >= self.max_events {
            false
        } else {
            self.events.push(now);
            true
        }
    }

    pub fn current_count(&mut self) -> u32 {
        self.prune_old(Utc::now());
        self.events.len() as u32
    }

    fn prune_old(&mut self, now: DateTime<Utc>) {
        let cutoff = now - self.window;
        self.events.retain(|t| *t > cutoff);
    }
}
```

### Strategy 4: Fixed Window

Simplest counter. Resets at fixed boundaries (e.g., top of each hour). Can allow burst at window edges.

```rust
pub struct FixedWindowLimiter {
    window_seconds: i64,
    max_events: u32,
    window_start: i64, // unix timestamp of current window start
    count: u32,
}

impl FixedWindowLimiter {
    pub fn new(window_seconds: i64, max_events: u32) -> Self {
        let now = Utc::now().timestamp();
        Self {
            window_seconds,
            max_events,
            window_start: now - (now % window_seconds),
            count: 0,
        }
    }

    pub fn try_record(&mut self) -> bool {
        let now = Utc::now().timestamp();
        let current_window = now - (now % self.window_seconds);
        if current_window != self.window_start {
            self.window_start = current_window;
            self.count = 0;
        }
        if self.count >= self.max_events {
            false
        } else {
            self.count += 1;
            true
        }
    }
}
```

### Which strategy to use for CruxBot

| Strategy | Use case |
|----------|----------|
| **Hard minimum interval** | Social posting (30min between posts) — primary choice |
| Token bucket | API calls with burst/sustained limits (Typefully API) |
| Sliding window | Daily post caps (max 6 posts per 24h) |
| Fixed window | Hourly cost budgets (reset each hour) |

**Recommended: layer minimum interval + sliding window.** The interval prevents rapid-fire posting. The sliding window enforces daily caps.

```rust
pub struct PostingRateLimiter {
    interval: MinimumIntervalLimiter,
    daily_cap: SlidingWindowLimiter,
}

impl PostingRateLimiter {
    pub fn new() -> Self {
        Self {
            interval: MinimumIntervalLimiter::new(),
            daily_cap: SlidingWindowLimiter::new(
                chrono::Duration::hours(24),
                6, // max 6 posts per 24h
            ),
        }
    }

    pub fn allows(&mut self, kind: &JobKind, last_completed: Option<DateTime<Utc>>) -> bool {
        self.interval.allows(kind, last_completed)
            && self.daily_cap.current_count() < 6
    }
}
```

---

## 9. Scheduling

### Cron-like scheduling

For jobs that recur on a schedule. Use the `cron` crate or implement a minimal next-occurrence calculator.

```rust
/// Simplified cron-like schedule: specific hours on specific days.
/// Full cron parsing can use the `cron` crate if needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostingSchedule {
    /// Hours of day (UTC) when posting is allowed.
    pub allowed_hours: Vec<u32>,
    /// Days of week (0=Mon, 6=Sun) when posting is allowed.
    pub allowed_days: Vec<u32>,
    /// Minimum minutes between posts.
    pub min_interval_minutes: u32,
}

impl PostingSchedule {
    /// Default: weekdays, business hours (9-17 UTC), 30min apart.
    pub fn default_social() -> Self {
        Self {
            allowed_hours: (9..=17).collect(),
            allowed_days: (0..=4).collect(), // Mon-Fri
            min_interval_minutes: 30,
        }
    }

    /// Find the next allowed posting time at or after `from`.
    pub fn next_slot(&self, from: DateTime<Utc>) -> DateTime<Utc> {
        let mut candidate = from;
        loop {
            let weekday = candidate.weekday().num_days_from_monday();
            let hour = candidate.hour();

            if self.allowed_days.contains(&weekday)
                && self.allowed_hours.contains(&hour)
            {
                return candidate;
            }

            // Advance to next hour
            candidate = candidate
                .date_naive()
                .and_hms_opt(hour + 1, 0, 0)
                .map(|dt| dt.and_utc())
                .unwrap_or_else(|| {
                    // Rolled past midnight, advance to next day 00:00
                    (candidate.date_naive() + chrono::Duration::days(1))
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .and_utc()
                });
        }
    }
}
```

### Optimal time slot selection

When scheduling a post, pick the slot that maximizes engagement based on historical data.

```rust
/// Given a set of allowed slots, pick the one with the best historical engagement.
/// Falls back to the earliest slot if no data exists.
pub fn pick_optimal_slot(
    schedule: &PostingSchedule,
    from: DateTime<Utc>,
    engagement_data: &HashMap<u32, f64>, // hour -> avg engagement score
) -> DateTime<Utc> {
    let mut best_slot = schedule.next_slot(from);
    let mut best_score = engagement_data
        .get(&best_slot.hour())
        .copied()
        .unwrap_or(0.0);

    // Check the next 24 hours of slots
    let mut candidate = best_slot + chrono::Duration::hours(1);
    let horizon = from + chrono::Duration::hours(24);

    while candidate < horizon {
        let slot = schedule.next_slot(candidate);
        if slot >= horizon {
            break;
        }
        let score = engagement_data
            .get(&slot.hour())
            .copied()
            .unwrap_or(0.0);
        if score > best_score {
            best_score = score;
            best_slot = slot;
        }
        candidate = slot + chrono::Duration::hours(1);
    }

    best_slot
}
```

---

## 10. Failure Handling

### Retry with exponential backoff

```rust
/// Calculate the next retry time using exponential backoff with jitter.
pub fn next_retry_time(attempt: u32) -> DateTime<Utc> {
    // Base: 1 min, multiplied by 4^(attempt-1)
    // Attempt 1: 1 min, Attempt 2: 4 min, Attempt 3: 16 min
    let base_seconds = 60_i64 * 4_i64.pow(attempt.saturating_sub(1));

    // Add jitter: +/- 20% to prevent thundering herd
    let jitter_range = base_seconds / 5;
    let jitter = if jitter_range > 0 {
        // Simple deterministic jitter from attempt number
        (attempt as i64 * 7919) % (jitter_range * 2) - jitter_range
    } else {
        0
    };

    Utc::now() + chrono::Duration::seconds(base_seconds + jitter)
}
```

### Error classification

Not all errors should be retried. Classify errors to decide the correct response.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorClass {
    /// Temporary failure — retry after backoff (network timeout, 503, rate limit)
    Transient,
    /// Permanent failure — do not retry (400 bad request, 401 auth, invalid content)
    Permanent,
    /// Unknown — retry with caution (unexpected status codes)
    Unknown,
}

pub fn classify_error(status: Option<u16>, message: &str) -> ErrorClass {
    match status {
        Some(429) => ErrorClass::Transient, // Rate limited
        Some(503 | 502 | 504) => ErrorClass::Transient, // Server overloaded
        Some(400 | 401 | 403 | 404 | 422) => ErrorClass::Permanent, // Client error
        Some(200..=299) => ErrorClass::Permanent, // Shouldn't happen, but don't retry success
        None if message.contains("timeout") => ErrorClass::Transient,
        None if message.contains("connection") => ErrorClass::Transient,
        _ => ErrorClass::Unknown,
    }
}

/// Handle a job failure with error classification.
pub fn handle_failure(queue: &JobQueue, job_id: &str, error: &str, status: Option<u16>) -> Result<()> {
    match classify_error(status, error) {
        ErrorClass::Permanent => {
            // Skip retry, go straight to dead letter
            queue.update_status(job_id, |job| {
                job.status = JobStatus::DeadLetter;
                job.last_error = Some(format!("permanent: {error}"));
                job.failed_at = Some(Utc::now());
            })
        }
        ErrorClass::Transient | ErrorClass::Unknown => {
            queue.fail(job_id, error)
        }
    }
}
```

### Dead letter queue

Jobs that exhaust their retries move to `DeadLetter` status. They stay in the JSONL file for inspection but are never re-processed automatically.

To review dead-lettered jobs:

```bash
# List all dead-lettered jobs
grep '"dead_letter"' queue.jsonl | jq '{id, kind, last_error, attempts}'
```

To manually retry a dead-lettered job, reset its status:

```rust
impl JobQueue {
    /// Manually retry a dead-lettered job. Resets attempts and moves to Pending.
    pub fn resurrect(&self, job_id: &str) -> Result<()> {
        self.update_status(job_id, |job| {
            if job.status != JobStatus::DeadLetter {
                return;
            }
            job.status = JobStatus::Pending;
            job.attempts = 0;
            job.last_error = None;
            job.next_retry_at = None;
            job.scheduled_at = Utc::now(); // Schedule for immediate retry
        })
    }
}
```

---

## 11. Concurrency Safety

### File locking with `fs2`

The `fs2` crate provides cross-platform advisory file locking. All queue operations acquire an exclusive lock before reading or writing.

```toml
# Cargo.toml
[dependencies]
fs2 = "0.4"
```

```rust
use fs2::FileExt;
use std::fs::OpenOptions;

/// RAII lock guard. Lock is released when the File is dropped.
fn acquire_lock(lock_path: &Path) -> Result<fs::File> {
    let lock_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
        .context("failed to open lock file")?;

    // Blocking exclusive lock
    lock_file
        .lock_exclusive()
        .context("failed to acquire exclusive lock")?;

    Ok(lock_file)
    // Lock released when `lock_file` is dropped
}
```

**Important:** `fs2` uses `flock()` on Unix and `LockFileEx` on Windows. These are **advisory** locks — they only work if all processes use them. Since CruxBot is the only process accessing the queue, this is sufficient.

### Single-writer architecture

For CruxBot, the strongest concurrency guarantee is architectural: **only one process writes to the queue.** The sentinel loop is single-threaded for queue operations. File locking is a safety net, not the primary concurrency mechanism.

```rust
/// The queue processor runs as a single task in the tokio runtime.
/// No concurrent dequeue operations — process one job at a time.
pub async fn process_loop(queue: &JobQueue, limiter: &mut PostingRateLimiter) {
    loop {
        // Recover any stuck jobs from previous crash
        if let Ok(recovered) = queue.recover_stuck(chrono::Duration::minutes(15)) {
            for id in &recovered {
                tracing::warn!(job_id = %id, "recovered stuck job");
            }
        }

        // Try to dequeue and process
        match queue.dequeue() {
            Ok(Some(job)) => {
                let last = last_completion_time(queue, &job.kind);
                if !limiter.allows(&job.kind, last) {
                    // Put it back — not ready yet
                    let _ = queue.update_status(&job.id, |j| {
                        j.status = JobStatus::Scheduled;
                        j.started_at = None;
                        j.attempts -= 1; // Don't count rate-limit holds as attempts
                    });
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    continue;
                }

                tracing::info!(job_id = %job.id, kind = ?job.kind, "processing job");
                match execute_job(&job).await {
                    Ok(()) => {
                        let _ = queue.complete(&job.id);
                        tracing::info!(job_id = %job.id, "job completed");
                    }
                    Err(e) => {
                        let status = extract_status(&e);
                        let _ = handle_failure(queue, &job.id, &e.to_string(), status);
                        tracing::error!(job_id = %job.id, error = %e, "job failed");
                    }
                }
            }
            Ok(None) => {
                // Nothing ready — sleep before checking again
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
            Err(e) => {
                tracing::error!(error = %e, "queue dequeue error");
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        }

        // Periodic compaction — prune completed jobs older than 7 days
        let _ = queue.prune(chrono::Duration::days(7));
    }
}
```

---

## 12. Monitoring

### Queue metrics struct

```rust
#[derive(Debug, Clone, Serialize)]
pub struct QueueMetrics {
    pub total_jobs: usize,
    pub pending: usize,
    pub scheduled: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub dead_letter: usize,
    pub cancelled: usize,
    pub oldest_pending_age_seconds: Option<i64>,
    pub avg_completion_seconds: Option<f64>,
    pub failure_rate: f64, // failed / (completed + failed)
}

impl JobQueue {
    pub fn metrics(&self) -> Result<QueueMetrics> {
        let _lock = self.lock()?;
        let jobs = read_jobs(&self.path)?;
        let now = Utc::now();

        let mut m = QueueMetrics {
            total_jobs: jobs.len(),
            pending: 0,
            scheduled: 0,
            running: 0,
            completed: 0,
            failed: 0,
            dead_letter: 0,
            cancelled: 0,
            oldest_pending_age_seconds: None,
            avg_completion_seconds: None,
            failure_rate: 0.0,
        };

        let mut completion_times = Vec::new();

        for job in &jobs {
            match job.status {
                JobStatus::Pending => {
                    m.pending += 1;
                    let age = (now - job.created_at).num_seconds();
                    m.oldest_pending_age_seconds = Some(
                        m.oldest_pending_age_seconds.map_or(age, |prev| prev.max(age)),
                    );
                }
                JobStatus::Scheduled => m.scheduled += 1,
                JobStatus::Running => m.running += 1,
                JobStatus::Completed => {
                    m.completed += 1;
                    if let (Some(started), Some(completed)) =
                        (job.started_at, job.completed_at)
                    {
                        completion_times.push((completed - started).num_seconds() as f64);
                    }
                }
                JobStatus::Failed => m.failed += 1,
                JobStatus::DeadLetter => m.dead_letter += 1,
                JobStatus::Cancelled => m.cancelled += 1,
            }
        }

        if !completion_times.is_empty() {
            let sum: f64 = completion_times.iter().sum();
            m.avg_completion_seconds = Some(sum / completion_times.len() as f64);
        }

        let terminal = (m.completed + m.failed + m.dead_letter) as f64;
        if terminal > 0.0 {
            m.failure_rate = (m.failed + m.dead_letter) as f64 / terminal;
        }

        Ok(m)
    }
}
```

### Logging integration

The process loop uses `tracing` for structured logging. Key events to log:

| Event | Level | Fields |
|-------|-------|--------|
| Job enqueued | INFO | `job_id`, `kind`, `scheduled_at` |
| Job dequeued | INFO | `job_id`, `kind`, `attempt` |
| Job completed | INFO | `job_id`, `duration_ms` |
| Job failed (transient) | WARN | `job_id`, `error`, `next_retry_at` |
| Job dead-lettered | ERROR | `job_id`, `error`, `total_attempts` |
| Stuck job recovered | WARN | `job_id`, `stuck_duration` |
| Rate limit held | DEBUG | `kind`, `time_until_allowed` |
| Queue pruned | DEBUG | `pruned_count`, `remaining_count` |

### Health check

```rust
/// Quick health check for the queue. Returns issues if any.
pub fn health_check(queue: &JobQueue) -> Vec<String> {
    let mut issues = Vec::new();

    match queue.metrics() {
        Err(e) => {
            issues.push(format!("cannot read queue: {e}"));
            return issues;
        }
        Ok(m) => {
            if m.running > 1 {
                issues.push(format!(
                    "multiple running jobs ({}): possible stuck jobs",
                    m.running
                ));
            }
            if m.dead_letter > 0 {
                issues.push(format!(
                    "{} dead-lettered jobs need attention",
                    m.dead_letter
                ));
            }
            if m.failure_rate > 0.5 {
                issues.push(format!(
                    "high failure rate: {:.0}%",
                    m.failure_rate * 100.0
                ));
            }
            if let Some(age) = m.oldest_pending_age_seconds {
                if age > 3600 {
                    issues.push(format!(
                        "oldest pending job is {} hours old",
                        age / 3600
                    ));
                }
            }
        }
    }

    issues
}
```

---

## 13. Complete Queue Implementation

Putting it all together — the recommended file layout for CruxBot:

```
src/
├── queue/
│   ├── mod.rs          # pub mod job, store, limiter, schedule, monitor;
│   ├── job.rs          # Job struct, JobStatus, JobKind, status transitions
│   ├── store.rs        # JobQueue: JSONL read/write, atomic ops, file locking
│   ├── limiter.rs      # Rate limiters: MinimumInterval, TokenBucket, SlidingWindow
│   ├── schedule.rs     # PostingSchedule, optimal slot selection
│   └── monitor.rs      # QueueMetrics, health_check
```

**Cargo.toml additions:**

```toml
[dependencies]
fs2 = "0.4"
# serde, serde_json, chrono, uuid, anyhow, tokio, tracing already present
```

### Integration with CruxBot's sentinel loop

```rust
// In sentinel.rs or cycle.rs — add queue processing to the wake cycle

use crate::queue::{JobQueue, PostingRateLimiter};

pub async fn run_posting_queue(
    queue: &JobQueue,
    limiter: &mut PostingRateLimiter,
    typefully_api_key: &str,
) -> Result<u32> {
    let mut processed = 0;

    // Process up to 3 jobs per wake cycle
    for _ in 0..3 {
        match queue.dequeue()? {
            Some(job) => {
                let last = queue.last_completion_for_kind(&job.kind)?;
                if !limiter.allows(&job.kind, last) {
                    // Revert dequeue
                    queue.update_status(&job.id, |j| {
                        j.status = JobStatus::Scheduled;
                        j.started_at = None;
                        j.attempts -= 1;
                    })?;
                    break; // Can't post yet, stop trying
                }

                match execute_post(&job, typefully_api_key).await {
                    Ok(()) => {
                        queue.complete(&job.id)?;
                        processed += 1;
                    }
                    Err(e) => {
                        let status = extract_http_status(&e);
                        handle_failure(queue, &job.id, &e.to_string(), status)?;
                    }
                }
            }
            None => break,
        }
    }

    Ok(processed)
}
```

---

## 14. Testing Patterns

Every component should be tested in isolation using `tempfile` for queue files.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_queue() -> (TempDir, JobQueue) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test_queue.jsonl");
        let queue = JobQueue::new(path);
        (dir, queue)
    }

    #[test]
    fn enqueue_and_dequeue() {
        let (_dir, queue) = test_queue();
        let job = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({"content": "hello"}),
            Utc::now() - chrono::Duration::minutes(1), // already ready
        );
        let id = queue.enqueue(job).unwrap();
        let dequeued = queue.dequeue().unwrap().unwrap();
        assert_eq!(dequeued.id, id);
        assert_eq!(dequeued.status, JobStatus::Running);
        assert_eq!(dequeued.attempts, 1);
    }

    #[test]
    fn idempotency_rejects_duplicate() {
        let (_dir, queue) = test_queue();
        let mut job1 = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({}),
            Utc::now(),
        );
        job1.idempotency_key = "unique-key".into();
        queue.enqueue(job1).unwrap();

        let mut job2 = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({}),
            Utc::now(),
        );
        job2.idempotency_key = "unique-key".into();
        assert!(queue.enqueue(job2).is_err());
    }

    #[test]
    fn fail_and_retry() {
        let (_dir, queue) = test_queue();
        let job = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({}),
            Utc::now() - chrono::Duration::hours(1),
        );
        let id = queue.enqueue(job).unwrap();
        queue.dequeue().unwrap(); // moves to Running
        queue.fail(&id, "timeout").unwrap();

        // Job should be Failed, not DeadLetter (only 1 attempt)
        let jobs = read_jobs(&queue.path).unwrap();
        let job = jobs.iter().find(|j| j.id == id).unwrap();
        assert_eq!(job.status, JobStatus::Failed);
        assert!(job.next_retry_at.is_some());
    }

    #[test]
    fn max_attempts_leads_to_dead_letter() {
        let (_dir, queue) = test_queue();
        let mut job = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({}),
            Utc::now() - chrono::Duration::hours(1),
        );
        job.max_attempts = 2;
        let id = queue.enqueue(job).unwrap();

        // Attempt 1: dequeue + fail
        queue.dequeue().unwrap();
        queue.fail(&id, "error 1").unwrap();

        // Attempt 2: dequeue + fail — should be last attempt
        // Manually make retry eligible by clearing next_retry_at
        queue.update_status(&id, |j| {
            j.next_retry_at = None;
        }).unwrap();
        queue.dequeue().unwrap();
        queue.fail(&id, "error 2").unwrap();

        let jobs = read_jobs(&queue.path).unwrap();
        let job = jobs.iter().find(|j| j.id == id).unwrap();
        assert_eq!(job.status, JobStatus::DeadLetter);
    }

    #[test]
    fn prune_removes_old_completed() {
        let (_dir, queue) = test_queue();
        let job = Job::new(
            JobKind::PostTypefully,
            serde_json::json!({}),
            Utc::now() - chrono::Duration::days(10),
        );
        let id = queue.enqueue(job).unwrap();
        queue.dequeue().unwrap();
        queue.complete(&id).unwrap();

        // Prune with 7-day retention
        let pruned = queue.prune(chrono::Duration::days(7)).unwrap();
        assert_eq!(pruned, 1);
        assert!(read_jobs(&queue.path).unwrap().is_empty());
    }

    #[test]
    fn minimum_interval_limiter() {
        let limiter = MinimumIntervalLimiter::new();

        // No previous completion — should allow
        assert!(limiter.allows(&JobKind::PostTypefully, None));

        // Recent completion — should block
        let recent = Utc::now() - chrono::Duration::minutes(10);
        assert!(!limiter.allows(&JobKind::PostTypefully, Some(recent)));

        // Old completion — should allow
        let old = Utc::now() - chrono::Duration::hours(1);
        assert!(limiter.allows(&JobKind::PostTypefully, Some(old)));
    }

    #[test]
    fn sliding_window_enforces_cap() {
        let mut limiter = SlidingWindowLimiter::new(
            chrono::Duration::hours(1),
            3,
        );
        assert!(limiter.try_record());
        assert!(limiter.try_record());
        assert!(limiter.try_record());
        assert!(!limiter.try_record()); // 4th should fail
    }

    #[test]
    fn token_bucket_basic() {
        let mut bucket = TokenBucket::new(2, 60.0); // 2 capacity, 1/sec refill
        assert!(bucket.try_acquire());
        assert!(bucket.try_acquire());
        assert!(!bucket.try_acquire()); // exhausted
    }

    #[test]
    fn metrics_calculation() {
        let (_dir, queue) = test_queue();

        // Enqueue 3 jobs
        for i in 0..3 {
            let mut job = Job::new(
                JobKind::PostTypefully,
                serde_json::json!({"n": i}),
                Utc::now() - chrono::Duration::hours(1),
            );
            job.idempotency_key = format!("key-{i}");
            queue.enqueue(job).unwrap();
        }

        let m = queue.metrics().unwrap();
        assert_eq!(m.total_jobs, 3);
        assert_eq!(m.pending, 3);
    }
}
```

---

## Summary: Decision Matrix

| Decision | Recommendation | Rationale |
|----------|---------------|-----------|
| Storage format | JSONL | Human-readable, grep-able, consistent with CruxBot's existing state files |
| Concurrency model | Single-writer + file lock | CruxBot is one process; file lock is a safety net |
| Primary rate limiter | Minimum interval (30 min) | Social posting cadence needs spacing, not throughput |
| Secondary rate limiter | Sliding window (6/24h) | Daily cap prevents over-posting even if intervals are met |
| Retry strategy | Exponential backoff, 3 attempts | Standard; prevents hammering a failing API |
| Error classification | Transient vs permanent | Don't waste retries on 401/400 errors |
| Crash recovery | Stuck job detection on startup | 15-minute timeout matches CruxDev's safety gate |
| Scheduling | Hour/day allowlist + optimal slot | Simple, no cron parser needed; engagement data improves over time |
| Monitoring | QueueMetrics struct + tracing | Structured logs for the daemon; metrics for health checks |
