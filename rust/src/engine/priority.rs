//! Autonomous priority engine — scans all work sources, scores items, picks the next task.
//!
//! Work sources: build plans, GitHub issues, competitive gaps, self-adoption findings,
//! SEO health, content backlog. Each item gets a priority score (lower = higher priority).

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    pub source: String,
    pub title: String,
    pub score: u32,
    pub action: String,
    pub description: String,
    pub metadata: serde_json::Value,
}

/// Scan all work sources and return scored, sorted work items.
pub fn scan_work_sources(project_dir: &str, github_repo: &str) -> Vec<WorkItem> {
    let mut items = Vec::new();

    items.extend(scan_build_plans(project_dir));
    items.extend(scan_github_issues(github_repo));
    items.extend(scan_competitive_gaps(project_dir));
    items.extend(scan_content_backlog(project_dir));
    items.extend(scan_self_adoption(project_dir));

    items.sort_by_key(|i| i.score);
    items
}

/// Pick the highest-priority work item (lowest score).
pub fn pick_next(items: &[WorkItem]) -> Option<&WorkItem> {
    items.first()
}

// --- Build Plan Scanner ---

fn scan_build_plans(project_dir: &str) -> Vec<WorkItem> {
    let bp_dir = Path::new(project_dir).join("build_plans");
    let mut items = Vec::new();

    let entries = match std::fs::read_dir(&bp_dir) {
        Ok(rd) => rd,
        Err(_) => return items,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Skip converged/escalated plans
        let status = extract_field(&content, "Status");
        if status.contains("CONVERGED") || status.contains("ESCALATED") || status.contains("BLOCKED") {
            continue;
        }
        if !status.contains("NOT STARTED") && !status.contains("IN PROGRESS") {
            continue;
        }

        let priority = extract_field(&content, "Priority");
        let title = content.lines()
            .find(|l| l.starts_with("# "))
            .map(|l| l.trim_start_matches("# ").to_string())
            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());

        let base_score = match priority.to_lowercase().as_str() {
            s if s.contains("critical") => 10,
            s if s.contains("high") || s.contains("p0") => 15,
            s if s.contains("p1") => 25,
            s if s.contains("must close") => 20,
            s if s.contains("should close") => 35,
            s if s.contains("p2") || s.contains("medium") => 40,
            s if s.contains("nice to have") || s.contains("p3") => 50,
            s if s.contains("future") => 60,
            _ => 45,
        };

        // Modifier: in-progress plans get priority boost
        let modifier: i32 = if status.contains("IN PROGRESS") { -5 } else { 0 };

        items.push(WorkItem {
            source: "build_plan".into(),
            title,
            score: (base_score as i32 + modifier).max(1) as u32,
            action: "converge_plan".into(),
            description: format!("Status: {status}, Priority: {priority}"),
            metadata: serde_json::json!({
                "file": path.to_string_lossy(),
                "status": status,
                "priority": priority,
            }),
        });
    }

    items
}

// --- GitHub Issues Scanner ---

fn scan_github_issues(github_repo: &str) -> Vec<WorkItem> {
    if github_repo.is_empty() {
        return Vec::new();
    }

    let output = match std::process::Command::new("gh")
        .args(["issue", "list", "--repo", github_repo, "--state", "open",
               "--json", "number,title,labels,createdAt", "--limit", "20"])
        .output()
    {
        Ok(o) if o.status.success() => o.stdout,
        _ => return Vec::new(),
    };

    let issues: Vec<serde_json::Value> = match serde_json::from_slice(&output) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    issues.iter().map(|issue| {
        let number = issue.get("number").and_then(|v| v.as_u64()).unwrap_or(0);
        let title = issue.get("title").and_then(|v| v.as_str()).unwrap_or("untitled");
        let labels: Vec<String> = issue.get("labels")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|l| l.get("name").and_then(|n| n.as_str()).map(String::from))
                .collect())
            .unwrap_or_default();

        let base_score = if labels.iter().any(|l| l.contains("bug")) {
            10
        } else if labels.iter().any(|l| l.contains("enhancement") || l.contains("feature")) {
            30
        } else if labels.iter().any(|l| l.starts_with("patterns:")) {
            35
        } else {
            25
        };

        WorkItem {
            source: "github_issue".into(),
            title: format!("#{number} {title}"),
            score: base_score,
            action: "fix_issue".into(),
            description: format!("Labels: {}", labels.join(", ")),
            metadata: serde_json::json!({
                "number": number,
                "labels": labels,
                "repo": github_repo,
            }),
        }
    }).collect()
}

