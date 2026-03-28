//! X posting queue — rate-limited, JSONL-backed, scheduled.
//!
//! The 30-minute minimum interval is a hard-coded invariant.
//! Changing it requires a code change, test update, and convergence cycle.

use std::fs;
use std::io::Write;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

/// Hard minimum interval between posts (30 minutes).
/// This is NOT configurable. Changing it requires a code change.
const MIN_POST_INTERVAL_SECS: i64 = 30 * 60;

/// Status of a queued post.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostStatus {
    Pending,
    Scheduled,
    Posted,
    Failed,
    Cancelled,
}

/// A queued post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedPost {
    pub id: String,
    pub content: String,
    pub scheduled_at: Option<String>,
    pub status: PostStatus,
    pub created_at: String,
    pub posted_at: Option<String>,
    pub tweet_id: Option<String>,
    pub error: Option<String>,
}

/// Summary of queue state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub pending: usize,
    pub scheduled: usize,
    pub posted_today: usize,
    pub can_post_now: bool,
    pub next_post_time: Option<String>,
    pub last_posted_at: Option<String>,
}

/// Generate a unique ID for a queue entry.
fn generate_id() -> String {
    let now = Utc::now();
    format!("xp_{}", now.format("%Y%m%d_%H%M%S_%3f"))
}

