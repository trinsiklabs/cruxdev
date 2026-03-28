//! Competitive analysis runner — single-call orchestration.
//!
//! Provides setup for competitive analysis from known competitor data,
//! writes results to disk, and parses competitor inputs.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::discovery::generate_discovery_queries;
use super::gap_analysis::{run_gap_analysis, GapAnalysisResult};
use super::research::{CompetitorCategory, CompetitorProfile, Feature};

/// Minimal input to define a competitor -- no LLM needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorInput {
    pub name: String,
    pub url: String,
    pub category: String, // "official", "watch", "noted"
    pub description: String,
    pub features: Vec<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub pricing: String,
    pub revenue_model: String,
}

impl CompetitorInput {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            category: "noted".to_string(),
            description: String::new(),
            features: Vec::new(),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            pricing: String::new(),
            revenue_model: String::new(),
        }
    }
}

/// Complete result of a competitive analysis run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub our_name: String,
    pub competitors_doc: String,
    pub gap_analysis: String,
    pub comparison_pages: HashMap<String, String>,
    pub discovery_queries: Vec<String>,
    pub summary: HashMap<String, usize>,
}

fn input_to_profile(inp: &CompetitorInput) -> CompetitorProfile {
    let mut profile = CompetitorProfile::new(&inp.name, &inp.url);
    profile.category = CompetitorCategory::from_str_loose(&inp.category);
    profile.description = inp.description.clone();
    profile.features = inp.features.iter().map(|f| Feature::new(f, "")).collect();
    profile.strengths = inp.strengths.clone();
    profile.weaknesses = inp.weaknesses.clone();
    profile.pricing = inp.pricing.clone();
    profile.revenue_model = inp.revenue_model.clone();
    profile.last_researched = chrono::Local::now().format("%Y-%m-%d").to_string();
    profile
}

/// Generate full COMPETITORS.md content from profiles and gap analysis.
pub fn generate_competitors_doc(
    title: &str,
    overview: &str,
    profiles: &[CompetitorProfile],
    gap_result: Option<&GapAnalysisResult>,
) -> String {
    let mut lines = vec![format!("# {title}"), String::new()];

    if !overview.is_empty() {
        lines.push(overview.to_string());
        lines.push(String::new());
    }

    for category in [CompetitorCategory::Official, CompetitorCategory::Watch, CompetitorCategory::Noted] {
        let cat_profiles: Vec<&CompetitorProfile> = profiles
            .iter()
            .filter(|p| p.category == category)
            .collect();
        if !cat_profiles.is_empty() {
            let label = match category {
                CompetitorCategory::Official => "Official",
                CompetitorCategory::Watch => "Watch",
                CompetitorCategory::Noted => "Noted",
            };
            lines.push(format!("## {label} Competitors"));
            lines.push(String::new());
            for profile in cat_profiles {
                lines.push(profile.to_markdown());
                lines.push(String::new());
            }
        }
    }

    if let Some(gap) = gap_result {
        lines.push(gap.to_markdown());
    }

    lines.join("\n")
}

