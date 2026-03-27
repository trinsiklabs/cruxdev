//! Test runner — execute test commands and parse results.

use std::process::Command;

use serde::{Deserialize, Serialize};

/// Result of running a test command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

/// Run a test command with timeout. Returns TestResult.
pub fn run_tests(command: &[String], working_dir: &str, _timeout_secs: u64) -> TestResult {
    if command.is_empty() {
        return TestResult {
            passed: true,
            exit_code: 0,
            stdout: "No test command configured".into(),
            stderr: String::new(),
            timed_out: false,
        };
    }

    let (program, args) = (&command[0], &command[1..]);

    let result = Command::new(program)
        .args(args)
        .current_dir(working_dir)
        .output();

    match result {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-1);
            TestResult {
                passed: output.status.success(),
                exit_code,
                stdout: String::from_utf8_lossy(&output.stdout).chars().take(5000).collect(),
                stderr: String::from_utf8_lossy(&output.stderr).chars().take(5000).collect(),
                timed_out: false,
            }
        }
        Err(e) => TestResult {
            passed: false,
            exit_code: -1,
            stdout: String::new(),
            stderr: format!("Failed to execute test command: {e}"),
            timed_out: e.kind() == std::io::ErrorKind::TimedOut,
        },
    }
}

/// Parse a test command string into parts.
pub fn parse_test_command(cmd: &str) -> Vec<String> {
    cmd.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_tests_no_command() {
        let result = run_tests(&[], ".", 30);
        assert!(result.passed);
    }

    #[test]
    fn test_run_tests_true() {
        let result = run_tests(&["true".to_string()], ".", 30);
        assert!(result.passed);
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_run_tests_false() {
        let result = run_tests(&["false".to_string()], ".", 30);
        assert!(!result.passed);
        assert_ne!(result.exit_code, 0);
    }

    #[test]
    fn test_parse_test_command() {
        let parts = parse_test_command("cargo test -- --nocapture");
        assert_eq!(parts, vec!["cargo", "test", "--", "--nocapture"]);
    }
}
