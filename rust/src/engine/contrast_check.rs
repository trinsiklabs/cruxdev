//! Automated color contrast checker for Tailwind CSS classes.
//!
//! Scans HTML/HEEx/JSX files for known-bad Tailwind color combinations
//! that fail WCAG 2.1 AA contrast requirements.

use std::path::Path;

#[derive(Debug, Clone)]
pub struct ContrastViolation {
    pub file: String,
    pub line: usize,
    pub text_class: String,
    pub bg_class: String,
    pub estimated_ratio: f64,
    pub required_ratio: f64,
    pub severity: String,
    pub fix: String,
}

/// Known-bad text/bg combinations in Tailwind.
/// Each entry: (text_class_pattern, bg_context, estimated_ratio, fix)
const BAD_COMBOS: &[(&str, &str, f64, &str)] = &[
    // Gray scale violations at small text sizes
    ("text-gray-400", "", 3.0, "Use text-gray-500 minimum (4.6:1)"),
    ("text-gray-300", "", 2.2, "Use text-gray-500 minimum"),
    ("text-slate-400", "", 3.0, "Use text-slate-500 minimum"),
    ("text-zinc-400", "", 3.0, "Use text-zinc-500 minimum"),
    ("text-neutral-400", "", 3.0, "Use text-neutral-500 minimum"),

    // Red scale on red backgrounds
    ("text-red-100", "bg-red", 2.2, "Use text-white or text-red-50"),
    ("text-red-200", "bg-red", 2.8, "Use text-white or text-red-50"),

    // Light text on light backgrounds
    ("text-blue-100", "bg-blue", 2.0, "Use text-white or text-blue-50"),
    ("text-green-100", "bg-green", 2.0, "Use text-white or text-green-50"),
    ("text-yellow-100", "bg-yellow", 1.8, "Use text-yellow-900 or text-black"),
    ("text-yellow-200", "bg-yellow", 2.2, "Use text-yellow-900"),

    // White text on light colored backgrounds
    ("text-white", "bg-yellow", 1.5, "Use text-yellow-900"),
    ("text-white", "bg-lime", 1.8, "Use text-lime-900"),
    ("text-white", "bg-cyan-200", 1.6, "Use text-cyan-900"),
    ("text-white", "bg-sky-200", 1.8, "Use text-sky-900"),
];

/// Small text classes that require stricter contrast (4.5:1 instead of 3:1 for large text).
const SMALL_TEXT_CLASSES: &[&str] = &["text-xs", "text-sm", "text-[10px]", "text-[11px]", "text-[12px]"];

/// Borderline text colors that pass for large text but fail for small text.
const BORDERLINE_AT_SMALL: &[(&str, f64, &str)] = &[
    ("text-gray-500", 4.6, "Use text-gray-600 for small text"),
    ("text-slate-500", 4.6, "Use text-slate-600 for small text"),
    ("text-zinc-500", 4.6, "Use text-zinc-600 for small text"),
];

/// Scan a project directory for contrast violations.
pub fn scan_project(project_dir: &str) -> Vec<ContrastViolation> {
    let mut violations = Vec::new();
    let extensions = ["html", "astro", "jsx", "tsx", "vue", "svelte", "heex", "ex"];

    walk_dir(Path::new(project_dir), &extensions, &mut |file_path, content| {
        violations.extend(scan_file(file_path, content));
    });

    violations
}

fn walk_dir(dir: &Path, extensions: &[&str], callback: &mut dyn FnMut(&str, &str)) {
    let entries = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden dirs, node_modules, deps, _build, target
        if name.starts_with('.') || name == "node_modules" || name == "deps"
            || name == "_build" || name == "target" || name == "__pycache__"
        {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, extensions, callback);
        } else if let Some(content) = path.extension()
            .and_then(|e| e.to_str())
            .filter(|ext| extensions.contains(ext))
            .and_then(|_| std::fs::read_to_string(&path).ok())
        {
            callback(&path.to_string_lossy(), &content);
        }
    }
}

fn scan_file(file_path: &str, content: &str) -> Vec<ContrastViolation> {
    let mut violations = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        // Check known-bad combinations
        for (text_class, bg_context, ratio, fix) in BAD_COMBOS {
            if line.contains(text_class) {
                // If bg_context is specified, check if it's also on this line or nearby
                if bg_context.is_empty() || line.contains(bg_context) {
                    violations.push(ContrastViolation {
                        file: file_path.to_string(),
                        line: line_num + 1,
                        text_class: text_class.to_string(),
                        bg_class: if bg_context.is_empty() { "default bg".into() } else { bg_context.to_string() },
                        estimated_ratio: *ratio,
                        required_ratio: 4.5,
                        severity: if *ratio < 3.0 { "HIGH".into() } else { "MEDIUM".into() },
                        fix: fix.to_string(),
                    });
                }
            }
        }

        // Check borderline colors at small text sizes
        let has_small_text = SMALL_TEXT_CLASSES.iter().any(|cls| line.contains(cls));
        if has_small_text {
            for (text_class, ratio, fix) in BORDERLINE_AT_SMALL {
                if line.contains(text_class) {
                    violations.push(ContrastViolation {
                        file: file_path.to_string(),
                        line: line_num + 1,
                        text_class: text_class.to_string(),
                        bg_class: "default bg + small text".into(),
                        estimated_ratio: *ratio,
                        required_ratio: 4.5,
                        severity: "MEDIUM".into(),
                        fix: fix.to_string(),
                    });
                }
            }
        }
    }

    violations
}

/// Check if a project has any contrast violations.
pub fn project_has_violations(project_dir: &str) -> bool {
    !scan_project(project_dir).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_gray_400() {
        let violations = scan_file("test.html", "<span class=\"text-xs text-gray-400\">Label</span>");
        assert!(!violations.is_empty());
        assert!(violations.iter().any(|v| v.text_class == "text-gray-400"));
    }

    #[test]
    fn test_detects_red_100_on_red_bg() {
        let violations = scan_file("test.html", "<div class=\"bg-red-700\"><p class=\"text-red-100\">Subtitle</p></div>");
        assert!(violations.iter().any(|v| v.text_class == "text-red-100"));
    }

    #[test]
    fn test_detects_borderline_at_small_size() {
        let violations = scan_file("test.html", "<span class=\"text-sm text-gray-500\">Info</span>");
        assert!(violations.iter().any(|v| v.text_class == "text-gray-500" && v.fix.contains("small text")));
    }

    #[test]
    fn test_passes_safe_colors() {
        let violations = scan_file("test.html", "<span class=\"text-gray-700\">Safe text</span>");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_passes_explicit_white_on_dark() {
        let violations = scan_file("test.html", "<span class=\"text-white bg-gray-900\">High contrast</span>");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_scan_project_walks_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("page.html"), "<p class=\"text-gray-400\">Bad</p>").unwrap();
        std::fs::write(src.join("good.html"), "<p class=\"text-gray-800\">Good</p>").unwrap();

        let violations = scan_project(dir.path().to_str().unwrap());
        assert_eq!(violations.len(), 1);
        assert!(violations[0].file.contains("page.html"));
    }
}
