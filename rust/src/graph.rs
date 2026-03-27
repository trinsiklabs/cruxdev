//! Dependency graph — build import graphs, compute impact sets.
//!
//! Language-agnostic: uses regex-based import detection for Python, Rust, TypeScript.
//! Not AST-based (would require per-language parsers), but sufficient for impact analysis.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A dependency graph for a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Edges: file → set of files it imports/depends on.
    pub edges: BTreeMap<String, BTreeSet<String>>,
    /// Reverse edges: file → set of files that depend on it.
    pub reverse_edges: BTreeMap<String, BTreeSet<String>>,
    /// Token estimate per file (lines × 50).
    pub token_estimates: HashMap<String, usize>,
}

impl DependencyGraph {
    /// Build dependency graph by scanning all source files.
    pub fn build(project_dir: &str) -> Self {
        let mut graph = Self::default();
        let source_extensions = ["py", "rs", "ts", "tsx", "js", "jsx"];

        for entry in walkdir::WalkDir::new(project_dir)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') && name != "node_modules" && name != "target" && name != "__pycache__"
            })
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !source_extensions.contains(&ext) {
                continue;
            }

            let rel_path = path
                .strip_prefix(project_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            if let Ok(content) = fs::read_to_string(path) {
                let line_count = content.lines().count();
                graph.token_estimates.insert(rel_path.clone(), line_count * 50);

                let imports = parse_imports(&content, ext);
                let mut deps = BTreeSet::new();
                for imp in imports {
                    if let Some(resolved) = resolve_import(&imp, project_dir, &rel_path) {
                        deps.insert(resolved);
                    }
                }
                graph.edges.insert(rel_path, deps);
            }
        }

        graph.build_reverse_edges();
        graph
    }

    fn build_reverse_edges(&mut self) {
        self.reverse_edges.clear();
        for (file, deps) in &self.edges {
            for dep in deps {
                self.reverse_edges
                    .entry(dep.clone())
                    .or_default()
                    .insert(file.clone());
            }
        }
    }

    /// Get all files transitively affected by changes to the given files.
    pub fn impact_set(&self, changed_files: &[String]) -> BTreeSet<String> {
        let mut visited = BTreeSet::new();
        let mut queue: VecDeque<String> = changed_files.iter().cloned().collect();

        while let Some(file) = queue.pop_front() {
            if !visited.insert(file.clone()) {
                continue;
            }
            if let Some(dependents) = self.reverse_edges.get(&file) {
                for dep in dependents {
                    if !visited.contains(dep) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        visited
    }

    /// Select files for audit context within a token budget.
    pub fn audit_context(
        &self,
        target_files: &[String],
        token_budget: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = target_files.to_vec();
        let mut used_tokens: usize = result
            .iter()
            .map(|f| self.token_estimates.get(f).copied().unwrap_or(500))
            .sum();

        // Add direct dependencies if budget allows
        for file in target_files {
            if let Some(deps) = self.edges.get(file) {
                for dep in deps {
                    let cost = self.token_estimates.get(dep).copied().unwrap_or(500);
                    if used_tokens + cost <= token_budget && !result.contains(dep) {
                        result.push(dep.clone());
                        used_tokens += cost;
                    }
                }
            }
        }

        result
    }

    /// Partition files into N non-overlapping scopes (round-robin).
    pub fn assign_scopes(&self, n_agents: usize) -> Vec<Vec<String>> {
        let mut scopes: Vec<Vec<String>> = (0..n_agents).map(|_| Vec::new()).collect();
        for (i, file) in self.edges.keys().enumerate() {
            scopes[i % n_agents].push(file.clone());
        }
        scopes
    }

    /// Number of source files tracked.
    pub fn file_count(&self) -> usize {
        self.edges.len()
    }
}

/// Parse import statements from source code.
fn parse_imports(content: &str, ext: &str) -> Vec<String> {
    let mut imports = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        match ext {
            "py" => {
                if let Some(rest) = trimmed.strip_prefix("import ") {
                    imports.push(rest.split_once(" as ").map_or(rest, |(m, _)| m).trim().to_string());
                } else if let Some(rest) = trimmed.strip_prefix("from ")
                    && let Some((module, _)) = rest.split_once(" import")
                {
                    imports.push(module.trim().to_string());
                }
            }
            "rs" => {
                if let Some(rest) = trimmed.strip_prefix("use ") {
                    let path = rest.trim_end_matches(';').split("::").next().unwrap_or("");
                    if !path.is_empty() && path != "std" && path != "super" && path != "self" && path != "crate" {
                        imports.push(path.to_string());
                    }
                }
                if trimmed.starts_with("mod ") && trimmed.ends_with(';') {
                    let name = trimmed.strip_prefix("mod ").unwrap().trim_end_matches(';').trim();
                    imports.push(name.to_string());
                }
            }
            "ts" | "tsx" | "js" | "jsx" => {
                if trimmed.contains("import ") && trimmed.contains("from ")
                    && let Some(from_idx) = trimmed.rfind("from ")
                {
                    let path = trimmed[from_idx + 5..]
                        .trim()
                        .trim_matches(|c| c == '\'' || c == '"' || c == ';');
                    imports.push(path.to_string());
                }
                if let Some(rest) = trimmed.strip_prefix("require(") {
                    let path = rest.trim_end_matches(')').trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"');
                    imports.push(path.to_string());
                }
            }
            _ => {}
        }
    }

    imports
}

/// Resolve an import path to a relative file path (best-effort).
fn resolve_import(import: &str, project_dir: &str, _source_file: &str) -> Option<String> {
    // Python: dot-separated module → path
    if import.contains('.') {
        let path = import.replace('.', "/");
        for candidate in [format!("{path}.py"), format!("{path}/__init__.py")] {
            if Path::new(project_dir).join(&candidate).exists() {
                return Some(candidate);
            }
        }
    }

    // Direct file reference
    for ext in ["py", "rs", "ts", "tsx", "js", "jsx"] {
        let candidate = format!("{import}.{ext}");
        if Path::new(project_dir).join(&candidate).exists() {
            return Some(candidate);
        }
    }

    // Relative TS/JS imports
    if import.starts_with("./") || import.starts_with("../") {
        for ext in ["ts", "tsx", "js", "jsx", ""] {
            let candidate = if ext.is_empty() {
                import.to_string()
            } else {
                format!("{import}.{ext}")
            };
            if Path::new(project_dir).join(&candidate).exists() {
                return Some(candidate);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_imports_python() {
        let code = "import os\nfrom pathlib import Path\nimport json as j\n";
        let imports = parse_imports(code, "py");
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"pathlib".to_string()));
        assert!(imports.contains(&"json".to_string()));
    }

    #[test]
    fn test_parse_imports_rust() {
        let code = "use serde::Serialize;\nuse std::collections::HashMap;\nmod router;\n";
        let imports = parse_imports(code, "rs");
        assert!(imports.contains(&"serde".to_string()));
        assert!(imports.contains(&"router".to_string()));
        // std is excluded
        assert!(!imports.contains(&"std".to_string()));
    }

    #[test]
    fn test_parse_imports_typescript() {
        let code = "import { foo } from './bar';\nimport React from 'react';\n";
        let imports = parse_imports(code, "ts");
        assert!(imports.contains(&"./bar".to_string()));
        assert!(imports.contains(&"react".to_string()));
    }

    #[test]
    fn test_impact_set() {
        let mut graph = DependencyGraph::default();
        graph.edges.insert("a.py".into(), ["b.py".into()].into());
        graph.edges.insert("b.py".into(), ["c.py".into()].into());
        graph.edges.insert("c.py".into(), BTreeSet::new());
        graph.build_reverse_edges();

        let impact = graph.impact_set(&["c.py".into()]);
        assert!(impact.contains("c.py"));
        assert!(impact.contains("b.py"));
        assert!(impact.contains("a.py"));
    }

    #[test]
    fn test_impact_set_isolated() {
        let mut graph = DependencyGraph::default();
        graph.edges.insert("a.py".into(), BTreeSet::new());
        graph.edges.insert("b.py".into(), BTreeSet::new());
        graph.build_reverse_edges();

        let impact = graph.impact_set(&["a.py".into()]);
        assert_eq!(impact.len(), 1);
        assert!(impact.contains("a.py"));
    }

    #[test]
    fn test_audit_context_within_budget() {
        let mut graph = DependencyGraph::default();
        graph.edges.insert("a.py".into(), ["b.py".into()].into());
        graph.edges.insert("b.py".into(), BTreeSet::new());
        graph.token_estimates.insert("a.py".into(), 500);
        graph.token_estimates.insert("b.py".into(), 500);

        let context = graph.audit_context(&["a.py".into()], 2000);
        assert!(context.contains(&"a.py".to_string()));
        assert!(context.contains(&"b.py".to_string()));
    }

    #[test]
    fn test_audit_context_over_budget() {
        let mut graph = DependencyGraph::default();
        graph.edges.insert("a.py".into(), ["b.py".into()].into());
        graph.token_estimates.insert("a.py".into(), 900);
        graph.token_estimates.insert("b.py".into(), 200);

        let context = graph.audit_context(&["a.py".into()], 1000);
        assert!(context.contains(&"a.py".to_string()));
        // b.py should fit (900 + 200 = 1100 > 1000) — won't fit
        assert!(!context.contains(&"b.py".to_string()));
    }

    #[test]
    fn test_assign_scopes() {
        let mut graph = DependencyGraph::default();
        for i in 0..6 {
            graph.edges.insert(format!("file{i}.py"), BTreeSet::new());
        }
        let scopes = graph.assign_scopes(3);
        assert_eq!(scopes.len(), 3);
        assert_eq!(scopes[0].len(), 2);
        assert_eq!(scopes[1].len(), 2);
        assert_eq!(scopes[2].len(), 2);
    }

    #[test]
    fn test_build_on_tempdir() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        let py = src.join("main.py");
        std::fs::write(&py, "import os\nfrom helper import util\n").unwrap();
        let helper = src.join("helper.py");
        std::fs::write(&helper, "def util(): pass\n").unwrap();

        let graph = DependencyGraph::build(src.to_str().unwrap());
        assert_eq!(graph.file_count(), 2);
        assert!(graph.edges.contains_key("main.py"));
    }
}
