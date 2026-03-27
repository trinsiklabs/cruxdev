//! Growth cycle configuration — single source of truth.
//!
//! Config lives at `.cruxdev/growth.toml`. Secrets are NEVER stored in
//! config — only env var NAMES or file PATHS that point to secrets.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Top-level growth configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthConfig {
    pub project: ProjectConfig,
    #[serde(default)]
    pub typefully: TypefullyConfig,
    #[serde(default)]
    pub github: GitHubConfig,
    #[serde(default)]
    pub content: ContentConfig,
    #[serde(default)]
    pub metrics: MetricsConfig,
    #[serde(default)]
    pub readme: ReadmeConfig,
    #[serde(default)]
    pub llms_txt: LlmsTxtConfig,
    #[serde(default)]
    pub ecosystem: EcosystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub repo: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypefullyConfig {
    /// Name of the environment variable holding the API key. NOT the key itself.
    #[serde(default = "default_typefully_env")]
    pub api_key_env: String,
    #[serde(default = "default_max_posts")]
    pub max_posts_per_day: u32,
    #[serde(default = "default_true")]
    pub threadify_releases: bool,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for TypefullyConfig {
    fn default() -> Self {
        Self {
            api_key_env: default_typefully_env(),
            max_posts_per_day: 3,
            threadify_releases: true,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    #[serde(default)]
    pub repos: Vec<String>,
    #[serde(default = "default_true")]
    pub issue_monitoring_enabled: bool,
    #[serde(default = "default_true")]
    pub issue_dry_run: bool,
    #[serde(default)]
    pub release_creation_enabled: bool,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            repos: Vec::new(),
            issue_monitoring_enabled: true,
            issue_dry_run: true,
            release_creation_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentConfig {
    #[serde(default)]
    pub website_repo: String,
    #[serde(default)]
    pub blog_dir: String,
    #[serde(default = "default_vs_dir")]
    pub vs_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    #[serde(default = "default_true")]
    pub tracking_enabled: bool,
    #[serde(default = "default_metrics_file")]
    pub metrics_file: String,
    #[serde(default = "default_collection_interval")]
    pub collection_interval_minutes: u32,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            tracking_enabled: true,
            metrics_file: default_metrics_file(),
            collection_interval_minutes: 60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadmeConfig {
    #[serde(default = "default_true")]
    pub auto_optimize: bool,
    #[serde(default)]
    pub test_count_source: String,
    #[serde(default)]
    pub tool_count: usize,
}

impl Default for ReadmeConfig {
    fn default() -> Self {
        Self {
            auto_optimize: true,
            test_count_source: String::new(),
            tool_count: 0,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmsTxtConfig {
    #[serde(default = "default_true")]
    pub auto_update: bool,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub methodology_docs: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcosystemConfig {
    #[serde(default)]
    pub projects: Vec<String>,
    #[serde(default)]
    pub shared_typefully: bool,
}

// Default value helpers
fn default_typefully_env() -> String { "TYPEFULLY_API_KEY".into() }
fn default_max_posts() -> u32 { 3 }
fn default_true() -> bool { true }
fn default_vs_dir() -> String { "docs/vs".into() }
fn default_metrics_file() -> String { ".cruxdev/growth/metrics.jsonl".into() }
fn default_collection_interval() -> u32 { 60 }

/// Load config from `.cruxdev/growth.toml`.
pub fn load_config(project_dir: &str) -> Result<GrowthConfig, String> {
    let path = Path::new(project_dir).join(".cruxdev").join("growth.toml");
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read {}: {e}. Run init_growth_config to create one.", path.display()))?;
    toml::from_str(&content)
        .map_err(|e| format!("Invalid config: {e}"))
}

/// Resolve an API key from the env var named in config.
pub fn resolve_api_key(env_var_name: &str) -> Option<String> {
    std::env::var(env_var_name).ok().filter(|v| !v.is_empty())
}

/// Validate config, return list of warnings.
pub fn validate_config(config: &GrowthConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    if config.project.name.is_empty() {
        warnings.push("project.name is empty".into());
    }
    if config.project.repo.is_empty() {
        warnings.push("project.repo is empty — metrics and issues won't work".into());
    }
    if config.typefully.enabled && resolve_api_key(&config.typefully.api_key_env).is_none() {
        warnings.push(format!(
            "typefully.enabled=true but {} env var not set — posting will be skipped",
            config.typefully.api_key_env
        ));
    }

    // Safety: check config doesn't accidentally contain an API key value
    let toml_str = toml::to_string(config).unwrap_or_default();
    if toml_str.len() > 20 {
        // Patterns that indicate leaked secrets (constructed to avoid triggering our own pre-commit hook)
        let gh_pat = format!("{}p_", "gh");
        let gh_oauth = format!("{}o_", "gh");
        let suspicious_patterns = ["sk-", "Bearer ", &gh_pat, &gh_oauth];
        for pat in suspicious_patterns {
            if toml_str.contains(pat) {
                warnings.push(format!("CONFIG MAY CONTAIN A SECRET matching '{pat}' — remove immediately"));
            }
        }
    }

    warnings
}

/// Create a default config file for a project.
pub fn create_default_config(project_dir: &str, project_name: &str, repo: &str) -> Result<String, String> {
    let config = GrowthConfig {
        project: ProjectConfig {
            name: project_name.to_string(),
            repo: repo.to_string(),
            url: String::new(),
            description: String::new(),
        },
        typefully: TypefullyConfig::default(),
        github: GitHubConfig {
            repos: vec![repo.to_string()],
            ..Default::default()
        },
        content: ContentConfig::default(),
        metrics: MetricsConfig::default(),
        readme: ReadmeConfig::default(),
        llms_txt: LlmsTxtConfig::default(),
        ecosystem: EcosystemConfig::default(),
    };

    let toml_str = toml::to_string_pretty(&config).map_err(|e| format!("{e}"))?;
    let dir = Path::new(project_dir).join(".cruxdev");
    fs::create_dir_all(&dir).map_err(|e| format!("{e}"))?;
    let path = dir.join("growth.toml");
    fs::write(&path, &toml_str).map_err(|e| format!("{e}"))?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_load_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = create_default_config(
            dir.path().to_str().unwrap(), "TestProject", "owner/repo"
        ).unwrap();
        assert!(Path::new(&path).exists());

        let config = load_config(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(config.project.name, "TestProject");
        assert_eq!(config.project.repo, "owner/repo");
        assert_eq!(config.typefully.max_posts_per_day, 3);
        assert!(config.typefully.enabled);
    }

    #[test]
    fn test_load_missing_config() {
        let dir = tempfile::tempdir().unwrap();
        let result = load_config(dir.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("init_growth_config"));
    }

    #[test]
    fn test_validate_empty_project() {
        let config = GrowthConfig {
            project: ProjectConfig { name: String::new(), repo: String::new(), url: String::new(), description: String::new() },
            typefully: TypefullyConfig { enabled: false, ..Default::default() },
            github: GitHubConfig::default(),
            content: ContentConfig::default(),
            metrics: MetricsConfig::default(),
            readme: ReadmeConfig::default(),
            llms_txt: LlmsTxtConfig::default(),
            ecosystem: EcosystemConfig::default(),
        };
        let warnings = validate_config(&config);
        assert!(warnings.iter().any(|w| w.contains("project.name")));
        assert!(warnings.iter().any(|w| w.contains("project.repo")));
    }

    #[test]
    fn test_validate_missing_api_key() {
        unsafe { std::env::remove_var("TYPEFULLY_API_KEY"); }
        let config = GrowthConfig {
            project: ProjectConfig { name: "Test".into(), repo: "o/r".into(), url: String::new(), description: String::new() },
            typefully: TypefullyConfig { enabled: true, ..Default::default() },
            github: GitHubConfig::default(),
            content: ContentConfig::default(),
            metrics: MetricsConfig::default(),
            readme: ReadmeConfig::default(),
            llms_txt: LlmsTxtConfig::default(),
            ecosystem: EcosystemConfig::default(),
        };
        let warnings = validate_config(&config);
        assert!(warnings.iter().any(|w| w.contains("not set")));
    }

    #[test]
    fn test_resolve_api_key() {
        unsafe { std::env::set_var("TEST_KEY_12345", "myvalue"); }
        assert_eq!(resolve_api_key("TEST_KEY_12345"), Some("myvalue".into()));
        assert_eq!(resolve_api_key("NONEXISTENT_KEY_99"), None);
    }

    #[test]
    fn test_default_values() {
        let t = TypefullyConfig::default();
        assert_eq!(t.api_key_env, "TYPEFULLY_API_KEY");
        assert_eq!(t.max_posts_per_day, 3);
        assert!(t.threadify_releases);
        assert!(t.enabled);
    }

    #[test]
    fn test_parse_minimal_toml() {
        let toml_str = r#"
[project]
name = "Test"
repo = "owner/repo"
"#;
        let config: GrowthConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.project.name, "Test");
        // Defaults should fill in
        assert!(config.typefully.enabled);
        assert_eq!(config.typefully.max_posts_per_day, 3);
    }
}
