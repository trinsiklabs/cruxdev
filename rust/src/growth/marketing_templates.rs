//! Marketing plan templates by product type.
//!
//! Each template pre-populates audience segments, channels, calendar skeleton,
//! and KPIs appropriate for the product type. LLM fills in product-specific details.

use super::marketing_plan::*;
use std::collections::HashMap;

/// Generate a template plan for the given product type.
pub fn template_for(product_type: ProductType) -> MarketingPlan {
    match product_type {
        ProductType::Book => book_launch_template(),
        ProductType::Course => course_launch_template(),
        ProductType::CoachingProgram => coaching_template(),
        ProductType::Membership => membership_template(),
        ProductType::General => general_template(),
    }
}

/// Book launch: 12-week arc (4 pre-launch, 1 launch, 7 post-launch).
fn book_launch_template() -> MarketingPlan {
    MarketingPlan {
        product: ProductInfo {
            name: String::new(),
            product_type: ProductType::Book,
            description: String::new(),
            url: None,
        },
        audience_segments: vec![
            AudienceSegment {
                name: "Existing readers".into(),
                demographics: "Fans of the genre/topic".into(),
                pain_points: vec!["Want more content from this author".into()],
                messaging_hooks: vec!["The next chapter in the journey".into()],
                preferred_channels: vec!["Newsletter".into(), "X".into()],
            },
            AudienceSegment {
                name: "New discovery".into(),
                demographics: "Genre/topic enthusiasts who haven't found this author".into(),
                pain_points: vec!["Looking for fresh voices".into()],
                messaging_hooks: vec!["If you liked X, you'll love this".into()],
                preferred_channels: vec!["Instagram".into(), "TikTok".into(), "Medium".into()],
            },
        ],
        messaging: MessagingMatrix {
            core_message: String::new(),
            tagline: String::new(),
            per_channel: HashMap::new(),
        },
        channels: vec![
            ChannelStrategy {
                channel: "Newsletter".into(),
                frequency: "1x/week".into(),
                content_types: vec!["behind-the-scenes".into(), "excerpt".into(), "launch announcement".into()],
                best_times: "Tuesday 10am".into(),
                audience_overlap: 0.0,
            },
            ChannelStrategy {
                channel: "X".into(),
                frequency: "3x/week".into(),
                content_types: vec!["thread".into(), "quote".into(), "countdown".into()],
                best_times: "9am, 12pm, 5pm".into(),
                audience_overlap: 0.0,
            },
            ChannelStrategy {
                channel: "Instagram".into(),
                frequency: "2x/week".into(),
                content_types: vec!["cover reveal".into(), "quote card".into(), "carousel".into()],
                best_times: "11am, 7pm".into(),
                audience_overlap: 0.0,
            },
        ],
        calendar: generate_calendar_skeleton(12, &["Newsletter", "X", "Instagram"]),
        timeline: vec![
            Milestone { name: "Cover reveal".into(), week: 2, dependencies: vec![], success_criteria: "500+ impressions".into() },
            Milestone { name: "ARC distribution".into(), week: 3, dependencies: vec!["Cover reveal".into()], success_criteria: "20+ ARC readers".into() },
            Milestone { name: "Launch day".into(), week: 5, dependencies: vec!["ARC distribution".into()], success_criteria: "50+ day-one sales".into() },
            Milestone { name: "First reviews live".into(), week: 7, dependencies: vec!["Launch day".into()], success_criteria: "10+ reviews".into() },
        ],
        kpis: vec![
            KPI { metric: "Pre-orders".into(), target: "100".into(), measurement_method: "Sales platform dashboard".into(), current_value: None },
            KPI { metric: "Launch week sales".into(), target: "200".into(), measurement_method: "Sales platform dashboard".into(), current_value: None },
            KPI { metric: "Newsletter signups".into(), target: "500".into(), measurement_method: "Email provider".into(), current_value: None },
            KPI { metric: "Reviews".into(), target: "25".into(), measurement_method: "Amazon/Goodreads".into(), current_value: None },
        ],
        status: PlanStatus::Draft,
    }
}

