//! Gap analysis — compare project state against template requirements.
//!
//! Detects missing documents, stub documents, and known deficiencies.
//! Generates GAPS.md from analysis.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::inventory::Inventory;
use super::templates::{RequirementLevel, TemplateSet};

/// Gap severity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GapSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl GapSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
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
    NotApplicable,
}

/// A single gap between project state and template requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gap {
    pub template_name: String,
    pub template_file: String,
    pub severity: GapSeverity,
    pub status: GapStatus,
    pub reason: String,
    pub justification: String,
}

/// Complete gap analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub project_dir: String,
    pub gaps: Vec<Gap>,
}

impl GapAnalysis {
    pub fn critical(&self) -> Vec<&Gap> {
        self.gaps
            .iter()
            .filter(|g| g.severity == GapSeverity::Critical && g.status == GapStatus::Open)
            .collect()
    }

    pub fn open_gaps(&self) -> Vec<&Gap> {
        self.gaps
            .iter()
            .filter(|g| g.status == GapStatus::Open)
            .collect()
    }

    pub fn by_severity(&self) -> std::collections::HashMap<String, Vec<&Gap>> {
        let mut result: std::collections::HashMap<String, Vec<&Gap>> = std::collections::HashMap::new();
        for g in &self.gaps {
            if g.status != GapStatus::Open {
                continue;
            }
            result
                .entry(g.severity.as_str().to_string())
                .or_default()
                .push(g);
        }
        result
    }

    /// Generate GAPS.md content.
    pub fn to_markdown(&self) -> String {
        let mut lines = vec![
            "# GAPS — Gap Analysis".to_string(),
            String::new(),
            format!("**Total gaps:** {} open", self.open_gaps().len()),
            format!("**Critical:** {}", self.critical().len()),
            String::new(),
        ];

        for severity in ["critical", "high", "medium", "low"] {
            let gaps_at_level: Vec<&Gap> = self
                .gaps
                .iter()
                .filter(|g| g.severity.as_str() == severity && g.status == GapStatus::Open)
                .collect();
            if !gaps_at_level.is_empty() {
                let title = severity[..1].to_uppercase() + &severity[1..];
                lines.push(format!("## {title}"));
                lines.push(String::new());
                for gap in &gaps_at_level {
                    lines.push(format!(
                        "- [ ] **{}** (`{}`)",
                        gap.template_name, gap.template_file
                    ));
                    if !gap.reason.is_empty() {
                        lines.push(format!("  - {}", gap.reason));
                    }
                }
                lines.push(String::new());
            }
        }

        // Not-applicable items
        let na_items: Vec<&Gap> = self
            .gaps
            .iter()
            .filter(|g| g.status == GapStatus::NotApplicable)
            .collect();
        if !na_items.is_empty() {
            lines.push("## Not Applicable".to_string());
            lines.push(String::new());
            for gap in &na_items {
                lines.push(format!("- **{}**: {}", gap.template_name, gap.justification));
            }
            lines.push(String::new());
        }

        lines.join("\n")
    }
}

const TODO_MARKERS: &[&str] = &["TODO", "FIXME", "TBD", "PLACEHOLDER", "STUB"];

fn is_stub(filepath: &str) -> bool {
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if content.trim().len() < 50 {
        return true;
    }
    let upper = content.to_uppercase();
    TODO_MARKERS.iter().any(|marker| upper.contains(marker))
}

fn severity_from_requirement(req: &RequirementLevel) -> GapSeverity {
    match req {
        RequirementLevel::R => GapSeverity::Critical,
        RequirementLevel::P => GapSeverity::High,
        RequirementLevel::M => GapSeverity::Medium,
        RequirementLevel::O => GapSeverity::Low,
    }
}

