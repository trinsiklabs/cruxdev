//! Install CruxDev into a project — MCP config, gitignore, pre-commit hook.

use serde_json::json;
use std::fs;
use std::path::Path;

const SECURITY_GITIGNORE: &str = "\n# CruxDev security — NEVER commit these\n\
**/tasks/*.output\n**/-Users-*\n*.key\n*.pem\n*_deploy\n\
.env\n.env.local\n.env.production\n.env.*.local\n.crux/\n\
.cruxdev/convergence_state/\n*.jsonl\n";

pub fn install(project_dir: &str) -> serde_json::Value {
    let root = Path::new(project_dir);

    // Create .cruxdev/
    let _ = fs::create_dir_all(root.join(".cruxdev"));

    // Write .mcp.json
    let mcp_path = root.join(".mcp.json");
    let mut config: serde_json::Value = if mcp_path.exists() {
        fs::read_to_string(&mcp_path).ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_else(|| json!({"mcpServers": {}}))
    } else {
        json!({"mcpServers": {}})
    };

    let cruxdev_bin = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "cruxdev".into());

    config["mcpServers"]["cruxdev"] = json!({
        "type": "stdio",
        "command": cruxdev_bin,
        "args": ["mcp", "start"],
    });
    let _ = fs::write(&mcp_path, serde_json::to_string_pretty(&config).unwrap_or_default());

    // Ensure .gitignore has security patterns
    ensure_gitignore_security(project_dir);

    // Install pre-commit hook
    install_secret_scanner(project_dir);

    // Copy skills and commands for slash command support
    let skills_installed = install_skills_and_commands(project_dir);

    let mut items = vec![
        format!("Created .cruxdev/ in {project_dir}"),
        "Added cruxdev to .mcp.json".into(),
        "Updated .gitignore with security patterns".into(),
        "Installed pre-commit secret scanner".into(),
    ];
    if skills_installed {
        items.push("Installed .claude/skills/ and .claude/commands/ for slash commands".into());
    }

    json!({
        "status": "installed",
        "items": items
    })
}

fn ensure_gitignore_security(project_dir: &str) -> bool {
    let path = Path::new(project_dir).join(".gitignore");
    let existing = fs::read_to_string(&path).unwrap_or_default();
    if existing.contains("CruxDev security") {
        return false;
    }
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(SECURITY_GITIGNORE.as_bytes())
        });
    true
}

fn install_secret_scanner(project_dir: &str) -> bool {
    let git_dir = Path::new(project_dir).join(".git");
    if !git_dir.is_dir() {
        return false;
    }
    let hooks_dir = git_dir.join("hooks");
    let _ = fs::create_dir_all(&hooks_dir);
    let hook = hooks_dir.join("pre-commit");
    if hook.exists() {
        return false;
    }
    let script = r#"#!/bin/bash
FOUND=0
for pattern in sk-or-v1- sk-ant- sk-proj- ghp_ ghs_; do
  if git diff --cached --no-color -- . ':(exclude)scripts/pre-commit-secrets' 2>/dev/null | LC_ALL=C grep -qF "$pattern"; then
    echo "BLOCKED: '$pattern' found in staged changes"
    FOUND=1
  fi
done
if git diff --cached --no-color -- . ':(exclude)scripts/pre-commit-secrets' 2>/dev/null | LC_ALL=C grep -qF "PRIVATE KEY"; then
  echo "BLOCKED: Private key found in staged changes"
  FOUND=1
fi
if [ $FOUND -eq 1 ]; then
  echo "Commit blocked. Remove secrets before committing."
  exit 1
fi
"#;
    if fs::write(&hook, script).is_ok() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&hook, fs::Permissions::from_mode(0o755));
        }
        return true;
    }
    false
}

/// Copy .claude/skills/ and .claude/commands/ from the CruxDev source directory
/// into the target project, giving it slash command support.
fn install_skills_and_commands(project_dir: &str) -> bool {
    // Find the CruxDev source directory (parent of the binary's rust/ dir, or known path)
    let cruxdev_src = find_cruxdev_source();
    let Some(src_root) = cruxdev_src else {
        return false;
    };

    let src_skills = src_root.join(".claude/skills");
    let src_commands = src_root.join(".claude/commands");
    let dst_root = Path::new(project_dir);

    if !src_skills.is_dir() && !src_commands.is_dir() {
        return false;
    }

    let mut installed = false;

    // Copy skills
    if src_skills.is_dir() {
        let dst_skills = dst_root.join(".claude/skills");
        if copy_dir_recursive(&src_skills, &dst_skills).is_ok() {
            installed = true;
        }
    }

    // Copy commands
    if src_commands.is_dir() {
        let dst_commands = dst_root.join(".claude/commands");
        if copy_dir_recursive(&src_commands, &dst_commands).is_ok() {
            installed = true;
        }
    }

    installed
}

/// Find the CruxDev source directory.
fn find_cruxdev_source() -> Option<std::path::PathBuf> {
    // Try: binary is at cruxdev/rust/target/release/cruxdev → source is cruxdev/
    if let Ok(exe) = std::env::current_exe() {
        // Walk up from the exe to find a directory with .claude/skills/
        let mut path = exe.as_path();
        for _ in 0..6 {
            if let Some(parent) = path.parent() {
                if parent.join(".claude/skills").is_dir() {
                    return Some(parent.to_path_buf());
                }
                path = parent;
            }
        }
    }

    // Fallback: well-known path
    let home = std::env::var("HOME").unwrap_or_default();
    let known = Path::new(&home).join("personal/cruxdev");
    if known.join(".claude/skills").is_dir() {
        return Some(known);
    }

    None
}

/// Recursively copy a directory.
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_creates_cruxdev_dir() {
        let dir = tempfile::tempdir().unwrap();
        install(dir.path().to_str().unwrap());
        assert!(dir.path().join(".cruxdev").is_dir());
    }

    #[test]
    fn install_writes_mcp_json() {
        let dir = tempfile::tempdir().unwrap();
        install(dir.path().to_str().unwrap());
        let content = fs::read_to_string(dir.path().join(".mcp.json")).unwrap();
        assert!(content.contains("cruxdev"));
    }

    #[test]
    fn install_adds_gitignore() {
        let dir = tempfile::tempdir().unwrap();
        install(dir.path().to_str().unwrap());
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("CruxDev security"));
    }

    #[test]
    fn gitignore_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        install(dir.path().to_str().unwrap());
        install(dir.path().to_str().unwrap());
        let content = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert_eq!(content.matches("CruxDev security").count(), 1);
    }

    #[test]
    fn install_adds_hook() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".git").join("hooks")).unwrap();
        install(dir.path().to_str().unwrap());
        assert!(dir.path().join(".git").join("hooks").join("pre-commit").exists());
    }

    #[test]
    fn preserves_existing_hook() {
        let dir = tempfile::tempdir().unwrap();
        let hooks = dir.path().join(".git").join("hooks");
        fs::create_dir_all(&hooks).unwrap();
        fs::write(hooks.join("pre-commit"), "existing").unwrap();
        install(dir.path().to_str().unwrap());
        assert_eq!(fs::read_to_string(hooks.join("pre-commit")).unwrap(), "existing");
    }
}
