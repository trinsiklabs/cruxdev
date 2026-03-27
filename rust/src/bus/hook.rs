//! Session bus hook — rate-limited inbox check with notification fast path.

use std::fs;
use std::path::PathBuf;
use super::broker::Broker;

const RATE_LIMIT_SECONDS: f64 = 60.0;

pub fn last_check_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".cruxdev").join("bus_last_check")
}

pub fn should_check(path: &str) -> bool {
    match fs::read_to_string(path) {
        Ok(content) => {
            let last: f64 = content.trim().parse().unwrap_or(0.0);
            (now() - last) >= RATE_LIMIT_SECONDS
        }
        Err(_) => true,
    }
}

pub fn mark_checked(path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, now().to_string());
}

pub fn check_and_notify(project: &str, broker: &Broker, last_check_file: &str) -> Vec<serde_json::Value> {
    if !should_check(last_check_file) {
        return vec![];
    }
    mark_checked(last_check_file);

    if broker.read_notification(project).is_none() {
        return vec![];
    }

    let messages = broker.check_inbox(project).unwrap_or_default();
    broker.clear_notification(project);

    if messages.is_empty() {
        return vec![];
    }

    messages.iter().map(|m| {
        serde_json::json!({
            "id": m.id,
            "type": m.msg_type,
            "from": m.source_project,
            "title": m.title,
            "severity": m.severity,
        })
    }).collect()
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_no_file() {
        assert!(should_check("/nonexistent/check"));
    }

    #[test]
    fn should_check_recent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("check");
        mark_checked(path.to_str().unwrap());
        assert!(!should_check(path.to_str().unwrap()));
    }

    #[test]
    fn should_check_stale() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("check");
        fs::write(&path, (now() - 120.0).to_string()).unwrap();
        assert!(should_check(path.to_str().unwrap()));
    }

    #[test]
    fn check_and_notify_with_messages() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("bus.db");
        let broker = Broker::new(db.to_str().unwrap()).unwrap();
        let check_file = dir.path().join("check");

        broker.send_message("issue", "other", "myproject", "Bug", "d", "high").unwrap();
        let msgs = check_and_notify("myproject", &broker, check_file.to_str().unwrap());
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0]["title"], "Bug");
        // Notification cleared
        assert!(broker.read_notification("myproject").is_none());
    }

    #[test]
    fn check_and_notify_no_notification() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("bus.db");
        let broker = Broker::new(db.to_str().unwrap()).unwrap();
        let check_file = dir.path().join("check");

        let msgs = check_and_notify("myproject", &broker, check_file.to_str().unwrap());
        assert!(msgs.is_empty());
    }

    #[test]
    fn check_and_notify_rate_limited() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("bus.db");
        let broker = Broker::new(db.to_str().unwrap()).unwrap();
        let check_file = dir.path().join("check");

        broker.send_message("issue", "o", "myproject", "Bug", "d", "medium").unwrap();
        check_and_notify("myproject", &broker, check_file.to_str().unwrap());

        broker.send_message("issue", "o", "myproject", "Bug2", "d", "medium").unwrap();
        let msgs = check_and_notify("myproject", &broker, check_file.to_str().unwrap());
        assert!(msgs.is_empty()); // rate limited
    }
}
