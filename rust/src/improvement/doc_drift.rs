//! Documentation-code drift detection — stale file paths and function references.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A documentation drift finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftFinding {
    pub doc_file: String,
    pub line: usize,
    pub category: String,
    pub description: String,
}

/// Extract code references from documentation content.
///
/// Returns `(line_number, reference)` tuples for file paths and function names.
pub fn find_code_references(doc_content: &str) -> Vec<(usize, String)> {
    let path_re = Regex::new(r"`([a-zA-Z0-9_/.\-]+\.py)`").unwrap();
    let func_re = Regex::new(r"`([a-zA-Z_][a-zA-Z0-9_]*)\(\)`").unwrap();

    let mut refs = Vec::new();
    for (i, line) in doc_content.lines().enumerate() {
        let line_num = i + 1;
        for cap in path_re.captures_iter(line) {
            refs.push((line_num, cap[1].to_string()));
        }
        for cap in func_re.captures_iter(line) {
            refs.push((line_num, format!("{}()", &cap[1])));
        }
    }
    refs
}

/// Check if file path references in a doc still exist.
pub fn check_path_references(
    doc_file: &str,
    doc_content: &str,
    project_dir: &str,
) -> Vec<DriftFinding> {
    let refs = find_code_references(doc_content);
    let mut findings = Vec::new();

    for (line_num, reference) in refs {
        if reference.ends_with("()") {
            continue;
        }
        let full_path = Path::new(project_dir).join(&reference);
        if !full_path.exists() {
            findings.push(DriftFinding {
                doc_file: doc_file.to_string(),
                line: line_num,
                category: "path".into(),
                description: format!("Referenced path `{}` does not exist", reference),
            });
        }
    }
    findings
}

/// Check if function references in a doc exist in a known function set.
pub fn check_function_references(
    doc_file: &str,
    doc_content: &str,
    known_functions: &std::collections::HashSet<String>,
) -> Vec<DriftFinding> {
    let refs = find_code_references(doc_content);
    let mut findings = Vec::new();

    for (line_num, reference) in refs {
        if !reference.ends_with("()") {
            continue;
        }
        let func_name = &reference[..reference.len() - 2];
        if !known_functions.contains(func_name) {
            findings.push(DriftFinding {
                doc_file: doc_file.to_string(),
                line: line_num,
                category: "signature".into(),
                description: format!(
                    "Referenced function `{}()` not found in codebase",
                    func_name
                ),
            });
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_code_references_paths() {
        let content = "See `src/engine/state.py` for details.";
        let refs = find_code_references(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].1, "src/engine/state.py");
    }

    #[test]
    fn test_find_code_references_functions() {
        let content = "Call `validate_plan()` to check.";
        let refs = find_code_references(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].1, "validate_plan()");
    }

    #[test]
    fn test_check_function_references_missing() {
        let content = "Use `nonexistent_func()` here.";
        let known = HashSet::new();
        let findings = check_function_references("doc.md", content, &known);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].category, "signature");
    }

    #[test]
    fn test_check_function_references_found() {
        let content = "Use `my_func()` here.";
        let mut known = HashSet::new();
        known.insert("my_func".to_string());
        let findings = check_function_references("doc.md", content, &known);
        assert!(findings.is_empty());
    }
}