/// Generate a comparison page for one competitor.
pub fn generate_comparison_page(
    our_name: &str,
    our_features: &[String],
    profile: &CompetitorProfile,
) -> (String, String) {
    let slug = profile.name.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
    let title = format!("{our_name} vs {}", profile.name);
    let meta_desc = format!(
        "Compare {our_name} and {}. See features, pricing, and which is right for you.",
        profile.name
    );

    let mut lines = vec![
        "---".to_string(),
        format!("title: \"{title}\""),
        format!("description: \"{meta_desc}\""),
        format!("slug: \"{slug}\""),
        "---".to_string(),
        String::new(),
        format!("# {our_name} vs {}", profile.name),
        String::new(),
    ];

    if !profile.tagline.is_empty() {
        lines.push(format!("**{}:** {}", profile.name, profile.tagline));
        lines.push(String::new());
    }

    // Feature comparison table
    let competitor_features: std::collections::BTreeSet<String> = profile
        .feature_names()
        .iter()
        .map(|f| f.to_lowercase())
        .collect();
    let our_set: std::collections::BTreeSet<String> = our_features
        .iter()
        .map(|f| f.to_lowercase())
        .collect();
    let mut all_features: std::collections::BTreeSet<String> = our_set.clone();
    all_features.extend(competitor_features.clone());

    if !all_features.is_empty() {
        lines.push("## Feature Comparison".to_string());
        lines.push(String::new());
        lines.push(format!("| Feature | {our_name} | {} |", profile.name));
        lines.push("|---|---|---|".to_string());
        for feat in &all_features {
            let our_has = if our_set.contains(feat) { "Y" } else { "N" };
            let comp_has = if competitor_features.contains(feat) { "Y" } else { "N" };
            let display = feat
                .split(['-', '_'])
                .map(|w| {
                    let mut chars = w.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            lines.push(format!("| {display} | {our_has} | {comp_has} |"));
        }
        lines.push(String::new());
    }

    if !profile.strengths.is_empty() {
        lines.push(format!("## {} Strengths", profile.name));
        lines.push(String::new());
        for s in &profile.strengths {
            lines.push(format!("- {s}"));
        }
        lines.push(String::new());
    }

    if !profile.weaknesses.is_empty() {
        lines.push(format!("## {} Weaknesses", profile.name));
        lines.push(String::new());
        for w in &profile.weaknesses {
            lines.push(format!("- {w}"));
        }
        lines.push(String::new());
    }

    if !profile.pricing.is_empty() {
        lines.push("## Pricing".to_string());
        lines.push(String::new());
        lines.push(format!("**{}:** {}", profile.name, profile.pricing));
        lines.push(String::new());
    }

    (slug, lines.join("\n"))
}

/// Run complete competitive analysis from known competitor data.
pub fn setup(
    our_name: &str,
    our_description: &str,
    our_category: &str,
    our_features: &[String],
    competitors: &[CompetitorInput],
) -> AnalysisResult {
    let profiles: Vec<CompetitorProfile> = competitors.iter().map(input_to_profile).collect();

    let official: Vec<String> = profiles
        .iter()
        .filter(|p| p.category == CompetitorCategory::Official)
        .map(|p| p.name.clone())
        .collect();

    let gap_result = run_gap_analysis(our_name, our_features, &profiles, Some(&official));
    let queries = generate_discovery_queries(our_description, our_category, 10);

    let mut summary = HashMap::new();
    summary.insert("total_competitors".to_string(), profiles.len());
    summary.insert(
        "official".to_string(),
        profiles.iter().filter(|p| p.category == CompetitorCategory::Official).count(),
    );
    summary.insert(
        "watch".to_string(),
        profiles.iter().filter(|p| p.category == CompetitorCategory::Watch).count(),
    );
    summary.insert(
        "noted".to_string(),
        profiles.iter().filter(|p| p.category == CompetitorCategory::Noted).count(),
    );
    summary.insert("total_features_compared".to_string(), gap_result.feature_matrix.len());
    summary.insert("total_gaps".to_string(), gap_result.gaps.len());
    summary.insert("must_close".to_string(), gap_result.must_close().len());
    summary.insert("should_close".to_string(), gap_result.should_close().len());
    summary.insert("discovery_queries_for_research".to_string(), queries.len());

    // Generate COMPETITORS.md content
    let overview = format!("Competitive landscape for {our_name} in the {our_category} category.");
    let competitors_doc = generate_competitors_doc(
        &format!("{our_name} — Competitive Analysis"),
        &overview,
        &profiles,
        Some(&gap_result),
    );

    // Generate comparison pages for official + watch tier
    let mut comparison_pages = HashMap::new();
    for profile in &profiles {
        if profile.category == CompetitorCategory::Official || profile.category == CompetitorCategory::Watch {
            let (slug, content) = generate_comparison_page(our_name, our_features, profile);
            comparison_pages.insert(slug, content);
        }
    }

    AnalysisResult {
        our_name: our_name.to_string(),
        competitors_doc,
        gap_analysis: gap_result.to_markdown(),
        comparison_pages,
        discovery_queries: queries,
        summary,
    }
}

/// Write analysis results to disk.
pub fn write_results(
    result: &AnalysisResult,
    project_dir: &str,
    docs_dir: &str,
    vs_dir: &str,
) -> Result<Vec<PathBuf>> {
    let mut written = Vec::new();

    // Write COMPETITORS.md — guard against writing empty content
    let comp_path = Path::new(project_dir).join(docs_dir).join("COMPETITORS.md");
    if result.competitors_doc.trim().is_empty() {
        return Err(anyhow::anyhow!("Refusing to write empty COMPETITORS.md — generation returned no content"));
    }
    if let Some(parent) = comp_path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Atomic write: write to temp file then rename
    let tmp_path = comp_path.with_extension("md.tmp");
    fs::write(&tmp_path, &result.competitors_doc)?;
    fs::rename(&tmp_path, &comp_path)?;
    written.push(comp_path);

    // Write comparison pages
    if !vs_dir.is_empty() && !result.comparison_pages.is_empty() {
        let vs_path = Path::new(project_dir).join(vs_dir);
        fs::create_dir_all(&vs_path)?;
        for (slug, content) in &result.comparison_pages {
            if content.trim().is_empty() {
                continue; // Skip empty pages
            }
            let page_path = vs_path.join(format!("{slug}.md"));
            let tmp_path = page_path.with_extension("md.tmp");
            fs::write(&tmp_path, content)?;
            fs::rename(&tmp_path, &page_path)?;
            written.push(page_path);
        }
    }

    Ok(written)
}

/// Parse competitor inputs from any format a model might send.
pub fn parse_competitor_inputs(raw: &Value) -> Vec<CompetitorInput> {
    if raw.is_null() {
        return Vec::new();
    }

    if let Some(s) = raw.as_str() {
        match serde_json::from_str::<Value>(s) {
            Ok(parsed) => return parse_competitor_inputs(&parsed),
            Err(_) => return Vec::new(),
        }
    }

    let arr = match raw.as_array() {
        Some(a) => a,
        None => return Vec::new(),
    };

    let mut results = Vec::new();
    for item in arr {
        let obj = match item.as_object() {
            Some(o) => o,
            None => continue,
        };

        let features = parse_string_or_list(obj.get("features"));
        let strengths = parse_string_or_list(obj.get("strengths"));
        let weaknesses = parse_string_or_list(obj.get("weaknesses"));

        results.push(CompetitorInput {
            name: obj.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            url: obj.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            category: obj.get("category").and_then(|v| v.as_str()).unwrap_or("noted").to_string(),
            description: obj.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            features,
            strengths,
            weaknesses,
            pricing: obj.get("pricing").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            revenue_model: obj.get("revenue_model").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        });
    }

    results
}

fn parse_string_or_list(val: Option<&Value>) -> Vec<String> {
    match val {
        None => Vec::new(),
        Some(Value::String(s)) => s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| {
                if let Some(s) = v.as_str() {
                    Some(s.to_string())
                } else if let Some(obj) = v.as_object() {
                    obj.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_competitor_inputs_from_json_array() {
        let raw = serde_json::json!([
            {"name": "Cursor", "url": "https://cursor.sh", "category": "official", "features": ["autocomplete", "chat"]},
            {"name": "Copilot", "url": "https://copilot.github.com"}
        ]);
        let inputs = parse_competitor_inputs(&raw);
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].name, "Cursor");
        assert_eq!(inputs[0].category, "official");
        assert_eq!(inputs[0].features, vec!["autocomplete", "chat"]);
        assert_eq!(inputs[1].name, "Copilot");
        assert_eq!(inputs[1].category, "noted"); // default
    }

    #[test]
    fn test_parse_competitor_inputs_from_string() {
        let raw = serde_json::json!(r#"[{"name": "Test", "url": "https://test.com"}]"#);
        let inputs = parse_competitor_inputs(&raw);
        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].name, "Test");
    }

    #[test]
    fn test_parse_competitor_inputs_null() {
        let raw = serde_json::json!(null);
        let inputs = parse_competitor_inputs(&raw);
        assert!(inputs.is_empty());
    }

    #[test]
    fn test_parse_competitor_inputs_string_features() {
        let raw = serde_json::json!([{"name": "X", "features": "a, b, c"}]);
        let inputs = parse_competitor_inputs(&raw);
        assert_eq!(inputs[0].features, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_setup_basic() {
        let competitors = vec![CompetitorInput {
            name: "Rival".to_string(),
            url: "https://rival.com".to_string(),
            category: "official".to_string(),
            description: "A rival product".to_string(),
            features: vec!["Search".to_string()],
            strengths: vec!["Fast".to_string()],
            weaknesses: vec![],
            pricing: "$10/mo".to_string(),
            revenue_model: "subscription".to_string(),
        }];
        let result = setup("Us", "Our great product", "AI tools", &["Chat".to_string()], &competitors);
        assert_eq!(result.our_name, "Us");
        assert!(!result.discovery_queries.is_empty());
        assert_eq!(*result.summary.get("total_competitors").unwrap(), 1);
        // Verify doc generation works (F1 fix)
        assert!(!result.competitors_doc.is_empty(), "competitors_doc should not be empty");
        assert!(result.competitors_doc.contains("# Us"), "doc should have title");
        assert!(result.competitors_doc.contains("### Rival"), "doc should contain competitor profile");
        // Verify comparison pages generated (F2 fix)
        assert!(!result.comparison_pages.is_empty(), "comparison_pages should not be empty");
        assert!(result.comparison_pages.contains_key("rival"), "should have rival comparison page");
    }

    #[test]
    fn test_generate_competitors_doc() {
        use crate::competitors::research::CompetitorProfile;
        let mut p1 = CompetitorProfile::new("Alpha", "https://alpha.com");
        p1.category = CompetitorCategory::Official;
        p1.pricing = "$20/mo".to_string();
        let mut p2 = CompetitorProfile::new("Beta", "https://beta.com");
        p2.category = CompetitorCategory::Watch;
        let profiles = vec![p1, p2];
        let doc = generate_competitors_doc("Test Analysis", "Overview text.", &profiles, None);
        assert!(doc.contains("# Test Analysis"));
        assert!(doc.contains("Overview text."));
        assert!(doc.contains("## Official Competitors"));
        assert!(doc.contains("### Alpha"));
        assert!(doc.contains("## Watch Competitors"));
        assert!(doc.contains("### Beta"));
    }

    #[test]
    fn test_generate_comparison_page() {
        let mut profile = CompetitorProfile::new("Rival", "https://rival.com");
        profile.tagline = "The best tool".to_string();
        profile.features = vec![Feature::new("Search", ""), Feature::new("Chat", "")];
        profile.strengths = vec!["Fast".to_string()];
        profile.weaknesses = vec!["Expensive".to_string()];
        profile.pricing = "$10/mo".to_string();

        let (slug, content) = generate_comparison_page("Us", &["Chat".to_string(), "Export".to_string()], &profile);
        assert_eq!(slug, "rival");
        assert!(content.contains("title: \"Us vs Rival\""));
        assert!(content.contains("# Us vs Rival"));
        assert!(content.contains("## Feature Comparison"));
        assert!(content.contains("| Chat | Y | Y |"));
        assert!(content.contains("## Rival Strengths"));
        assert!(content.contains("- Fast"));
        assert!(content.contains("## Rival Weaknesses"));
        assert!(content.contains("- Expensive"));
        assert!(content.contains("## Pricing"));
        assert!(content.contains("**Rival:** $10/mo"));
    }

    #[test]
    fn test_write_results() {
        let dir = tempfile::tempdir().unwrap();
        let result = AnalysisResult {
            our_name: "Us".to_string(),
            competitors_doc: "# Competitors".to_string(),
            gap_analysis: "## Gaps".to_string(),
            comparison_pages: HashMap::new(),
            discovery_queries: vec![],
            summary: HashMap::new(),
        };
        let written = write_results(&result, dir.path().to_str().unwrap(), "docs", "").unwrap();
        assert_eq!(written.len(), 1);
        assert!(written[0].exists());
    }
}
