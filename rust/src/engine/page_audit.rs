//! Page-level convergence — audit every page, not just the project.
//!
//! Inventories routes, classifies page types, generates per-page audit tasks.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A discovered page/route in a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRoute {
    pub path: String,
    pub route: String,
    pub page_type: PageType,
    pub applicable_dimensions: Vec<String>,
}

/// Classification of a page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageType {
    Form,
    Dashboard,
    List,
    Detail,
    Auth,
    Blog,
    Static,
    Api,
    Index,
    Unknown,
}

/// Per-page audit task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAuditTask {
    pub route: String,
    pub file_path: String,
    pub page_type: PageType,
    pub dimensions: Vec<String>,
    pub description: String,
}

/// Inventory all pages/routes in a project.
pub fn inventory_routes(project_dir: &str) -> Vec<PageRoute> {
    let root = Path::new(project_dir);
    let mut pages = Vec::new();

    // Astro: scan src/pages/
    let astro_pages = root.join("src/pages");
    if astro_pages.is_dir() {
        scan_directory(&astro_pages, &astro_pages, "astro", &mut pages);
        scan_directory(&astro_pages, &astro_pages, "md", &mut pages);
    }

    // Also check sibling -dev directory (CruxDev convention)
    let dev_pages_str = format!("{}-dev/src/pages", project_dir.trim_end_matches('/'));
    let dev_pages = Path::new(&dev_pages_str);
    if dev_pages.is_dir() {
        scan_directory(dev_pages, dev_pages, "astro", &mut pages);
        scan_directory(dev_pages, dev_pages, "md", &mut pages);
    }

    // Phoenix: parse lib/*_web/router.ex
    let phoenix_routers: Vec<_> = glob::glob(&format!("{}/lib/*_web/router.ex", project_dir))
        .into_iter()
        .flatten()
        .filter_map(|p| p.ok())
        .collect();
    for router in phoenix_routers {
        pages.extend(parse_phoenix_router(&router));
    }

    // Next.js: scan app/ for page.tsx
    let nextjs_app = root.join("app");
    if nextjs_app.is_dir() {
        scan_directory(&nextjs_app, &nextjs_app, "tsx", &mut pages);
    }

    // Classify and assign dimensions
    for page in &mut pages {
        page.page_type = classify_page(&page.path, &page.route);
        page.applicable_dimensions = dimensions_for_type(&page.page_type);
    }

    pages
}

/// Scan a directory for files with a given extension.
fn scan_directory(dir: &Path, base: &Path, ext: &str, pages: &mut Vec<PageRoute>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, base, ext, pages);
            } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                let rel = path.strip_prefix(base).unwrap_or(&path);
                let route = path_to_route(rel.to_str().unwrap_or(""), ext);
                pages.push(PageRoute {
                    path: path.to_string_lossy().to_string(),
                    route,
                    page_type: PageType::Unknown,
                    applicable_dimensions: Vec::new(),
                });
            }
        }
    }
}

/// Convert a file path to a URL route.
fn path_to_route(rel_path: &str, ext: &str) -> String {
    let route = rel_path
        .replace(&format!(".{ext}"), "")
        .replace("index", "")
        .replace('\\', "/");
    let route = route.trim_end_matches('/');
    if route.is_empty() { "/".to_string() } else { format!("/{route}") }
}