/// Course launch: 8-week arc (3 pre-launch, 1 launch, 4 post-launch).
fn course_launch_template() -> MarketingPlan {
    MarketingPlan {
        product: ProductInfo {
            name: String::new(),
            product_type: ProductType::Course,
            description: String::new(),
            url: None,
        },
        audience_segments: vec![
            AudienceSegment {
                name: "Career changers".into(),
                demographics: "Professionals looking to upskill".into(),
                pain_points: vec!["Need structured learning path".into(), "Overwhelmed by free resources".into()],
                messaging_hooks: vec!["From zero to competent in 8 weeks".into()],
                preferred_channels: vec!["LinkedIn".into(), "YouTube".into()],
            },
            AudienceSegment {
                name: "Hobbyists".into(),
                demographics: "Enthusiasts wanting to go deeper".into(),
                pain_points: vec!["Hit a plateau".into(), "No feedback loop".into()],
                messaging_hooks: vec!["Break through your plateau".into()],
                preferred_channels: vec!["X".into(), "Instagram".into()],
            },
        ],
        messaging: MessagingMatrix {
            core_message: String::new(),
            tagline: String::new(),
            per_channel: HashMap::new(),
        },
        channels: vec![
            ChannelStrategy {
                channel: "LinkedIn".into(),
                frequency: "3x/week".into(),
                content_types: vec!["insight".into(), "student story".into(), "lesson preview".into()],
                best_times: "8am, 12pm".into(),
                audience_overlap: 0.0,
            },
            ChannelStrategy {
                channel: "YouTube".into(),
                frequency: "1x/week".into(),
                content_types: vec!["free lesson".into(), "student interview".into()],
                best_times: "Saturday 10am".into(),
                audience_overlap: 0.0,
            },
        ],
        calendar: generate_calendar_skeleton(8, &["LinkedIn", "YouTube"]),
        timeline: vec![
            Milestone { name: "Waitlist open".into(), week: 1, dependencies: vec![], success_criteria: "200+ signups".into() },
            Milestone { name: "Free preview lesson".into(), week: 2, dependencies: vec![], success_criteria: "1000+ views".into() },
            Milestone { name: "Cart open".into(), week: 4, dependencies: vec!["Waitlist open".into()], success_criteria: "50+ enrollments".into() },
            Milestone { name: "Cart close".into(), week: 5, dependencies: vec!["Cart open".into()], success_criteria: "100+ total enrollments".into() },
        ],
        kpis: vec![
            KPI { metric: "Waitlist signups".into(), target: "500".into(), measurement_method: "Email provider".into(), current_value: None },
            KPI { metric: "Enrollments".into(), target: "100".into(), measurement_method: "Course platform".into(), current_value: None },
            KPI { metric: "Completion rate".into(), target: "60%".into(), measurement_method: "Course platform analytics".into(), current_value: None },
        ],
        status: PlanStatus::Draft,
    }
}

