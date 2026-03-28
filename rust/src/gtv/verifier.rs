//! Verification logic for extracted claims.

use std::path::Path;
use std::process::Command;

use super::{Claim, ClaimType, Confidence, VerificationResult};

/// Verify a single claim against the project.
pub fn verify_claim(claim: &Claim, project_dir: &str) -> VerificationResult {
    match claim.claim_type {
        ClaimType::Numeric => verify_numeric(claim, project_dir),
        ClaimType::Feature => verify_feature(claim, project_dir),
        ClaimType::Status => verify_status(claim),
        ClaimType::Path => verify_path(claim, project_dir),
        ClaimType::Url => verify_url(claim),
    }
}

/// Verify all claims from a scan.
pub fn verify_all(claims: &[Claim], project_dir: &str) -> Vec<VerificationResult> {
    claims.iter().map(|c| verify_claim(c, project_dir)).collect()
}

/// Verify a numeric claim (e.g., "485 tests").
fn verify_numeric(claim: &Claim, project_dir: &str) -> VerificationResult {
    let parts: Vec<&str> = claim.value.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return uncertain(claim, "Could not parse numeric claim");
    }

    let claimed_number: u64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return uncertain(claim, "Could not parse number"),
    };

    let keyword = parts[1].to_lowercase();

    let (actual, source) = if keyword.contains("test") {
        count_tests(project_dir)
    } else if keyword.contains("tool") {
        count_tools(project_dir)
    } else if keyword.contains("page") {
        count_pages(project_dir)
    } else {
        return uncertain(claim, &format!("No verifier for numeric type '{keyword}'"));
    };

    match actual {
        Some(actual_count) => {
            let verified = actual_count >= claimed_number;
            VerificationResult {
                claim: claim.clone(),
                verified,
                actual_value: Some(actual_count.to_string()),
                message: if verified {
                    format!("Verified: actual {actual_count} >= claimed {claimed_number} ({source})")
                } else {
                    format!("FAILED: actual {actual_count} < claimed {claimed_number} ({source})")
                },
                confidence: Confidence::High,
            }
        }
        None => uncertain(claim, &format!("Could not determine actual count ({source})")),
    }
}

/// Count tests by running `cargo test`.
fn count_tests(project_dir: &str) -> (Option<u64>, String) {
    let rust_dir = Path::new(project_dir).join("rust");
    let dir = if rust_dir.exists() { &rust_dir } else { Path::new(project_dir) };

    let output = Command::new("cargo")
        .args(["test", "--", "--list"])
        .current_dir(dir)
        .env("PATH", format!("{}/.cargo/bin:{}", env_home(), std::env::var("PATH").unwrap_or_default()))
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let count = stdout.lines().filter(|l| l.ends_with(": test")).count() as u64;
            (Some(count), "cargo test --list".into())
        }
        _ => (None, "cargo test failed".into()),
    }
}

/// Count MCP tools by grepping server.rs for tool registrations.
fn count_tools(project_dir: &str) -> (Option<u64>, String) {
    let server_path = Path::new(project_dir).join("rust/src/server.rs");
    if !server_path.exists() {
        return (None, "server.rs not found".into());
    }

    match std::fs::read_to_string(&server_path) {
        Ok(content) => {
            // Count tool registrations — lines matching the tool name pattern
            let count = content.lines()
                .filter(|l| l.trim().starts_with("\"") && l.contains("=> {"))
                .count() as u64;

            if count == 0 {
                // Fallback: count unique tool definitions
                let re = regex::Regex::new(r#""([a-z_]+)"\s*=>"#).unwrap();
                let count = re.captures_iter(&content).count() as u64;
                (Some(count), "grep tool definitions in server.rs".into())
            } else {
                (Some(count), "grep tool registrations in server.rs".into())
            }
        }
        Err(_) => (None, "could not read server.rs".into()),
    }
}

/// Count pages by listing .astro and .md files in the website.
fn count_pages(project_dir: &str) -> (Option<u64>, String) {
    // Try sibling -dev directory (convention: cruxdev → cruxdev-dev)
    let dev_dir = format!("{}-dev/src/pages", project_dir.trim_end_matches('/'));
    let pages_dir = if Path::new(&dev_dir).exists() {
        dev_dir
    } else {
        format!("{}/src/pages", project_dir)
    };

    if !Path::new(&pages_dir).exists() {
        return (None, "pages directory not found".into());
    }

    let output = Command::new("find")
        .args([&pages_dir, "-name", "*.astro", "-o", "-name", "*.md"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let count = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .count() as u64;
            (Some(count), format!("find in {pages_dir}"))
        }
        _ => (None, "find failed".into()),
    }
}

/// Verify a feature claim by grepping the codebase.
fn verify_feature(claim: &Claim, project_dir: &str) -> VerificationResult {
    // Extract the feature keyword (after the verb)
    let value_lower = claim.value.to_lowercase();
    let keyword = value_lower
        .split_once(' ')
        .map(|(_, rest)| rest.trim())
        .unwrap_or(&value_lower);

    // Grep the Rust source for the keyword
    let src_dir = Path::new(project_dir).join("rust/src");
    let output = Command::new("grep")
        .args(["-r", "-l", "-i", keyword])
        .arg(&src_dir)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let files = String::from_utf8_lossy(&out.stdout);
            let file_count = files.lines().count();
            if file_count > 0 {
                VerificationResult {
                    claim: claim.clone(),
                    verified: true,
                    actual_value: Some(format!("{file_count} files match")),
                    message: format!("Found '{keyword}' in {file_count} source files (grep heuristic — may be comment/test, not implementation)"),
                    confidence: Confidence::Medium,
                }
            } else {
                VerificationResult {
                    claim: claim.clone(),
                    verified: false,
                    actual_value: None,
                    message: format!("'{keyword}' not found in source files"),
                    confidence: Confidence::Medium,
                }
            }
        }
        _ => uncertain(claim, &format!("grep for '{keyword}' failed")),
    }
}

