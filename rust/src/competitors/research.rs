//! Structured competitor profiling — deep research on a single competitor.
//!
//! Produces a CompetitorProfile with pricing, features, strengths, weaknesses,
//! tech stack, and differentiation.

use serde::{Deserialize, Serialize};

/// Moat type categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MoatType {
    NetworkEffects,
    SwitchingCosts,
    Brand,
    DataFlywheel,
    Regulatory,
    ExecutionSpeed,
    CostAdvantage,
}

impl MoatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NetworkEffects => "network_effects",
            Self::SwitchingCosts => "switching_costs",
            Self::Brand => "brand",
            Self::DataFlywheel => "data_flywheel",
            Self::Regulatory => "regulatory",
            Self::ExecutionSpeed => "execution_speed",
            Self::CostAdvantage => "cost_advantage",
        }
    }
}

/// A single feature of a competitor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub has_feature: bool,
    pub notes: String,
}

impl Feature {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            has_feature: true,
            notes: String::new(),
        }
    }
}

/// Moat assessment for a single moat type. Score 0-3.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoatScore {
    pub moat_type: MoatType,
    /// 0=none, 1=weak, 2=moderate, 3=strong
    pub score: u8,
    pub evidence: String,
}

/// Threat assessment for a competitor. Each dimension 1-5.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreatAssessment {
    pub market_overlap: u8,
    pub growth_velocity: u8,
    pub resource_asymmetry: u8,
    pub technical_proximity: u8,
    pub time_to_relevance_months: u32,
}

impl Default for ThreatAssessment {
    fn default() -> Self {
        Self {
            market_overlap: 1,
            growth_velocity: 1,
            resource_asymmetry: 1,
            technical_proximity: 1,
            time_to_relevance_months: 24,
        }
    }
}

impl ThreatAssessment {
    /// Average of the four scored dimensions.
    pub fn threat_score(&self) -> f64 {
        (self.market_overlap as f64
            + self.growth_velocity as f64
            + self.resource_asymmetry as f64
            + self.technical_proximity as f64)
            / 4.0
    }

    pub fn threat_level(&self) -> &'static str {
        let s = self.threat_score();
        if s >= 4.0 {
            "existential"
        } else if s >= 3.0 {
            "significant"
        } else if s >= 2.0 {
            "moderate"
        } else {
            "low"
        }
    }
}

/// Competitor category.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompetitorCategory {
    Official,
    Watch,
    #[default]
    Noted,
}

impl CompetitorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Watch => "watch",
            Self::Noted => "noted",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "official" => Self::Official,
            "watch" => Self::Watch,
            _ => Self::Noted,
        }
    }
}

/// Structured profile of a single competitor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorProfile {
    pub name: String,
    pub url: String,
    pub tagline: String,
    pub description: String,
    pub category: CompetitorCategory,
    pub pricing: String,
    pub revenue_model: String,
    pub tech_stack: Vec<String>,
    pub features: Vec<Feature>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub differentiation: String,
    pub last_researched: String,
    pub moats: Vec<MoatScore>,
    pub threat: ThreatAssessment,
    pub funding: String,
    pub growth_signals: Vec<String>,
    #[serde(default)]
    pub integrations: Vec<Integration>,
}

/// A platform/tool integration for a competitor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Integration {
    pub name: String,
    pub depth: IntegrationDepth,
    pub description: String,
}

/// How deep the integration is.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationDepth {
    Native,    // Built into the core product
    Plugin,    // Official or first-party plugin
    Api,       // REST/GraphQL API integration
    Community, // Third-party / community-maintained
}

impl IntegrationDepth {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Plugin => "plugin",
            Self::Api => "api",
            Self::Community => "community",
        }
    }
}

