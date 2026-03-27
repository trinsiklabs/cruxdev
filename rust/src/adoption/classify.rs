//! Project classification — determine type(s), maturity, and required templates.
//!
//! Classifies a project by analyzing its contents: code, docs, configs, etc.
//! Supports composite types (most projects are multi-type).

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Project type classification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectType {
    SoftwareExisting,
    SoftwareGreenfield,
    BusinessExisting,
    BusinessNew,
    ProductSaas,
    Website,
    Infrastructure,
    ConsultingClient,
    Research,
    Campaign,
    Book,
    BookSeries,
    Podcast,
    Newsletter,
    YouTube,
    Course,
    OpenSource,
    Composite,
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SoftwareExisting => "software-existing",
            Self::SoftwareGreenfield => "software-greenfield",
            Self::BusinessExisting => "business-existing",
            Self::BusinessNew => "business-new",
            Self::ProductSaas => "product-saas",
            Self::Website => "website",
            Self::Infrastructure => "infrastructure",
            Self::ConsultingClient => "consulting-client",
            Self::Research => "research",
            Self::Campaign => "campaign",
            Self::Book => "book",
            Self::BookSeries => "book-series",
            Self::Podcast => "podcast",
            Self::Newsletter => "newsletter",
            Self::YouTube => "youtube",
            Self::Course => "course",
            Self::OpenSource => "open-source",
            Self::Composite => "composite",
        }
    }
}

/// Project maturity level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Maturity {
    Idea,
    Minimal,
    Growing,
    Production,
    Mature,
}

impl Maturity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idea => "idea",
            Self::Minimal => "minimal",
            Self::Growing => "growing",
            Self::Production => "production",
            Self::Mature => "mature",
        }
    }
}

/// Project classification result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub primary_type: ProjectType,
    pub secondary_types: Vec<ProjectType>,
    pub maturity: Maturity,
    pub confidence: f64,
    pub signals: HashMap<String, Vec<String>>,
}

impl Classification {
    pub fn all_types(&self) -> Vec<ProjectType> {
        let mut result = vec![self.primary_type.clone()];
        result.extend(self.secondary_types.clone());
        result
    }
}

/// File/dir patterns that indicate project type.
fn type_signals() -> Vec<(ProjectType, Vec<&'static str>)> {
    vec![
        (
            ProjectType::SoftwareExisting,
            vec![
                "src/",
                "lib/",
                "tests/",
                "setup.py",
                "pyproject.toml",
                "package.json",
                "Cargo.toml",
                "go.mod",
            ],
        ),
        (ProjectType::SoftwareGreenfield, vec![]),
        (
            ProjectType::Website,
            vec![
                "index.html",
                "astro.config.*",
                "next.config.*",
                "public/",
                "pages/",
                "src/pages/",
            ],
        ),
        (
            ProjectType::ProductSaas,
            vec![
                "Dockerfile",
                "docker-compose.*",
                ".env.example",
                "api/",
                "app/",
            ],
        ),
        (
            ProjectType::Infrastructure,
            vec![
                "terraform/",
                "*.tf",
                "ansible/",
                "k8s/",
                "Makefile",
                ".github/workflows/",
            ],
        ),
        (
            ProjectType::Research,
            vec!["papers/", "experiments/", "data/", "notebooks/", "*.ipynb"],
        ),
        (
            ProjectType::BusinessExisting,
            vec![
                "docs/BUSINESS_PLAN.md",
                "docs/BUDGET.md",
                "docs/OPERATIONS.md",
            ],
        ),
        (ProjectType::BusinessNew, vec![]),
        (
            ProjectType::ConsultingClient,
            vec!["clients/", "proposals/", "deliverables/"],
        ),
        (
            ProjectType::Campaign,
            vec!["campaigns/", "marketing/", "ads/", "content/"],
        ),
        (
            ProjectType::Book,
            vec![
                "BOOK_OUTLINE.md", "docs/BOOK_OUTLINE.md",
                "CHAPTER_TEMPLATE.md", "MANUSCRIPT_TRACKING.md",
                "chapters/", "manuscript/",
            ],
        ),
        (
            ProjectType::BookSeries,
            vec![
                "SERIES_BIBLE.md", "docs/SERIES_BIBLE.md",
                "books/", "series/",
            ],
        ),
        (
            ProjectType::Podcast,
            vec![
                "SHOW_FORMAT.md", "docs/SHOW_FORMAT.md",
                "EPISODE_PLAN.md", "episodes/",
                "PODCAST_PRODUCTION_GUIDE.md",
            ],
        ),
        (
            ProjectType::Newsletter,
            vec![
                "NEWSLETTER_STRATEGY.md", "docs/NEWSLETTER_STRATEGY.md",
                "ISSUE_PLAN.md", "issues/", "newsletters/",
            ],
        ),
        (
            ProjectType::YouTube,
            vec![
                "CHANNEL_STRATEGY.md", "docs/CHANNEL_STRATEGY.md",
                "VIDEO_SEO_STRATEGY.md", "videos/", "thumbnails/",
            ],
        ),
        (
            ProjectType::Course,
            vec![
                "COURSE_OUTLINE.md", "docs/COURSE_OUTLINE.md",
                "lessons/", "modules/", "curriculum/",
            ],
        ),
        (
            ProjectType::OpenSource,
            vec![
                "CONTRIBUTING.md", "CONTRIBUTING.rst",
                "GOVERNANCE.md", "CODE_OF_CONDUCT.md",
                ".github/ISSUE_TEMPLATE/",
            ],
        ),
    ]
}

