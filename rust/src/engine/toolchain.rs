//! Toolchain detection and integration — detect project type, test framework,
//! linters, and parse their output for structured findings.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Detected project toolchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toolchain {
    pub language: String,
    pub test_framework: String,
    pub test_command: Vec<String>,
    pub lint_command: Vec<String>,
    pub type_check_command: Vec<String>,
    pub audit_command: Vec<String>,
    pub coverage_command: Vec<String>,
}

/// Detect toolchain from project files.
pub fn detect_toolchain(project_dir: &str) -> Option<Toolchain> {
    let root = Path::new(project_dir);

    // Rust: Cargo.toml
    if root.join("Cargo.toml").exists() {
        return Some(Toolchain {
            language: "rust".into(),
            test_framework: "cargo-test".into(),
            test_command: vec!["cargo".into(), "test".into()],
            lint_command: vec!["cargo".into(), "clippy".into(), "--".into(), "-D".into(), "warnings".into()],
            type_check_command: vec!["cargo".into(), "check".into()],
            audit_command: vec!["cargo".into(), "audit".into()],
            coverage_command: vec![],
        });
    }

    // Check for nested Rust
    if root.join("rust/Cargo.toml").exists() {
        return Some(Toolchain {
            language: "rust".into(),
            test_framework: "cargo-test".into(),
            test_command: vec!["cargo".into(), "test".into()],
            lint_command: vec!["cargo".into(), "clippy".into(), "--".into(), "-D".into(), "warnings".into()],
            type_check_command: vec!["cargo".into(), "check".into()],
            audit_command: vec!["cargo".into(), "audit".into()],
            coverage_command: vec![],
        });
    }

    // Node/TypeScript: package.json
    if root.join("package.json").exists() {
        let pkg = read_package_json(root);
        let (test_fw, test_cmd) = detect_js_test_framework(root, &pkg);
        let lint_cmd = detect_js_linter(root, &pkg);
        let has_ts = root.join("tsconfig.json").exists();

        return Some(Toolchain {
            language: if has_ts { "typescript" } else { "javascript" }.into(),
            test_framework: test_fw,
            test_command: test_cmd,
            lint_command: lint_cmd,
            type_check_command: if has_ts {
                vec!["npx".into(), "tsc".into(), "--noEmit".into()]
            } else {
                vec![]
            },
            audit_command: vec!["npm".into(), "audit".into(), "--json".into()],
            coverage_command: detect_js_coverage_command(root, &pkg),
        });
    }

    // Go: go.mod
    if root.join("go.mod").exists() {
        return Some(Toolchain {
            language: "go".into(),
            test_framework: "go-test".into(),
            test_command: vec!["go".into(), "test".into(), "./...".into()],
            lint_command: vec!["golangci-lint".into(), "run".into()],
            type_check_command: vec!["go".into(), "vet".into(), "./...".into()],
            audit_command: vec!["govulncheck".into(), "./...".into()],
            coverage_command: vec!["go".into(), "test".into(), "-coverprofile=coverage.out".into(), "./...".into()],
        });
    }

    // Python: pyproject.toml or setup.py
    if root.join("pyproject.toml").exists() || root.join("setup.py").exists() {
        return Some(Toolchain {
            language: "python".into(),
            test_framework: "pytest".into(),
            test_command: vec!["python3".into(), "-m".into(), "pytest".into()],
            lint_command: vec!["ruff".into(), "check".into(), ".".into()],
            type_check_command: vec!["mypy".into(), ".".into()],
            audit_command: vec!["pip-audit".into()],
            coverage_command: vec!["python3".into(), "-m".into(), "pytest".into(), "--cov".into(), "--cov-report=json".into()],
        });
    }

    None
}

/// Parse Jest/Vitest JSON reporter output into structured findings.
pub fn parse_test_json(output: &str) -> Vec<TestFinding> {
    let parsed: serde_json::Value = match serde_json::from_str(output) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut findings = Vec::new();

    // Jest format: { testResults: [{ assertionResults: [...] }] }
    if let Some(results) = parsed.get("testResults").and_then(|v| v.as_array()) {
        for suite in results {
            let file = suite.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
            if let Some(assertions) = suite.get("assertionResults").and_then(|v| v.as_array()) {
                for assertion in assertions {
                    let status = assertion.get("status").and_then(|v| v.as_str()).unwrap_or("");
                    if status == "failed" {
                        let title = assertion.get("fullName").and_then(|v| v.as_str()).unwrap_or("unknown test");
                        let messages: Vec<String> = assertion
                            .get("failureMessages")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|m| m.as_str().map(String::from)).collect())
                            .unwrap_or_default();
                        findings.push(TestFinding {
                            file: file.to_string(),
                            test_name: title.to_string(),
                            status: "failed".into(),
                            message: messages.join("\n").chars().take(500).collect(),
                        });
                    }
                }
            }
        }
    }

    // Vitest format: { testResults: [...] } (similar but slight differences)
    // Vitest with --reporter=json uses same format as Jest

    findings
}

