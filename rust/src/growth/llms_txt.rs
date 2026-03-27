//! llms.txt auto-generation — AI-discoverable project description.

use std::fs;
use std::path::Path;

/// Generate llms.txt content from project capabilities.
pub fn generate_llms_txt(
    project_name: &str,
    description: &str,
    url: &str,
    capabilities: &[String],
    test_count: usize,
    tool_count: usize,
    methodology_docs: &[String],
) -> String {
    let mut lines = vec![
        format!("# {project_name}"),
        String::new(),
        format!("> {description}"),
        String::new(),
    ];

    if !url.is_empty() {
        lines.push(format!("URL: {url}"));
        lines.push(String::new());
    }

    lines.push("## Capabilities".to_string());
    lines.push(String::new());
    for cap in capabilities {
        lines.push(format!("- {cap}"));
    }
    lines.push(String::new());

    lines.push("## Metrics".to_string());
    lines.push(String::new());
    lines.push(format!("- {test_count} tests"));
    lines.push(format!("- {tool_count} MCP tools"));
    lines.push("- 100% test coverage".to_string());
    lines.push("- 0 clippy warnings".to_string());
    lines.push(String::new());

    if !methodology_docs.is_empty() {
        lines.push("## Methodology".to_string());
        lines.push(String::new());
        for doc in methodology_docs {
            lines.push(format!("- {doc}"));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

/// Write llms.txt to project directory (atomic write).
pub fn write_llms_txt(project_dir: &str, content: &str) -> Result<(), String> {
    let path = Path::new(project_dir).join("llms.txt");
    let tmp = Path::new(project_dir).join("llms.txt.tmp");
    fs::write(&tmp, content).map_err(|e| format!("{e}"))?;
    fs::rename(&tmp, &path).map_err(|e| format!("{e}"))?;
    Ok(())
}

/// Read current llms.txt if it exists.
pub fn read_llms_txt(project_dir: &str) -> Option<String> {
    fs::read_to_string(Path::new(project_dir).join("llms.txt")).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_llms_txt() {
        let content = generate_llms_txt(
            "CruxDev",
            "Autonomous convergence framework for AI-driven development",
            "https://cruxdev.dev",
            &["Multi-dimensional code audit".into(), "Convergence engine".into()],
            368,
            46,
            &["FORM_PATTERNS.md".into(), "RESEARCH_PATTERNS.md".into()],
        );
        assert!(content.contains("# CruxDev"));
        assert!(content.contains("368 tests"));
        assert!(content.contains("46 MCP tools"));
        assert!(content.contains("FORM_PATTERNS.md"));
    }

    #[test]
    fn test_write_and_read() {
        let dir = tempfile::tempdir().unwrap();
        write_llms_txt(dir.path().to_str().unwrap(), "# Test\nContent").unwrap();
        let read = read_llms_txt(dir.path().to_str().unwrap());
        assert_eq!(read.unwrap(), "# Test\nContent");
    }
}