/// Coaching program: ongoing with quarterly campaigns.
fn coaching_template() -> MarketingPlan {
    MarketingPlan {
        product: ProductInfo {
            name: String::new(),
            product_type: ProductType::CoachingProgram,
            description: String::new(),
            url: None,
        },
        audience_segments: vec![AudienceSegment {
            name: "High-intent prospects".into(),
            demographics: "Professionals willing to invest in personal growth".into(),
            pain_points: vec!["Stuck in career".into(), "Need accountability".into()],
            messaging_hooks: vec!["Personalized guidance to your next level".into()],
            preferred_channels: vec!["LinkedIn".into(), "Newsletter".into()],
        }],
        messaging: MessagingMatrix {
            core_message: String::new(),
            tagline: String::new(),
            per_channel: HashMap::new(),
        },
        channels: vec![ChannelStrategy {
            channel: "LinkedIn".into(),
            frequency: "4x/week".into(),
            content_types: vec!["insight".into(), "client win".into(), "question".into(), "framework".into()],
            best_times: "7am, 12pm".into(),
            audience_overlap: 0.0,
        }],
        calendar: generate_calendar_skeleton(12, &["LinkedIn"]),
        timeline: vec![
            Milestone { name: "Discovery calls booked".into(), week: 2, dependencies: vec![], success_criteria: "10+ calls".into() },
            Milestone { name: "First cohort full".into(), week: 6, dependencies: vec![], success_criteria: "5+ clients".into() },
        ],
        kpis: vec![
            KPI { metric: "Discovery calls".into(), target: "20/month".into(), measurement_method: "Calendar".into(), current_value: None },
            KPI { metric: "Conversion rate".into(), target: "25%".into(), measurement_method: "CRM".into(), current_value: None },
        ],
        status: PlanStatus::Draft,
    }
}

/// Membership: evergreen funnel with seasonal pushes.
fn membership_template() -> MarketingPlan {
    MarketingPlan {
        product: ProductInfo {
            name: String::new(),
            product_type: ProductType::Membership,
            description: String::new(),
            url: None,
        },
        audience_segments: vec![AudienceSegment {
            name: "Community seekers".into(),
            demographics: "People wanting connection and ongoing value".into(),
            pain_points: vec!["Isolated in their journey".into(), "Need ongoing support".into()],
            messaging_hooks: vec!["Join a community of people like you".into()],
            preferred_channels: vec!["X".into(), "Instagram".into(), "Ko-Fi".into()],
        }],
        messaging: MessagingMatrix {
            core_message: String::new(),
            tagline: String::new(),
            per_channel: HashMap::new(),
        },
        channels: vec![
            ChannelStrategy {
                channel: "X".into(),
                frequency: "Daily".into(),
                content_types: vec!["member highlight".into(), "community moment".into(), "value preview".into()],
                best_times: "9am, 5pm".into(),
                audience_overlap: 0.0,
            },
            ChannelStrategy {
                channel: "Ko-Fi".into(),
                frequency: "2x/week".into(),
                content_types: vec!["behind-the-scenes".into(), "exclusive preview".into()],
                best_times: "Any".into(),
                audience_overlap: 0.0,
            },
        ],
        calendar: generate_calendar_skeleton(12, &["X", "Ko-Fi"]),
        timeline: vec![
            Milestone { name: "Founding members drive".into(), week: 1, dependencies: vec![], success_criteria: "50 members".into() },
            Milestone { name: "First community event".into(), week: 4, dependencies: vec![], success_criteria: "30+ attendees".into() },
        ],
        kpis: vec![
            KPI { metric: "Active members".into(), target: "200".into(), measurement_method: "Platform dashboard".into(), current_value: None },
            KPI { metric: "Monthly churn".into(), target: "<5%".into(), measurement_method: "Subscription metrics".into(), current_value: None },
            KPI { metric: "MRR".into(), target: "$2000".into(), measurement_method: "Stripe".into(), current_value: None },
        ],
        status: PlanStatus::Draft,
    }
}

