//! Metrics extraction — compute project facts from code.
//!
//! One command computes the truth. The build injects it. No hardcoded numbers.

use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// All computed metrics for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub tests: u64,
    pub unit_tests: u64,
    pub e2e_tests: u64,
    pub tools: u64,
    pub dimensions: u64,
    pub dimension_sets: u64,
    pub pattern_docs: u64,
    pub pages: u64,
    pub blog_posts: u64,
    pub vs_pages: u64,
    pub binary_size_bytes: u64,
    pub computed_at: String,
    pub schema_version: u32,
}

impl ProjectMetrics {
    pub fn schema_version() -> u32 {
        1
    }
}

/// Compute all metrics for a project.
pub fn compute_metrics(project_dir: &str) -> ProjectMetrics {
    let (unit, e2e) = count_tests(project_dir);
    let (dims, sets) = count_dimensions(project_dir);

    ProjectMetrics {
        tests: unit + e2e,
        unit_tests: unit,
        e2e_tests: e2e,
        tools: count_tools(project_dir),
        dimensions: dims,
        dimension_sets: sets,
        pattern_docs: count_glob(project_dir, "docs/*PATTERNS*.md")
            + count_glob(project_dir, "docs/*REFERENCE*.md"),
        pages: count_website_pages(project_dir),
        blog_posts: count_website_blog(project_dir),
        vs_pages: count_website_vs(project_dir),
        binary_size_bytes: binary_size(project_dir),
        computed_at: chrono::Utc::now().to_rfc3339(),
        schema_version: ProjectMetrics::schema_version(),
    }
}

/// Write metrics to JSON file (atomic).
pub fn write_metrics(metrics: &ProjectMetrics, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(metrics).map_err(|e| format!("serialize: {e}"))?;
    let tmp = format!("{path}.tmp");
    fs::write(&tmp, &json).map_err(|e| format!("write: {e}"))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename: {e}"))?;
    Ok(())
}

/// Read metrics from JSON file.
pub fn read_metrics(path: &str) -> Result<ProjectMetrics, String> {
    let json = fs::read_to_string(path).map_err(|e| format!("read: {e}"))?;
    serde_json::from_str(&json).map_err(|e| format!("parse: {e}"))
}

// ── Extractors ──────────────────────────────────────────────────

fn count_tests(project_dir: &str) -> (u64, u64) {
    let rust_dir = Path::new(project_dir).join("rust");
    let dir = if rust_dir.exists() { rust_dir } else { Path::new(project_dir).into() };

    let cargo = home_cargo_bin();

    // Unit tests
    let unit = Command::new(&cargo)
        .args(["test", "--lib", "--bins", "--", "--list"])
        .current_dir(&dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.ends_with(": test"))
                .count() as u64
        })
        .unwrap_or(0);

    // E2E tests
    let e2e = Command::new(&cargo)
        .args(["test", "--test", "mcp_e2e", "--", "--list"])
        .current_dir(&dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.ends_with(": test"))
                .count() as u64
        })
        .unwrap_or(0);

    (unit, e2e)
}

fn count_tools(project_dir: &str) -> u64 {
    let server_path = Path::new(project_dir).join("rust/src/server.rs");
    if !server_path.exists() {
        return 0;
    }

    // Count #[tool(description lines — each is one MCP tool
    fs::read_to_string(&server_path)
        .ok()
        .map(|content| {
            content.lines()
                .filter(|l| l.trim().starts_with("#[tool("))
                .count() as u64
        })
        .unwrap_or(0)
}

fn count_dimensions(project_dir: &str) -> (u64, u64) {
    let router_path = Path::new(project_dir).join("rust/src/engine/router.rs");
    if !router_path.exists() {
        return (0, 0);
    }

    let content = match fs::read_to_string(&router_path) {
        Ok(c) => c,
        Err(_) => return (0, 0),
    };

    let mut total_dims = 0u64;
    let mut sets = 0u64;

    for line in content.lines() {
        if line.contains("DIMENSIONS:") && line.contains("&[") {
            sets += 1;
            // Count quoted strings in this line
            total_dims += line.matches('"').count() as u64 / 2;
        }
    }

    (total_dims, sets)
}

