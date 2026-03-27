//! Document inventory — scan and catalog all project materials.
//!
//! Produces a structured inventory of documents, code, assets, and configs
//! with format detection and quality classification.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// File format type.
pub type Format = String; // "markdown", "code", "config", "data", "image", "pdf", "other"

/// Quality classification.
pub type Quality = String; // "usable", "reference-only", "extract-info", "obsolete"

fn extension_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert(".md", "markdown");
    m.insert(".mdx", "markdown");
    m.insert(".txt", "markdown");
    m.insert(".py", "code");
    m.insert(".js", "code");
    m.insert(".ts", "code");
    m.insert(".tsx", "code");
    m.insert(".jsx", "code");
    m.insert(".go", "code");
    m.insert(".rs", "code");
    m.insert(".rb", "code");
    m.insert(".ex", "code");
    m.insert(".exs", "code");
    m.insert(".sh", "code");
    m.insert(".json", "config");
    m.insert(".yaml", "config");
    m.insert(".yml", "config");
    m.insert(".toml", "config");
    m.insert(".ini", "config");
    m.insert(".cfg", "config");
    m.insert(".env", "config");
    m.insert(".csv", "data");
    m.insert(".tsv", "data");
    m.insert(".sql", "data");
    m.insert(".png", "image");
    m.insert(".jpg", "image");
    m.insert(".jpeg", "image");
    m.insert(".svg", "image");
    m.insert(".gif", "image");
    m.insert(".webp", "image");
    m.insert(".pdf", "pdf");
    m
}

/// A single item in the project inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub path: String,
    pub format: Format,
    pub size_bytes: u64,
    pub last_modified: f64,
    pub quality: Quality,
    pub notes: String,
}

/// Complete project inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub project_dir: String,
    pub items: Vec<InventoryItem>,
}

impl Inventory {
    pub fn by_format(&self) -> HashMap<String, Vec<&InventoryItem>> {
        let mut result: HashMap<String, Vec<&InventoryItem>> = HashMap::new();
        for item in &self.items {
            result.entry(item.format.clone()).or_default().push(item);
        }
        result
    }

    pub fn total_size(&self) -> u64 {
        self.items.iter().map(|item| item.size_bytes).sum()
    }

    /// Generate intake-inventory.md content.
    pub fn to_markdown(&self) -> String {
        let mut lines = vec![
            "# Project Inventory".to_string(),
            String::new(),
            format!("**Total items:** {}", self.items.len()),
            format!("**Total size:** {} bytes", self.total_size()),
            String::new(),
        ];

        let by_fmt = self.by_format();
        let mut sorted_formats: Vec<&String> = by_fmt.keys().collect();
        sorted_formats.sort();

        for fmt in sorted_formats {
            let items = &by_fmt[fmt];
            let title = format!("{}{}", &fmt[..1].to_uppercase(), &fmt[1..]);
            lines.push(format!("## {title} ({} items)", items.len()));
            lines.push(String::new());
            let mut sorted_items: Vec<&&InventoryItem> = items.iter().collect();
            sorted_items.sort_by_key(|i| &i.path);
            for item in sorted_items {
                lines.push(format!(
                    "- `{}` ({}b) [{}]",
                    item.path, item.size_bytes, item.quality
                ));
            }
            lines.push(String::new());
        }

        lines.join("\n")
    }
}

/// Detect file format from extension.
pub fn detect_format(path: &str) -> String {
    let ext_map = extension_map();
    let path_lower = path.to_lowercase();

    for (ext, fmt) in &ext_map {
        if path_lower.ends_with(ext) {
            return fmt.to_string();
        }
    }
    "other".to_string()
}

