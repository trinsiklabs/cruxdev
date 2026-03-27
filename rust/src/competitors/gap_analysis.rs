//! Gap analysis — compare our features against competitors.
//!
//! Reads competitor profiles, builds a feature matrix, classifies gaps
//! by priority (must-close, should-close, nice-to-have, won't-do).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use super::research::CompetitorProfile;

/// Gap priority classification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GapPriority {
    MustClose,
    ShouldClose,
    NiceToHave,
    WontDo,
}

impl GapPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MustClose => "must-close",
            Self::ShouldClose => "should-close",
            Self::NiceToHave => "nice-to-have",
            Self::WontDo => "wont-do",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::MustClose => "Must-Close",
            Self::ShouldClose => "Should-Close",
            Self::NiceToHave => "Nice-To-Have",
            Self::WontDo => "Wont-Do",
        }
    }
}

/// Gap status.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GapStatus {
    #[default]
    Open,
    InProgress,
    Closed,
}

impl GapStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in-progress",
            Self::Closed => "closed",
        }
    }
}

/// A gap where competitors have a feature we don't.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGap {
    pub feature_name: String,
    pub competitors_with_feature: Vec<String>,
    pub priority: GapPriority,
    pub rationale: String,
    pub status: GapStatus,
    pub build_plan: String,
}

impl FeatureGap {
    pub fn new(feature_name: &str, competitors_with_feature: Vec<String>, priority: GapPriority) -> Self {
        Self {
            feature_name: feature_name.to_string(),
            competitors_with_feature,
            priority,
            rationale: String::new(),
            status: GapStatus::default(),
            build_plan: String::new(),
        }
    }
}

/// One row in the feature comparison matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMatrixEntry {
    pub feature: String,
    pub our_status: bool,
    pub competitor_status: HashMap<String, bool>,
}

/// Result of a gap analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysisResult {
    pub our_name: String,
    pub feature_matrix: Vec<FeatureMatrixEntry>,
    pub gaps: Vec<FeatureGap>,
}

impl GapAnalysisResult {
    pub fn must_close(&self) -> Vec<&FeatureGap> {
        self.gaps.iter().filter(|g| g.priority == GapPriority::MustClose).collect()
    }

    pub fn should_close(&self) -> Vec<&FeatureGap> {
        self.gaps.iter().filter(|g| g.priority == GapPriority::ShouldClose).collect()
    }

    pub fn open_gaps(&self) -> Vec<&FeatureGap> {
        self.gaps.iter().filter(|g| g.status == GapStatus::Open).collect()
    }

    /// Render as markdown for COMPETITORS.md gap section.
    pub fn to_markdown(&self) -> String {
        let mut lines = vec!["## Gap Analysis".to_string(), String::new()];

        // Feature matrix
        if !self.feature_matrix.is_empty() {
            let mut competitors = BTreeSet::new();
            for entry in &self.feature_matrix {
                for k in entry.competitor_status.keys() {
                    competitors.insert(k.clone());
                }
            }
            let comp_list: Vec<String> = competitors.into_iter().collect();

            let header = format!(
                "| Feature | {} | {} |",
                self.our_name,
                comp_list.join(" | ")
            );
            let sep = format!(
                "|---|---|{}",
                comp_list.iter().map(|_| "---|").collect::<Vec<_>>().join("")
            );
            lines.push(header);
            lines.push(sep);

            for entry in &self.feature_matrix {
                let our = if entry.our_status { "Y" } else { "N" };
                let comps: Vec<&str> = comp_list
                    .iter()
                    .map(|c| {
                        if *entry.competitor_status.get(c).unwrap_or(&false) {
                            "Y"
                        } else {
                            "N"
                        }
                    })
                    .collect();
                lines.push(format!("| {} | {} | {} |", entry.feature, our, comps.join(" | ")));
            }
            lines.push(String::new());
        }

        // Gaps by priority
        for priority in [GapPriority::MustClose, GapPriority::ShouldClose, GapPriority::NiceToHave] {
            let gaps_at_priority: Vec<&FeatureGap> =
                self.gaps.iter().filter(|g| g.priority == priority).collect();
            if !gaps_at_priority.is_empty() {
                lines.push(format!("### {}", priority.title()));
                for gap in &gaps_at_priority {
                    let status = if gap.status != GapStatus::Open {
                        format!(" [{}]", gap.status.as_str())
                    } else {
                        String::new()
                    };
                    let comps = gap.competitors_with_feature.join(", ");
                    lines.push(format!("- **{}**{status} — has: {comps}", gap.feature_name));
                    if !gap.rationale.is_empty() {
                        lines.push(format!("  - {}", gap.rationale));
                    }
                }
                lines.push(String::new());
            }
        }

        lines.join("\n")
    }
}

/// Build a feature comparison matrix.
pub fn build_feature_matrix(
    _our_name: &str,
    our_features: &[String],
    profiles: &[CompetitorProfile],
) -> Vec<FeatureMatrixEntry> {
    let mut all_features: BTreeMap<String, FeatureMatrixEntry> = BTreeMap::new();

    // Our features
    for f in our_features {
        all_features.insert(
            f.to_lowercase(),
            FeatureMatrixEntry {
                feature: f.clone(),
                our_status: true,
                competitor_status: HashMap::new(),
            },
        );
    }

    // Competitor features
    for profile in profiles {
        for feat in &profile.features {
            let key = feat.name.to_lowercase();
            let entry = all_features.entry(key).or_insert_with(|| FeatureMatrixEntry {
                feature: feat.name.clone(),
                our_status: false,
                competitor_status: HashMap::new(),
            });
            entry.competitor_status.insert(profile.name.clone(), feat.has_feature);
        }
    }

    // BTreeMap is already sorted by key
    all_features.into_values().collect()
}

