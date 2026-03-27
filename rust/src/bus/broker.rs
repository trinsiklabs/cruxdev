//! SQLite-backed message broker for cross-project session communication.

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project: String,
    pub directory: String,
    pub registered_at: f64,
    pub last_heartbeat: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub source_project: String,
    pub target_project: String,
    pub title: String,
    pub body: String,
    pub severity: String,
    pub created_at: f64,
    pub acknowledged: bool,
}

pub struct Broker {
    db_path: PathBuf,
}

impl Broker {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let path = PathBuf::from(db_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let broker = Self { db_path: path };
        broker.init_db()?;
        Ok(broker)
    }

    pub fn default_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".cruxdev").join("bus.db")
    }

    fn conn(&self) -> anyhow::Result<Connection> {
        Ok(Connection::open(&self.db_path)?)
    }

    fn init_db(&self) -> anyhow::Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                project TEXT NOT NULL,
                directory TEXT NOT NULL,
                registered_at REAL NOT NULL,
                last_heartbeat REAL NOT NULL
            );
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                type TEXT NOT NULL,
                source_project TEXT NOT NULL,
                target_project TEXT NOT NULL,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                severity TEXT NOT NULL DEFAULT 'medium',
                created_at REAL NOT NULL,
                acknowledged INTEGER NOT NULL DEFAULT 0,
                acknowledged_at REAL
            );
            CREATE INDEX IF NOT EXISTS idx_messages_target
            ON messages(target_project, acknowledged);"
        )?;
        Ok(())
    }

    pub fn register_session(&self, project: &str, directory: &str) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let now = now();
        self.conn()?.execute(
            "INSERT INTO sessions (id, project, directory, registered_at, last_heartbeat) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, project, directory, now, now],
        )?;
        Ok(id)
    }

    pub fn list_sessions(&self, max_age_seconds: f64) -> anyhow::Result<Vec<Session>> {
        let cutoff = now() - max_age_seconds;
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, project, directory, registered_at, last_heartbeat FROM sessions WHERE last_heartbeat > ?1 ORDER BY project"
        )?;
        let sessions = stmt.query_map(params![cutoff], |row| {
            Ok(Session {
                id: row.get(0)?,
                project: row.get(1)?,
                directory: row.get(2)?,
                registered_at: row.get(3)?,
                last_heartbeat: row.get(4)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(sessions)
    }

    pub fn send_message(&self, msg_type: &str, source: &str, target: &str, title: &str, body: &str, severity: &str) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        self.conn()?.execute(
            "INSERT INTO messages (id, type, source_project, target_project, title, body, severity, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id, msg_type, source, target, title, body, severity, now()],
        )?;

        // Write notification file
        if target != "*" {
            self.write_notification(target, title);
        } else if let Ok(sessions) = self.list_sessions(3600.0) {
            for s in sessions {
                if s.project != source {
                    self.write_notification(&s.project, title);
                }
            }
        }
        Ok(id)
    }

    pub fn check_inbox(&self, project: &str) -> anyhow::Result<Vec<Message>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, type, source_project, target_project, title, body, severity, created_at, acknowledged FROM messages WHERE target_project IN (?1, '*') AND acknowledged = 0 ORDER BY created_at DESC"
        )?;
        let msgs = stmt.query_map(params![project], |row| {
            Ok(Message {
                id: row.get(0)?,
                msg_type: row.get(1)?,
                source_project: row.get(2)?,
                target_project: row.get(3)?,
                title: row.get(4)?,
                body: row.get(5)?,
                severity: row.get(6)?,
                created_at: row.get(7)?,
                acknowledged: row.get::<_, i32>(8)? != 0,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(msgs)
    }

    pub fn acknowledge(&self, message_id: &str) -> anyhow::Result<bool> {
        let n = self.conn()?.execute(
            "UPDATE messages SET acknowledged = 1, acknowledged_at = ?1 WHERE id = ?2",
            params![now(), message_id],
        )?;
        Ok(n > 0)
    }

    // --- Notification files ---

    fn notify_dir(&self) -> PathBuf {
        self.db_path.parent().unwrap_or(Path::new(".")).join("notifications")
    }

    fn write_notification(&self, project: &str, title: &str) {
        let dir = self.notify_dir();
        let _ = fs::create_dir_all(&dir);
        let path = dir.join(format!("{project}.notify"));

        let count = fs::read_to_string(&path).ok()
            .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
            .and_then(|v| v.get("count").and_then(|c| c.as_u64()))
            .unwrap_or(0);

        let data = serde_json::json!({"count": count + 1, "latest_title": title, "updated_at": now()});
        let tmp = format!("{}.tmp", path.display());
        if fs::write(&tmp, data.to_string()).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }

    pub fn read_notification(&self, project: &str) -> Option<serde_json::Value> {
        let path = self.notify_dir().join(format!("{project}.notify"));
        let content = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn clear_notification(&self, project: &str) -> bool {
        let path = self.notify_dir().join(format!("{project}.notify"));
        fs::remove_file(&path).is_ok()
    }
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

    fn test_broker() -> (tempfile::TempDir, Broker) {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("bus.db");
        let broker = Broker::new(db.to_str().unwrap()).unwrap();
        (dir, broker)
    }

    #[test]
    fn register_and_list() {
        let (_dir, broker) = test_broker();
        broker.register_session("crux", "/crux").unwrap();
        broker.register_session("cruxdev", "/cruxdev").unwrap();
        let sessions = broker.list_sessions(3600.0).unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn send_and_receive() {
        let (_dir, broker) = test_broker();
        broker.send_message("issue", "crux", "cruxdev", "Bug", "details", "high").unwrap();
        let msgs = broker.check_inbox("cruxdev").unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].title, "Bug");
    }

    #[test]
    fn acknowledge_message() {
        let (_dir, broker) = test_broker();
        broker.send_message("issue", "crux", "cruxdev", "Bug", "d", "medium").unwrap();
        let msgs = broker.check_inbox("cruxdev").unwrap();
        assert!(broker.acknowledge(&msgs[0].id).unwrap());
        assert!(broker.check_inbox("cruxdev").unwrap().is_empty());
    }

    #[test]
    fn notification_files() {
        let (_dir, broker) = test_broker();
        broker.send_message("issue", "src", "target", "Bug", "d", "medium").unwrap();
        let notif = broker.read_notification("target");
        assert!(notif.is_some());
        assert_eq!(notif.unwrap()["count"], 1);
        assert!(broker.clear_notification("target"));
        assert!(broker.read_notification("target").is_none());
    }

    #[test]
    fn broadcast_notifies_others() {
        let (_dir, broker) = test_broker();
        broker.register_session("a", "/a").unwrap();
        broker.register_session("b", "/b").unwrap();
        broker.send_message("pattern", "a", "*", "Tip", "desc", "low").unwrap();
        assert!(broker.read_notification("b").is_some());
        assert!(broker.read_notification("a").is_none()); // sender excluded
    }
}
