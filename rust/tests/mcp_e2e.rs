mod common;

use std::time::Duration;
use tokio::time::timeout;

const TIMEOUT: Duration = Duration::from_secs(30);

// ── Phase 1: Protocol handshake ──────────────────────────────────────

#[tokio::test]
async fn test_mcp_handshake_and_tool_listing() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout starting client");

    let tools = timeout(TIMEOUT, client.list_all_tools())
        .await
        .expect("timeout listing tools");

    assert_eq!(tools.len(), 60, "expected 60 tools, got {}", tools.len());

    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    let expected = [
        "start_convergence",
        "convergence_submit_result",
        "convergence_status",
        "convergence_cancel",
        "convergence_next_task",
        "session_register",
        "check_inbox",
        "cruxdev_status",
        "validate_plan_structure",
        "create_plan_template",
        "install_cruxdev",
        "classify_project",
        "research_topic",
        "discover_competitors",
        "setup_competitive_analysis",
        "generate_content",
        "list_content_drafts",
        "publish_drafts",
    ];
    for name in &expected {
        assert!(names.contains(name), "missing tool: {name}");
    }

    // Every tool should have a description
    for tool in &tools {
        assert!(
            !tool.description.as_deref().unwrap_or("").is_empty(),
            "tool {} has empty description",
            tool.name
        );
    }

    client.shutdown().await;
}

#[tokio::test]
async fn test_server_info_has_instructions() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout starting client");

    let info = client.peer_info();
    let instructions = info.instructions.as_deref().unwrap_or("");
    assert!(
        instructions.contains("session_register"),
        "instructions should mention session_register bootstrap"
    );
    assert!(
        instructions.contains("convergence"),
        "instructions should mention convergence"
    );

    client.shutdown().await;
}

// ── Phase 2: Tool call round-trip ────────────────────────────────────

#[tokio::test]
async fn test_session_register_round_trip() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool("session_register", serde_json::json!({"project_name": "test-project"})),
    )
    .await
    .expect("timeout");

    assert!(result.get("session_id").is_some(), "should return session_id");
    assert_eq!(
        result.get("project").and_then(|v| v.as_str()),
        Some("test-project")
    );

    client.shutdown().await;
}

#[tokio::test]
async fn test_check_inbox_empty() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    // Register first
    timeout(
        TIMEOUT,
        client.call_tool("session_register", serde_json::json!({"project_name": "test-inbox"})),
    )
    .await
    .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool("check_inbox", serde_json::json!({"project_name": "test-inbox"})),
    )
    .await
    .expect("timeout");

    // Should be an empty array or have empty messages
    if let Some(arr) = result.as_array() {
        assert!(arr.is_empty(), "inbox should be empty");
    } else if let Some(msgs) = result.get("messages") {
        assert!(
            msgs.as_array().map_or(true, |a| a.is_empty()),
            "inbox should be empty"
        );
    }

    client.shutdown().await;
}

#[tokio::test]
async fn test_cruxdev_status() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "cruxdev_status",
            serde_json::json!({"project_dir": client.tempdir().to_str().unwrap()}),
        ),
    )
    .await
    .expect("timeout");

    // Should have some structure — at minimum a status field
    assert!(
        result.is_object(),
        "cruxdev_status should return an object, got: {result}"
    );

    client.shutdown().await;
}

// ── Phase 3: Convergence lifecycle ───────────────────────────────────