/// Classify gaps from the feature matrix.
///
/// Rules:
/// - Feature we don't have + 2+ official competitors have -> must-close
/// - Feature we don't have + 1 official competitor has -> should-close
/// - Feature we don't have + only non-official competitors -> nice-to-have
pub fn classify_gaps(
    feature_matrix: &[FeatureMatrixEntry],
    official_competitors: &[String],
) -> Vec<FeatureGap> {
    let mut gaps = Vec::new();

    for entry in feature_matrix {
        if entry.our_status {
            continue; // We have it, no gap
        }

        let comps_with: Vec<String> = entry
            .competitor_status
            .iter()
            .filter(|(_, has)| **has)
            .map(|(name, _)| name.clone())
            .collect();

        if comps_with.is_empty() {
            continue;
        }

        let official_with: Vec<&String> = comps_with
            .iter()
            .filter(|c| official_competitors.contains(c))
            .collect();

        let priority = if official_with.len() >= 2 {
            GapPriority::MustClose
        } else if official_with.len() == 1 {
            GapPriority::ShouldClose
        } else {
            GapPriority::NiceToHave
        };

        gaps.push(FeatureGap::new(&entry.feature, comps_with, priority));
    }

    gaps
}

/// Run a complete gap analysis.
pub fn run_gap_analysis(
    our_name: &str,
    our_features: &[String],
    profiles: &[CompetitorProfile],
    official_competitors: Option<&[String]>,
) -> GapAnalysisResult {
    let default_officials: Vec<String> = profiles.iter().map(|p| p.name.clone()).collect();
    let officials = official_competitors.unwrap_or(&default_officials);

    let matrix = build_feature_matrix(our_name, our_features, profiles);
    let gaps = classify_gaps(&matrix, officials);

    GapAnalysisResult {
        our_name: our_name.to_string(),
        feature_matrix: matrix,
        gaps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::competitors::research::Feature;

    fn make_profile(name: &str, features: Vec<&str>) -> CompetitorProfile {
        let mut p = CompetitorProfile::new(name, &format!("https://{}.com", name.to_lowercase()));
        p.features = features.iter().map(|f| Feature::new(f, "")).collect();
        p
    }

    #[test]
    fn test_build_feature_matrix() {
        let our_features = vec!["CodeGen".to_string(), "Refactor".to_string()];
        let profiles = vec![make_profile("Rival", vec!["CodeGen", "Debug"])];
        let matrix = build_feature_matrix("Us", &our_features, &profiles);

        assert_eq!(matrix.len(), 3); // CodeGen, Debug, Refactor (sorted)
        let codegen = matrix.iter().find(|e| e.feature == "CodeGen").unwrap();
        assert!(codegen.our_status);
        assert_eq!(codegen.competitor_status.get("Rival"), Some(&true));

        let debug = matrix.iter().find(|e| e.feature == "Debug").unwrap();
        assert!(!debug.our_status);
    }

    #[test]
    fn test_classify_gaps_must_close() {
        let our_features = vec!["A".to_string()];
        let profiles = vec![
            make_profile("Official1", vec!["A", "B"]),
            make_profile("Official2", vec!["B", "C"]),
        ];
        let officials = vec!["Official1".to_string(), "Official2".to_string()];
        let matrix = build_feature_matrix("Us", &our_features, &profiles);
        let gaps = classify_gaps(&matrix, &officials);

        let b_gap = gaps.iter().find(|g| g.feature_name.to_lowercase() == "b").unwrap();
        assert_eq!(b_gap.priority, GapPriority::MustClose);
    }

    #[test]
    fn test_classify_gaps_should_close() {
        let our_features: Vec<String> = vec![];
        let profiles = vec![
            make_profile("Official1", vec!["X"]),
            make_profile("Noted1", vec!["X"]),
        ];
        let officials = vec!["Official1".to_string()];
        let matrix = build_feature_matrix("Us", &our_features, &profiles);
        let gaps = classify_gaps(&matrix, &officials);

        let x_gap = gaps.iter().find(|g| g.feature_name == "X").unwrap();
        assert_eq!(x_gap.priority, GapPriority::ShouldClose);
    }

    #[test]
    fn test_classify_gaps_nice_to_have() {
        let our_features: Vec<String> = vec![];
        let profiles = vec![make_profile("Noted1", vec!["Y"])];
        let officials: Vec<String> = vec![];
        let matrix = build_feature_matrix("Us", &our_features, &profiles);
        let gaps = classify_gaps(&matrix, &officials);

        let y_gap = gaps.iter().find(|g| g.feature_name == "Y").unwrap();
        assert_eq!(y_gap.priority, GapPriority::NiceToHave);
    }

    #[test]
    fn test_run_gap_analysis_defaults_to_all_official() {
        let profiles = vec![
            make_profile("A", vec!["Feat1"]),
            make_profile("B", vec!["Feat1"]),
        ];
        let result = run_gap_analysis("Us", &[], &profiles, None);
        // Both are "official" by default, so 2 officials -> must-close
        let gap = &result.gaps[0];
        assert_eq!(gap.priority, GapPriority::MustClose);
    }

    #[test]
    fn test_gap_analysis_to_markdown() {
        let profiles = vec![make_profile("Rival", vec!["Search"])];
        let result = run_gap_analysis("Us", &["Chat".to_string()], &profiles, None);
        let md = result.to_markdown();
        assert!(md.contains("## Gap Analysis"));
        assert!(md.contains("| Feature |"));
    }
}
