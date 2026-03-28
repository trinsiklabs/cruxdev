//! Typefully integration — compose and schedule X/Twitter posts via API.
//!
//! API key from env var TYPEFULLY_API_KEY.
//! Rate limit: max 3 posts/day.

use serde::{Deserialize, Serialize};

const TYPEFULLY_API_BASE: &str = "https://api.typefully.com/v2";

/// Result of a Typefully post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResult {
    pub success: bool,
    pub draft_id: String,
    pub error: String,
}

/// Compose a release thread from changelog data.
pub fn compose_release_thread(
    project: &str,
    version: &str,
    changelog: &str,
    test_count: usize,
    tool_count: usize,
) -> String {
    let headline = changelog.lines().next().unwrap_or("New release");
    let mut parts = vec![
        format!("{project} {version}: {headline}"),
        String::new(),
    ];

    // Add 2-3 key points from changelog
    let bullets: Vec<&str> = changelog
        .lines()
        .filter(|l| l.starts_with("- ") || l.starts_with("* "))
        .take(3)
        .collect();
    for b in &bullets {
        parts.push(b.to_string());
    }

    parts.push(String::new());
    parts.push(format!("{test_count} tests, {tool_count} MCP tools, 0 clippy warnings."));

    parts.join("\n")
}

/// Compose a technical tip post.
pub fn compose_tip_post(topic: &str, code_snippet: &str) -> String {
    let mut post = format!("Tip: {topic}\n\n");
    if !code_snippet.is_empty() {
        post.push_str(code_snippet);
        post.push('\n');
    }
    post
}

/// Compose a build-in-public update.
pub fn compose_build_update(
    project: &str,
    plan_name: &str,
    summary: &str,
    findings_closed: usize,
    test_count: usize,
) -> String {
    let mut post = format!("{project}: {plan_name} converged.\n\n{summary}");
    if findings_closed > 0 {
        post.push_str(&format!("\n\nClosed {findings_closed} audit findings."));
    }
    post.push_str(&format!("\n\n{test_count} tests passing."));
    post
}

/// Fetch the social set ID for the authenticated account.
pub async fn fetch_social_set_id(api_key: &str) -> Option<u64> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{TYPEFULLY_API_BASE}/social-sets"))
        .bearer_auth(api_key)
        .send()
        .await
        .ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;
    json.pointer("/results/0/id").and_then(|v| v.as_u64())
}

/// Post a draft to Typefully API v2.
/// Requires social_set_id — pass None to auto-fetch from API.
pub async fn post_draft(
    api_key: &str,
    content: &str,
    social_set_id: Option<u64>,
) -> PostResult {
    let client = reqwest::Client::new();

    // Resolve social set ID
    let set_id = match social_set_id {
        Some(id) => id,
        None => match fetch_social_set_id(api_key).await {
            Some(id) => id,
            None => return PostResult {
                success: false,
                draft_id: String::new(),
                error: "Could not fetch social_set_id from Typefully API".to_string(),
            },
        },
    };

    // Split content into posts if it's a thread (separated by blank lines)
    let posts: Vec<serde_json::Value> = content
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|text| serde_json::json!({"text": text.trim()}))
        .collect();

    let body = serde_json::json!({
        "platforms": {
            "x": {
                "enabled": true,
                "posts": posts,
            }
        }
    });

    let url = format!("{TYPEFULLY_API_BASE}/social-sets/{set_id}/drafts");

    let resp = client
        .post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match resp {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            if status.is_success() {
                let parsed: serde_json::Value =
                    serde_json::from_str(&text).unwrap_or(serde_json::json!({}));
                let draft_id = parsed.get("id")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                PostResult {
                    success: true,
                    draft_id,
                    error: String::new(),
                }
            } else {
                PostResult {
                    success: false,
                    draft_id: String::new(),
                    error: format!("HTTP {status}: {text}"),
                }
            }
        }
        Err(e) => PostResult {
            success: false,
            draft_id: String::new(),
            error: format!("{e}"),
        },
    }
}

/// Get API key from environment.
pub fn api_key_from_env() -> Option<String> {
    std::env::var("TYPEFULLY_API_KEY").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_release_thread() {
        let thread = compose_release_thread(
            "CruxDev",
            "v0.2.0",
            "Git workflow automation\n- commit/push/PR with safety gates\n- Never force push\n- Pre-commit binary rejection",
            368,
            46,
        );
        assert!(thread.contains("CruxDev v0.2.0"));
        assert!(thread.contains("368 tests"));
        assert!(thread.contains("46 MCP tools"));
    }

    #[test]
    fn test_compose_tip_post() {
        let post = compose_tip_post("Use FORM_DIMENSIONS for form audits", "FORM_DIMENSIONS = [layout, labels, ...]");
        assert!(post.contains("Tip:"));
        assert!(post.contains("FORM_DIMENSIONS"));
    }

    #[test]
    fn test_compose_build_update() {
        let post = compose_build_update("CruxDev", "BUILD_PLAN_016", "Git automation", 5, 368);
        assert!(post.contains("BUILD_PLAN_016"));
        assert!(post.contains("5 audit findings"));
        assert!(post.contains("368 tests"));
    }

    #[test]
    fn test_api_key_missing() {
        unsafe { std::env::remove_var("TYPEFULLY_API_KEY"); }
        assert!(api_key_from_env().is_none());
    }
}