#[tokio::test]
async fn test_convergence_full_lifecycle() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let plan_path = client.write_plan(
        "# Test Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] Write a test\n",
    );

    let start_result = timeout(
        TIMEOUT,
        client.call_tool(
            "start_convergence",
            serde_json::json!({
                "plan_file": plan_path.to_str().unwrap(),
                "project_dir": client.tempdir().to_str().unwrap(),
                "max_rounds": 3,
                "timeout_minutes": 5,
            }),
        ),
    )
    .await
    .expect("timeout");

    let conv_id = start_result
        .get("convergence_id")
        .and_then(|v| v.as_str())
        .expect("should return convergence_id");

    // Submit clean passes until convergence or max iterations
    let mut converged = false;
    for _ in 0..50 {
        let submit = timeout(
            TIMEOUT,
            client.call_tool(
                "convergence_submit_result",
                serde_json::json!({
                    "convergence_id": conv_id,
                    "findings_json": "[]",
                }),
            ),
        )
        .await
        .expect("timeout");

        // Check for terminal state
        let task_type = submit
            .get("next_task")
            .and_then(|t| t.get("task_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if task_type == "done" {
            converged = true;
            break;
        }
        if task_type == "escalated" {
            break;
        }
    }

    assert!(converged, "convergence should complete with clean passes");

    // Verify final status
    let status = timeout(
        TIMEOUT,
        client.call_tool(
            "convergence_status",
            serde_json::json!({"convergence_id": conv_id}),
        ),
    )
    .await
    .expect("timeout");

    let terminal = status
        .get("terminal")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(terminal, "should be in terminal state after convergence");

    client.shutdown().await;
}

#[tokio::test]
async fn test_convergence_submit_with_findings() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let plan_path = client.write_plan(
        "# Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] task\n",
    );

    let start = timeout(
        TIMEOUT,
        client.call_tool(
            "start_convergence",
            serde_json::json!({
                "plan_file": plan_path.to_str().unwrap(),
                "project_dir": client.tempdir().to_str().unwrap(),
                "max_rounds": 5,
            }),
        ),
    )
    .await
    .expect("timeout");

    let conv_id = start["convergence_id"].as_str().unwrap();

    let findings = serde_json::json!([{
        "id": "f1",
        "file": "test.rs",
        "dimension": "correctness",
        "severity": "high",
        "description": "test finding",
        "fix": "fix it"
    }]);

    let submit = timeout(
        TIMEOUT,
        client.call_tool(
            "convergence_submit_result",
            serde_json::json!({
                "convergence_id": conv_id,
                "findings_json": findings.to_string(),
            }),
        ),
    )
    .await
    .expect("timeout");

    // consecutive_clean should be 0 after findings
    let consecutive = submit
        .get("consecutive_clean")
        .and_then(|v| v.as_u64())
        .unwrap_or(999);
    assert_eq!(consecutive, 0, "consecutive_clean should reset after findings");

    client.shutdown().await;
}

#[tokio::test]
async fn test_convergence_submit_invalid_json_rejected() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let plan_path = client.write_plan(
        "# Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] task\n",
    );

    let start = timeout(
        TIMEOUT,
        client.call_tool(
            "start_convergence",
            serde_json::json!({
                "plan_file": plan_path.to_str().unwrap(),
                "project_dir": client.tempdir().to_str().unwrap(),
            }),
        ),
    )
    .await
    .expect("timeout");

    let conv_id = start["convergence_id"].as_str().unwrap();

    let submit = timeout(
        TIMEOUT,
        client.call_tool(
            "convergence_submit_result",
            serde_json::json!({
                "convergence_id": conv_id,
                "findings_json": "not valid json",
            }),
        ),
    )
    .await
    .expect("timeout");

    // Should indicate rejection
    let status = submit
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(
        status == "rejected" || submit.get("error").is_some(),
        "invalid JSON should be rejected, got: {submit}"
    );

    client.shutdown().await;
}

#[tokio::test]
async fn test_convergence_cancel() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let plan_path = client.write_plan(
        "# Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] task\n",
    );

    let start = timeout(
        TIMEOUT,
        client.call_tool(
            "start_convergence",
            serde_json::json!({
                "plan_file": plan_path.to_str().unwrap(),
                "project_dir": client.tempdir().to_str().unwrap(),
            }),
        ),
    )
    .await
    .expect("timeout");

    let conv_id = start["convergence_id"].as_str().unwrap();

    let cancel = timeout(
        TIMEOUT,
        client.call_tool(
            "convergence_cancel",
            serde_json::json!({"convergence_id": conv_id}),
        ),
    )
    .await
    .expect("timeout");

    assert!(
        cancel.get("status").is_some() || cancel.get("phase").is_some(),
        "cancel should return status info"
    );

    // Verify terminal via status
    let status = timeout(
        TIMEOUT,
        client.call_tool(
            "convergence_status",
            serde_json::json!({"convergence_id": conv_id}),
        ),
    )
    .await
    .expect("timeout");

    let terminal = status
        .get("terminal")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    assert!(terminal, "cancelled convergence should be terminal");

    client.shutdown().await;
}

