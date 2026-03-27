//! Technical debt auto-detection — complexity, anti-patterns, hardcoded paths.

use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// A single technical debt finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtFinding {
    pub file: String,
    pub line: usize,
    pub category: String,
    pub severity: String,
    pub description: String,
    pub suggestion: String,
}

/// Directories to skip when scanning.
const SKIP_DIRS: &[&str] = &[
    "__pycache__",
    ".git",
    "node_modules",
    ".venv",
    "venv",
    ".tox",
    ".mypy_cache",
    ".pytest_cache",
    "target",
];

/// Simple cyclomatic complexity estimation from source text.
///
/// Counts decision-point keywords: if, else if, for, while, match, catch/except,
/// assert, and boolean operators (&&, ||).
pub fn estimate_complexity(source: &str) -> usize {
    let mut complexity: usize = 1;
    for line in source.lines() {
        let trimmed = line.trim();
        // Skip comments and empty lines
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("if ")
            || trimmed.starts_with("} else if ")
            || trimmed.starts_with("elif ")
        {
            complexity += 1;
        }
        if trimmed.starts_with("for ") || trimmed.starts_with("while ") {
            complexity += 1;
        }
        if trimmed.starts_with("match ") {
            complexity += 1;
        }
        if trimmed.contains("except") || trimmed.contains("catch") {
            complexity += 1;
        }
        // Count boolean operators as decision points
        complexity += trimmed.matches("&&").count();
        complexity += trimmed.matches("||").count();
    }
    complexity
}

/// Find anti-patterns in source code: bare except, hardcoded temp paths.
pub fn find_anti_patterns(filepath: &str, source: &str) -> Vec<DebtFinding> {
    let mut findings = Vec::new();
    let bare_except_re = Regex::new(r"^\s*except\s*:").unwrap();
    let tmp_path_re = Regex::new(r#"["'](/tmp/|/var/tmp/)[^"']*["']"#).unwrap();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;

        if bare_except_re.is_match(line) {
            findings.push(DebtFinding {
                file: filepath.to_string(),
                line: line_num,
                category: "anti_pattern".into(),
                severity: "medium".into(),
                description: format!(
                    "Bare 'except:' on line {} catches all exceptions",
                    line_num
                ),
                suggestion: "Catch specific exceptions (e.g., except ValueError:)".into(),
            });
        }

        if let Some(m) = tmp_path_re.find(line) {
            let path_str = &m.as_str()[1..m.as_str().len() - 1]; // strip quotes
            let display = if path_str.len() > 50 {
                &path_str[..50]
            } else {
                path_str
            };
            findings.push(DebtFinding {
                file: filepath.to_string(),
                line: line_num,
                category: "anti_pattern".into(),
                severity: "low".into(),
                description: format!("Hardcoded temp path '{}' on line {}", display, line_num),
                suggestion: "Use tempfile APIs instead of hardcoded paths".into(),
            });
        }
    }
    findings
}

/// Find functions with complexity above threshold (text-based scan for Python/Rust).
pub fn find_complex_functions(
    filepath: &str,
    source: &str,
    threshold: usize,
) -> Vec<DebtFinding> {
    let mut findings = Vec::new();
    let func_re = Regex::new(r"^\s*(async\s+)?def\s+(\w+)|^\s*(pub\s+)?(async\s+)?fn\s+(\w+)")
        .unwrap();

    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if let Some(caps) = func_re.captures(lines[i]) {
            let func_name = caps
                .get(2)
                .or(caps.get(5))
                .map(|m| m.as_str())
                .unwrap_or("unknown");
            let start = i;
            // Collect function body (simple heuristic: until next function or dedent)
            let mut end = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();
            while end < lines.len() {
                let line = lines[end];
                if !line.trim().is_empty() {
                    let cur_indent = line.len() - line.trim_start().len();
                    if cur_indent <= indent && func_re.is_match(line) {
                        break;
                    }
                }
                end += 1;
            }
            let body = lines[start..end].join("\n");
            let complexity = estimate_complexity(&body);
            if complexity > threshold {
                let severity = if complexity > 20 { "high" } else { "medium" };
                findings.push(DebtFinding {
                    file: filepath.to_string(),
                    line: start + 1,
                    category: "complexity".into(),
                    severity: severity.into(),
                    description: format!(
                        "Function '{}' has complexity {} (threshold: {})",
                        func_name, complexity, threshold
                    ),
                    suggestion: format!("Refactor '{}' into smaller functions", func_name),
                });
            }
            i = end;
        } else {
            i += 1;
        }
    }
    findings
}

/// Scan all Python/Rust files in a project directory for technical debt.
pub fn scan_project(project_dir: &str, complexity_threshold: usize) -> Vec<DebtFinding> {
    let mut findings = Vec::new();
    for entry in WalkDir::new(project_dir)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !SKIP_DIRS.contains(&name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "py" && ext != "rs" {
            continue;
        }
        let filepath = path.to_string_lossy().to_string();
        if let Ok(source) = std::fs::read_to_string(path) {
            findings.extend(find_complex_functions(&filepath, &source, complexity_threshold));
            findings.extend(find_anti_patterns(&filepath, &source));
        }
    }
    // Sort: high severity first
    findings.sort_by(|a, b| {
        let sev = |s: &str| match s {
            "high" => 0,
            "medium" => 1,
            _ => 2,
        };
        sev(&a.severity).cmp(&sev(&b.severity))
    });
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_complexity_simple() {
        let src = "fn main() {\n    println!(\"hello\");\n}\n";
        assert_eq!(estimate_complexity(src), 1);
    }

    #[test]
    fn test_estimate_complexity_branches() {
        let src = "fn check(x: i32) {\n    if x > 0 {\n    } else if x < 0 {\n    }\n    for i in 0..10 {\n    }\n}\n";
        // base 1 + if 1 + else if 1 + for 1 = 4
        assert_eq!(estimate_complexity(src), 4);
    }

    #[test]
    fn test_find_anti_patterns_bare_except() {
        let src = "try:\n    x = 1\nexcept:\n    pass\n";
        let findings = find_anti_patterns("test.py", src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].category, "anti_pattern");
        assert_eq!(findings[0].severity, "medium");
    }

    #[test]
    fn test_find_anti_patterns_hardcoded_tmp() {
        let src = "path = \"/tmp/myfile.txt\"\n";
        let findings = find_anti_patterns("test.py", src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, "low");
    }

    #[test]
    fn test_find_anti_patterns_clean() {
        let src = "x = 42\ny = x + 1\n";
        let findings = find_anti_patterns("test.py", src);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_find_complex_functions() {
        let src = "def simple():\n    return 1\n\ndef complex_fn():\n    if a:\n        pass\n    if b:\n        pass\n    if c:\n        pass\n    for x in y:\n        pass\n    while z:\n        pass\n    if d:\n        pass\n    if e:\n        pass\n    if f:\n        pass\n    if g:\n        pass\n    if h:\n        pass\n    if i:\n        pass\n";
        let findings = find_complex_functions("test.py", src, 5);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].description.contains("complex_fn"));
    }
}