/// Compare project inventory against required templates.
pub fn analyze_gaps(project_dir: &str, inventory: &Inventory, template_set: &TemplateSet) -> GapAnalysis {
    let mut analysis = GapAnalysis {
        project_dir: project_dir.to_string(),
        gaps: Vec::new(),
    };

    let existing_files: HashSet<String> = inventory
        .items
        .iter()
        .map(|item| item.path.to_lowercase())
        .collect();

    for template in &template_set.templates {
        let template_lower = template.filename.to_lowercase();

        let mut found = existing_files.contains(&template_lower);
        if !found {
            // Also check without docs/ prefix
            let alt = template_lower.replace("docs/", "");
            found = existing_files.contains(&alt);
        }

        if !found {
            analysis.gaps.push(Gap {
                template_name: template.name.clone(),
                template_file: template.filename.clone(),
                severity: severity_from_requirement(&template.requirement),
                status: GapStatus::Open,
                reason: "Document does not exist".to_string(),
                justification: String::new(),
            });
            continue;
        }

        // Check if it's a stub
        let full_path = Path::new(project_dir).join(&template.filename);
        if full_path.exists() && is_stub(&full_path.to_string_lossy()) {
            analysis.gaps.push(Gap {
                template_name: template.name.clone(),
                template_file: template.filename.clone(),
                severity: severity_from_requirement(&template.requirement),
                status: GapStatus::Open,
                reason: "Document is a stub (contains TODO markers or is too short)".to_string(),
                justification: String::new(),
            });
        }
    }

    analysis
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adoption::inventory::{Inventory, InventoryItem};
    use crate::adoption::templates::{Template, TemplateSet};

    #[test]
    fn test_analyze_gaps_missing_document() {
        let inventory = Inventory {
            project_dir: "/test".to_string(),
            items: vec![],
        };
        let ts = TemplateSet {
            templates: vec![Template::new(
                "code",
                "README",
                "Project overview",
                "README.md",
                RequirementLevel::R,
            )],
        };
        let result = analyze_gaps("/test", &inventory, &ts);
        assert_eq!(result.gaps.len(), 1);
        assert_eq!(result.gaps[0].severity, GapSeverity::Critical);
        assert_eq!(result.gaps[0].reason, "Document does not exist");
    }

    #[test]
    fn test_analyze_gaps_existing_document() {
        let inventory = Inventory {
            project_dir: "/test".to_string(),
            items: vec![InventoryItem {
                path: "README.md".to_string(),
                format: "markdown".to_string(),
                size_bytes: 0,
                last_modified: 0.0,
                quality: "usable".to_string(),
                notes: String::new(),
            }],
        };
        let ts = TemplateSet {
            templates: vec![Template::new(
                "code",
                "README",
                "Overview",
                "README.md",
                RequirementLevel::R,
            )],
        };
        // The file doesn't actually exist on disk, so is_stub won't trigger
        let result = analyze_gaps("/test", &inventory, &ts);
        assert!(result.gaps.is_empty());
    }

    #[test]
    fn test_analyze_gaps_stub_detection() {
        let dir = tempfile::tempdir().unwrap();
        let readme_path = dir.path().join("README.md");
        std::fs::write(&readme_path, "# TODO: Fill this in").unwrap();

        let inventory = Inventory {
            project_dir: dir.path().to_str().unwrap().to_string(),
            items: vec![InventoryItem {
                path: "README.md".to_string(),
                format: "markdown".to_string(),
                size_bytes: 21,
                last_modified: 0.0,
                quality: "usable".to_string(),
                notes: String::new(),
            }],
        };
        let ts = TemplateSet {
            templates: vec![Template::new(
                "code",
                "README",
                "Overview",
                "README.md",
                RequirementLevel::R,
            )],
        };
        let result = analyze_gaps(dir.path().to_str().unwrap(), &inventory, &ts);
        assert_eq!(result.gaps.len(), 1);
        assert!(result.gaps[0].reason.contains("stub"));
    }

    #[test]
    fn test_gap_analysis_to_markdown() {
        let analysis = GapAnalysis {
            project_dir: "/test".to_string(),
            gaps: vec![
                Gap {
                    template_name: "README".to_string(),
                    template_file: "README.md".to_string(),
                    severity: GapSeverity::Critical,
                    status: GapStatus::Open,
                    reason: "Missing".to_string(),
                    justification: String::new(),
                },
                Gap {
                    template_name: "LICENSE".to_string(),
                    template_file: "LICENSE".to_string(),
                    severity: GapSeverity::High,
                    status: GapStatus::Open,
                    reason: "Missing".to_string(),
                    justification: String::new(),
                },
            ],
        };
        let md = analysis.to_markdown();
        assert!(md.contains("# GAPS"));
        assert!(md.contains("**Total gaps:** 2 open"));
        assert!(md.contains("## Critical"));
        assert!(md.contains("README"));
    }

    #[test]
    fn test_gap_analysis_critical_filter() {
        let analysis = GapAnalysis {
            project_dir: "/test".to_string(),
            gaps: vec![
                Gap {
                    template_name: "A".to_string(),
                    template_file: "a".to_string(),
                    severity: GapSeverity::Critical,
                    status: GapStatus::Open,
                    reason: String::new(),
                    justification: String::new(),
                },
                Gap {
                    template_name: "B".to_string(),
                    template_file: "b".to_string(),
                    severity: GapSeverity::Low,
                    status: GapStatus::Open,
                    reason: String::new(),
                    justification: String::new(),
                },
            ],
        };
        assert_eq!(analysis.critical().len(), 1);
        assert_eq!(analysis.open_gaps().len(), 2);
    }
}
