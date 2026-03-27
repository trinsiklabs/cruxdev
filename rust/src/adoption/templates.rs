//! Template registry — all key document types with requirement levels.
//!
//! Maps project types and maturity levels to required document templates.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Requirement level: R=Required, P=Production, M=Mature, O=Optional
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequirementLevel {
    R, // Required
    P, // Production
    M, // Mature
    O, // Optional
}

impl RequirementLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R => "R",
            Self::P => "P",
            Self::M => "M",
            Self::O => "O",
        }
    }
}

/// A document template definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub category: String,
    pub name: String,
    pub description: String,
    pub filename: String,
    pub requirement: RequirementLevel,
}

impl Template {
    pub fn new(
        category: &str,
        name: &str,
        description: &str,
        filename: &str,
        requirement: RequirementLevel,
    ) -> Self {
        Self {
            category: category.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            filename: filename.to_string(),
            requirement,
        }
    }
}

/// A set of templates applicable to a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateSet {
    pub templates: Vec<Template>,
}

impl TemplateSet {
    pub fn required(&self) -> Vec<&Template> {
        self.templates
            .iter()
            .filter(|t| t.requirement == RequirementLevel::R)
            .collect()
    }

    pub fn production(&self) -> Vec<&Template> {
        self.templates
            .iter()
            .filter(|t| matches!(t.requirement, RequirementLevel::R | RequirementLevel::P))
            .collect()
    }

    pub fn by_category(&self) -> HashMap<String, Vec<&Template>> {
        let mut result: HashMap<String, Vec<&Template>> = HashMap::new();
        for t in &self.templates {
            result.entry(t.category.clone()).or_default().push(t);
        }
        result
    }
}

/// Master template registry.
pub fn templates() -> Vec<Template> {
    use RequirementLevel::*;
    vec![
        // Code templates
        Template::new("code", "README", "Project overview and setup", "README.md", R),
        Template::new("code", "CLAUDE.md", "AI assistant configuration", "CLAUDE.md", R),
        Template::new("code", "CHANGELOG", "Version history", "CHANGELOG.md", P),
        Template::new("code", "CONTRIBUTING", "Contribution guidelines", "CONTRIBUTING.md", M),
        Template::new("code", "LICENSE", "License file", "LICENSE", P),
        Template::new("code", "SECURITY", "Security policy", "SECURITY.md", M),
        // Business templates
        Template::new("business", "Business Plan", "Strategy and goals", "docs/BUSINESS_PLAN.md", R),
        Template::new("business", "Budget", "Financial planning", "docs/BUDGET.md", P),
        Template::new("business", "Operations", "Operational procedures", "docs/OPERATIONS.md", P),
        // Product templates
        Template::new("product", "Product Spec", "Feature specifications", "docs/PRODUCT_SPEC.md", R),
        Template::new("product", "User Stories", "User requirements", "docs/USER_STORIES.md", R),
        Template::new("product", "Roadmap", "Development roadmap", "docs/ROADMAP.md", P),
        // Website templates
        Template::new("website", "Deployment", "Deployment procedures", "docs/DEPLOYMENT.md", R),
        Template::new("website", "Website Config", "Site URL, hosting, domain", "docs/WEBSITE.md", R),
        Template::new("website", "SEO Strategy", "Search engine optimization", "docs/SEO_STRATEGY.md", P),
        // Research templates
        Template::new("research", "Research Plan", "Research methodology", "docs/RESEARCH_PLAN.md", R),
        Template::new("research", "Findings", "Research results", "docs/FINDINGS.md", R),
        // Governance templates
        Template::new("governance", "GAPS", "Gap analysis tracking", "GAPS.md", R),
        Template::new("governance", "Competitors", "Competitive analysis", "docs/COMPETITORS.md", P),
        Template::new("governance", "Architecture", "System architecture", "docs/ARCHITECTURE.md", P),
    ]
}

fn type_categories() -> HashMap<&'static str, Vec<&'static str>> {
    let mut m = HashMap::new();
    m.insert("software-existing", vec!["code", "governance"]);
    m.insert("software-greenfield", vec!["code", "governance"]);
    m.insert("business-existing", vec!["business", "governance"]);
    m.insert("business-new", vec!["business", "governance"]);
    m.insert("product-saas", vec!["code", "product", "governance"]);
    m.insert("website", vec!["code", "website", "governance"]);
    m.insert("infrastructure", vec!["code", "governance"]);
    m.insert("consulting-client", vec!["business", "governance"]);
    m.insert("research", vec!["research", "governance"]);
    m.insert("campaign", vec!["business", "governance"]);
    m
}

/// Get applicable templates for a project type and maturity.
pub fn get_templates_for_type(project_type: &str, maturity: &str) -> TemplateSet {
    let cats = type_categories();
    let categories = cats.get(project_type).cloned().unwrap_or(vec!["code", "governance"]);

    let all_templates = templates();
    let applicable: Vec<Template> = all_templates
        .into_iter()
        .filter(|t| {
            if !categories.contains(&t.category.as_str()) {
                return false;
            }
            match maturity {
                "idea" | "minimal" => t.requirement == RequirementLevel::R,
                "growing" => matches!(t.requirement, RequirementLevel::R | RequirementLevel::P),
                _ => true, // production and mature get everything
            }
        })
        .collect();

    TemplateSet { templates: applicable }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templates_count() {
        let t = templates();
        assert_eq!(t.len(), 20);
    }

    #[test]
    fn test_get_templates_for_software_minimal() {
        let ts = get_templates_for_type("software-existing", "minimal");
        // Should only get Required templates from code + governance
        assert!(ts.templates.iter().all(|t| t.requirement == RequirementLevel::R));
        let names: Vec<&str> = ts.templates.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"README"));
        assert!(names.contains(&"CLAUDE.md"));
        assert!(names.contains(&"GAPS"));
    }

    #[test]
    fn test_get_templates_for_website_growing() {
        let ts = get_templates_for_type("website", "growing");
        let names: Vec<&str> = ts.templates.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"Deployment"));
        assert!(names.contains(&"SEO Strategy")); // P-level, included at growing
    }

    #[test]
    fn test_template_set_required() {
        let ts = get_templates_for_type("software-existing", "production");
        let required = ts.required();
        assert!(required.iter().all(|t| t.requirement == RequirementLevel::R));
    }

    #[test]
    fn test_template_set_by_category() {
        let ts = get_templates_for_type("product-saas", "production");
        let by_cat = ts.by_category();
        assert!(by_cat.contains_key("code"));
        assert!(by_cat.contains_key("product"));
        assert!(by_cat.contains_key("governance"));
    }

    #[test]
    fn test_unknown_project_type_defaults() {
        let ts = get_templates_for_type("unknown-type", "minimal");
        // Should fall back to code + governance
        assert!(!ts.templates.is_empty());
    }
}