/// Scan a project directory and produce a complete inventory.
pub fn inventory_project(project_dir: &str, skip_hidden: bool, skip_generated: bool) -> Inventory {
    let skip_dirs: Vec<&str> = vec![
        "node_modules",
        "__pycache__",
        ".git",
        "dist",
        "build",
        "venv",
        ".venv",
    ];

    let mut inventory = Inventory {
        project_dir: project_dir.to_string(),
        items: Vec::new(),
    };

    let base = Path::new(project_dir);
    if !base.is_dir() {
        return inventory;
    }

    fn walk(
        base: &Path,
        dir: &Path,
        items: &mut Vec<InventoryItem>,
        skip_hidden: bool,
        skip_generated: bool,
        skip_dirs: &[&str],
    ) {
        let read_dir = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return,
        };

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            if skip_hidden && name.starts_with('.') {
                continue;
            }

            let path = entry.path();

            if path.is_dir() {
                if skip_generated && skip_dirs.contains(&name.as_str()) {
                    continue;
                }
                walk(base, &path, items, skip_hidden, skip_generated, skip_dirs);
            } else {
                let rel_path = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();

                let (size, mtime) = match fs::metadata(&path) {
                    Ok(meta) => {
                        let size = meta.len();
                        let mtime = meta
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs_f64())
                            .unwrap_or(0.0);
                        (size, mtime)
                    }
                    Err(_) => (0, 0.0),
                };

                items.push(InventoryItem {
                    format: detect_format(&name),
                    path: rel_path,
                    size_bytes: size,
                    last_modified: mtime,
                    quality: "usable".to_string(),
                    notes: String::new(),
                });
            }
        }
    }

    walk(
        base,
        base,
        &mut inventory.items,
        skip_hidden,
        skip_generated,
        &skip_dirs,
    );

    inventory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("README.md"), "markdown");
        assert_eq!(detect_format("main.py"), "code");
        assert_eq!(detect_format("config.json"), "config");
        assert_eq!(detect_format("photo.png"), "image");
        assert_eq!(detect_format("unknown.xyz"), "other");
        assert_eq!(detect_format("data.csv"), "data");
        assert_eq!(detect_format("doc.pdf"), "pdf");
    }

    #[test]
    fn test_inventory_project_basic() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path();
        fs::write(d.join("README.md"), "# Hello").unwrap();
        fs::write(d.join("main.py"), "print('hello')").unwrap();
        fs::create_dir_all(d.join("src")).unwrap();
        fs::write(d.join("src").join("lib.rs"), "fn main() {}").unwrap();

        let inv = inventory_project(d.to_str().unwrap(), true, true);
        assert_eq!(inv.items.len(), 3);
        assert!(inv.total_size() > 0);
    }

    #[test]
    fn test_inventory_project_skips_hidden() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path();
        fs::write(d.join("visible.md"), "content").unwrap();
        fs::write(d.join(".hidden"), "secret").unwrap();

        let inv = inventory_project(d.to_str().unwrap(), true, true);
        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].path, "visible.md");
    }

    #[test]
    fn test_inventory_project_nonexistent_dir() {
        let inv = inventory_project("/nonexistent/path", true, true);
        assert!(inv.items.is_empty());
    }

    #[test]
    fn test_inventory_to_markdown() {
        let inv = Inventory {
            project_dir: "/test".to_string(),
            items: vec![
                InventoryItem {
                    path: "README.md".to_string(),
                    format: "markdown".to_string(),
                    size_bytes: 100,
                    last_modified: 0.0,
                    quality: "usable".to_string(),
                    notes: String::new(),
                },
                InventoryItem {
                    path: "main.py".to_string(),
                    format: "code".to_string(),
                    size_bytes: 200,
                    last_modified: 0.0,
                    quality: "usable".to_string(),
                    notes: String::new(),
                },
            ],
        };
        let md = inv.to_markdown();
        assert!(md.contains("# Project Inventory"));
        assert!(md.contains("**Total items:** 2"));
        assert!(md.contains("**Total size:** 300 bytes"));
        assert!(md.contains("## Markdown"));
        assert!(md.contains("## Code"));
    }

    #[test]
    fn test_inventory_by_format() {
        let inv = Inventory {
            project_dir: "/test".to_string(),
            items: vec![
                InventoryItem {
                    path: "a.md".to_string(),
                    format: "markdown".to_string(),
                    size_bytes: 10,
                    last_modified: 0.0,
                    quality: "usable".to_string(),
                    notes: String::new(),
                },
                InventoryItem {
                    path: "b.md".to_string(),
                    format: "markdown".to_string(),
                    size_bytes: 20,
                    last_modified: 0.0,
                    quality: "usable".to_string(),
                    notes: String::new(),
                },
            ],
        };
        let by_fmt = inv.by_format();
        assert_eq!(by_fmt.get("markdown").unwrap().len(), 2);
    }
}
