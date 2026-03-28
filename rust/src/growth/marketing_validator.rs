//! Marketing plan validation — structural and content checks.
//!
//! Every generated plan passes through validation before being marked ready.
//! Returns structured findings so the convergence engine can fix specific issues.

use super::marketing_plan::*;

/// A validation finding.
#[derive(Debug, Clone)]
pub struct ValidationFinding {
    pub severity: Severity,
    pub rule: &'static str,
    pub message: String,
}

/// Finding severity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

/// Validate a marketing plan. Returns all findings.
pub fn validate(plan: &MarketingPlan) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    check_audience_segments(plan, &mut findings);
    check_messaging(plan, &mut findings);
    check_channels(plan, &mut findings);
    check_calendar(plan, &mut findings);
    check_kpis(plan, &mut findings);

    findings
}

/// Check that the plan passes validation (no errors).
pub fn is_valid(plan: &MarketingPlan) -> bool {
    validate(plan)
        .iter()
        .all(|f| f.severity != Severity::Error)
}

/// Every audience segment must have at least one messaging hook.
fn check_audience_segments(plan: &MarketingPlan, findings: &mut Vec<ValidationFinding>) {
    if plan.audience_segments.is_empty() {
        findings.push(ValidationFinding {
            severity: Severity::Error,
            rule: "audience_required",
            message: "Plan has no audience segments".into(),
        });
        return;
    }

    for segment in &plan.audience_segments {
        if segment.messaging_hooks.is_empty() {
            findings.push(ValidationFinding {
                severity: Severity::Error,
                rule: "audience_hooks_required",
                message: format!("Audience '{}' has no messaging hooks", segment.name),
            });
        }

        if segment.pain_points.is_empty() {
            findings.push(ValidationFinding {
                severity: Severity::Warning,
                rule: "audience_pain_points",
                message: format!("Audience '{}' has no pain points defined", segment.name),
            });
        }
    }
}

/// Core message and tagline must not be empty.
fn check_messaging(plan: &MarketingPlan, findings: &mut Vec<ValidationFinding>) {
    if plan.messaging.core_message.is_empty() {
        findings.push(ValidationFinding {
            severity: Severity::Error,
            rule: "core_message_required",
            message: "Core message is empty".into(),
        });
    }

    if plan.messaging.tagline.is_empty() {
        findings.push(ValidationFinding {
            severity: Severity::Warning,
            rule: "tagline_recommended",
            message: "Tagline is empty".into(),
        });
    }
}

/// Channel strategy must cover all channels mentioned in calendar.
fn check_channels(plan: &MarketingPlan, findings: &mut Vec<ValidationFinding>) {
    let strategy_channels: Vec<&str> = plan.channels.iter().map(|c| c.channel.as_str()).collect();

    for entry in &plan.calendar {
        if !strategy_channels.contains(&entry.channel.as_str()) {
            findings.push(ValidationFinding {
                severity: Severity::Error,
                rule: "calendar_channel_in_strategy",
                message: format!(
                    "Calendar entry for '{}' week {} but no channel strategy defined",
                    entry.channel, entry.week
                ),
            });
        }
    }
}

/// Calendar must not have gaps longer than 7 days during the campaign.
fn check_calendar(plan: &MarketingPlan, findings: &mut Vec<ValidationFinding>) {
    if plan.calendar.is_empty() {
        findings.push(ValidationFinding {
            severity: Severity::Error,
            rule: "calendar_required",
            message: "Plan has no calendar entries".into(),
        });
        return;
    }

    // Check for week gaps
    let mut weeks: Vec<u32> = plan.calendar.iter().map(|e| e.week).collect();
    weeks.sort();
    weeks.dedup();

    if weeks.len() > 1 {
        for window in weeks.windows(2) {
            if window[1] - window[0] > 1 {
                findings.push(ValidationFinding {
                    severity: Severity::Warning,
                    rule: "calendar_no_gaps",
                    message: format!(
                        "Calendar gap: no entries between week {} and week {}",
                        window[0], window[1]
                    ),
                });
            }
        }
    }

    // Check all entries have a topic
    for entry in &plan.calendar {
        if entry.topic.is_empty() {
            findings.push(ValidationFinding {
                severity: Severity::Warning,
                rule: "calendar_topics_filled",
                message: format!("Calendar entry week {} {} has no topic", entry.week, entry.channel),
            });
        }
    }
}

