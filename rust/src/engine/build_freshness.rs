//! Build artifact freshness detection — prevent stale binaries, bundles, sites.
//!
//! Detects build targets from project files (Cargo.toml, package.json, go.mod, etc.),
//! checks if artifacts are older than source files, and optionally rebuilds.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// A build target — an artifact that must be rebuilt when source changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTarget {
    pub artifact: String,
    pub command: String,
    pub working_dir: String,
    pub source_dirs: Vec<String>,
    pub artifact_type: String, // binary, static_site, bundle, image, wheel
}

/// Result of a freshness check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessResult {
    pub artifact: String,
    pub stale: bool,
    pub artifact_mtime: Option<f64>,
    pub newest_source_mtime: Option<f64>,
    pub newest_source_file: String,
    pub build_command: String,
}

/// Result of a rebuild attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildResult {
    pub artifact: String,
    pub success: bool,
    pub output: String,
    pub error: String,
}

/// Get file modification time as seconds since epoch.
fn mtime(path: &Path) -> Option<f64> {
    fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
}

/// Find the newest file modification time in a directory tree.
fn newest_mtime_in_dir(dir: &Path) -> Option<(f64, String)> {
    let mut newest: Option<(f64, String)> = None;

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.')
                && name != "node_modules"
                && name != "target"
                && name != "__pycache__"
                && name != "dist"
                && name != "build"
        })
        .filter_map(|e| e.ok())
    {
        if !entry.path().is_file() {
            continue;
        }
        if let Some(mt) = mtime(entry.path()) {
            let path_str = entry.path().to_string_lossy().to_string();
            if newest.is_none() || mt > newest.as_ref().unwrap().0 {
                newest = Some((mt, path_str));
            }
        }
    }

    newest
}

/// Check if a build artifact is stale relative to its source directories.
pub fn check_freshness(project_dir: &str, target: &BuildTarget) -> FreshnessResult {
    let artifact_path = Path::new(project_dir).join(&target.artifact);
    let artifact_mt = mtime(&artifact_path);

    // Find newest source file across all source dirs
    let mut newest_source: Option<(f64, String)> = None;
    for src_dir in &target.source_dirs {
        let full = Path::new(project_dir).join(src_dir);
        if let Some((mt, path)) = newest_mtime_in_dir(&full)
            && (newest_source.is_none() || mt > newest_source.as_ref().unwrap().0)
        {
            newest_source = Some((mt, path));
        }
    }

    let stale = match (artifact_mt, &newest_source) {
        (None, _) => true, // artifact doesn't exist
        (_, None) => false, // no source files found
        (Some(art), Some((src, _))) => *src > art, // source newer than artifact
    };

    FreshnessResult {
        artifact: target.artifact.clone(),
        stale,
        artifact_mtime: artifact_mt,
        newest_source_mtime: newest_source.as_ref().map(|(mt, _)| *mt),
        newest_source_file: newest_source.map(|(_, f)| f).unwrap_or_default(),
        build_command: target.command.clone(),
    }
}

/// Check all build targets in a project.
pub fn check_all_freshness(project_dir: &str, targets: &[BuildTarget]) -> Vec<FreshnessResult> {
    targets.iter().map(|t| check_freshness(project_dir, t)).collect()
}