/// Verify a status claim — currently returns uncertain (needs service-specific checks).
fn verify_status(claim: &Claim) -> VerificationResult {
    // Status claims require service-specific verification
    // For v1, flag as needing manual review
    VerificationResult {
        claim: claim.clone(),
        verified: false,
        actual_value: None,
        message: format!("Status claim '{}' requires manual verification", claim.value),
        confidence: Confidence::Low,
    }
}

/// Verify a file path exists.
fn verify_path(claim: &Claim, project_dir: &str) -> VerificationResult {
    let full_path = Path::new(project_dir).join(&claim.value);
    let exists = full_path.exists();

    VerificationResult {
        claim: claim.clone(),
        verified: exists,
        actual_value: Some(exists.to_string()),
        message: if exists {
            format!("Path exists: {}", full_path.display())
        } else {
            format!("Path NOT FOUND: {}", full_path.display())
        },
        confidence: Confidence::High,
    }
}

/// Verify a URL is accessible (HTTP HEAD with timeout).
fn verify_url(claim: &Claim) -> VerificationResult {
    let output = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", "-m", "5", "-L", &claim.value])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let status_code = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let ok = status_code.starts_with('2') || status_code.starts_with('3');

            VerificationResult {
                claim: claim.clone(),
                verified: ok,
                actual_value: Some(status_code.clone()),
                message: if ok {
                    format!("URL accessible (HTTP {})", status_code)
                } else {
                    format!("URL returned HTTP {}", status_code)
                },
                confidence: Confidence::High,
            }
        }
        _ => VerificationResult {
            claim: claim.clone(),
            verified: false,
            actual_value: None,
            message: "URL check failed (timeout or network error)".to_string(),
            confidence: Confidence::High,
        },
    }
}

fn uncertain(claim: &Claim, message: &str) -> VerificationResult {
    VerificationResult {
        claim: claim.clone(),
        verified: false,
        actual_value: None,
        message: message.to_string(),
        confidence: Confidence::Low,
    }
}

fn env_home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/root".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_claim(claim_type: ClaimType, value: &str) -> Claim {
        Claim {
            claim_type,
            value: value.into(),
            line_number: 1,
            source_text: format!("test: {value}"),
        }
    }

    #[test]
    fn test_verify_path_exists() {
        let claim = make_claim(ClaimType::Path, "src/gtv/mod.rs");
        let result = verify_path(&claim, env!("CARGO_MANIFEST_DIR"));
        assert!(result.verified);
        assert_eq!(result.confidence, Confidence::High);
    }

    #[test]
    fn test_verify_path_not_exists() {
        let claim = make_claim(ClaimType::Path, "src/nonexistent/file.rs");
        let result = verify_path(&claim, env!("CARGO_MANIFEST_DIR"));
        assert!(!result.verified);
        assert_eq!(result.confidence, Confidence::High);
    }

    #[test]
    fn test_verify_numeric_unparseable() {
        let claim = make_claim(ClaimType::Numeric, "many tests");
        let result = verify_numeric(&claim, "/tmp");
        assert_eq!(result.confidence, Confidence::Low);
    }

    #[test]
    fn test_verify_status_returns_low_confidence() {
        let claim = make_claim(ClaimType::Status, "active");
        let result = verify_status(&claim);
        assert_eq!(result.confidence, Confidence::Low);
    }

    #[test]
    fn test_verify_feature_in_own_codebase() {
        let claim = make_claim(ClaimType::Feature, "supports claim extraction");
        let result = verify_feature(&claim, &format!("{}/..", env!("CARGO_MANIFEST_DIR")));
        // Should find "claim" or "extraction" in our own source
        assert_eq!(result.confidence, Confidence::Medium);
    }

    #[test]
    fn test_uncertain_helper() {
        let claim = make_claim(ClaimType::Numeric, "5 things");
        let result = uncertain(&claim, "test message");
        assert!(!result.verified);
        assert_eq!(result.confidence, Confidence::Low);
        assert_eq!(result.message, "test message");
    }
}