/// Read the queue from a JSONL file.
pub fn read_queue(queue_path: &str) -> Vec<QueuedPost> {
    let content = match fs::read_to_string(queue_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Append a post to the queue (atomic write).
pub fn enqueue(queue_path: &str, content: &str, scheduled_at: Option<&str>) -> Result<QueuedPost, String> {
    let post = QueuedPost {
        id: generate_id(),
        content: content.to_string(),
        scheduled_at: scheduled_at.map(|s| s.to_string()),
        status: if scheduled_at.is_some() { PostStatus::Scheduled } else { PostStatus::Pending },
        created_at: Utc::now().to_rfc3339(),
        posted_at: None,
        tweet_id: None,
        error: None,
    };

    let json = serde_json::to_string(&post).map_err(|e| format!("serialize: {e}"))?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(queue_path)
        .map_err(|e| format!("open queue: {e}"))?;

    writeln!(file, "{json}").map_err(|e| format!("write queue: {e}"))?;

    Ok(post)
}

/// Check if we can post right now (30-minute minimum since last post).
pub fn can_post_now(queue_path: &str) -> bool {
    let queue = read_queue(queue_path);
    let last_posted = queue
        .iter()
        .filter(|p| p.status == PostStatus::Posted)
        .filter_map(|p| p.posted_at.as_ref())
        .filter_map(|t| DateTime::parse_from_rfc3339(t).ok())
        .max();

    match last_posted {
        None => true, // No posts yet
        Some(last) => {
            let elapsed = Utc::now().signed_duration_since(last.with_timezone(&Utc));
            elapsed.num_seconds() >= MIN_POST_INTERVAL_SECS
        }
    }
}

/// Get the next allowed post time.
pub fn next_post_time(queue_path: &str) -> Option<DateTime<Utc>> {
    let queue = read_queue(queue_path);
    let last_posted = queue
        .iter()
        .filter(|p| p.status == PostStatus::Posted)
        .filter_map(|p| p.posted_at.as_ref())
        .filter_map(|t| DateTime::parse_from_rfc3339(t).ok())
        .max();

    last_posted.map(|last| {
        last.with_timezone(&Utc) + chrono::Duration::seconds(MIN_POST_INTERVAL_SECS)
    })
}

/// Get the next pending/scheduled post that's ready to send.
pub fn dequeue_next(queue_path: &str) -> Option<QueuedPost> {
    if !can_post_now(queue_path) {
        return None;
    }

    let queue = read_queue(queue_path);
    let now = Utc::now();

    queue
        .into_iter()
        .filter(|p| p.status == PostStatus::Pending || p.status == PostStatus::Scheduled)
        .find(|p| {
            match &p.scheduled_at {
                None => true,
                Some(t) => {
                    DateTime::parse_from_rfc3339(t)
                        .map(|scheduled| scheduled.with_timezone(&Utc) <= now)
                        .unwrap_or(true)
                }
            }
        })
}

/// Mark a post as posted (rewrite the queue file).
pub fn mark_posted(queue_path: &str, id: &str, tweet_id: &str) -> Result<(), String> {
    update_status(queue_path, id, PostStatus::Posted, Some(tweet_id), None)
}

/// Mark a post as failed.
pub fn mark_failed(queue_path: &str, id: &str, error: &str) -> Result<(), String> {
    update_status(queue_path, id, PostStatus::Failed, None, Some(error))
}

/// Cancel a queued post.
pub fn cancel(queue_path: &str, id: &str) -> Result<(), String> {
    update_status(queue_path, id, PostStatus::Cancelled, None, None)
}

/// List pending posts.
pub fn list_pending(queue_path: &str) -> Vec<QueuedPost> {
    read_queue(queue_path)
        .into_iter()
        .filter(|p| p.status == PostStatus::Pending || p.status == PostStatus::Scheduled)
        .collect()
}

/// Get queue status summary.
pub fn queue_status(queue_path: &str) -> QueueStatus {
    let queue = read_queue(queue_path);
    let today = Local::now().format("%Y-%m-%d").to_string();

    let pending = queue.iter().filter(|p| p.status == PostStatus::Pending).count();
    let scheduled = queue.iter().filter(|p| p.status == PostStatus::Scheduled).count();
    let posted_today = queue
        .iter()
        .filter(|p| p.status == PostStatus::Posted)
        .filter(|p| p.posted_at.as_ref().map(|t| t.starts_with(&today)).unwrap_or(false))
        .count();

    let last_posted_at = queue
        .iter()
        .filter(|p| p.status == PostStatus::Posted)
        .filter_map(|p| p.posted_at.clone())
        .max();

    QueueStatus {
        pending,
        scheduled,
        posted_today,
        can_post_now: can_post_now(queue_path),
        next_post_time: next_post_time(queue_path).map(|t| t.to_rfc3339()),
        last_posted_at,
    }
}

/// Update status of a post (atomic rewrite).
fn update_status(
    queue_path: &str,
    id: &str,
    new_status: PostStatus,
    tweet_id: Option<&str>,
    error: Option<&str>,
) -> Result<(), String> {
    let mut queue = read_queue(queue_path);

    let found = queue.iter_mut().find(|p| p.id == id);
    match found {
        None => Err(format!("post {id} not found in queue")),
        Some(post) => {
            post.status = new_status;
            if let Some(tid) = tweet_id {
                post.tweet_id = Some(tid.to_string());
            }
            if let Some(err) = error {
                post.error = Some(err.to_string());
            }
            if post.status == PostStatus::Posted {
                post.posted_at = Some(Utc::now().to_rfc3339());
            }

            // Atomic rewrite
            let tmp_path = format!("{queue_path}.tmp");
            let mut file = fs::File::create(&tmp_path)
                .map_err(|e| format!("create tmp: {e}"))?;
            for p in &queue {
                let json = serde_json::to_string(p).map_err(|e| format!("serialize: {e}"))?;
                writeln!(file, "{json}").map_err(|e| format!("write: {e}"))?;
            }
            fs::rename(&tmp_path, queue_path).map_err(|e| format!("rename: {e}"))?;

            Ok(())
        }
    }
}

/// Prune old entries (older than N days).
pub fn prune_old(queue_path: &str, days: i64) -> Result<usize, String> {
    let queue = read_queue(queue_path);
    let cutoff = Utc::now() - chrono::Duration::days(days);

    let (keep, pruned): (Vec<_>, Vec<_>) = queue.into_iter().partition(|p| {
        DateTime::parse_from_rfc3339(&p.created_at)
            .map(|t| t.with_timezone(&Utc) > cutoff)
            .unwrap_or(true)
    });

    let pruned_count = pruned.len();

    // Atomic rewrite
    let tmp_path = format!("{queue_path}.tmp");
    let mut file = fs::File::create(&tmp_path).map_err(|e| format!("create tmp: {e}"))?;
    for p in &keep {
        let json = serde_json::to_string(p).map_err(|e| format!("serialize: {e}"))?;
        writeln!(file, "{json}").map_err(|e| format!("write: {e}"))?;
    }
    fs::rename(&tmp_path, queue_path).map_err(|e| format!("rename: {e}"))?;

    Ok(pruned_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_queue() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("x_queue.jsonl");
        (dir, path.to_str().unwrap().to_string())
    }

    #[test]
    fn test_enqueue_creates_entry() {
        let (_dir, path) = tmp_queue();
        let post = enqueue(&path, "Hello world", None).unwrap();
        assert_eq!(post.status, PostStatus::Pending);
        assert!(!post.id.is_empty());

        let queue = read_queue(&path);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].content, "Hello world");
    }

    #[test]
    fn test_enqueue_with_schedule() {
        let (_dir, path) = tmp_queue();
        let future = (Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
        let post = enqueue(&path, "Scheduled post", Some(&future)).unwrap();
        assert_eq!(post.status, PostStatus::Scheduled);
        assert!(post.scheduled_at.is_some());
    }

    #[test]
    fn test_can_post_now_empty_queue() {
        let (_dir, path) = tmp_queue();
        assert!(can_post_now(&path));
    }

    #[test]
    fn test_rate_limiter_blocks_within_30_min() {
        let (_dir, path) = tmp_queue();
        let post = enqueue(&path, "First post", None).unwrap();
        // Simulate posting just now
        mark_posted(&path, &post.id, "12345").unwrap();

        // Should be blocked — less than 30 min
        assert!(!can_post_now(&path));
    }

    #[test]
    fn test_dequeue_respects_rate_limit() {
        let (_dir, path) = tmp_queue();
        let post = enqueue(&path, "First", None).unwrap();
        mark_posted(&path, &post.id, "111").unwrap();

        enqueue(&path, "Second", None).unwrap();

        // Should return None because rate limited
        assert!(dequeue_next(&path).is_none());
    }

    #[test]
    fn test_cancel_post() {
        let (_dir, path) = tmp_queue();
        let post = enqueue(&path, "Cancel me", None).unwrap();
        cancel(&path, &post.id).unwrap();

        let queue = read_queue(&path);
        assert_eq!(queue[0].status, PostStatus::Cancelled);
    }

    #[test]
    fn test_list_pending_ordered() {
        let (_dir, path) = tmp_queue();
        enqueue(&path, "First", None).unwrap();
        enqueue(&path, "Second", None).unwrap();
        enqueue(&path, "Third", None).unwrap();

        let pending = list_pending(&path);
        assert_eq!(pending.len(), 3);
        assert_eq!(pending[0].content, "First");
    }

    #[test]
    fn test_mark_failed() {
        let (_dir, path) = tmp_queue();
        let post = enqueue(&path, "Fail me", None).unwrap();
        mark_failed(&path, &post.id, "401 Unauthorized").unwrap();

        let queue = read_queue(&path);
        assert_eq!(queue[0].status, PostStatus::Failed);
        assert_eq!(queue[0].error.as_deref(), Some("401 Unauthorized"));
    }

    #[test]
    fn test_queue_status() {
        let (_dir, path) = tmp_queue();
        enqueue(&path, "Pending 1", None).unwrap();
        enqueue(&path, "Pending 2", None).unwrap();

        let status = queue_status(&path);
        assert_eq!(status.pending, 2);
        assert!(status.can_post_now);
    }

    #[test]
    fn test_prune_old_entries() {
        let (_dir, path) = tmp_queue();
        // Create a post with old timestamp
        let mut post = QueuedPost {
            id: "old_post".into(),
            content: "old".into(),
            scheduled_at: None,
            status: PostStatus::Posted,
            created_at: "2020-01-01T00:00:00Z".into(),
            posted_at: Some("2020-01-01T00:00:00Z".into()),
            tweet_id: Some("old_tweet".into()),
            error: None,
        };
        let json = serde_json::to_string(&post).unwrap();
        fs::write(&path, format!("{json}\n")).unwrap();

        // Add a new post
        enqueue(&path, "New post", None).unwrap();

        let pruned = prune_old(&path, 30).unwrap();
        assert_eq!(pruned, 1);

        let queue = read_queue(&path);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].content, "New post");
    }

    #[test]
    fn test_atomic_write_on_update() {
        let (_dir, path) = tmp_queue();
        enqueue(&path, "Post 1", None).unwrap();
        enqueue(&path, "Post 2", None).unwrap();

        let queue = read_queue(&path);
        cancel(&path, &queue[0].id).unwrap();

        // Verify both entries still exist
        let queue = read_queue(&path);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0].status, PostStatus::Cancelled);
        assert_eq!(queue[1].status, PostStatus::Pending);
    }
}