/// Auto-detect build targets from project files.
pub fn detect_build_targets(project_dir: &str) -> Vec<BuildTarget> {
    let mut targets = Vec::new();
    let root = Path::new(project_dir);

    // Rust: Cargo.toml
    for cargo_dir in find_file_recursive(root, "Cargo.toml", 2) {
        let rel = cargo_dir.strip_prefix(root).unwrap_or(&cargo_dir);
        let name = parse_cargo_name(&cargo_dir.join("Cargo.toml"));
        let src_dir = rel.join("src").to_string_lossy().to_string();
        let artifact = rel.join("target/release").join(&name).to_string_lossy().to_string();
        let wd = rel.to_string_lossy().to_string();
        targets.push(BuildTarget {
            artifact,
            command: "cargo build --release".into(),
            working_dir: wd,
            source_dirs: vec![src_dir],
            artifact_type: "binary".into(),
        });
    }

    // Node/JS: package.json with build script
    for pkg_dir in find_file_recursive(root, "package.json", 2) {
        if has_npm_build_script(&pkg_dir.join("package.json")) {
            let rel = pkg_dir.strip_prefix(root).unwrap_or(&pkg_dir);
            let wd = rel.to_string_lossy().to_string();
            targets.push(BuildTarget {
                artifact: rel.join("dist").to_string_lossy().to_string(),
                command: "npm run build".into(),
                working_dir: wd,
                source_dirs: vec![
                    rel.join("src").to_string_lossy().to_string(),
                    rel.join("public").to_string_lossy().to_string(),
                ],
                artifact_type: "bundle".into(),
            });
        }
    }

    // Go: go.mod
    for go_dir in find_file_recursive(root, "go.mod", 2) {
        let rel = go_dir.strip_prefix(root).unwrap_or(&go_dir);
        let name = rel.file_name().unwrap_or_default().to_string_lossy().to_string();
        let wd = rel.to_string_lossy().to_string();
        targets.push(BuildTarget {
            artifact: rel.join(&name).to_string_lossy().to_string(),
            command: "go build -o .".into(),
            working_dir: wd,
            source_dirs: vec![rel.to_string_lossy().to_string()],
            artifact_type: "binary".into(),
        });
    }

    // Docker: Dockerfile
    for docker_dir in find_file_recursive(root, "Dockerfile", 1) {
        let rel = docker_dir.strip_prefix(root).unwrap_or(&docker_dir);
        let wd = rel.to_string_lossy().to_string();
        targets.push(BuildTarget {
            artifact: ".docker-build-marker".into(),
            command: "docker build -t app .".into(),
            working_dir: wd,
            source_dirs: vec![rel.to_string_lossy().to_string()],
            artifact_type: "image".into(),
        });
    }

    targets
}

/// Rebuild a stale target.
pub fn rebuild(project_dir: &str, target: &BuildTarget) -> RebuildResult {
    let wd = if target.working_dir.is_empty() {
        project_dir.to_string()
    } else {
        Path::new(project_dir).join(&target.working_dir).to_string_lossy().to_string()
    };

    let parts: Vec<&str> = target.command.split_whitespace().collect();
    if parts.is_empty() {
        return RebuildResult {
            artifact: target.artifact.clone(),
            success: false,
            output: String::new(),
            error: "Empty build command".into(),
        };
    }

    match Command::new(parts[0]).args(&parts[1..]).current_dir(&wd).output() {
        Ok(output) => RebuildResult {
            artifact: target.artifact.clone(),
            success: output.status.success(),
            output: String::from_utf8_lossy(&output.stdout).chars().take(2000).collect(),
            error: if output.status.success() {
                String::new()
            } else {
                String::from_utf8_lossy(&output.stderr).chars().take(2000).collect()
            },
        },
        Err(e) => RebuildResult {
            artifact: target.artifact.clone(),
            success: false,
            output: String::new(),
            error: format!("{e}"),
        },
    }
}

/// Rebuild all stale targets. Returns results.
pub fn rebuild_all_stale(project_dir: &str, targets: &[BuildTarget]) -> Vec<RebuildResult> {
    let mut results = Vec::new();
    for target in targets {
        let freshness = check_freshness(project_dir, target);
        if freshness.stale {
            results.push(rebuild(project_dir, target));
        }
    }
    results
}

// --- Helpers ---

fn find_file_recursive(root: &Path, filename: &str, max_depth: usize) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    for entry in walkdir::WalkDir::new(root)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 { return true; }
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.') && name != "node_modules" && name != "target"
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_name().to_string_lossy() == filename
            && let Some(parent) = entry.path().parent()
        {
            found.push(parent.to_path_buf());
        }
    }
    found
}

fn parse_cargo_name(cargo_path: &Path) -> String {
    fs::read_to_string(cargo_path)
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("name"))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_else(|| "app".into())
}