impl CompetitorProfile {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            tagline: String::new(),
            description: String::new(),
            category: CompetitorCategory::default(),
            pricing: String::new(),
            revenue_model: String::new(),
            tech_stack: Vec::new(),
            features: Vec::new(),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            differentiation: String::new(),
            last_researched: String::new(),
            moats: Vec::new(),
            threat: ThreatAssessment::default(),
            funding: String::new(),
            growth_signals: Vec::new(),
            integrations: Vec::new(),
        }
    }

    /// Get list of feature names this competitor has.
    pub fn feature_names(&self) -> Vec<String> {
        self.features
            .iter()
            .filter(|f| f.has_feature)
            .map(|f| f.name.clone())
            .collect()
    }

    /// Render as markdown section for COMPETITORS.md.
    pub fn to_markdown(&self) -> String {
        let mut lines = vec![format!("### {}", self.name)];
        lines.push(format!("**URL:** {}", self.url));
        if !self.tagline.is_empty() {
            lines.push(format!("**Tagline:** {}", self.tagline));
        }
        lines.push(format!("**Category:** {}", self.category.as_str()));
        if !self.pricing.is_empty() {
            lines.push(format!("**Pricing:** {}", self.pricing));
        }
        if !self.description.is_empty() {
            lines.push(String::new());
            lines.push(self.description.clone());
        }
        if !self.tech_stack.is_empty() {
            lines.push(String::new());
            lines.push(format!("**Tech Stack:** {}", self.tech_stack.join(", ")));
        }
        if !self.strengths.is_empty() {
            lines.push(String::new());
            lines.push("**Strengths:**".to_string());
            for s in &self.strengths {
                lines.push(format!("- {s}"));
            }
        }
        if !self.weaknesses.is_empty() {
            lines.push(String::new());
            lines.push("**Weaknesses:**".to_string());
            for w in &self.weaknesses {
                lines.push(format!("- {w}"));
            }
        }
        if !self.differentiation.is_empty() {
            lines.push(String::new());
            lines.push(format!("**Differentiation:** {}", self.differentiation));
        }
        if !self.revenue_model.is_empty() {
            lines.push(format!("**Revenue Model:** {}", self.revenue_model));
        }
        if !self.funding.is_empty() {
            lines.push(format!("**Funding:** {}", self.funding));
        }
        if !self.moats.is_empty() {
            lines.push(String::new());
            lines.push("**Moat Analysis:**".to_string());
            let labels = ["none", "weak", "moderate", "strong"];
            for m in &self.moats {
                let label = labels.get(m.score as usize).unwrap_or(&"none");
                lines.push(format!("- {}: {label}", m.moat_type.as_str()));
                if !m.evidence.is_empty() {
                    lines.push(format!("  - {}", m.evidence));
                }
            }
        }
        if self.threat.threat_score() > 1.0 {
            lines.push(String::new());
            lines.push(format!(
                "**Threat Level:** {} ({:.1}/5)",
                self.threat.threat_level(),
                self.threat.threat_score()
            ));
            if self.threat.time_to_relevance_months < 24 {
                lines.push(format!(
                    "**Time to Relevance:** {} months",
                    self.threat.time_to_relevance_months
                ));
            }
        }
        if !self.growth_signals.is_empty() {
            lines.push(String::new());
            lines.push("**Growth Signals:**".to_string());
            for g in &self.growth_signals {
                lines.push(format!("- {g}"));
            }
        }
        if !self.integrations.is_empty() {
            lines.push(String::new());
            lines.push("**Integrations:**".to_string());
            for i in &self.integrations {
                lines.push(format!("- {} ({}): {}", i.name, i.depth.as_str(), i.description));
            }
        }
        lines.join("\n")
    }
}

