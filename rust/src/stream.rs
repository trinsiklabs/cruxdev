//! SSE (Server-Sent Events) streaming endpoint for live terminal viewer.
//!
//! Tails evolution/session logs and streams filtered events to web clients.
//! Filters out file contents, only emits summaries, tool calls, and results.

use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use tokio::sync::broadcast;

/// A single terminal line to stream.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TerminalLine {
    pub text: String,
    pub line_type: String, // "info", "success", "error", "tool", "heading"
    pub timestamp: String,
}

/// Classify a log line by type for color coding.
fn classify_line(text: &str) -> &'static str {
    let lower = text.to_lowercase();
    if lower.contains("error") || lower.contains("failed") || lower.contains("fail") {
        "error"
    } else if lower.contains("converged") || lower.contains("passed") || lower.contains("ok") || lower.contains("purged") {
        "success"
    } else if lower.contains("converge_plan") || lower.contains("build_plan") || lower.contains("bp0") {
        "heading"
    } else if lower.contains("cargo") || lower.contains("npm") || lower.contains("rsync") || lower.contains("curl") || lower.contains("git") {
        "tool"
    } else {
        "info"
    }
}

/// Filter a log line — return None if it should be suppressed.
fn should_emit(text: &str) -> bool {
    // Skip empty lines
    if text.trim().is_empty() {
        return false;
    }
    // Skip file content dumps (long lines with code)
    if text.len() > 500 {
        return false;
    }
    // Skip binary/path noise
    if text.contains("/target/debug/") || text.contains("/target/release/") {
        return false;
    }
    true
}

/// Tail a log file and send new lines to a broadcast channel.
pub async fn tail_log(
    log_path: &str,
    tx: broadcast::Sender<TerminalLine>,
) {
    let path = Path::new(log_path);

    // Start from end of file
    let mut last_pos = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let current_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        if current_size <= last_pos {
            if current_size < last_pos {
                // File was truncated — reset
                last_pos = 0;
            }
            continue;
        }

        let mut reader = BufReader::new(file);
        if reader.seek(SeekFrom::Start(last_pos)).is_err() {
            continue;
        }

        let mut line = String::new();
        while reader.read_line(&mut line).unwrap_or(0) > 0 {
            let trimmed = line.trim().to_string();
            if should_emit(&trimmed) {
                let ts = chrono::Local::now().format("%H:%M:%S").to_string();
                let line_type = classify_line(&trimmed).to_string();
                let _ = tx.send(TerminalLine {
                    text: trimmed,
                    line_type,
                    timestamp: ts,
                });
            }
            line.clear();
        }

        last_pos = current_size;
    }
}

/// Start the SSE server on the given port.
pub async fn run_sse_server(log_path: String, port: u16) {
    use tokio::io::AsyncWriteExt;

    let (tx, _) = broadcast::channel::<TerminalLine>(256);
    let tx_clone = tx.clone();

    // Spawn log tailer
    tokio::spawn(async move {
        tail_log(&log_path, tx_clone).await;
    });

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind SSE server on port {port}: {e}");
            return;
        }
    };

    eprintln!("SSE server listening on port {port}");

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // Send HTTP headers
            let headers = "HTTP/1.1 200 OK\r\n\
                 Content-Type: text/event-stream\r\n\
                 Cache-Control: no-cache\r\n\
                 Connection: keep-alive\r\n\
                 Access-Control-Allow-Origin: *\r\n\
                 \r\n";
            if socket.write_all(headers.as_bytes()).await.is_err() {
                return;
            }

            // Stream events
            while let Ok(line) = rx.recv().await {
                let data = serde_json::to_string(&line).unwrap_or_default();
                let event = format!("event: line\ndata: {data}\n\n");
                if socket.write_all(event.as_bytes()).await.is_err() {
                    break;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_line() {
        assert_eq!(classify_line("CONVERGED: BP050"), "success");
        assert_eq!(classify_line("error: compilation failed"), "error");
        assert_eq!(classify_line("cargo test"), "tool");
        assert_eq!(classify_line("just a normal line"), "info");
    }

    #[test]
    fn test_should_emit() {
        assert!(should_emit("BP050 CONVERGED"));
        assert!(!should_emit(""));
        assert!(!should_emit("   "));
        assert!(!should_emit(&"x".repeat(600)));
    }
}
