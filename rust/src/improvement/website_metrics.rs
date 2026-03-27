//! Website metrics — count tests, MCP tools, modes for site stats.

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Aggregated project metrics for website display.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectMetrics {
    #[serde(default)]
    pub test_count: usize,
    #[serde(default)]
    pub coverage: f64,
    #[serde(default)]
    pub tool_count: usize,
    #[serde(default)]
    pub mode_count: usize,
    #[serde(default)]
    pub provider_count: usize,
    #[serde(default)]
    pub github_stars: usize,
}

/// Count `@mcp.tool()` decorators in a server file's content.
pub fn count_mcp_tools(content: &str) -> usize {
    content.matches("@mcp.tool()").count()
}

/// Count mode files (.md) in a list of filenames.
pub fn count_modes(filenames: &[&str]) -> usize {
    filenames.iter().filter(|f| f.ends_with(".md")).count()
}

/// Parse pytest output to extract test count.
pub fn parse_test_count(output: &str) -> usize {
    let re = Regex::new(r"(\d+)\s+test").unwrap();
    if let Some(caps) = re.captures(output) {
        caps[1].parse().unwrap_or(0)
    } else {
        0
    }
}

/// Format metrics as JSON string.
pub fn format_metrics_json(metrics: &ProjectMetrics) -> String {
    serde_json::to_string_pretty(metrics).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_mcp_tools() {
        let content = "@mcp.tool()\ndef foo():\n    pass\n\n@mcp.tool()\ndef bar():\n    pass\n";
        assert_eq!(count_mcp_tools(content), 2);
    }

    #[test]
    fn test_count_mcp_tools_none() {
        assert_eq!(count_mcp_tools("no tools here"), 0);
    }

    #[test]
    fn test_count_modes() {
        let files = vec!["mode1.md", "mode2.md", "readme.txt", "config.json"];
        assert_eq!(count_modes(&files), 2);
    }

    #[test]
    fn test_parse_test_count() {
        assert_eq!(parse_test_count("314 tests collected"), 314);
        assert_eq!(parse_test_count("no tests"), 0);
    }

    #[test]
    fn test_format_metrics_json() {
        let metrics = ProjectMetrics {
            test_count: 100,
            coverage: 99.5,
            ..Default::default()
        };
        let json = format_metrics_json(&metrics);
        assert!(json.contains("\"test_count\": 100"));
    }
}