fn matches_pattern(entry: &str, pattern: &str) -> bool {
    if pattern.ends_with('/') {
        // Directory pattern
        entry.ends_with('/')
            && entry
                .trim_end_matches('/')
                .ends_with(pattern.trim_end_matches('/'))
    } else if pattern.starts_with("*.") {
        // Extension pattern
        entry.ends_with(&pattern[1..])
    } else if pattern.contains('*') {
        // Simple wildcard -- match prefix
        let prefix = pattern.split('*').next().unwrap_or("");
        entry.starts_with(prefix) || entry.contains(&format!("/{prefix}"))
    } else {
        // Exact match
        entry == pattern || entry.ends_with(&format!("/{pattern}"))
    }
}

fn scan_entries(project_dir: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let base = Path::new(project_dir);
    if !base.is_dir() {
        return entries;
    }

    fn walk(base: &Path, dir: &Path, entries: &mut Vec<String>) {
        let read_dir = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return,
        };

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || name == "node_modules" || name == "__pycache__" || name == "templates" || name == "target" {
                continue;
            }
            let path = entry.path();
            let rel = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            if path.is_dir() {
                entries.push(format!("{rel}/"));
                walk(base, &path, entries);
            } else {
                entries.push(rel);
            }
        }
    }

    walk(base, base, &mut entries);
    entries
}

fn assess_maturity(entries: &[String]) -> Maturity {
    let entry_str = entries.join(" ").to_lowercase();

    let mature_indicators = ["contributing", "license", "security.md", "code_of_conduct"]
        .iter()
        .filter(|s| entry_str.contains(**s))
        .count();
    if mature_indicators >= 3 {
        return Maturity::Mature;
    }

    let production_indicators = [".github/workflows/", "dockerfile", "deployment"]
        .iter()
        .filter(|s| entry_str.contains(**s))
        .count();
    if production_indicators >= 2 {
        return Maturity::Production;
    }

    let growing_indicators = ["tests/", "docs/", "changelog"]
        .iter()
        .filter(|s| entry_str.contains(**s))
        .count();
    if growing_indicators >= 2 {
        return Maturity::Growing;
    }

    if ["readme", "src/"].iter().any(|s| entry_str.contains(*s)) {
        return Maturity::Minimal;
    }

    Maturity::Idea
}