// --- Competitive Gap Scanner ---

fn scan_competitive_gaps(project_dir: &str) -> Vec<WorkItem> {
    let comp_path = Path::new(project_dir).join("docs/COMPETITORS.md");
    let content = match std::fs::read_to_string(&comp_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut items = Vec::new();

    // Parse the gap closure queue table
    let in_table = content.lines()
        .skip_while(|l| !l.contains("Gap Closure Queue"))
        .skip(1) // header
        .skip_while(|l| !l.starts_with('|') || l.contains("---"))
        .take_while(|l| l.starts_with('|'));

    for line in in_table {
        let cols: Vec<&str> = line.split('|').map(|c| c.trim()).filter(|c| !c.is_empty()).collect();
        if cols.len() < 4 {
            continue;
        }

        let gap = cols[0];
        let competitor = cols[1];
        let classification = cols[2].to_lowercase();
        let status = cols[3];

        // Skip table headers and separators
        if gap.contains("---") || gap == "Gap" || competitor == "Competitor" || classification.contains("---") {
            continue;
        }

        // Skip completed gaps
        let status_lower = status.to_lowercase();
        if status_lower.contains("done") || status_lower.contains("converged")
            || status_lower.contains("closed") || status_lower.contains("live")
            || status_lower.contains("patterns doc")
            || status_lower.contains("n/a") || classification.contains("intentional") {
            continue;
        }

        let base_score = if classification.contains("must close") {
            20
        } else if classification.contains("should close") {
            40
        } else if classification.contains("differentiator") {
            30
        } else {
            55
        };

        items.push(WorkItem {
            source: "competitive_gap".into(),
            title: format!("{gap} (vs {competitor})"),
            score: base_score,
            action: "close_gap".into(),
            description: format!("Classification: {}, Status: {status}", cols[2]),
            metadata: serde_json::json!({
                "gap": gap,
                "competitor": competitor,
                "classification": cols[2],
            }),
        });
    }

    items
}

// --- Content Backlog Scanner ---

fn scan_content_backlog(project_dir: &str) -> Vec<WorkItem> {
    let bp_dir = Path::new(project_dir).join("build_plans");
    let blog_dir = Path::new(project_dir).join("../cruxdev-dev/src/pages/blog");
    let mut items = Vec::new();

    // Count converged plans vs blog posts
    let converged_count = std::fs::read_dir(&bp_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            let content = std::fs::read_to_string(e.path()).unwrap_or_default();
            content.contains("CONVERGED")
        })
        .count();

    let blog_count = std::fs::read_dir(&blog_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .count();

    if converged_count > blog_count + 30 {
        let behind = converged_count - blog_count;
        items.push(WorkItem {
            source: "content_backlog".into(),
            title: format!("{behind} converged plans without blog posts"),
            score: if behind > 10 { 25 } else { 45 },
            action: "generate_content".into(),
            description: format!("{converged_count} converged plans, {blog_count} blog posts"),
            metadata: serde_json::json!({
                "converged_plans": converged_count,
                "blog_posts": blog_count,
                "behind": behind,
            }),
        });
    }

    items
}

// --- Self-Adoption Scanner ---

fn scan_self_adoption(project_dir: &str) -> Vec<WorkItem> {
    let mut items = Vec::new();

    // Check for patterns docs without dimensions
    let docs_dir = Path::new(project_dir).join("docs");
    let router_path = Path::new(project_dir).join("rust/src/engine/router.rs");

    let router_content = std::fs::read_to_string(&router_path).unwrap_or_default();

    // Patterns docs covered by a different dimension set (not false positives)
    let covered_aliases: &[(&str, &str)] = &[
        ("X_POST", "CONTENT_DIMENSIONS"),
        ("BLOG_POST", "CONTENT_DIMENSIONS"),
        ("BLOG_TAGGING", "CONTENT_DIMENSIONS"),
        ("BLOG_PAGINATION", "CONTENT_DIMENSIONS"),
        ("DRY_UI_COMPONENT", "UI_COMPONENT_DIMENSIONS"),
        ("WEBSITE_LOGO", "LOGO_DIMENSIONS"),
        ("AI_SKILLS", "SKILL_DIMENSIONS"),
        ("RESEARCH", "CONTENT_DIMENSIONS"),
        ("AUTONOMOUS_SELF_IMPROVEMENT", "PLAN_DIMENSIONS"),
        ("KV_CACHE", "CODE_DIMENSIONS"),
        ("COMPETITORS", "BUSINESS_DIMENSIONS"),
        ("BLOG", "CONTENT_DIMENSIONS"),
        ("CROSS_MODEL_VALIDATION", "CODE_DIMENSIONS"),
        ("POST_DEPLOYMENT", "POST_DEPLOYMENT_DIMENSIONS"),
        ("COLOR_CONTRAST", "COLOR_CONTRAST_DIMENSIONS"),
        ("SKILLS_AUTO_ACTIVATION", "SKILL_DIMENSIONS"),
        ("MULTI_AGENT", "CODE_DIMENSIONS"),
        ("VISUAL_VERIFICATION", "E2E_TEST_DIMENSIONS"),
        ("KERNEL_SANDBOXING", "CODE_DIMENSIONS"),
        ("LIFECYCLE_HOOK", "CODE_DIMENSIONS"),
        ("REGRESSION_DETECTION", "CODE_DIMENSIONS"),
        ("MOBILE_WEB", "MOBILE_WEB_DIMENSIONS"),
        ("GEO", "GEO_DIMENSIONS"),
    ];

    if let Ok(entries) = std::fs::read_dir(&docs_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("PATTERNS") && name.ends_with(".md") && !name.contains("DEVELOPMENT_PATTERNS") {
                let stem = name.replace("_PATTERNS.md", "").replace(".md", "");
                let dim_name = format!("{}_DIMENSIONS", stem.to_uppercase());

                // Check if covered by an alias
                let is_aliased = covered_aliases.iter().any(|(pat, dim)| {
                    stem.to_uppercase() == *pat && router_content.contains(dim)
                });

                if is_aliased || router_content.contains(&dim_name) {
                    continue; // Covered — skip
                }

                // Genuinely missing
                {
                    items.push(WorkItem {
                        source: "self_adoption".into(),
                        title: format!("{name} has no {dim_name} in router"),
                        score: 20,
                        action: "self_adopt".into(),
                        description: "Patterns doc exists but dimensions not wired into convergence".into(),
                        metadata: serde_json::json!({"doc": name, "missing_dimensions": dim_name}),
                    });
                }
            }
        }
    }

    items
}