fn has_npm_build_script(package_json: &Path) -> bool {
    fs::read_to_string(package_json)
        .ok()
        .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
        .and_then(|v| v.get("scripts")?.get("build").cloned())
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_target(artifact: &str, source_dir: &str) -> BuildTarget {
        BuildTarget {
            artifact: artifact.into(),
            command: "echo built".into(),
            working_dir: String::new(),
            source_dirs: vec![source_dir.into()],
            artifact_type: "binary".into(),
        }
    }

    #[test]
    fn test_fresh_artifact() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("main.rs"), "fn main() {}").unwrap();

        // Sleep briefly so artifact is newer
        std::thread::sleep(std::time::Duration::from_millis(50));
        fs::write(dir.path().join("binary"), "compiled").unwrap();

        let target = make_target("binary", "src");
        let result = check_freshness(dir.path().to_str().unwrap(), &target);
        assert!(!result.stale, "artifact should be fresh");
    }

    #[test]
    fn test_stale_artifact() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("binary"), "old").unwrap();

        // Sleep so source is newer
        std::thread::sleep(std::time::Duration::from_millis(50));
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("main.rs"), "fn main() { new() }").unwrap();

        let target = make_target("binary", "src");
        let result = check_freshness(dir.path().to_str().unwrap(), &target);
        assert!(result.stale, "artifact should be stale");
    }

    #[test]
    fn test_missing_artifact() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("main.rs"), "fn main() {}").unwrap();

        let target = make_target("nonexistent", "src");
        let result = check_freshness(dir.path().to_str().unwrap(), &target);
        assert!(result.stale, "missing artifact should be stale");
    }

    #[test]
    fn test_rebuild_echo() {
        let dir = tempfile::tempdir().unwrap();
        let target = BuildTarget {
            artifact: "out.txt".into(),
            command: "echo hello".into(),
            working_dir: String::new(),
            source_dirs: vec![],
            artifact_type: "binary".into(),
        };
        let result = rebuild(dir.path().to_str().unwrap(), &target);
        assert!(result.success);
        assert!(result.output.contains("hello"));
    }

    #[test]
    fn test_rebuild_failure() {
        let dir = tempfile::tempdir().unwrap();
        let target = BuildTarget {
            artifact: "out".into(),
            command: "false".into(),
            working_dir: String::new(),
            source_dirs: vec![],
            artifact_type: "binary".into(),
        };
        let result = rebuild(dir.path().to_str().unwrap(), &target);
        assert!(!result.success);
    }

    #[test]
    fn test_detect_cargo_target() {
        let dir = tempfile::tempdir().unwrap();
        let rust = dir.path().join("rust");
        fs::create_dir_all(rust.join("src")).unwrap();
        fs::write(rust.join("Cargo.toml"), "[package]\nname = \"myapp\"\nversion = \"0.1.0\"\n").unwrap();
        fs::write(rust.join("src/main.rs"), "fn main() {}").unwrap();

        let targets = detect_build_targets(dir.path().to_str().unwrap());
        assert!(!targets.is_empty(), "should detect Cargo target");
        assert!(targets[0].artifact.contains("myapp"));
        assert_eq!(targets[0].command, "cargo build --release");
    }

    #[test]
    fn test_detect_npm_target() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"scripts":{"build":"astro build"}}"#).unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();

        let targets = detect_build_targets(dir.path().to_str().unwrap());
        assert!(!targets.is_empty(), "should detect npm build target");
        assert_eq!(targets[0].command, "npm run build");
    }

    #[test]
    fn test_no_targets_in_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let targets = detect_build_targets(dir.path().to_str().unwrap());
        assert!(targets.is_empty());
    }

    #[test]
    fn test_rebuild_all_stale_skips_fresh() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("main.rs"), "fn main() {}").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));
        fs::write(dir.path().join("binary"), "compiled").unwrap();

        let target = make_target("binary", "src");
        let results = rebuild_all_stale(dir.path().to_str().unwrap(), &[target]);
        assert!(results.is_empty(), "should skip fresh artifact");
    }
}