fn count_glob(project_dir: &str, pattern: &str) -> u64 {
    let full_pattern = format!("{}/{}", project_dir, pattern);
    glob::glob(&full_pattern)
        .map(|paths| paths.filter_map(|p| p.ok()).count() as u64)
        .unwrap_or(0)
}

fn count_website_pages(project_dir: &str) -> u64 {
    let dev_dir = format!("{}-dev/src/pages", project_dir.trim_end_matches('/'));
    if !Path::new(&dev_dir).exists() {
        return 0;
    }
    count_files_recursive(&dev_dir, &["astro", "md"])
}

fn count_website_blog(project_dir: &str) -> u64 {
    let blog_dir = format!("{}-dev/src/pages/blog", project_dir.trim_end_matches('/'));
    if !Path::new(&blog_dir).exists() {
        return 0;
    }
    count_files_recursive(&blog_dir, &["md"])
}

fn count_website_vs(project_dir: &str) -> u64 {
    let vs_dir = format!("{}-dev/src/pages/vs", project_dir.trim_end_matches('/'));
    if !Path::new(&vs_dir).exists() {
        return 0;
    }
    // Exclude index.astro
    count_files_recursive(&vs_dir, &["astro"]).saturating_sub(1)
}

fn binary_size(project_dir: &str) -> u64 {
    let release = Path::new(project_dir).join("rust/target/release/cruxdev");
    fs::metadata(&release).map(|m| m.len()).unwrap_or(0)
}

fn count_files_recursive(dir: &str, extensions: &[&str]) -> u64 {
    let mut count = 0u64;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                count += count_files_recursive(path.to_str().unwrap_or(""), extensions);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn home_cargo_bin() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let cargo = format!("{home}/.cargo/bin/cargo");
    if Path::new(&cargo).exists() { cargo } else { "cargo".into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_dir() -> String {
        // rust/ is CARGO_MANIFEST_DIR, project root is parent
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_count_tools_nonzero() {
        let tools = count_tools(&project_dir());
        assert!(tools >= 60, "expected >= 60 tools, got {tools}");
    }

    #[test]
    fn test_count_dimensions_matches_router() {
        let (dims, sets) = count_dimensions(&project_dir());
        assert!(dims >= 100, "expected >= 100 dimensions, got {dims}");
        assert!(sets >= 20, "expected >= 20 sets, got {sets}");
    }

    #[test]
    fn test_count_glob_patterns() {
        let count = count_glob(&project_dir(), "docs/*PATTERNS*.md");
        assert!(count >= 10, "expected >= 10 pattern docs, got {count}");
    }

    #[test]
    fn test_metrics_json_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("metrics.json");
        let metrics = ProjectMetrics {
            tests: 520, unit_tests: 502, e2e_tests: 18,
            tools: 63, dimensions: 146, dimension_sets: 21,
            pattern_docs: 55, pages: 267, blog_posts: 50,
            vs_pages: 10, binary_size_bytes: 5_000_000,
            computed_at: "2026-03-28T00:00:00Z".into(),
            schema_version: 1,
        };
        write_metrics(&metrics, path.to_str().unwrap()).unwrap();
        let loaded = read_metrics(path.to_str().unwrap()).unwrap();
        assert_eq!(loaded.tests, 520);
        assert_eq!(loaded.tools, 63);
        assert_eq!(loaded.schema_version, 1);
    }

    #[test]
    fn test_compute_metrics_produces_nonzero() {
        let metrics = compute_metrics(&project_dir());
        assert!(metrics.tools > 0);
        assert!(metrics.dimensions > 0);
        assert!(metrics.dimension_sets > 0);
        assert!(metrics.pattern_docs > 0);
    }
}