/// Parse Istanbul/NYC coverage-summary.json.
pub fn parse_coverage_summary(json_str: &str) -> Option<CoverageSummary> {
    let parsed: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let total = parsed.get("total")?;

    Some(CoverageSummary {
        lines: extract_pct(total.get("lines")?),
        statements: extract_pct(total.get("statements")?),
        functions: extract_pct(total.get("functions")?),
        branches: extract_pct(total.get("branches")?),
    })
}

/// Parse per-file coverage from Istanbul JSON.
pub fn parse_coverage_per_file(json_str: &str) -> Vec<FileCoverage> {
    let parsed: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let obj = match parsed.as_object() {
        Some(o) => o,
        None => return Vec::new(),
    };

    obj.iter()
        .filter(|(k, _)| *k != "total")
        .map(|(file, data)| FileCoverage {
            file: file.clone(),
            lines: extract_pct_opt(data.get("lines")),
            statements: extract_pct_opt(data.get("statements")),
            functions: extract_pct_opt(data.get("functions")),
            branches: extract_pct_opt(data.get("branches")),
        })
        .collect()
}

/// Parse npm audit --json output.
pub fn parse_npm_audit(json_str: &str) -> Vec<AuditFinding> {
    let parsed: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut findings = Vec::new();

    // npm audit v2 format: { vulnerabilities: { "pkg-name": { severity, ... } } }
    if let Some(vulns) = parsed.get("vulnerabilities").and_then(|v| v.as_object()) {
        for (pkg, data) in vulns {
            let severity = data.get("severity").and_then(|v| v.as_str()).unwrap_or("unknown");
            let title = data.get("name").and_then(|v| v.as_str()).unwrap_or(pkg);
            let via = data.get("via").and_then(|v| {
                if let Some(arr) = v.as_array() {
                    arr.first().and_then(|item| {
                        item.get("title").and_then(|t| t.as_str()).map(String::from)
                    })
                } else {
                    None
                }
            }).unwrap_or_default();

            findings.push(AuditFinding {
                package: title.to_string(),
                severity: severity.to_string(),
                description: via,
            });
        }
    }

    findings
}