// ── Phase 4: Escalation ──────────────────────────────────────────────

#[tokio::test]
async fn test_escalation_max_rounds() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let plan_path = client.write_plan(
        "# Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] task\n",
    );

    let start = timeout(
        TIMEOUT,
        client.call_tool(
            "start_convergence",
            serde_json::json!({
                "plan_file": plan_path.to_str().unwrap(),
                "project_dir": client.tempdir().to_str().unwrap(),
                "max_rounds": 1,
            }),
        ),
    )
    .await
    .expect("timeout");

    let conv_id = start["convergence_id"].as_str().unwrap();

    let findings = serde_json::json!([{
        "id": "f1",
        "file": "test.rs",
        "dimension": "correctness",
        "severity": "high",
        "description": "persistent issue",
        "fix": "cannot fix"
    }]);

    // Submit findings until escalation
    let mut escalated = false;
    for _ in 0..20 {
        let submit = timeout(
            TIMEOUT,
            client.call_tool(
                "convergence_submit_result",
                serde_json::json!({
                    "convergence_id": conv_id,
                    "findings_json": findings.to_string(),
                }),
            ),
        )
        .await
        .expect("timeout");

        let task_type = submit
            .get("next_task")
            .and_then(|t| t.get("task_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if task_type == "escalated" {
            escalated = true;
            break;
        }
    }

    assert!(escalated, "should escalate after exceeding max_rounds");

    client.shutdown().await;
}

// ── Phase 5: Session bus ─────────────────────────────────────────────

#[tokio::test]
async fn test_session_bus_full_flow() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    // Register sender
    timeout(
        TIMEOUT,
        client.call_tool(
            "session_register",
            serde_json::json!({"project_name": "sender-project"}),
        ),
    )
    .await
    .expect("timeout");

    // Register receiver
    timeout(
        TIMEOUT,
        client.call_tool(
            "session_register",
            serde_json::json!({"project_name": "receiver-project"}),
        ),
    )
    .await
    .expect("timeout");

    // Send an issue from sender to receiver
    timeout(
        TIMEOUT,
        client.call_tool(
            "report_issue",
            serde_json::json!({
                "target_project": "receiver-project",
                "title": "Test Issue",
                "body": "This is a test issue",
                "severity": "medium"
            }),
        ),
    )
    .await
    .expect("timeout");

    // Check inbox for receiver
    let inbox = timeout(
        TIMEOUT,
        client.call_tool(
            "check_inbox",
            serde_json::json!({"project_name": "receiver-project"}),
        ),
    )
    .await
    .expect("timeout");

    // Should have at least one message
    let has_message = if let Some(arr) = inbox.as_array() {
        !arr.is_empty()
    } else if let Some(msgs) = inbox.get("messages") {
        msgs.as_array().map_or(false, |a| !a.is_empty())
    } else {
        false
    };
    assert!(has_message, "receiver should have a message in inbox: {inbox}");

    // Extract message ID and acknowledge it
    let msg_id = if let Some(arr) = inbox.as_array() {
        arr[0].get("id").and_then(|v| v.as_str()).map(String::from)
    } else if let Some(msgs) = inbox.get("messages") {
        msgs.as_array()
            .and_then(|a| a[0].get("id"))
            .and_then(|v| v.as_str())
            .map(String::from)
    } else {
        None
    };

    if let Some(id) = msg_id {
        timeout(
            TIMEOUT,
            client.call_tool(
                "acknowledge_message",
                serde_json::json!({"message_id": id}),
            ),
        )
        .await
        .expect("timeout");

        // Check inbox again — should be empty
        let inbox2 = timeout(
            TIMEOUT,
            client.call_tool(
                "check_inbox",
                serde_json::json!({"project_name": "receiver-project"}),
            ),
        )
        .await
        .expect("timeout");

        let still_has = if let Some(arr) = inbox2.as_array() {
            !arr.is_empty()
        } else if let Some(msgs) = inbox2.get("messages") {
            msgs.as_array().map_or(false, |a| !a.is_empty())
        } else {
            false
        };
        assert!(!still_has, "inbox should be empty after acknowledge: {inbox2}");
    }

    client.shutdown().await;
}

