//! Marketing plan schema — structured, serializable, convergence-ready.
//!
//! A MarketingPlan is a first-class CruxDev artifact: generated from templates,
//! validated by rules, iterable by the convergence engine.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A complete marketing plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingPlan {
    pub product: ProductInfo,
    pub audience_segments: Vec<AudienceSegment>,
    pub messaging: MessagingMatrix,
    pub channels: Vec<ChannelStrategy>,
    pub calendar: Vec<CalendarEntry>,
    pub timeline: Vec<Milestone>,
    pub kpis: Vec<KPI>,
    #[serde(default)]
    pub status: PlanStatus,
}

/// Product information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    pub name: String,
    pub product_type: ProductType,
    pub description: String,
    #[serde(default)]
    pub url: Option<String>,
}

/// Product type determines which template to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    Book,
    Course,
    CoachingProgram,
    Membership,
    General,
}

impl ProductType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "book" | "ebook" | "novel" | "fiction" | "nonfiction" => ProductType::Book,
            "course" | "workshop" | "class" | "tutorial" => ProductType::Course,
            "coaching" | "consulting" | "mentoring" => ProductType::CoachingProgram,
            "membership" | "subscription" | "community" | "club" => ProductType::Membership,
            _ => ProductType::General,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProductType::Book => "Book Launch",
            ProductType::Course => "Course Launch",
            ProductType::CoachingProgram => "Coaching Program",
            ProductType::Membership => "Membership",
            ProductType::General => "General",
        }
    }
}

/// An audience segment with pain points and messaging hooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudienceSegment {
    pub name: String,
    pub demographics: String,
    pub pain_points: Vec<String>,
    pub messaging_hooks: Vec<String>,
    pub preferred_channels: Vec<String>,
}

/// Messaging matrix — core message plus per-channel adaptations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagingMatrix {
    pub core_message: String,
    pub tagline: String,
    pub per_channel: HashMap<String, String>,
}

/// Strategy for a specific channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStrategy {
    pub channel: String,
    pub frequency: String,
    pub content_types: Vec<String>,
    pub best_times: String,
    #[serde(default)]
    pub audience_overlap: f64,
}

/// A calendar entry — one piece of content on one channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEntry {
    pub week: u32,
    pub day: String,
    pub channel: String,
    pub content_type: String,
    pub topic: String,
    #[serde(default)]
    pub status: EntryStatus,
}

/// Status of a calendar entry.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum EntryStatus {
    #[default]
    Planned,
    InProgress,
    Published,
    Skipped,
}

/// A milestone in the campaign timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub week: u32,
    pub dependencies: Vec<String>,
    pub success_criteria: String,
}

/// A KPI with target and measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPI {
    pub metric: String,
    pub target: String,
    pub measurement_method: String,
    #[serde(default)]
    pub current_value: Option<String>,
}

/// Plan status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum PlanStatus {
    #[default]
    Draft,
    Converging,
    Active,
    Completed,
    Archived,
}

impl MarketingPlan {
    /// Write the plan to a JSON file (atomic write).
    pub fn write_to(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialize plan: {e}"))?;

        let tmp_path = format!("{path}.tmp");
        fs::write(&tmp_path, &json)
            .map_err(|e| format!("Write tmp: {e}"))?;
        fs::rename(&tmp_path, path)
            .map_err(|e| format!("Rename: {e}"))?;

        Ok(())
    }

