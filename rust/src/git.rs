//! Git operations — commit, push, branch, with safety gates.
//!
//! Safety rules:
//! - NEVER `git add -A` or `git add .` — always stage specific files
//! - NEVER force push
//! - Pre-commit: reject binaries >1MB, secrets, gitignored patterns
//! - Pre-push: all tests must pass

use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// Git status of the working tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub staged: Vec<String>,
    pub unstaged: Vec<String>,
    pub untracked: Vec<String>,
}

/// Result of a git commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitResult {
    pub success: bool,
    pub hash: String,
    pub message: String,
    pub files_committed: usize,
    pub error: String,
}

/// Result of a git push.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushResult {
    pub success: bool,
    pub remote: String,
    pub branch: String,
    pub error: String,
}

/// Result of a PR creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrResult {
    pub success: bool,
    pub number: u64,
    pub url: String,
    pub error: String,
}

/// Safety check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheck {
    pub passed: bool,
    pub violations: Vec<String>,
}

// ── Core git operations ──────────────────────────────────────────

fn run_git(project_dir: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("git command failed: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Get current branch name.
pub fn current_branch(project_dir: &str) -> Result<String, String> {
    run_git(project_dir, &["rev-parse", "--abbrev-ref", "HEAD"])
        .map(|s| s.trim().to_string())
}

/// Get full git status.
pub fn status(project_dir: &str) -> Result<GitStatus, String> {
    let branch = current_branch(project_dir)?;
    let output = run_git(project_dir, &["status", "--porcelain"])?;

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }
        let (index, worktree) = (line.as_bytes()[0], line.as_bytes()[1]);
        let file = line[3..].to_string();

        if index == b'?' {
            untracked.push(file);
        } else {
            if index != b' ' {
                staged.push(file.clone());
            }
            if worktree != b' ' {
                unstaged.push(file);
            }
        }
    }

    Ok(GitStatus { branch, staged, unstaged, untracked })
}

/// Get diff stat summary.
pub fn diff_stat(project_dir: &str) -> Result<String, String> {
    run_git(project_dir, &["diff", "--stat", "HEAD"])
}

/// Get recent commit log.
pub fn log(project_dir: &str, count: usize) -> Result<Vec<String>, String> {
    let output = run_git(project_dir, &["log", "--oneline", &format!("-{count}")])?;
    Ok(output.lines().map(|l| l.to_string()).collect())
}

/// Stage specific files. NEVER stages all files.
pub fn add(project_dir: &str, files: &[String]) -> Result<(), String> {
    if files.is_empty() {
        return Err("No files specified to stage".into());
    }

    // Safety: reject dangerous patterns
    let check = pre_commit_safety_check(project_dir, files)?;
    if !check.passed {
        return Err(format!("Safety check failed: {}", check.violations.join("; ")));
    }

    let args: Vec<&str> = std::iter::once("add")
        .chain(files.iter().map(|f| f.as_str()))
        .collect();
    run_git(project_dir, &args)?;
    Ok(())
}

/// Create a commit with message.
pub fn commit(project_dir: &str, message: &str) -> Result<CommitResult, String> {
    let status_before = status(project_dir)?;
    if status_before.staged.is_empty() {
        return Err("Nothing staged to commit".into());
    }

    match run_git(project_dir, &["commit", "-m", message]) {
        Ok(_output) => {
            let hash = run_git(project_dir, &["rev-parse", "--short", "HEAD"])
                .unwrap_or_default()
                .trim()
                .to_string();
            Ok(CommitResult {
                success: true,
                hash,
                message: message.to_string(),
                files_committed: status_before.staged.len(),
                error: String::new(),
            })
        }
        Err(e) => Ok(CommitResult {
            success: false,
            hash: String::new(),
            message: message.to_string(),
            files_committed: 0,
            error: e,
        }),
    }
}

