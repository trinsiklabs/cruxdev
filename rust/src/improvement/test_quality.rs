//! Test quality analysis — weak assertions, no-assertion tests, duplicate names.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A test quality finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestQualityFinding {
    pub file: String,
    pub test_name: String,
    pub line: usize,
    pub category: String,
    pub description: String,
}

/// Known weak assertion methods and their suggestions.
pub const WEAK_ASSERTIONS: &[(&str, &str)] = &[
    (
        "assertTrue",
        "Consider asserting a specific value instead of True",
    ),
    (
        "assertIsNotNone",
        "Consider asserting the specific expected value",
    ),
    (
        "assertFalse",
        "Consider asserting a specific condition",
    ),
];

/// Find weak assertions in test source code.
pub fn find_weak_assertions(filepath: &str, source: &str) -> Vec<TestQualityFinding> {
    let mut findings = Vec::new();
    let test_fn_re = Regex::new(r"^\s*(async\s+)?def\s+(test_\w+)").unwrap();

    let lines: Vec<&str> = source.lines().collect();
    let mut current_test: Option<(String, usize)> = None;

    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;

        if let Some(caps) = test_fn_re.captures(line) {
            current_test = Some((caps[2].to_string(), line_num));
        }

        if let Some((ref test_name, _)) = current_test {
            for &(method, suggestion) in WEAK_ASSERTIONS {
                if line.contains(method) {
                    findings.push(TestQualityFinding {
                        file: filepath.to_string(),
                        test_name: test_name.clone(),
                        line: line_num,
                        category: "weak_assertion".into(),
                        description: format!(
                            "{}() in {}: {}",
                            method, test_name, suggestion
                        ),
                    });
                }
            }
        }
    }
    findings
}

/// Find test functions that contain no assert statements.
pub fn find_tests_without_assertions(filepath: &str, source: &str) -> Vec<TestQualityFinding> {
    let mut findings = Vec::new();
    let test_fn_re = Regex::new(r"^\s*(async\s+)?def\s+(test_\w+)").unwrap();

    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if let Some(caps) = test_fn_re.captures(lines[i]) {
            let test_name = caps[2].to_string();
            let start_line = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();

            // Collect body
            let mut end = i + 1;
            while end < lines.len() {
                let line = lines[end];
                if !line.trim().is_empty() {
                    let cur_indent = line.len() - line.trim_start().len();
                    if cur_indent <= indent && test_fn_re.is_match(line) {
                        break;
                    }
                }
                end += 1;
            }

            let body = lines[i..end].join("\n");
            let has_assert = body.contains("assert")
                || body.contains(".raises(")
                || body.contains("assert_eq!")
                || body.contains("assert_ne!");

            if !has_assert {
                findings.push(TestQualityFinding {
                    file: filepath.to_string(),
                    test_name: test_name.clone(),
                    line: start_line,
                    category: "no_assertion".into(),
                    description: format!("Test '{}' has no assertions", test_name),
                });
            }
            i = end;
        } else {
            i += 1;
        }
    }
    findings
}

/// Find duplicate test function names across multiple files.
pub fn find_duplicate_test_names(
    file_tests: &[(String, Vec<(String, usize)>)],
) -> Vec<TestQualityFinding> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut findings = Vec::new();

    for (filepath, tests) in file_tests {
        for (test_name, line) in tests {
            if let Some(first_file) = seen.get(test_name) {
                if first_file != filepath {
                    findings.push(TestQualityFinding {
                        file: filepath.clone(),
                        test_name: test_name.clone(),
                        line: *line,
                        category: "duplicate_name".into(),
                        description: format!(
                            "Test '{}' also exists in {}",
                            test_name, first_file
                        ),
                    });
                }
            } else {
                seen.insert(test_name.clone(), filepath.clone());
            }
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_weak_assertions() {
        let src = "def test_foo():\n    self.assertTrue(result)\n";
        let findings = find_weak_assertions("test_x.py", src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].category, "weak_assertion");
    }

    #[test]
    fn test_find_tests_without_assertions() {
        let src = "def test_noop():\n    x = 1\n    y = 2\n";
        let findings = find_tests_without_assertions("test_x.py", src);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].category, "no_assertion");
    }

    #[test]
    fn test_find_tests_with_assertions() {
        let src = "def test_good():\n    assert x == 1\n";
        let findings = find_tests_without_assertions("test_x.py", src);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_find_duplicate_test_names() {
        let file_tests = vec![
            (
                "test_a.py".to_string(),
                vec![("test_foo".to_string(), 1)],
            ),
            (
                "test_b.py".to_string(),
                vec![("test_foo".to_string(), 5)],
            ),
        ];
        let findings = find_duplicate_test_names(&file_tests);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].category, "duplicate_name");
    }
}