/// Parse Phoenix router.ex for routes.
fn parse_phoenix_router(router_path: &Path) -> Vec<PageRoute> {
    let content = match fs::read_to_string(router_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let re = regex::Regex::new(r#"(?:get|post|put|patch|delete|live)\s+"([^"]+)""#).unwrap();
    re.captures_iter(&content)
        .map(|cap| PageRoute {
            path: router_path.to_string_lossy().to_string(),
            route: cap[1].to_string(),
            page_type: PageType::Unknown,
            applicable_dimensions: Vec::new(),
        })
        .collect()
}

/// Classify a page based on its path and route.
pub fn classify_page(file_path: &str, route: &str) -> PageType {
    let lower_path = file_path.to_lowercase();
    let lower_route = route.to_lowercase();

    if lower_route.contains("/api/") || lower_path.contains("/api/") {
        PageType::Api
    } else if lower_route.contains("/login") || lower_route.contains("/register")
        || lower_route.contains("/auth") || lower_route.contains("/signup")
    {
        PageType::Auth
    } else if lower_route.contains("/dashboard") || lower_route.contains("/admin") {
        PageType::Dashboard
    } else if lower_route.contains("/blog") || lower_path.contains("/blog/") {
        PageType::Blog
    } else if lower_path.contains("index.") || lower_route == "/" {
        PageType::Index
    } else if has_form_indicators(&lower_path) {
        PageType::Form
    } else {
        PageType::Static
    }
}

fn has_form_indicators(path: &str) -> bool {
    let form_keywords = ["form", "contact", "checkout", "subscribe", "signup", "register", "settings", "profile", "edit", "new", "create"];
    form_keywords.iter().any(|k| path.contains(k))
}

/// Get applicable dimensions for a page type.
pub fn dimensions_for_type(page_type: &PageType) -> Vec<String> {
    let mut dims: Vec<String> = vec![
        // All pages get these
        "color_contrast".into(),
        "mobile_responsiveness".into(),
        "accessibility".into(),
        "link_integrity".into(),
        "seo_meta".into(),
    ];

    match page_type {
        PageType::Form => {
            dims.extend(["layout", "labels", "label_positioning", "required_indicators",
                "input_sizing", "textarea_usage", "field_grouping", "validation",
                "errors", "error_display", "cta", "trust", "progressive_disclosure", "input_types"]
                .iter().map(|s| s.to_string()));
        }
        PageType::Dashboard => {
            dims.extend(["hierarchy", "density", "visualization", "real_time", "actionability"]
                .iter().map(|s| s.to_string()));
        }
        PageType::Blog => {
            dims.extend(["structured_data", "rss", "reading_time", "social_meta"]
                .iter().map(|s| s.to_string()));
        }
        PageType::Auth => {
            dims.extend(["security", "password_requirements", "error_messages", "rate_limiting"]
                .iter().map(|s| s.to_string()));
        }
        _ => {}
    }

    dims
}

/// Generate per-page audit tasks from an inventory.
pub fn generate_audit_tasks(pages: &[PageRoute]) -> Vec<PageAuditTask> {
    pages.iter().filter(|p| p.page_type != PageType::Api).map(|page| {
        PageAuditTask {
            route: page.route.clone(),
            file_path: page.path.clone(),
            page_type: page.page_type.clone(),
            dimensions: page.applicable_dimensions.clone(),
            description: format!(
                "Audit {} ({:?}) against {} dimensions: {}",
                page.route,
                page.page_type,
                page.applicable_dimensions.len(),
                page.applicable_dimensions.join(", ")
            ),
        }
    }).collect()
}

/// Summary of page audit results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAuditSummary {
    pub total_pages: usize,
    pub pages_passed: usize,
    pub pages_failed: usize,
    pub all_passed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_page_type_form() {
        assert_eq!(classify_page("/src/pages/contact.astro", "/contact"), PageType::Form);
        assert_eq!(classify_page("/src/pages/checkout.astro", "/checkout"), PageType::Form);
    }

    #[test]
    fn test_classify_page_type_dashboard() {
        assert_eq!(classify_page("/src/pages/dashboard.astro", "/dashboard"), PageType::Dashboard);
        assert_eq!(classify_page("/src/pages/admin/index.astro", "/admin"), PageType::Dashboard);
    }

    #[test]
    fn test_classify_page_type_blog() {
        assert_eq!(classify_page("/src/pages/blog/post.md", "/blog/post"), PageType::Blog);
    }

    #[test]
    fn test_classify_page_type_auth() {
        assert_eq!(classify_page("/src/pages/login.astro", "/login"), PageType::Auth);
    }

    #[test]
    fn test_classify_page_type_api() {
        assert_eq!(classify_page("/src/pages/api/data.ts", "/api/data"), PageType::Api);
    }

    #[test]
    fn test_classify_page_type_static() {
        assert_eq!(classify_page("/src/pages/about.astro", "/about"), PageType::Static);
    }

    #[test]
    fn test_dimensions_for_form_include_form_dims() {
        let dims = dimensions_for_type(&PageType::Form);
        assert!(dims.contains(&"labels".to_string()));
        assert!(dims.contains(&"validation".to_string()));
        assert!(dims.contains(&"color_contrast".to_string())); // universal
        assert!(dims.len() >= 19); // 5 universal + 14 form
    }

    #[test]
    fn test_dimensions_for_static_only_universal() {
        let dims = dimensions_for_type(&PageType::Static);
        assert_eq!(dims.len(), 5);
    }

    #[test]
    fn test_path_to_route() {
        assert_eq!(path_to_route("blog/post.md", "md"), "/blog/post");
        assert_eq!(path_to_route("index.astro", "astro"), "/");
        assert_eq!(path_to_route("about.astro", "astro"), "/about");
        assert_eq!(path_to_route("vs/cursor.astro", "astro"), "/vs/cursor");
    }

    #[test]
    fn test_generate_audit_tasks_excludes_api() {
        let pages = vec![
            PageRoute { path: "a.astro".into(), route: "/about".into(), page_type: PageType::Static, applicable_dimensions: vec!["contrast".into()] },
            PageRoute { path: "b.ts".into(), route: "/api/data".into(), page_type: PageType::Api, applicable_dimensions: vec![] },
        ];
        let tasks = generate_audit_tasks(&pages);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].route, "/about");
    }

    #[test]
    fn test_inventory_routes_finds_pages() {
        // Use the actual cruxdev-dev site if it exists
        let dev_dir = format!("{}/..", env!("CARGO_MANIFEST_DIR"));
        let pages = inventory_routes(&dev_dir);
        // Should find at least some pages (from the -dev sibling)
        // This test is environment-dependent but validates the scanning works
        assert!(pages.len() > 0 || !Path::new(&format!("{}-dev/src/pages", dev_dir)).exists(),
            "Expected pages from inventory, got 0");
    }
}