/// Check if tsconfig.json has strict mode enabled.
pub fn check_typescript_strict(project_dir: &str) -> Option<bool> {
    let tsconfig = Path::new(project_dir).join("tsconfig.json");
    let content = fs::read_to_string(tsconfig).ok()?;
    // Strip comments (tsconfig allows them)
    let stripped: String = content.lines()
        .map(|l| {
            if let Some(idx) = l.find("//") { &l[..idx] } else { l }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let parsed: serde_json::Value = serde_json::from_str(&stripped).ok()?;
    parsed
        .get("compilerOptions")
        .and_then(|co| co.get("strict"))
        .and_then(|v| v.as_bool())
}

// --- Types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFinding {
    pub file: String,
    pub test_name: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageSummary {
    pub lines: f64,
    pub statements: f64,
    pub functions: f64,
    pub branches: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub file: String,
    pub lines: f64,
    pub statements: f64,
    pub functions: f64,
    pub branches: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub package: String,
    pub severity: String,
    pub description: String,
}

// --- Helpers ---

fn extract_pct(v: &serde_json::Value) -> f64 {
    v.get("pct").and_then(|p| p.as_f64()).unwrap_or(0.0)
}

fn extract_pct_opt(v: Option<&serde_json::Value>) -> f64 {
    v.and_then(|v| v.get("pct")).and_then(|p| p.as_f64()).unwrap_or(0.0)
}

fn read_package_json(root: &Path) -> serde_json::Value {
    fs::read_to_string(root.join("package.json"))
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or(serde_json::json!({}))
}

fn detect_js_test_framework(root: &Path, pkg: &serde_json::Value) -> (String, Vec<String>) {
    let deps = pkg.get("devDependencies").or(pkg.get("dependencies"));

    // Check for vitest
    if deps.and_then(|d| d.get("vitest")).is_some() || root.join("vitest.config.ts").exists() || root.join("vitest.config.js").exists() {
        return ("vitest".into(), vec!["npx".into(), "vitest".into(), "run".into(), "--reporter=json".into()]);
    }

    // Check for jest
    if deps.and_then(|d| d.get("jest")).is_some() || root.join("jest.config.ts").exists() || root.join("jest.config.js").exists() {
        return ("jest".into(), vec!["npx".into(), "jest".into(), "--json".into()]);
    }

    // Fallback to npm test
    if pkg.get("scripts").and_then(|s| s.get("test")).is_some() {
        return ("npm-test".into(), vec!["npm".into(), "test".into()]);
    }

    ("none".into(), vec![])
}

fn detect_js_linter(root: &Path, pkg: &serde_json::Value) -> Vec<String> {
    let deps = pkg.get("devDependencies").or(pkg.get("dependencies"));

    // Biome
    if deps.and_then(|d| d.get("@biomejs/biome")).is_some() || root.join("biome.json").exists() {
        return vec!["npx".into(), "biome".into(), "check".into(), ".".into()];
    }

    // ESLint
    if deps.and_then(|d| d.get("eslint")).is_some() || root.join(".eslintrc.json").exists() || root.join("eslint.config.js").exists() {
        return vec!["npx".into(), "eslint".into(), ".".into()];
    }

    vec![]
}

fn detect_js_coverage_command(_root: &Path, pkg: &serde_json::Value) -> Vec<String> {
    let deps = pkg.get("devDependencies").or(pkg.get("dependencies"));

    if deps.and_then(|d| d.get("vitest")).is_some() {
        return vec!["npx".into(), "vitest".into(), "run".into(), "--coverage".into(), "--reporter=json".into()];
    }

    if deps.and_then(|d| d.get("jest")).is_some() {
        return vec!["npx".into(), "jest".into(), "--coverage".into(), "--coverageReporters=json-summary".into()];
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust_toolchain() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(tc.language, "rust");
        assert_eq!(tc.test_framework, "cargo-test");
        assert_eq!(tc.test_command, vec!["cargo", "test"]);
    }

    #[test]
    fn test_detect_typescript_toolchain() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"devDependencies":{"vitest":"^1.0"}}"#).unwrap();
        fs::write(dir.path().join("tsconfig.json"), r#"{"compilerOptions":{"strict":true}}"#).unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(tc.language, "typescript");
        assert_eq!(tc.test_framework, "vitest");
        assert!(tc.test_command.contains(&"vitest".to_string()));
        assert!(!tc.type_check_command.is_empty());
    }

    #[test]
    fn test_detect_jest() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"devDependencies":{"jest":"^29"}}"#).unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(tc.test_framework, "jest");
    }

    #[test]
    fn test_detect_eslint() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"devDependencies":{"eslint":"^8"}}"#).unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert!(tc.lint_command.contains(&"eslint".to_string()));
    }

    #[test]
    fn test_detect_biome() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.json"), r#"{"devDependencies":{"@biomejs/biome":"^1"}}"#).unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert!(tc.lint_command.contains(&"biome".to_string()));
    }

    #[test]
    fn test_detect_python() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("pyproject.toml"), "[project]\nname = \"test\"\n").unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(tc.language, "python");
        assert_eq!(tc.test_framework, "pytest");
    }

    #[test]
    fn test_detect_go() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("go.mod"), "module test\ngo 1.21\n").unwrap();
        let tc = detect_toolchain(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(tc.language, "go");
    }

    #[test]
    fn test_detect_nothing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(detect_toolchain(dir.path().to_str().unwrap()).is_none());
    }

    #[test]
    fn test_parse_jest_json() {
        let json = r#"{"testResults":[{"name":"test.ts","assertionResults":[{"fullName":"should work","status":"passed"},{"fullName":"should fail","status":"failed","failureMessages":["Expected true to be false"]}]}]}"#;
        let findings = parse_test_json(json);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].test_name, "should fail");
        assert!(findings[0].message.contains("Expected true"));
    }

    #[test]
    fn test_parse_coverage_summary() {
        let json = r#"{"total":{"lines":{"pct":85.5},"statements":{"pct":84.2},"functions":{"pct":90.0},"branches":{"pct":75.3}}}"#;
        let summary = parse_coverage_summary(json).unwrap();
        assert_eq!(summary.lines, 85.5);
        assert_eq!(summary.functions, 90.0);
    }

    #[test]
    fn test_parse_npm_audit() {
        let json = r#"{"vulnerabilities":{"lodash":{"name":"lodash","severity":"high","via":[{"title":"Prototype Pollution"}]}}}"#;
        let findings = parse_npm_audit(json);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].package, "lodash");
        assert_eq!(findings[0].severity, "high");
    }

    #[test]
    fn test_check_typescript_strict_true() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("tsconfig.json"), r#"{"compilerOptions":{"strict":true}}"#).unwrap();
        assert_eq!(check_typescript_strict(dir.path().to_str().unwrap()), Some(true));
    }

    #[test]
    fn test_check_typescript_strict_false() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("tsconfig.json"), r#"{"compilerOptions":{"strict":false}}"#).unwrap();
        assert_eq!(check_typescript_strict(dir.path().to_str().unwrap()), Some(false));
    }

    #[test]
    fn test_check_typescript_strict_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(check_typescript_strict(dir.path().to_str().unwrap()), None);
    }

    #[test]
    fn test_parse_empty_json() {
        assert!(parse_test_json("not json").is_empty());
        assert!(parse_coverage_summary("not json").is_none());
        assert!(parse_npm_audit("not json").is_empty());
    }
}
