//! Growth metrics tracking — GitHub stars, issues, contributors via API.

use std::fs;
use std::io::Write;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// Growth metrics snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub timestamp: f64,
    pub stars: u64,
    pub forks: u64,
    pub open_issues: u64,
    pub contributors: u64,
    pub watchers: u64,
}

/// Collect growth metrics from GitHub via `gh` CLI.
pub fn collect_metrics(repo: &str) -> Result<GrowthMetrics, String> {
    let output = Command::new("gh")
        .args([
            "repo", "view", repo,
            "--json", "stargazerCount,forkCount,openIssues,watchers",
        ])
        .output()
        .map_err(|e| format!("gh repo view failed: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("parse: {e}"))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    Ok(GrowthMetrics {
        timestamp: now,
        stars: parsed.get("stargazerCount").and_then(|v| v.as_u64()).unwrap_or(0),
        forks: parsed.get("forkCount").and_then(|v| v.as_u64()).unwrap_or(0),
        open_issues: parsed
            .get("openIssues")
            .and_then(|v| v.get("totalCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
        contributors: 0, // Requires separate API call
        watchers: parsed
            .get("watchers")
            .and_then(|v| v.get("totalCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
    })
}

/// Append metrics to JSONL file.
pub fn append_metrics(metrics_path: &str, metrics: &GrowthMetrics) -> Result<(), String> {
    let json = serde_json::to_string(metrics).map_err(|e| format!("{e}"))?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(metrics_path)
        .map_err(|e| format!("{e}"))?;
    writeln!(file, "{json}").map_err(|e| format!("{e}"))?;
    Ok(())
}

/// Read metrics history from JSONL file.
pub fn read_metrics_history(metrics_path: &str) -> Vec<GrowthMetrics> {
    fs::read_to_string(metrics_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Generate a growth report from metrics history.
pub fn growth_report(history: &[GrowthMetrics]) -> String {
    if history.is_empty() {
        return "No metrics collected yet.".to_string();
    }

    let latest = &history[history.len() - 1];
    let mut lines = vec![
        "## Growth Report".to_string(),
        String::new(),
        format!("**Stars:** {}", latest.stars),
        format!("**Forks:** {}", latest.forks),
        format!("**Open Issues:** {}", latest.open_issues),
        format!("**Watchers:** {}", latest.watchers),
    ];

    if history.len() >= 2 {
        let prev = &history[history.len() - 2];
        let star_delta = latest.stars as i64 - prev.stars as i64;
        let direction = if star_delta > 0 { "+" } else { "" };
        lines.push(String::new());
        lines.push(format!("**Star velocity:** {direction}{star_delta} since last check"));
    }

    lines.push(String::new());
    lines.push(format!("**History depth:** {} snapshots", history.len()));

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_read_metrics() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("metrics.jsonl");
        let metrics = GrowthMetrics {
            timestamp: 1000.0,
            stars: 42,
            forks: 5,
            open_issues: 3,
            contributors: 2,
            watchers: 10,
        };
        append_metrics(path.to_str().unwrap(), &metrics).unwrap();
        let history = read_metrics_history(path.to_str().unwrap());
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].stars, 42);
    }

    #[test]
    fn test_growth_report_empty() {
        let report = growth_report(&[]);
        assert!(report.contains("No metrics"));
    }

    #[test]
    fn test_growth_report_with_data() {
        let history = vec![
            GrowthMetrics { timestamp: 1000.0, stars: 10, forks: 1, open_issues: 0, contributors: 1, watchers: 5 },
            GrowthMetrics { timestamp: 2000.0, stars: 15, forks: 2, open_issues: 1, contributors: 2, watchers: 8 },
        ];
        let report = growth_report(&history);
        assert!(report.contains("Stars:** 15"));
        assert!(report.contains("+5"));
    }
}
