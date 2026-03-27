//! Checklist parser — extract tasks from build plan markdown + durable mark complete.

use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub id: String,
    pub phase: String,
    pub description: String,
    pub completed: bool,
}

/// Parse all checklist items from a build plan.
pub fn parse_checklist(plan_file: &str) -> Vec<ChecklistItem> {
    let content = match fs::read_to_string(plan_file) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let phase_re = Regex::new(r"(?i)^##\s+(?:Phase\s+)?(\d+)[\s:.]+").unwrap();
    let item_re = Regex::new(r"^\s*-\s*\[\s*([xX ])?\s*\]\s*(\d+\.\d+)\s+(.*)").unwrap();

    let mut items = Vec::new();
    let mut current_phase = String::new();

    for line in content.lines() {
        if let Some(caps) = phase_re.captures(line) {
            current_phase = format!("Phase {}", &caps[1]);
            continue;
        }
        if let Some(caps) = item_re.captures(line) {
            let checked = caps.get(1).is_some_and(|m| m.as_str() == "x" || m.as_str() == "X");
            items.push(ChecklistItem {
                id: caps[2].to_string(),
                phase: current_phase.clone(),
                description: caps[3].trim().to_string(),
                completed: checked,
            });
        }
    }
    items
}

/// Get the next incomplete checklist item.
pub fn get_next_incomplete(items: &[ChecklistItem]) -> Option<&ChecklistItem> {
    items.iter().find(|i| !i.completed)
}

/// Check if all items are complete.
pub fn all_complete(items: &[ChecklistItem]) -> bool {
    items.is_empty() || items.iter().all(|i| i.completed)
}

/// Completion statistics.
pub fn completion_summary(items: &[ChecklistItem]) -> (usize, usize, f64) {
    let total = items.len();
    let done = items.iter().filter(|i| i.completed).count();
    let pct = if total > 0 { done as f64 / total as f64 * 100.0 } else { 100.0 };
    (total, done, pct)
}

/// Durably mark a checklist item as complete in the plan file.
pub fn mark_complete_in_file(plan_file: &str, item_id: &str) -> bool {
    let content = match fs::read_to_string(plan_file) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let pattern = format!(r"(?m)(^\s*-\s*)\[\s*\](\s*{}\s)", regex::escape(item_id));
    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let updated = re.replace(&content, "${1}[x]${2}").to_string();
    if updated == content {
        return false; // Not found or already checked
    }

    fs::write(plan_file, &updated).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const SAMPLE: &str = r#"# Plan
## Phase 1: Build
- [ ] 1.1 First task
- [x] 1.2 Done task
- [ ] 1.3 Third task
## Phase 2: Test
- [ ] 2.1 Write tests
"#;

    #[test]
    fn parse_basic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();

        let items = parse_checklist(path.to_str().unwrap());
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].id, "1.1");
        assert!(!items[0].completed);
        assert_eq!(items[1].id, "1.2");
        assert!(items[1].completed);
        assert_eq!(items[0].phase, "Phase 1");
        assert_eq!(items[3].phase, "Phase 2");
    }

    #[test]
    fn parse_nonexistent() {
        assert!(parse_checklist("/nonexistent.md").is_empty());
    }

    #[test]
    fn next_incomplete() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();

        let items = parse_checklist(path.to_str().unwrap());
        let next = get_next_incomplete(&items).unwrap();
        assert_eq!(next.id, "1.1");
    }

    #[test]
    fn all_complete_true() {
        let items = vec![
            ChecklistItem { id: "1.1".into(), phase: "P1".into(), description: "d".into(), completed: true },
        ];
        assert!(all_complete(&items));
    }

    #[test]
    fn all_complete_false() {
        let items = vec![
            ChecklistItem { id: "1.1".into(), phase: "P1".into(), description: "d".into(), completed: false },
        ];
        assert!(!all_complete(&items));
    }

    #[test]
    fn all_complete_empty() {
        assert!(all_complete(&[]));
    }

    #[test]
    fn summary() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();

        let items = parse_checklist(path.to_str().unwrap());
        let (total, done, _pct) = completion_summary(&items);
        assert_eq!(total, 4);
        assert_eq!(done, 1);
    }

    #[test]
    fn mark_complete() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();
        let p = path.to_str().unwrap();

        assert!(mark_complete_in_file(p, "1.1"));
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("- [x] 1.1 First task"));
        assert!(content.contains("- [ ] 1.3 Third task")); // untouched
    }

    #[test]
    fn mark_complete_already_checked() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();
        assert!(!mark_complete_in_file(path.to_str().unwrap(), "1.2")); // already [x]
    }

    #[test]
    fn mark_complete_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, SAMPLE).unwrap();
        assert!(!mark_complete_in_file(path.to_str().unwrap(), "9.9"));
    }

    #[test]
    fn mark_complete_nonexistent_file() {
        assert!(!mark_complete_in_file("/nonexistent.md", "1.1"));
    }
}