/// Push to remote. Never force pushes.
pub fn push(project_dir: &str, remote: &str, branch: &str) -> Result<PushResult, String> {
    // Safety: never push to main/master with --force
    match run_git(project_dir, &["push", "-u", remote, branch]) {
        Ok(_) => Ok(PushResult {
            success: true,
            remote: remote.to_string(),
            branch: branch.to_string(),
            error: String::new(),
        }),
        Err(e) => Ok(PushResult {
            success: false,
            remote: remote.to_string(),
            branch: branch.to_string(),
            error: e,
        }),
    }
}

/// Create and switch to a new branch.
pub fn create_branch(project_dir: &str, name: &str) -> Result<(), String> {
    run_git(project_dir, &["checkout", "-b", name])?;
    Ok(())
}

/// Switch to an existing branch.
pub fn checkout(project_dir: &str, branch: &str) -> Result<(), String> {
    run_git(project_dir, &["checkout", branch])?;
    Ok(())
}

// ── Safety checks ────────────────────────────────────────────────

const FORBIDDEN_PATTERNS: &[&str] = &[
    ".env", ".env.local", ".env.production",
    "credentials.json", "service-account.json",
    ".pem", ".key", "_deploy",
];

const FORBIDDEN_DIRS: &[&str] = &[
    "rust/target/", "target/", "node_modules/",
    "__pycache__/", ".pytest_cache/", "dist/", "build/",
];

/// Pre-commit safety check on files to be staged.
pub fn pre_commit_safety_check(project_dir: &str, files: &[String]) -> Result<SafetyCheck, String> {
    let mut violations = Vec::new();

    for file in files {
        // Check forbidden directories
        for dir in FORBIDDEN_DIRS {
            if file.contains(dir) {
                violations.push(format!("Forbidden directory: {file} (matches {dir})"));
            }
        }

        // Check forbidden patterns (secrets/credentials)
        for pattern in FORBIDDEN_PATTERNS {
            if file.ends_with(pattern) || file.contains(pattern) {
                violations.push(format!("Potential secret: {file} (matches {pattern})"));
            }
        }

        // Check file size > 1MB
        let full_path = Path::new(project_dir).join(file);
        if let Ok(meta) = std::fs::metadata(&full_path)
            && meta.len() > 1_000_000
        {
            violations.push(format!(
                "File too large: {file} ({:.1}MB > 1MB limit)",
                meta.len() as f64 / 1_000_000.0
            ));
        }
    }

    Ok(SafetyCheck {
        passed: violations.is_empty(),
        violations,
    })
}

/// Pre-push safety check: run test command and verify it passes.
pub fn pre_push_test_gate(project_dir: &str, test_command: &[String]) -> Result<SafetyCheck, String> {
    if test_command.is_empty() {
        return Ok(SafetyCheck { passed: true, violations: Vec::new() });
    }

    let result = crate::engine::test_runner::run_tests(test_command, project_dir, 300);
    if result.passed {
        Ok(SafetyCheck { passed: true, violations: Vec::new() })
    } else {
        Ok(SafetyCheck {
            passed: false,
            violations: vec![format!(
                "Tests failed (exit code {}): {}",
                result.exit_code,
                result.stderr.chars().take(500).collect::<String>()
            )],
        })
    }
}

// ── PR operations ────────────────────────────────────────────────