/// Classify a project by analyzing its directory structure.
pub fn classify_project(project_dir: &str) -> Classification {
    let entries = scan_entries(project_dir);
    let mut found_signals: HashMap<String, Vec<String>> = HashMap::new();
    let mut type_scores: HashMap<ProjectType, usize> = HashMap::new();

    for (proj_type, patterns) in type_signals() {
        let mut matches = Vec::new();
        for pattern in &patterns {
            for entry in &entries {
                if matches_pattern(entry, pattern) {
                    matches.push(entry.clone());
                }
            }
        }
        if !matches.is_empty() {
            type_scores.insert(proj_type.clone(), matches.len());
            found_signals.insert(proj_type.as_str().to_string(), matches);
        }
    }

    if type_scores.is_empty() {
        return Classification {
            primary_type: ProjectType::SoftwareGreenfield,
            secondary_types: Vec::new(),
            maturity: Maturity::Idea,
            confidence: 0.3,
            signals: found_signals,
        };
    }

    let mut sorted_types: Vec<(ProjectType, usize)> = type_scores.into_iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(&a.1));

    let primary = sorted_types[0].0.clone();
    let secondary: Vec<ProjectType> = sorted_types[1..]
        .iter()
        .filter(|(_, score)| *score > 0)
        .map(|(t, _)| t.clone())
        .collect();

    let maturity = assess_maturity(&entries);
    let max_score = sorted_types[0].1;
    let confidence = (max_score as f64 / 5.0).min(1.0);

    Classification {
        primary_type: primary,
        secondary_types: secondary,
        maturity,
        confidence: (confidence * 100.0).round() / 100.0,
        signals: found_signals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_matches_pattern_directory() {
        assert!(matches_pattern("src/", "src/"));
        assert!(matches_pattern("my/src/", "src/"));
        assert!(!matches_pattern("src.rs", "src/"));
    }

    #[test]
    fn test_matches_pattern_extension() {
        assert!(matches_pattern("main.tf", "*.tf"));
        assert!(matches_pattern("infra/main.tf", "*.tf"));
        assert!(!matches_pattern("main.rs", "*.tf"));
    }

    #[test]
    fn test_matches_pattern_exact() {
        assert!(matches_pattern("setup.py", "setup.py"));
        assert!(matches_pattern("my/setup.py", "setup.py"));
        assert!(!matches_pattern("setup.pyc", "setup.py"));
    }

    #[test]
    fn test_classify_software_project() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path();
        fs::create_dir_all(d.join("src")).unwrap();
        fs::create_dir_all(d.join("tests")).unwrap();
        fs::write(d.join("pyproject.toml"), "[tool]").unwrap();

        let result = classify_project(d.to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::SoftwareExisting);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_classify_empty_project() {
        let dir = tempfile::tempdir().unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::SoftwareGreenfield);
        assert_eq!(result.maturity, Maturity::Idea);
    }

    #[test]
    fn test_classify_nonexistent_dir() {
        let result = classify_project("/nonexistent/directory");
        assert_eq!(result.primary_type, ProjectType::SoftwareGreenfield);
    }

    #[test]
    fn test_all_types() {
        let c = Classification {
            primary_type: ProjectType::Website,
            secondary_types: vec![ProjectType::SoftwareExisting],
            maturity: Maturity::Growing,
            confidence: 0.8,
            signals: HashMap::new(),
        };
        let types = c.all_types();
        assert_eq!(types.len(), 2);
        assert_eq!(types[0], ProjectType::Website);
    }

    #[test]
    fn test_assess_maturity_growing() {
        let entries = vec![
            "tests/".to_string(),
            "docs/".to_string(),
            "src/".to_string(),
        ];
        assert_eq!(assess_maturity(&entries), Maturity::Growing);
    }

    #[test]
    fn test_classify_book() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("chapters")).unwrap();
        fs::write(dir.path().join("BOOK_OUTLINE.md"), "# My Book").unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::Book);
    }

    #[test]
    fn test_classify_book_series() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("SERIES_BIBLE.md"), "# My Series").unwrap();
        fs::create_dir_all(dir.path().join("books")).unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::BookSeries);
    }

    #[test]
    fn test_classify_podcast() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("SHOW_FORMAT.md"), "# My Show").unwrap();
        fs::create_dir_all(dir.path().join("episodes")).unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::Podcast);
    }

    #[test]
    fn test_classify_newsletter() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("NEWSLETTER_STRATEGY.md"), "# Strategy").unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::Newsletter);
    }

    #[test]
    fn test_classify_youtube() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CHANNEL_STRATEGY.md"), "# Channel").unwrap();
        fs::create_dir_all(dir.path().join("videos")).unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        assert_eq!(result.primary_type, ProjectType::YouTube);
    }

    #[test]
    fn test_classify_opensource() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CONTRIBUTING.md"), "# Contributing").unwrap();
        fs::write(dir.path().join("CODE_OF_CONDUCT.md"), "# CoC").unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        let result = classify_project(dir.path().to_str().unwrap());
        // Should detect both open source and software
        let types = result.all_types();
        assert!(types.contains(&ProjectType::OpenSource) || types.contains(&ProjectType::SoftwareExisting));
    }

    #[test]
    fn test_new_project_types_as_str() {
        assert_eq!(ProjectType::Book.as_str(), "book");
        assert_eq!(ProjectType::BookSeries.as_str(), "book-series");
        assert_eq!(ProjectType::Podcast.as_str(), "podcast");
        assert_eq!(ProjectType::Newsletter.as_str(), "newsletter");
        assert_eq!(ProjectType::YouTube.as_str(), "youtube");
        assert_eq!(ProjectType::Course.as_str(), "course");
        assert_eq!(ProjectType::OpenSource.as_str(), "open-source");
        assert_eq!(ProjectType::Composite.as_str(), "composite");
    }
}
