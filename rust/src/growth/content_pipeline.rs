//! Universal content pipeline — classify events, generate blog + X posts from any project type.
//!
//! The trigger registry maps (project_type, event_type) → content templates.
//! The engine is generic; content rules are per-type.

use serde::{Deserialize, Serialize};

/// An event that may produce content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEvent {
    pub project_type: String,
    pub event_type: EventType,
    pub title: String,
    pub summary: String,
    pub details: String,
    pub metrics: Option<ContentMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    FeatureShipped,
    CompetitorDiscovered,
    GapClosed,
    IssueResolved,
    IntegrationAdded,
    MethodologyDoc,
    ChapterCompleted,
    BookPublished,
    EpisodePublished,
    IssuePublished,
    VideoPublished,
    ProductLaunch,
    ReleasePublished,
    MilestoneReached,
    BugFix,
    Refactor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetrics {
    pub test_count: Option<usize>,
    pub tool_count: Option<usize>,
    pub findings_closed: Option<usize>,
}

/// Content generation decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDecision {
    pub generate_blog: bool,
    pub generate_x_post: bool,
    pub blog_template: String,
    pub x_template: String,
    pub reason: String,
}

/// BIP state — tracks posting cadence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BipState {
    pub last_posted_at: String,
    pub posts_today: u32,
    pub cooldown_minutes: u32,
}

/// Classify an event and decide what content to generate.
pub fn classify_event(event: &ContentEvent) -> ContentDecision {
    match event.event_type {
        EventType::FeatureShipped => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "feature_announcement".into(),
            x_template: "feature_thread".into(),
            reason: "New feature — blog + X thread".into(),
        },
        EventType::CompetitorDiscovered => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "competitive_analysis".into(),
            x_template: "competitor_comparison".into(),
            reason: "New competitor — comparison content".into(),
        },
        EventType::GapClosed => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "gap_closure".into(),
            x_template: "parity_announcement".into(),
            reason: "Gap closed — parity announcement".into(),
        },
        EventType::IssueResolved => ContentDecision {
            generate_blog: false,
            generate_x_post: true,
            blog_template: String::new(),
            x_template: "issue_fixed".into(),
            reason: "Issue resolved — X post only".into(),
        },
        EventType::IntegrationAdded => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "integration_announcement".into(),
            x_template: "integration_thread".into(),
            reason: "New integration — expands market".into(),
        },
        EventType::MethodologyDoc => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "thought_leadership".into(),
            x_template: "methodology_post".into(),
            reason: "New methodology — thought leadership".into(),
        },
        EventType::ChapterCompleted => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "chapter_teaser".into(),
            x_template: "chapter_announcement".into(),
            reason: "Chapter done — teaser + announcement".into(),
        },
        EventType::BookPublished => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "book_launch".into(),
            x_template: "book_launch_thread".into(),
            reason: "Book published — launch content".into(),
        },
        EventType::EpisodePublished => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "episode_show_notes".into(),
            x_template: "episode_promo".into(),
            reason: "Episode published — show notes + promo".into(),
        },
        EventType::IssuePublished => ContentDecision {
            generate_blog: false,
            generate_x_post: true,
            blog_template: String::new(),
            x_template: "newsletter_highlight".into(),
            reason: "Newsletter issue — X highlight only".into(),
        },
        EventType::VideoPublished => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "video_companion".into(),
            x_template: "video_promo".into(),
            reason: "Video published — companion post + promo".into(),
        },
        EventType::ProductLaunch => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "product_launch".into(),
            x_template: "launch_thread".into(),
            reason: "Product launch — full content".into(),
        },
        EventType::ReleasePublished => ContentDecision {
            generate_blog: true,
            generate_x_post: true,
            blog_template: "release_notes".into(),
            x_template: "release_thread".into(),
            reason: "Release published — notes + thread".into(),
        },
        EventType::MilestoneReached => ContentDecision {
            generate_blog: false,
            generate_x_post: true,
            blog_template: String::new(),
            x_template: "milestone_post".into(),
            reason: "Milestone — X post only".into(),
        },
        EventType::BugFix => ContentDecision {
            generate_blog: false,
            generate_x_post: false,
            blog_template: String::new(),
            x_template: String::new(),
            reason: "Bug fix — no content unless publicly reported".into(),
        },
        EventType::Refactor => ContentDecision {
            generate_blog: false,
            generate_x_post: false,
            blog_template: String::new(),
            x_template: String::new(),
            reason: "Refactor — no content".into(),
        },
    }
}