/// General: flexible 4-12 week framework.
fn general_template() -> MarketingPlan {
    MarketingPlan {
        product: ProductInfo {
            name: String::new(),
            product_type: ProductType::General,
            description: String::new(),
            url: None,
        },
        audience_segments: vec![AudienceSegment {
            name: "Primary audience".into(),
            demographics: "To be defined".into(),
            pain_points: vec!["To be defined".into()],
            messaging_hooks: vec!["To be defined".into()],
            preferred_channels: vec!["X".into()],
        }],
        messaging: MessagingMatrix {
            core_message: String::new(),
            tagline: String::new(),
            per_channel: HashMap::new(),
        },
        channels: vec![ChannelStrategy {
            channel: "X".into(),
            frequency: "3x/week".into(),
            content_types: vec!["post".into()],
            best_times: "9am, 12pm, 5pm".into(),
            audience_overlap: 0.0,
        }],
        calendar: generate_calendar_skeleton(8, &["X"]),
        timeline: vec![
            Milestone { name: "Soft launch".into(), week: 2, dependencies: vec![], success_criteria: "Initial traction".into() },
            Milestone { name: "Full launch".into(), week: 4, dependencies: vec!["Soft launch".into()], success_criteria: "Growth target met".into() },
        ],
        kpis: vec![
            KPI { metric: "Reach".into(), target: "1000".into(), measurement_method: "Platform analytics".into(), current_value: None },
            KPI { metric: "Conversions".into(), target: "50".into(), measurement_method: "Sales dashboard".into(), current_value: None },
        ],
        status: PlanStatus::Draft,
    }
}

/// Generate a skeleton calendar with one entry per channel per week.
fn generate_calendar_skeleton(weeks: u32, channels: &[&str]) -> Vec<CalendarEntry> {
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let mut entries = Vec::new();

    for week in 1..=weeks {
        for (i, channel) in channels.iter().enumerate() {
            let day = days[i % days.len()];
            entries.push(CalendarEntry {
                week,
                day: day.to_string(),
                channel: channel.to_string(),
                content_type: "post".into(),
                topic: format!("Week {week} — {channel}"),
                status: EntryStatus::Planned,
            });
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_templates_produce_valid_plans() {
        for product_type in &[
            ProductType::Book,
            ProductType::Course,
            ProductType::CoachingProgram,
            ProductType::Membership,
            ProductType::General,
        ] {
            let plan = template_for(*product_type);
            assert_eq!(plan.product.product_type, *product_type);
            assert!(!plan.audience_segments.is_empty(), "{:?} has no audience segments", product_type);
            assert!(!plan.channels.is_empty(), "{:?} has no channels", product_type);
            assert!(!plan.calendar.is_empty(), "{:?} has no calendar", product_type);
            assert!(!plan.timeline.is_empty(), "{:?} has no timeline", product_type);
            assert!(!plan.kpis.is_empty(), "{:?} has no KPIs", product_type);
            assert_eq!(plan.status, PlanStatus::Draft);
        }
    }

    #[test]
    fn test_book_template_has_12_weeks() {
        let plan = template_for(ProductType::Book);
        let max_week = plan.calendar.iter().map(|e| e.week).max().unwrap_or(0);
        assert_eq!(max_week, 12);
    }

    #[test]
    fn test_course_template_has_8_weeks() {
        let plan = template_for(ProductType::Course);
        let max_week = plan.calendar.iter().map(|e| e.week).max().unwrap_or(0);
        assert_eq!(max_week, 8);
    }

    #[test]
    fn test_template_serialization() {
        for product_type in &[ProductType::Book, ProductType::Course, ProductType::General] {
            let plan = template_for(*product_type);
            let json = serde_json::to_string(&plan).unwrap();
            let parsed: MarketingPlan = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.product.product_type, *product_type);
        }
    }

    #[test]
    fn test_calendar_skeleton_generation() {
        let entries = generate_calendar_skeleton(4, &["X", "LinkedIn"]);
        assert_eq!(entries.len(), 8); // 4 weeks * 2 channels
        assert_eq!(entries[0].channel, "X");
        assert_eq!(entries[1].channel, "LinkedIn");
    }

    #[test]
    fn test_book_template_channels() {
        let plan = template_for(ProductType::Book);
        let channel_names: Vec<&str> = plan.channels.iter().map(|c| c.channel.as_str()).collect();
        assert!(channel_names.contains(&"Newsletter"));
        assert!(channel_names.contains(&"X"));
        assert!(channel_names.contains(&"Instagram"));
    }
}