/// Parse an LLM research response into a structured CompetitorProfile.
pub fn parse_profile_response(name: &str, url: &str, response_text: &str) -> CompetitorProfile {
    let mut profile = CompetitorProfile::new(name, url);
    let lines: Vec<&str> = response_text.trim().split('\n').collect();

    let mut current_section = String::new();

    for line in &lines {
        let stripped = line.trim();
        let lower = stripped.to_lowercase();

        // Detect field lines
        if lower.starts_with("tagline:") {
            profile.tagline = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }
        if lower.starts_with("description:") {
            profile.description = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }
        if lower.starts_with("pricing:") {
            profile.pricing = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }
        if lower.starts_with("category:") {
            let val = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_lowercase();
            if ["official", "watch", "noted"].contains(&val.as_str()) {
                profile.category = CompetitorCategory::from_str_loose(&val);
            }
            continue;
        }
        if lower.starts_with("differentiation:") {
            profile.differentiation = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }
        if lower.starts_with("revenue model:") {
            profile.revenue_model = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }
        if lower.starts_with("funding:") {
            profile.funding = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            continue;
        }

        // Section headers
        if lower == "strengths:" || lower == "strengths" {
            current_section = "strengths".to_string();
            continue;
        }
        if lower == "weaknesses:" || lower == "weaknesses" {
            current_section = "weaknesses".to_string();
            continue;
        }
        if lower.starts_with("tech stack:") || lower == "tech stack" {
            if stripped.contains(':') {
                let val = stripped.split_once(':').map(|x| x.1).unwrap_or("").trim();
                if !val.is_empty() {
                    profile.tech_stack = val.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
                }
            }
            current_section = "tech_stack".to_string();
            continue;
        }
        if lower == "features:" || lower == "features" {
            current_section = "features".to_string();
            continue;
        }

        // Parse list items under sections
        if let Some(after_dash) = stripped.strip_prefix("- ") {
            let item = after_dash.trim();
            match current_section.as_str() {
                "strengths" => profile.strengths.push(item.to_string()),
                "weaknesses" => profile.weaknesses.push(item.to_string()),
                "tech_stack" => profile.tech_stack.push(item.to_string()),
                "features" => {
                    if item.contains(':') {
                        let parts: Vec<&str> = item.splitn(2, ':').collect();
                        profile.features.push(Feature::new(
                            parts[0].trim(),
                            parts.get(1).unwrap_or(&"").trim(),
                        ));
                    } else {
                        profile.features.push(Feature::new(item, ""));
                    }
                }
                _ => {}
            }
        }
    }

    profile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_names() {
        let mut profile = CompetitorProfile::new("Test", "https://test.com");
        profile.features.push(Feature {
            name: "AutoComplete".to_string(),
            description: "code completion".to_string(),
            has_feature: true,
            notes: String::new(),
        });
        profile.features.push(Feature {
            name: "Debugging".to_string(),
            description: "debug support".to_string(),
            has_feature: false,
            notes: String::new(),
        });
        let names = profile.feature_names();
        assert_eq!(names, vec!["AutoComplete"]);
    }

    #[test]
    fn test_threat_assessment_score() {
        let threat = ThreatAssessment {
            market_overlap: 4,
            growth_velocity: 4,
            resource_asymmetry: 4,
            technical_proximity: 4,
            time_to_relevance_months: 6,
        };
        assert_eq!(threat.threat_score(), 4.0);
        assert_eq!(threat.threat_level(), "existential");
    }

    #[test]
    fn test_threat_levels() {
        let low = ThreatAssessment::default();
        assert_eq!(low.threat_level(), "low");

        let moderate = ThreatAssessment { market_overlap: 2, growth_velocity: 2, resource_asymmetry: 2, technical_proximity: 2, ..Default::default() };
        assert_eq!(moderate.threat_level(), "moderate");

        let significant = ThreatAssessment { market_overlap: 3, growth_velocity: 3, resource_asymmetry: 3, technical_proximity: 3, ..Default::default() };
        assert_eq!(significant.threat_level(), "significant");
    }

    #[test]
    fn test_to_markdown_basic() {
        let mut profile = CompetitorProfile::new("Cursor", "https://cursor.sh");
        profile.tagline = "The AI Code Editor".to_string();
        profile.pricing = "$20/mo".to_string();
        let md = profile.to_markdown();
        assert!(md.contains("### Cursor"));
        assert!(md.contains("**URL:** https://cursor.sh"));
        assert!(md.contains("**Tagline:** The AI Code Editor"));
        assert!(md.contains("**Pricing:** $20/mo"));
    }

    #[test]
    fn test_parse_profile_response() {
        let response = "Tagline: The AI Code Editor\nDescription: An AI-first code editor\nPricing: $20/mo\nStrengths:\n- Fast\n- Smart completions\nWeaknesses:\n- Expensive\n";
        let profile = parse_profile_response("Cursor", "https://cursor.sh", response);
        assert_eq!(profile.tagline, "The AI Code Editor");
        assert_eq!(profile.description, "An AI-first code editor");
        assert_eq!(profile.pricing, "$20/mo");
        assert_eq!(profile.strengths, vec!["Fast", "Smart completions"]);
        assert_eq!(profile.weaknesses, vec!["Expensive"]);
    }

    #[test]
    fn test_integrations_in_profile() {
        let mut profile = CompetitorProfile::new("Test", "https://test.com");
        profile.integrations.push(Integration {
            name: "VSCode".into(),
            depth: IntegrationDepth::Native,
            description: "Built-in extension".into(),
        });
        profile.integrations.push(Integration {
            name: "Slack".into(),
            depth: IntegrationDepth::Api,
            description: "Webhook notifications".into(),
        });
        let md = profile.to_markdown();
        assert!(md.contains("**Integrations:**"));
        assert!(md.contains("VSCode (native)"));
        assert!(md.contains("Slack (api)"));
    }

    #[test]
    fn test_integration_serde() {
        let i = Integration {
            name: "GitHub".into(),
            depth: IntegrationDepth::Plugin,
            description: "PR integration".into(),
        };
        let json = serde_json::to_string(&i).unwrap();
        let parsed: Integration = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "GitHub");
        assert_eq!(parsed.depth, IntegrationDepth::Plugin);
    }

    #[test]
    fn test_parse_profile_with_features_and_tech_stack() {
        let response = "Tech Stack: Rust, TypeScript, Electron\nFeatures:\n- Autocomplete: AI code completion\n- Chat: Ask questions\n";
        let profile = parse_profile_response("Test", "https://test.com", response);
        assert_eq!(profile.tech_stack, vec!["Rust", "TypeScript", "Electron"]);
        assert_eq!(profile.features.len(), 2);
        assert_eq!(profile.features[0].name, "Autocomplete");
        assert_eq!(profile.features[0].description, "AI code completion");
    }
}