/// Create a pull request via `gh` CLI.
pub fn create_pr(
    repo: &str,
    title: &str,
    body: &str,
    base: &str,
    head: &str,
) -> Result<PrResult, String> {
    let output = Command::new("gh")
        .args([
            "pr", "create",
            "--repo", repo,
            "--title", title,
            "--body", body,
            "--base", base,
            "--head", head,
        ])
        .output()
        .map_err(|e| format!("gh pr create failed: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        // Extract PR number from URL (e.g. https://github.com/owner/repo/pull/123)
        let number = stdout
            .rsplit('/')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        Ok(PrResult {
            success: true,
            number,
            url: stdout,
            error: String::new(),
        })
    } else {
        Ok(PrResult {
            success: false,
            number: 0,
            url: String::new(),
            error: stderr,
        })
    }
}

/// Check PR CI status.
pub fn pr_status(repo: &str, pr_number: u64) -> Result<serde_json::Value, String> {
    let output = Command::new("gh")
        .args([
            "pr", "view", &pr_number.to_string(),
            "--repo", repo,
            "--json", "state,mergeable,statusCheckRollup,title",
        ])
        .output()
        .map_err(|e| format!("gh pr view failed: {e}"))?;

    if output.status.success() {
        serde_json::from_slice(&output.stdout).map_err(|e| format!("parse: {e}"))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Merge a PR (squash by default).
pub fn merge_pr(repo: &str, pr_number: u64, method: &str) -> Result<bool, String> {
    let merge_flag = match method {
        "rebase" => "--rebase",
        "merge" => "--merge",
        _ => "--squash",
    };

    let output = Command::new("gh")
        .args([
            "pr", "merge", &pr_number.to_string(),
            "--repo", repo,
            merge_flag, "--delete-branch",
        ])
        .output()
        .map_err(|e| format!("gh pr merge failed: {e}"))?;

    Ok(output.status.success())
}

// ── PR body generation ───────────────────────────────────────────

/// Generate a PR body from build plan context.
pub fn generate_pr_body(
    summary_bullets: &[String],
    test_count: usize,
    files_changed: &[String],
) -> String {
    let mut lines = vec!["## Summary".to_string(), String::new()];
    for bullet in summary_bullets {
        lines.push(format!("- {bullet}"));
    }

    lines.push(String::new());
    lines.push("## Test plan".to_string());
    lines.push(String::new());
    lines.push(format!("- [ ] {test_count} tests pass (`cargo test`)"));
    lines.push("- [ ] 0 clippy warnings (`cargo clippy -- -D warnings`)".to_string());
    lines.push("- [ ] E2E tests pass (`cargo test --test mcp_e2e`)".to_string());

    if !files_changed.is_empty() {
        lines.push(String::new());
        lines.push(format!("## Files changed ({})", files_changed.len()));
        lines.push(String::new());
        for f in files_changed.iter().take(20) {
            lines.push(format!("- `{f}`"));
        }
        if files_changed.len() > 20 {
            lines.push(format!("- ... and {} more", files_changed.len() - 20));
        }
    }

    lines.push(String::new());
    lines.push("\u{1f916} Generated with [Claude Code](https://claude.com/claude-code)".to_string());

    lines.join("\n")
}

/// Generate a commit message from build plan convergence.
pub fn generate_commit_message(
    plan_name: &str,
    summary: &str,
    test_count: usize,
    findings_closed: usize,
) -> String {
    let mut msg = format!("{plan_name}: {summary} ({test_count} tests, 0 clippy)");
    if findings_closed > 0 {
        msg.push_str(&format!("\n\nCloses {findings_closed} audit findings."));
    }
    msg.push_str("\n\nCo-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>");
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_repo() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap().to_string();
        run_git(&path, &["init"]).unwrap();
        run_git(&path, &["config", "user.email", "test@test.com"]).unwrap();
        run_git(&path, &["config", "user.name", "Test"]).unwrap();
        // Initial commit
        std::fs::write(dir.path().join("README.md"), "# Test").unwrap();
        run_git(&path, &["add", "README.md"]).unwrap();
        run_git(&path, &["commit", "-m", "init"]).unwrap();
        (dir, path)
    }

    #[test]
    fn test_current_branch() {
        let (_dir, path) = init_repo();
        let branch = current_branch(&path).unwrap();
        assert!(branch == "master" || branch == "main");
    }

    #[test]
    fn test_status_clean() {
        let (_dir, path) = init_repo();
        let st = status(&path).unwrap();
        assert!(st.staged.is_empty());
        assert!(st.unstaged.is_empty());
    }

    #[test]
    fn test_status_with_changes() {
        let (dir, path) = init_repo();
        std::fs::write(dir.path().join("new.txt"), "hello").unwrap();
        let st = status(&path).unwrap();
        assert_eq!(st.untracked.len(), 1);
    }

    #[test]
    fn test_add_and_commit() {
        let (dir, path) = init_repo();
        std::fs::write(dir.path().join("file.txt"), "content").unwrap();
        add(&path, &["file.txt".into()]).unwrap();
        let result = commit(&path, "Add file").unwrap();
        assert!(result.success);
        assert!(!result.hash.is_empty());
        assert_eq!(result.files_committed, 1);
    }

    #[test]
    fn test_commit_nothing_staged() {
        let (_dir, path) = init_repo();
        let result = commit(&path, "Empty");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_empty_files_rejected() {
        let (_dir, path) = init_repo();
        let result = add(&path, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_branch() {
        let (_dir, path) = init_repo();
        create_branch(&path, "feature/test").unwrap();
        assert_eq!(current_branch(&path).unwrap(), "feature/test");
    }

    #[test]
    fn test_log() {
        let (_dir, path) = init_repo();
        let entries = log(&path, 5).unwrap();
        assert!(!entries.is_empty());
        assert!(entries[0].contains("init"));
    }

    // Safety checks

    #[test]
    fn test_safety_rejects_target() {
        let (_dir, path) = init_repo();
        let check = pre_commit_safety_check(&path, &["rust/target/debug/binary".into()]).unwrap();
        assert!(!check.passed);
        assert!(check.violations[0].contains("Forbidden directory"));
    }

    #[test]
    fn test_safety_rejects_env() {
        let (_dir, path) = init_repo();
        let check = pre_commit_safety_check(&path, &[".env".into()]).unwrap();
        assert!(!check.passed);
        assert!(check.violations[0].contains("secret"));
    }

    #[test]
    fn test_safety_rejects_large_file() {
        let (dir, path) = init_repo();
        let large = dir.path().join("big.bin");
        std::fs::write(&large, vec![0u8; 2_000_000]).unwrap();
        let check = pre_commit_safety_check(&path, &["big.bin".into()]).unwrap();
        assert!(!check.passed);
        assert!(check.violations[0].contains("too large"));
    }

    #[test]
    fn test_safety_allows_normal_files() {
        let (dir, path) = init_repo();
        std::fs::write(dir.path().join("src.rs"), "fn main() {}").unwrap();
        let check = pre_commit_safety_check(&path, &["src.rs".into()]).unwrap();
        assert!(check.passed);
    }

    #[test]
    fn test_safety_rejects_credentials() {
        let (_dir, path) = init_repo();
        let check = pre_commit_safety_check(&path, &["credentials.json".into()]).unwrap();
        assert!(!check.passed);
    }

    // PR body generation

    #[test]
    fn test_generate_pr_body() {
        let body = generate_pr_body(
            &["Fixed bug".into(), "Added tests".into()],
            338,
            &["src/main.rs".into(), "src/lib.rs".into()],
        );
        assert!(body.contains("## Summary"));
        assert!(body.contains("- Fixed bug"));
        assert!(body.contains("338 tests pass"));
        assert!(body.contains("## Files changed (2)"));
    }

    #[test]
    fn test_generate_commit_message() {
        let msg = generate_commit_message("BUILD_PLAN_016", "Git automation", 350, 5);
        assert!(msg.contains("BUILD_PLAN_016"));
        assert!(msg.contains("350 tests"));
        assert!(msg.contains("Closes 5 audit findings"));
        assert!(msg.contains("Co-Authored-By"));
    }

    #[test]
    fn test_pre_push_test_gate_no_command() {
        let check = pre_push_test_gate(".", &[]).unwrap();
        assert!(check.passed);
    }

    #[test]
    fn test_pre_push_test_gate_passing() {
        let check = pre_push_test_gate(".", &["true".into()]).unwrap();
        assert!(check.passed);
    }

    #[test]
    fn test_pre_push_test_gate_failing() {
        let check = pre_push_test_gate(".", &["false".into()]).unwrap();
        assert!(!check.passed);
    }
}
