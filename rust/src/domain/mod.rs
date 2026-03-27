//! Domain architecture — parent projects with typed sub-projects.
//!
//! A domain is a business, ecosystem, or initiative containing multiple projects
//! of different types. Projects can belong to multiple domains.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A domain configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    pub domain: DomainMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainMeta {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub projects: Vec<SubProject>,
}

/// A sub-project within a domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubProject {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub path: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub also_domain: bool,
}

/// Domain health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainHealth {
    pub name: String,
    pub project_count: usize,
    pub projects: Vec<SubProjectStatus>,
    pub missing_projects: Vec<String>,
    pub dependency_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubProjectStatus {
    pub name: String,
    pub project_type: String,
    pub path_exists: bool,
    pub has_cruxdev: bool,
}

/// Load domain config from domain.toml.
pub fn load_domain(domain_dir: &str) -> Result<DomainConfig, String> {
    let path = Path::new(domain_dir).join("domain.toml");
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read domain.toml: {e}"))?;
    toml::from_str(&content)
        .map_err(|e| format!("Invalid domain.toml: {e}"))
}

/// Check if a directory is a domain (has domain.toml or CHARTER.md).
pub fn is_domain(dir: &str) -> bool {
    let root = Path::new(dir);
    root.join("domain.toml").exists() || root.join("CHARTER.md").exists()
}

/// Validate domain — check all sub-project paths exist and deps are valid.
pub fn validate_domain(domain_dir: &str, config: &DomainConfig) -> DomainHealth {
    let root = Path::new(domain_dir);
    let mut projects = Vec::new();
    let mut missing = Vec::new();
    let mut dep_issues = Vec::new();

    let project_names: Vec<&str> = config.domain.projects.iter().map(|p| p.name.as_str()).collect();

    for proj in &config.domain.projects {
        let resolved = root.join(&proj.path);
        let path_exists = resolved.exists();
        let has_cruxdev = resolved.join(".cruxdev").exists();

        if !path_exists {
            missing.push(format!("{} ({})", proj.name, proj.path));
        }

        // Check dependencies reference valid projects
        for dep in &proj.depends_on {
            if !project_names.contains(&dep.as_str()) {
                dep_issues.push(format!("{} depends on {} which is not in the domain", proj.name, dep));
            }
        }

        projects.push(SubProjectStatus {
            name: proj.name.clone(),
            project_type: proj.project_type.clone(),
            path_exists,
            has_cruxdev,
        });
    }

    DomainHealth {
        name: config.domain.name.clone(),
        project_count: config.domain.projects.len(),
        projects,
        missing_projects: missing,
        dependency_issues: dep_issues,
    }
}

/// Create a new domain with template docs.
pub fn init_domain(
    domain_dir: &str,
    name: &str,
    description: &str,
) -> Result<String, String> {
    let root = Path::new(domain_dir);
    fs::create_dir_all(root.join("docs")).map_err(|e| format!("{e}"))?;

    // Write domain.toml
    let config = format!(
        "[domain]\nname = \"{name}\"\ndescription = \"{description}\"\nowner = \"\"\nprojects = []\n"
    );
    let toml_path = root.join("domain.toml");
    fs::write(&toml_path, config).map_err(|e| format!("{e}"))?;

    // Write CHARTER.md
    let charter = format!("# {name} Domain Charter\n\n## Purpose\n\n{description}\n\n## Scope\n\n## Ownership\n\n## Boundaries\n\n");
    fs::write(root.join("docs/CHARTER.md"), charter).map_err(|e| format!("{e}"))?;

    // Write STRATEGY.md
    let strategy = format!("# {name} Strategy\n\n## Goals\n\n## Approach\n\n## Timeline\n\n## Success Criteria\n\n");
    fs::write(root.join("docs/STRATEGY.md"), strategy).map_err(|e| format!("{e}"))?;

    // Write INVENTORY.md
    let inventory = format!("# {name} Inventory\n\n## Repositories\n\n## Services\n\n## Tools\n\n");
    fs::write(root.join("docs/INVENTORY.md"), inventory).map_err(|e| format!("{e}"))?;

    Ok(toml_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_domain_with_toml() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("domain.toml"), "[domain]\nname = \"test\"\n").unwrap();
        assert!(is_domain(dir.path().to_str().unwrap()));
    }

    #[test]
    fn test_is_domain_with_charter() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CHARTER.md"), "# Test").unwrap();
        assert!(is_domain(dir.path().to_str().unwrap()));
    }

    #[test]
    fn test_is_not_domain() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!is_domain(dir.path().to_str().unwrap()));
    }

    #[test]
    fn test_load_domain() {
        let dir = tempfile::tempdir().unwrap();
        let toml = r#"
[domain]
name = "TestDomain"
description = "A test"

[[domain.projects]]
name = "app"
type = "software-existing"
path = "../app"

[[domain.projects]]
name = "site"
type = "website"
path = "../site"
depends_on = ["app"]
"#;
        fs::write(dir.path().join("domain.toml"), toml).unwrap();
        let config = load_domain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(config.domain.name, "TestDomain");
        assert_eq!(config.domain.projects.len(), 2);
        assert_eq!(config.domain.projects[1].depends_on, vec!["app"]);
    }

    #[test]
    fn test_validate_domain_missing_project() {
        let dir = tempfile::tempdir().unwrap();
        let config = DomainConfig {
            domain: DomainMeta {
                name: "Test".into(),
                description: String::new(),
                owner: String::new(),
                projects: vec![SubProject {
                    name: "missing".into(),
                    project_type: "software-existing".into(),
                    path: "../nonexistent".into(),
                    role: String::new(),
                    depends_on: Vec::new(),
                    also_domain: false,
                }],
            },
        };
        let health = validate_domain(dir.path().to_str().unwrap(), &config);
        assert_eq!(health.missing_projects.len(), 1);
    }

    #[test]
    fn test_validate_domain_bad_dependency() {
        let dir = tempfile::tempdir().unwrap();
        let config = DomainConfig {
            domain: DomainMeta {
                name: "Test".into(),
                description: String::new(),
                owner: String::new(),
                projects: vec![SubProject {
                    name: "app".into(),
                    project_type: "software-existing".into(),
                    path: ".".into(),
                    role: String::new(),
                    depends_on: vec!["nonexistent".into()],
                    also_domain: false,
                }],
            },
        };
        let health = validate_domain(dir.path().to_str().unwrap(), &config);
        assert_eq!(health.dependency_issues.len(), 1);
    }

    #[test]
    fn test_init_domain() {
        let dir = tempfile::tempdir().unwrap();
        let path = init_domain(dir.path().to_str().unwrap(), "MyDomain", "A test domain").unwrap();
        assert!(Path::new(&path).exists());
        assert!(dir.path().join("docs/CHARTER.md").exists());
        assert!(dir.path().join("docs/STRATEGY.md").exists());
        assert!(dir.path().join("docs/INVENTORY.md").exists());
    }
}
