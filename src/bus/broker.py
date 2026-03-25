"""Session bus broker — SQLite-backed cross-project messaging.

Persistent broker for CruxDev sessions to discover each other,
report issues, share patterns, and notify breaking changes.
All state in a single SQLite database at ~/.cruxdev/bus.db.
"""

from __future__ import annotations

import json
import os
import sqlite3
import time
import uuid
from contextlib import contextmanager
from dataclasses import dataclass
from enum import Enum
from typing import Optional


DEFAULT_DB_PATH = os.path.join(
    os.environ.get("HOME", ""),
    ".cruxdev",
    "bus.db",
)


class MessageType(Enum):
    ISSUE = "issue"
    IMPROVEMENT = "improvement"
    PATTERN = "pattern"
    BREAKING_CHANGE = "breaking_change"
    CUSTOM = "custom"


class MessageSeverity(Enum):
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


@dataclass
class Session:
    id: str
    project: str
    directory: str
    registered_at: float
    last_heartbeat: float


@dataclass
class Message:
    id: str
    type: str
    source_project: str
    target_project: str  # "*" for broadcast
    title: str
    body: str
    severity: str
    created_at: float
    acknowledged: bool = False
    acknowledged_at: Optional[float] = None


class Broker:
    """SQLite-backed message broker for cross-project session communication."""

    def __init__(self, db_path: str = DEFAULT_DB_PATH):
        self.db_path = db_path
        os.makedirs(os.path.dirname(db_path), exist_ok=True)
        self._init_db()

    @contextmanager
    def _conn(self):
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        try:
            yield conn
            conn.commit()
        finally:
            conn.close()

    def _init_db(self) -> None:
        with self._conn() as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS sessions (
                    id TEXT PRIMARY KEY,
                    project TEXT NOT NULL,
                    directory TEXT NOT NULL,
                    registered_at REAL NOT NULL,
                    last_heartbeat REAL NOT NULL
                )
            """)
            conn.execute("""
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
                )
            """)
            conn.execute("""
                CREATE INDEX IF NOT EXISTS idx_messages_target
                ON messages(target_project, acknowledged)
            """)

    # --- Session management ---

    def register_session(self, project: str, directory: str) -> str:
        """Register a session. Returns session ID."""
        session_id = str(uuid.uuid4())[:8]
        now = time.time()
        with self._conn() as conn:
            conn.execute(
                "INSERT INTO sessions (id, project, directory, registered_at, last_heartbeat) VALUES (?, ?, ?, ?, ?)",
                (session_id, project, directory, now, now),
            )
        return session_id

    def heartbeat(self, session_id: str) -> None:
        """Update session heartbeat."""
        with self._conn() as conn:
            conn.execute(
                "UPDATE sessions SET last_heartbeat = ? WHERE id = ?",
                (time.time(), session_id),
            )

    def unregister_session(self, session_id: str) -> None:
        """Remove a session."""
        with self._conn() as conn:
            conn.execute("DELETE FROM sessions WHERE id = ?", (session_id,))

    def list_sessions(self, max_age_seconds: float = 3600) -> list[Session]:
        """List active sessions (heartbeat within max_age)."""
        cutoff = time.time() - max_age_seconds
        with self._conn() as conn:
            rows = conn.execute(
                "SELECT * FROM sessions WHERE last_heartbeat > ? ORDER BY project",
                (cutoff,),
            ).fetchall()
        return [
            Session(
                id=r["id"],
                project=r["project"],
                directory=r["directory"],
                registered_at=r["registered_at"],
                last_heartbeat=r["last_heartbeat"],
            )
            for r in rows
        ]

    def cleanup_stale_sessions(self, max_age_seconds: float = 3600) -> int:
        """Remove sessions with no heartbeat within max_age. Returns count removed."""
        cutoff = time.time() - max_age_seconds
        with self._conn() as conn:
            cursor = conn.execute(
                "DELETE FROM sessions WHERE last_heartbeat < ?",
                (cutoff,),
            )
        return cursor.rowcount

    # --- Messaging ---

    def send_message(
        self,
        type: str,
        source_project: str,
        target_project: str,
        title: str,
        body: str,
        severity: str = "medium",
    ) -> str:
        """Send a message. Returns message ID.

        Also writes a notification file so hook scripts can detect
        new messages without querying SQLite.
        """
        msg_id = str(uuid.uuid4())[:8]
        with self._conn() as conn:
            conn.execute(
                "INSERT INTO messages (id, type, source_project, target_project, title, body, severity, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (msg_id, type, source_project, target_project, title, body, severity, time.time()),
            )
        # Write notification file for push-based detection
        if target_project != "*":
            self._write_notification(target_project, title)
        else:
            # Broadcast — notify all registered projects
            for session in self.list_sessions():
                if session.project != source_project:
                    self._write_notification(session.project, title)
        return msg_id

    def _write_notification(self, project: str, title: str) -> None:
        """Write a notification file for a project."""
        notify_dir = os.path.join(os.path.dirname(self.db_path), "notifications")
        os.makedirs(notify_dir, exist_ok=True)
        notify_path = os.path.join(notify_dir, f"{project}.notify")

        # Read existing count if any
        count = 0
        try:
            with open(notify_path) as f:
                existing = json.loads(f.read())
                count = existing.get("count", 0)
        except (FileNotFoundError, json.JSONDecodeError, ValueError):
            pass

        data = json.dumps({
            "count": count + 1,
            "latest_title": title,
            "updated_at": time.time(),
        })
        # Atomic write
        tmp_path = notify_path + ".tmp"
        with open(tmp_path, "w") as f:
            f.write(data)
        os.replace(tmp_path, notify_path)

    def clear_notification(self, project: str) -> bool:
        """Clear the notification file for a project. Returns True if file existed."""
        notify_dir = os.path.join(os.path.dirname(self.db_path), "notifications")
        notify_path = os.path.join(notify_dir, f"{project}.notify")
        try:
            os.remove(notify_path)
            return True
        except FileNotFoundError:
            return False

    def read_notification(self, project: str) -> dict | None:
        """Read notification file for a project. Returns None if no notification."""
        notify_dir = os.path.join(os.path.dirname(self.db_path), "notifications")
        notify_path = os.path.join(notify_dir, f"{project}.notify")
        try:
            with open(notify_path) as f:
                return json.loads(f.read())
        except (FileNotFoundError, json.JSONDecodeError):
            return None

    def check_inbox(
        self,
        project: str,
        include_acknowledged: bool = False,
    ) -> list[Message]:
        """Get messages for a project (including broadcasts)."""
        with self._conn() as conn:
            if include_acknowledged:
                rows = conn.execute(
                    "SELECT * FROM messages WHERE target_project IN (?, '*') ORDER BY created_at DESC",
                    (project,),
                ).fetchall()
            else:
                rows = conn.execute(
                    "SELECT * FROM messages WHERE target_project IN (?, '*') AND acknowledged = 0 ORDER BY created_at DESC",
                    (project,),
                ).fetchall()
        return [self._row_to_message(r) for r in rows]

    def acknowledge(self, message_id: str) -> bool:
        """Mark a message as acknowledged. Returns True if found."""
        with self._conn() as conn:
            cursor = conn.execute(
                "UPDATE messages SET acknowledged = 1, acknowledged_at = ? WHERE id = ?",
                (time.time(), message_id),
            )
        return cursor.rowcount > 0

    def get_message(self, message_id: str) -> Optional[Message]:
        """Get a single message by ID."""
        with self._conn() as conn:
            row = conn.execute(
                "SELECT * FROM messages WHERE id = ?",
                (message_id,),
            ).fetchone()
        return self._row_to_message(row) if row else None

    def get_all_messages(
        self,
        type_filter: Optional[str] = None,
        limit: int = 100,
    ) -> list[Message]:
        """Get all messages, optionally filtered by type."""
        with self._conn() as conn:
            if type_filter:
                rows = conn.execute(
                    "SELECT * FROM messages WHERE type = ? ORDER BY created_at DESC LIMIT ?",
                    (type_filter, limit),
                ).fetchall()
            else:
                rows = conn.execute(
                    "SELECT * FROM messages ORDER BY created_at DESC LIMIT ?",
                    (limit,),
                ).fetchall()
        return [self._row_to_message(r) for r in rows]

    def _row_to_message(self, row: sqlite3.Row) -> Message:
        return Message(
            id=row["id"],
            type=row["type"],
            source_project=row["source_project"],
            target_project=row["target_project"],
            title=row["title"],
            body=row["body"],
            severity=row["severity"],
            created_at=row["created_at"],
            acknowledged=bool(row["acknowledged"]),
            acknowledged_at=row["acknowledged_at"],
        )

    # --- Convenience methods ---

    def report_issue(
        self,
        source_project: str,
        target_project: str,
        title: str,
        body: str,
        severity: str = "medium",
    ) -> str:
        """Report an issue found in another project."""
        return self.send_message(
            type=MessageType.ISSUE.value,
            source_project=source_project,
            target_project=target_project,
            title=title,
            body=body,
            severity=severity,
        )

    def report_improvement(
        self,
        source_project: str,
        target_project: str,
        title: str,
        body: str,
    ) -> str:
        """Suggest an improvement to another project."""
        return self.send_message(
            type=MessageType.IMPROVEMENT.value,
            source_project=source_project,
            target_project=target_project,
            title=title,
            body=body,
            severity="medium",
        )

    def share_pattern(
        self,
        source_project: str,
        pattern_name: str,
        description: str,
    ) -> str:
        """Broadcast a learned pattern to all sessions."""
        return self.send_message(
            type=MessageType.PATTERN.value,
            source_project=source_project,
            target_project="*",
            title=pattern_name,
            body=description,
            severity="low",
        )

    def notify_breaking_change(
        self,
        source_project: str,
        affected_projects: list[str],
        description: str,
    ) -> list[str]:
        """Notify affected projects of a breaking change."""
        msg_ids = []
        for project in affected_projects:
            msg_id = self.send_message(
                type=MessageType.BREAKING_CHANGE.value,
                source_project=source_project,
                target_project=project,
                title=f"Breaking change in {source_project}",
                body=description,
                severity="high",
            )
            msg_ids.append(msg_id)
        return msg_ids
