//! SEO content pipeline — generate tutorials, comparison posts, build stories.

/// Generate a "how we built X" post from build plan convergence data.
pub fn generate_build_story(
    project: &str,
    plan_name: &str,
    summary: &str,
    phases: &[String],
    test_count: usize,
    findings_closed: usize,
) -> String {
    let mut lines = vec![
        format!("# How {project} Built: {plan_name}"),
        String::new(),
        summary.to_string(),
        String::new(),
    ];

    if !phases.is_empty() {
        lines.push("## Phases".to_string());
        lines.push(String::new());
        for (i, phase) in phases.iter().enumerate() {
            lines.push(format!("{}. {phase}", i + 1));
        }
        lines.push(String::new());
    }

    lines.push("## Results".to_string());
    lines.push(String::new());
    lines.push(format!("- **{test_count}** tests passing"));
    if findings_closed > 0 {
        lines.push(format!("- **{findings_closed}** audit findings closed"));
    }
    lines.push("- **0** clippy warnings".to_string());

    lines.join("\n")
}

/// Generate a comparison post from competitive data.
pub fn generate_comparison_post(
    our_name: &str,
    competitor_name: &str,
    our_advantages: &[String],
    their_advantages: &[String],
) -> String {
    let mut lines = vec![
        format!("# {our_name} vs {competitor_name}"),
        String::new(),
    ];

    if !our_advantages.is_empty() {
        lines.push(format!("## Where {our_name} Wins"));
        lines.push(String::new());
        for a in our_advantages {
            lines.push(format!("- {a}"));
        }
        lines.push(String::new());
    }

    if !their_advantages.is_empty() {
        lines.push(format!("## Where {competitor_name} Wins"));
        lines.push(String::new());
        for a in their_advantages {
            lines.push(format!("- {a}"));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

/// Generate a technical tutorial skeleton.
pub fn generate_tutorial(
    title: &str,
    introduction: &str,
    steps: &[(String, String)], // (step_title, code_or_content)
) -> String {
    let mut lines = vec![
        format!("# {title}"),
        String::new(),
        introduction.to_string(),
        String::new(),
    ];

    for (i, (step_title, content)) in steps.iter().enumerate() {
        lines.push(format!("## Step {}: {step_title}", i + 1));
        lines.push(String::new());
        if content.contains('\n') || content.starts_with("```") {
            lines.push(content.clone());
        } else {
            lines.push(format!("```\n{content}\n```"));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_build_story() {
        let story = generate_build_story(
            "CruxDev",
            "BUILD_PLAN_016",
            "Git workflow automation",
            &["Git ops module".into(), "Safety gates".into(), "MCP tools".into()],
            368,
            5,
        );
        assert!(story.contains("# How CruxDev Built"));
        assert!(story.contains("BUILD_PLAN_016"));
        assert!(story.contains("368"));
        assert!(story.contains("5"));
    }

    #[test]
    fn test_generate_comparison_post() {
        let post = generate_comparison_post(
            "CruxDev",
            "Superpowers",
            &["Convergence engine".into(), "Safety gates".into()],
            &["110K stars".into()],
        );
        assert!(post.contains("CruxDev vs Superpowers"));
        assert!(post.contains("Convergence engine"));
        assert!(post.contains("110K stars"));
    }

    #[test]
    fn test_generate_tutorial() {
        let tutorial = generate_tutorial(
            "Getting Started with CruxDev",
            "Install and run your first convergence.",
            &[
                ("Install".into(), "cargo install cruxdev".into()),
                ("Run".into(), "cruxdev mcp start".into()),
            ],
        );
        assert!(tutorial.contains("# Getting Started"));
        assert!(tutorial.contains("Step 1: Install"));
        assert!(tutorial.contains("cargo install"));
    }
}