    /// Read a plan from a JSON file.
    pub fn read_from(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Read plan: {e}"))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Parse plan: {e}"))
    }

    /// Get a slug for file naming.
    pub fn slug(&self) -> String {
        self.product
            .name
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "-")
            .replace("--", "-")
            .trim_matches('-')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_plan() -> MarketingPlan {
        MarketingPlan {
            product: ProductInfo {
                name: "The Art of Vibecoding".into(),
                product_type: ProductType::Book,
                description: "A guide to building software with AI".into(),
                url: Some("https://example.com/book".into()),
            },
            audience_segments: vec![AudienceSegment {
                name: "Indie developers".into(),
                demographics: "25-45, technical, solo or small team".into(),
                pain_points: vec!["Too many tools".into(), "No process".into()],
                messaging_hooks: vec!["Ship faster with AI".into()],
                preferred_channels: vec!["X".into(), "LinkedIn".into()],
            }],
            messaging: MessagingMatrix {
                core_message: "Build better software, faster, with AI as your copilot".into(),
                tagline: "Vibecode your way to production".into(),
                per_channel: HashMap::from([
                    ("X".into(), "Ship 10x faster with vibecoding".into()),
                    ("LinkedIn".into(), "How AI is changing software development".into()),
                ]),
            },
            channels: vec![ChannelStrategy {
                channel: "X".into(),
                frequency: "3x/week".into(),
                content_types: vec!["thread".into(), "single".into()],
                best_times: "9am, 12pm, 5pm EST".into(),
                audience_overlap: 0.7,
            }],
            calendar: vec![
                CalendarEntry {
                    week: 1,
                    day: "Monday".into(),
                    channel: "X".into(),
                    content_type: "thread".into(),
                    topic: "Why I wrote this book".into(),
                    status: EntryStatus::Planned,
                },
                CalendarEntry {
                    week: 1,
                    day: "Wednesday".into(),
                    channel: "LinkedIn".into(),
                    content_type: "post".into(),
                    topic: "The problem with current dev tools".into(),
                    status: EntryStatus::Planned,
                },
            ],
            timeline: vec![Milestone {
                name: "Pre-launch teaser".into(),
                week: 1,
                dependencies: vec![],
                success_criteria: "100+ newsletter signups".into(),
            }],
            kpis: vec![KPI {
                metric: "Pre-orders".into(),
                target: "500".into(),
                measurement_method: "Stripe dashboard".into(),
                current_value: None,
            }],
            status: PlanStatus::Draft,
        }
    }

    #[test]
    fn test_serialization_roundtrip() {
        let plan = sample_plan();
        let json = serde_json::to_string(&plan).unwrap();
        let parsed: MarketingPlan = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.product.name, "The Art of Vibecoding");
        assert_eq!(parsed.audience_segments.len(), 1);
        assert_eq!(parsed.channels.len(), 1);
        assert_eq!(parsed.calendar.len(), 2);
    }

    #[test]
    fn test_product_type_from_str() {
        assert_eq!(ProductType::from_str("book"), ProductType::Book);
        assert_eq!(ProductType::from_str("ebook"), ProductType::Book);
        assert_eq!(ProductType::from_str("course"), ProductType::Course);
        assert_eq!(ProductType::from_str("coaching"), ProductType::CoachingProgram);
        assert_eq!(ProductType::from_str("membership"), ProductType::Membership);
        assert_eq!(ProductType::from_str("widget"), ProductType::General);
    }

    #[test]
    fn test_slug_generation() {
        let plan = sample_plan();
        assert_eq!(plan.slug(), "the-art-of-vibecoding");
    }

    #[test]
    fn test_write_and_read() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("plan.json");
        let path_str = path.to_str().unwrap();

        let plan = sample_plan();
        plan.write_to(path_str).unwrap();

        let loaded = MarketingPlan::read_from(path_str).unwrap();
        assert_eq!(loaded.product.name, plan.product.name);
        assert_eq!(loaded.kpis.len(), 1);
    }

    #[test]
    fn test_default_statuses() {
        let plan = sample_plan();
        assert_eq!(plan.status, PlanStatus::Draft);
        assert_eq!(plan.calendar[0].status, EntryStatus::Planned);
    }

    #[test]
    fn test_product_type_names() {
        assert_eq!(ProductType::Book.name(), "Book Launch");
        assert_eq!(ProductType::Course.name(), "Course Launch");
        assert_eq!(ProductType::Membership.name(), "Membership");
    }
}