#[tokio::test]
async fn test_share_pattern_broadcast() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    // Register a project
    timeout(
        TIMEOUT,
        client.call_tool(
            "session_register",
            serde_json::json!({"project_name": "pattern-test"}),
        ),
    )
    .await
    .expect("timeout");

    // Share a pattern (broadcasts to *)
    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "share_pattern",
            serde_json::json!({
                "pattern_name": "test-pattern",
                "description": "A test pattern for E2E"
            }),
        ),
    )
    .await
    .expect("timeout");

    assert!(result.is_object(), "share_pattern should return an object");

    client.shutdown().await;
}

// ── Phase 1 (BP015): Competitors content generation ──────────────

#[tokio::test]
async fn test_setup_competitive_analysis_writes_files() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let competitors = serde_json::json!([{
        "name": "TestRival",
        "url": "https://rival.test",
        "category": "official",
        "description": "A test rival",
        "features": ["search", "chat"],
        "strengths": ["fast"],
        "weaknesses": ["expensive"],
        "pricing": "$10/mo"
    }]);

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "setup_competitive_analysis",
            serde_json::json!({
                "our_name": "TestProduct",
                "our_description": "A test product",
                "our_category": "AI tools",
                "our_features": "export,chat",
                "competitors_json": competitors.to_string(),
                "project_dir": client.tempdir().to_str().unwrap(),
                "write_files": true,
            }),
        ),
    )
    .await
    .expect("timeout");

    // Should have written files
    let files = result
        .get("files_written")
        .and_then(|v| v.as_array())
        .expect("should have files_written array");
    assert!(!files.is_empty(), "should have written at least one file");

    // COMPETITORS.md should be non-empty
    let comp_path = client.tempdir().join("docs").join("COMPETITORS.md");
    let content = std::fs::read_to_string(&comp_path).expect("COMPETITORS.md should exist");
    assert!(!content.is_empty(), "COMPETITORS.md should not be empty");
    assert!(content.contains("TestRival"), "should contain competitor name");
    assert!(content.contains("## Official Competitors"), "should have category section");

    // Comparison page should exist
    let vs_path = client.tempdir().join("docs/vs");
    if vs_path.exists() {
        let entries: Vec<_> = std::fs::read_dir(&vs_path)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(!entries.is_empty(), "should have comparison pages in docs/vs/");
    }

    // Preview should be non-empty
    let preview = result
        .get("competitors_doc_preview")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(!preview.is_empty(), "competitors_doc_preview should not be empty");

    client.shutdown().await;
}

// ── Phase 6: State persistence ───────────────────────────────────────