/// KPIs must have measurable targets.
fn check_kpis(plan: &MarketingPlan, findings: &mut Vec<ValidationFinding>) {
    if plan.kpis.is_empty() {
        findings.push(ValidationFinding {
            severity: Severity::Error,
            rule: "kpis_required",
            message: "Plan has no KPIs".into(),
        });
        return;
    }

    let vague_targets = ["tbd", "to be defined", "tba", "n/a", ""];
    for kpi in &plan.kpis {
        if vague_targets.contains(&kpi.target.to_lowercase().as_str()) {
            findings.push(ValidationFinding {
                severity: Severity::Error,
                rule: "kpi_measurable",
                message: format!("KPI '{}' has vague target: '{}'", kpi.metric, kpi.target),
            });
        }

        if kpi.measurement_method.is_empty() {
            findings.push(ValidationFinding {
                severity: Severity::Warning,
                rule: "kpi_measurement_method",
                message: format!("KPI '{}' has no measurement method", kpi.metric),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::marketing_templates::template_for;
    use std::collections::HashMap;

    fn valid_plan() -> MarketingPlan {
        let mut plan = template_for(ProductType::Book);
        plan.product.name = "Test Book".into();
        plan.messaging.core_message = "A great message".into();
        plan.messaging.tagline = "Catchy tagline".into();
        plan
    }

    #[test]
    fn test_valid_plan_passes() {
        let plan = valid_plan();
        let findings = validate(&plan);
        let errors: Vec<_> = findings.iter().filter(|f| f.severity == Severity::Error).collect();
        assert!(errors.is_empty(), "Valid plan should have no errors, got: {:?}", errors.iter().map(|f| &f.message).collect::<Vec<_>>());
    }

    #[test]
    fn test_empty_audience_is_error() {
        let mut plan = valid_plan();
        plan.audience_segments.clear();
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "audience_required"));
    }

    #[test]
    fn test_no_messaging_hooks_is_error() {
        let mut plan = valid_plan();
        plan.audience_segments[0].messaging_hooks.clear();
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "audience_hooks_required"));
    }

    #[test]
    fn test_empty_core_message_is_error() {
        let mut plan = valid_plan();
        plan.messaging.core_message.clear();
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "core_message_required"));
    }

    #[test]
    fn test_calendar_channel_not_in_strategy() {
        let mut plan = valid_plan();
        plan.calendar.push(CalendarEntry {
            week: 1,
            day: "Monday".into(),
            channel: "TikTok".into(), // not in strategy
            content_type: "video".into(),
            topic: "Test".into(),
            status: EntryStatus::Planned,
        });
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "calendar_channel_in_strategy"));
    }

    #[test]
    fn test_vague_kpi_target_is_error() {
        let mut plan = valid_plan();
        plan.kpis[0].target = "TBD".into();
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "kpi_measurable"));
    }

    #[test]
    fn test_empty_calendar_is_error() {
        let mut plan = valid_plan();
        plan.calendar.clear();
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "calendar_required"));
    }

    #[test]
    fn test_calendar_gap_is_warning() {
        let mut plan = valid_plan();
        plan.calendar = vec![
            CalendarEntry { week: 1, day: "Mon".into(), channel: "X".into(), content_type: "post".into(), topic: "A".into(), status: EntryStatus::Planned },
            CalendarEntry { week: 5, day: "Mon".into(), channel: "X".into(), content_type: "post".into(), topic: "B".into(), status: EntryStatus::Planned },
        ];
        // Add X to channels
        plan.channels = vec![ChannelStrategy {
            channel: "X".into(),
            frequency: "1x/week".into(),
            content_types: vec!["post".into()],
            best_times: "9am".into(),
            audience_overlap: 0.0,
        }];
        let findings = validate(&plan);
        assert!(findings.iter().any(|f| f.rule == "calendar_no_gaps"));
    }

    #[test]
    fn test_all_templates_pass_validation_with_messaging() {
        for product_type in &[
            ProductType::Book,
            ProductType::Course,
            ProductType::CoachingProgram,
            ProductType::Membership,
            ProductType::General,
        ] {
            let mut plan = template_for(*product_type);
            plan.messaging.core_message = "Test message".into();
            plan.messaging.tagline = "Test tagline".into();
            let findings = validate(&plan);
            let errors: Vec<_> = findings.iter().filter(|f| f.severity == Severity::Error).collect();
            assert!(
                errors.is_empty(),
                "{:?} template has validation errors: {:?}",
                product_type,
                errors.iter().map(|f| &f.message).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_is_valid_helper() {
        let plan = valid_plan();
        assert!(is_valid(&plan));

        let mut bad_plan = valid_plan();
        bad_plan.audience_segments.clear();
        assert!(!is_valid(&bad_plan));
    }
}