/// Generate a blog post from an event using the appropriate template.
pub fn generate_blog_post(event: &ContentEvent, decision: &ContentDecision) -> Option<String> {
    if !decision.generate_blog {
        return None;
    }

    let mut lines = vec![
        format!("# {}", event.title),
        String::new(),
        event.summary.clone(),
        String::new(),
    ];

    if !event.details.is_empty() {
        lines.push("## Details".into());
        lines.push(String::new());
        lines.push(event.details.clone());
        lines.push(String::new());
    }

    if let Some(metrics) = &event.metrics {
        lines.push("## Numbers".into());
        lines.push(String::new());
        if let Some(tc) = metrics.test_count {
            lines.push(format!("- **{tc}** tests passing"));
        }
        if let Some(tools) = metrics.tool_count {
            lines.push(format!("- **{tools}** MCP tools"));
        }
        if let Some(fc) = metrics.findings_closed {
            lines.push(format!("- **{fc}** audit findings closed"));
        }
        lines.push(String::new());
    }

    Some(lines.join("\n"))
}

/// Generate an X/Twitter post from an event.
pub fn generate_x_post(event: &ContentEvent, decision: &ContentDecision) -> Option<String> {
    if !decision.generate_x_post {
        return None;
    }

    let metrics_str = event.metrics.as_ref().map(|m| {
        let mut parts = Vec::new();
        if let Some(tc) = m.test_count { parts.push(format!("{tc} tests")); }
        if let Some(tools) = m.tool_count { parts.push(format!("{tools} tools")); }
        if !parts.is_empty() { format!("\n\n{}", parts.join(" · ")) } else { String::new() }
    }).unwrap_or_default();

    let post = format!("{}\n\n{}{}", event.title, event.summary, metrics_str);

    // Truncate to 280 chars for single post (threads handled separately)
    Some(post.chars().take(280).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(event_type: EventType) -> ContentEvent {
        ContentEvent {
            project_type: "software-existing".into(),
            event_type,
            title: "Test Event".into(),
            summary: "Something happened.".into(),
            details: "More details.".into(),
            metrics: Some(ContentMetrics {
                test_count: Some(443),
                tool_count: Some(52),
                findings_closed: Some(5),
            }),
        }
    }

    #[test]
    fn test_feature_generates_both() {
        let event = make_event(EventType::FeatureShipped);
        let decision = classify_event(&event);
        assert!(decision.generate_blog);
        assert!(decision.generate_x_post);
    }

    #[test]
    fn test_bug_fix_generates_nothing() {
        let event = make_event(EventType::BugFix);
        let decision = classify_event(&event);
        assert!(!decision.generate_blog);
        assert!(!decision.generate_x_post);
    }

    #[test]
    fn test_issue_resolved_x_only() {
        let event = make_event(EventType::IssueResolved);
        let decision = classify_event(&event);
        assert!(!decision.generate_blog);
        assert!(decision.generate_x_post);
    }

    #[test]
    fn test_chapter_completed() {
        let mut event = make_event(EventType::ChapterCompleted);
        event.project_type = "book".into();
        let decision = classify_event(&event);
        assert!(decision.generate_blog);
        assert!(decision.generate_x_post);
        assert_eq!(decision.blog_template, "chapter_teaser");
    }

    #[test]
    fn test_blog_generation() {
        let event = make_event(EventType::FeatureShipped);
        let decision = classify_event(&event);
        let blog = generate_blog_post(&event, &decision).unwrap();
        assert!(blog.contains("# Test Event"));
        assert!(blog.contains("443"));
        assert!(blog.contains("52"));
    }

    #[test]
    fn test_x_post_generation() {
        let event = make_event(EventType::FeatureShipped);
        let decision = classify_event(&event);
        let post = generate_x_post(&event, &decision).unwrap();
        assert!(post.contains("Test Event"));
        assert!(post.len() <= 280);
    }

    #[test]
    fn test_no_blog_for_no_decision() {
        let event = make_event(EventType::BugFix);
        let decision = classify_event(&event);
        assert!(generate_blog_post(&event, &decision).is_none());
        assert!(generate_x_post(&event, &decision).is_none());
    }

    #[test]
    fn test_all_event_types_classified() {
        let types = vec![
            EventType::FeatureShipped, EventType::CompetitorDiscovered,
            EventType::GapClosed, EventType::IssueResolved,
            EventType::IntegrationAdded, EventType::MethodologyDoc,
            EventType::ChapterCompleted, EventType::BookPublished,
            EventType::EpisodePublished, EventType::IssuePublished,
            EventType::VideoPublished, EventType::ProductLaunch,
            EventType::ReleasePublished, EventType::MilestoneReached,
            EventType::BugFix, EventType::Refactor,
        ];
        for t in types {
            let event = make_event(t);
            let decision = classify_event(&event);
            // Every type should produce a reason
            assert!(!decision.reason.is_empty());
        }
    }
}
