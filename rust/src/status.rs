//! Health checks for CruxDev installation.

use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct StatusReport {
    pub healthy: bool,
    pub checks: Vec<HealthCheck>,
}

pub fn get_status(project_dir: &str) -> StatusReport {
    let root = Path::new(project_dir);
    let mut checks = Vec::new();

    // Check .cruxdev/ exists
    checks.push(HealthCheck {
        name: "cruxdev_dir".into(),
        passed: root.join(".cruxdev").is_dir(),
        message: if root.join(".cruxdev").is_dir() { "Found".into() } else { "Missing .cruxdev/".into() },
    });

    // Check .mcp.json exists and has cruxdev
    let mcp_path = root.join(".mcp.json");
    let mcp_ok = mcp_path.exists() && std::fs::read_to_string(&mcp_path)
        .map(|c| c.contains("cruxdev"))
        .unwrap_or(false);
    checks.push(HealthCheck {
        name: "mcp_config".into(),
        passed: mcp_ok,
        message: if mcp_ok { "cruxdev in .mcp.json".into() } else { "Missing or no cruxdev entry".into() },
    });

    // Check docs/ exists
    checks.push(HealthCheck {
        name: "docs_dir".into(),
        passed: root.join("docs").is_dir(),
        message: if root.join("docs").is_dir() { "Found".into() } else { "No docs/ directory".into() },
    });

    // Check .gitignore has security patterns
    let gitignore_ok = std::fs::read_to_string(root.join(".gitignore"))
        .map(|c| c.contains("CruxDev security"))
        .unwrap_or(false);
    checks.push(HealthCheck {
        name: "gitignore_security".into(),
        passed: gitignore_ok,
        message: if gitignore_ok { "Security patterns present".into() } else { "Missing security patterns".into() },
    });

    let healthy = checks.iter().all(|c| c.passed);
    StatusReport { healthy, checks }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn healthy_project() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join(".cruxdev")).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join(".mcp.json"), r#"{"mcpServers":{"cruxdev":{}}}"#).unwrap();
        fs::write(root.join(".gitignore"), "# CruxDev security\n").unwrap();

        let report = get_status(root.to_str().unwrap());
        assert!(report.healthy);
        assert!(report.checks.iter().all(|c| c.passed));
    }

    #[test]
    fn empty_project() {
        let dir = tempfile::tempdir().unwrap();
        let report = get_status(dir.path().to_str().unwrap());
        assert!(!report.healthy);
    }

    #[test]
    fn partial_project() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".cruxdev")).unwrap();
        let report = get_status(dir.path().to_str().unwrap());
        assert!(!report.healthy);
        assert!(report.checks[0].passed); // cruxdev_dir
        assert!(!report.checks[1].passed); // mcp_config
    }
}