#[tokio::test]
async fn test_state_survives_restart() {
    let tempdir = tempfile::TempDir::new().unwrap();
    let tempdir_path = tempdir.path().to_path_buf();
    std::fs::create_dir_all(tempdir_path.join(".cruxdev")).unwrap();

    let conv_id: String;

    // First session: start convergence and submit one clean pass
    {
        let client = timeout(TIMEOUT, common::McpTestClient::start_with_dir(tempdir_path.clone()))
            .await
            .expect("timeout");

        let plan_path = client.write_plan(
            "# Plan\n\n**Status:** NOT STARTED\n\n## Phase 1: Test\n- [ ] task\n",
        );

        let start = timeout(
            TIMEOUT,
            client.call_tool(
                "start_convergence",
                serde_json::json!({
                    "plan_file": plan_path.to_str().unwrap(),
                    "project_dir": client.tempdir().to_str().unwrap(),
                    "max_rounds": 5,
                }),
            ),
        )
        .await
        .expect("timeout");

        conv_id = start["convergence_id"]
            .as_str()
            .expect("should have convergence_id")
            .to_string();

        // Submit one clean pass
        timeout(
            TIMEOUT,
            client.call_tool(
                "convergence_submit_result",
                serde_json::json!({
                    "convergence_id": &conv_id,
                    "findings_json": "[]",
                }),
            ),
        )
        .await
        .expect("timeout");

        client.shutdown().await;
    }

    // Second session: verify state persisted
    {
        let client = timeout(TIMEOUT, common::McpTestClient::start_with_dir(tempdir_path))
            .await
            .expect("timeout");

        let status = timeout(
            TIMEOUT,
            client.call_tool(
                "convergence_status",
                serde_json::json!({"convergence_id": &conv_id}),
            ),
        )
        .await
        .expect("timeout");

        // Should find the state from the previous session
        assert!(
            status.get("phase").is_some() || status.get("round").is_some(),
            "should have persisted state, got: {status}"
        );

        client.shutdown().await;
    }

    drop(tempdir);
}

// ── BP016: Git workflow tools ────────────────────────────────────

#[tokio::test]
async fn test_git_status_check() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    // Init a git repo in tempdir with initial commit
    let td = client.tempdir();
    std::process::Command::new("git").args(["init"]).current_dir(td).output().unwrap();
    std::process::Command::new("git").args(["config", "user.email", "test@test.com"]).current_dir(td).output().unwrap();
    std::process::Command::new("git").args(["config", "user.name", "Test"]).current_dir(td).output().unwrap();
    std::fs::write(td.join("README.md"), "# Test").unwrap();
    std::process::Command::new("git").args(["add", "README.md"]).current_dir(td).output().unwrap();
    std::process::Command::new("git").args(["commit", "-m", "init"]).current_dir(td).output().unwrap();

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "git_status_check",
            serde_json::json!({"project_dir": client.tempdir().to_str().unwrap()}),
        ),
    )
    .await
    .expect("timeout");

    assert!(result.get("branch").is_some(), "should have branch: {result}");

    client.shutdown().await;
}

#[tokio::test]
async fn test_git_commit_dry_run() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "git_commit_changes",
            serde_json::json!({
                "message": "test commit",
                "files": "src/main.rs",
                "project_dir": client.tempdir().to_str().unwrap(),
                "dry_run": true,
            }),
        ),
    )
    .await
    .expect("timeout");

    assert_eq!(result.get("dry_run").and_then(|v| v.as_bool()), Some(true));
    assert!(result.get("would_commit").is_some());

    client.shutdown().await;
}

#[tokio::test]
async fn test_git_commit_safety_rejects_target() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "git_commit_changes",
            serde_json::json!({
                "message": "bad commit",
                "files": "rust/target/debug/binary",
                "project_dir": client.tempdir().to_str().unwrap(),
                "dry_run": false,
            }),
        ),
    )
    .await
    .expect("timeout");

    assert!(result.get("error").is_some() || result.get("violations").is_some(),
        "should reject target/ files: {result}");

    client.shutdown().await;
}

#[tokio::test]
async fn test_create_pr_dry_run() {
    let client = timeout(TIMEOUT, common::McpTestClient::start())
        .await
        .expect("timeout");

    let result = timeout(
        TIMEOUT,
        client.call_tool(
            "create_pull_request",
            serde_json::json!({
                "title": "Test PR",
                "body": "Test body",
                "dry_run": true,
            }),
        ),
    )
    .await
    .expect("timeout");

    assert_eq!(result.get("dry_run").and_then(|v| v.as_bool()), Some(true));
    assert!(result.get("would_create_pr").is_some());

    client.shutdown().await;
}