// --- Helpers ---

fn extract_field(content: &str, field: &str) -> String {
    content.lines()
        .find(|l| l.contains(&format!("**{field}:**")))
        .map(|l| {
            l.split(&format!("**{field}:**"))
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_field() {
        let content = "# Plan\n**Status:** NOT STARTED\n**Priority:** High\n";
        assert_eq!(extract_field(content, "Status"), "NOT STARTED");
        assert_eq!(extract_field(content, "Priority"), "High");
    }

    #[test]
    fn test_extract_field_missing() {
        let content = "# Plan\n**Status:** CONVERGED\n";
        assert_eq!(extract_field(content, "Priority"), "");
    }

    #[test]
    fn test_scan_build_plans_skips_converged() {
        let dir = tempfile::tempdir().unwrap();
        let bp_dir = dir.path().join("build_plans");
        std::fs::create_dir_all(&bp_dir).unwrap();

        std::fs::write(bp_dir.join("BP001.md"), "# Plan\n**Status:** CONVERGED\n**Priority:** High\n").unwrap();
        std::fs::write(bp_dir.join("BP002.md"), "# Plan Two\n**Status:** NOT STARTED\n**Priority:** P0\n").unwrap();

        let items = scan_build_plans(dir.path().to_str().unwrap());
        assert_eq!(items.len(), 1);
        assert!(items[0].title.contains("Plan Two"));
        assert_eq!(items[0].score, 15); // P0 = 15
    }

    #[test]
    fn test_scan_build_plans_priority_scoring() {
        let dir = tempfile::tempdir().unwrap();
        let bp_dir = dir.path().join("build_plans");
        std::fs::create_dir_all(&bp_dir).unwrap();

        std::fs::write(bp_dir.join("BP001.md"), "# Critical\n**Status:** NOT STARTED\n**Priority:** Critical\n").unwrap();
        std::fs::write(bp_dir.join("BP002.md"), "# Nice\n**Status:** NOT STARTED\n**Priority:** Nice to have\n").unwrap();
        std::fs::write(bp_dir.join("BP003.md"), "# Must\n**Status:** NOT STARTED\n**Priority:** Must Close\n").unwrap();

        let items = scan_build_plans(dir.path().to_str().unwrap());
        assert_eq!(items.len(), 3);

        // Sort and verify order
        let mut sorted = items.clone();
        sorted.sort_by_key(|i| i.score);
        assert!(sorted[0].title.contains("Critical")); // 10
        assert!(sorted[1].title.contains("Must"));     // 20
        assert!(sorted[2].title.contains("Nice"));     // 50
    }

    #[test]
    fn test_pick_next_returns_lowest_score() {
        let items = vec![
            WorkItem { source: "a".into(), title: "High".into(), score: 50, action: "x".into(), description: String::new(), metadata: serde_json::json!({}) },
            WorkItem { source: "b".into(), title: "Low".into(), score: 10, action: "y".into(), description: String::new(), metadata: serde_json::json!({}) },
        ];
        let mut sorted = items;
        sorted.sort_by_key(|i| i.score);
        assert_eq!(pick_next(&sorted).unwrap().title, "Low");
    }

    #[test]
    fn test_pick_next_empty() {
        let items: Vec<WorkItem> = Vec::new();
        assert!(pick_next(&items).is_none());
    }

    #[test]
    fn test_competitive_gap_scanning() {
        let dir = tempfile::tempdir().unwrap();
        let docs_dir = dir.path().join("docs");
        std::fs::create_dir_all(&docs_dir).unwrap();

        let content = r#"# Competitors

## Gap Closure Queue

| Gap | Competitor | Classification | Status |
|-----|-----------|---------------|--------|
| KV-cache context | Manus | Must close | BP049 written |
| Visual verification | Cursor | Should close | Not started |
| Enterprise readiness | Claude Code | Future | Not started |
"#;
        std::fs::write(docs_dir.join("COMPETITORS.md"), content).unwrap();

        let items = scan_competitive_gaps(dir.path().to_str().unwrap());
        assert!(items.len() >= 2);

        let must = items.iter().find(|i| i.title.contains("KV-cache")).unwrap();
        let should = items.iter().find(|i| i.title.contains("Visual")).unwrap();
        assert!(must.score < should.score); // must-close < should-close
    }

    #[test]
    fn test_self_adoption_detects_unwired_patterns() {
        let dir = tempfile::tempdir().unwrap();
        let docs_dir = dir.path().join("docs");
        let rust_dir = dir.path().join("rust/src/engine");
        std::fs::create_dir_all(&docs_dir).unwrap();
        std::fs::create_dir_all(&rust_dir).unwrap();

        std::fs::write(docs_dir.join("FORM_PATTERNS.md"), "# Forms").unwrap();
        std::fs::write(docs_dir.join("MYSTERY_PATTERNS.md"), "# Mystery").unwrap();
        std::fs::write(rust_dir.join("router.rs"), "FORM_DIMENSIONS").unwrap();

        let items = scan_self_adoption(dir.path().to_str().unwrap());
        // MYSTERY_PATTERNS.md should be flagged (no MYSTERY_DIMENSIONS in router)
        assert!(items.iter().any(|i| i.title.contains("MYSTERY")));
        // FORM_PATTERNS.md should NOT be flagged (FORM_DIMENSIONS exists)
        assert!(!items.iter().any(|i| i.title.contains("FORM")));
    }
}
