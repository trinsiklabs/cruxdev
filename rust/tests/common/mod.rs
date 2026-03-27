use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::transport::TokioChildProcess;
use rmcp::ServiceExt;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use tokio::process::Command;

pub struct McpTestClient {
    service: rmcp::service::RunningService<rmcp::service::RoleClient, ()>,
    _tempdir: Option<TempDir>,
    tempdir_path: PathBuf,
}

impl McpTestClient {
    pub async fn start() -> Self {
        let tempdir = TempDir::new().expect("failed to create tempdir");
        let tempdir_path = tempdir.path().to_path_buf();

        // Create .cruxdev dir for bus.db and state
        std::fs::create_dir_all(tempdir_path.join(".cruxdev")).unwrap();

        let service = Self::connect(&tempdir_path).await;

        Self {
            service,
            _tempdir: Some(tempdir),
            tempdir_path,
        }
    }

    pub async fn start_with_dir(path: PathBuf) -> Self {
        let service = Self::connect(&path).await;
        Self {
            service,
            _tempdir: None,
            tempdir_path: path,
        }
    }

    async fn connect(
        tempdir_path: &Path,
    ) -> rmcp::service::RunningService<rmcp::service::RoleClient, ()> {
        let binary = env!("CARGO_BIN_EXE_cruxdev");

        let mut cmd = Command::new(binary);
        cmd.args(["mcp", "start"])
            .env("HOME", tempdir_path)
            .current_dir(tempdir_path);

        let transport =
            TokioChildProcess::new(cmd).expect("failed to spawn cruxdev");

        ().serve(transport)
            .await
            .expect("MCP handshake failed")
    }

    pub fn peer(&self) -> &rmcp::Peer<rmcp::service::RoleClient> {
        self.service.peer()
    }

    pub fn peer_info(&self) -> &rmcp::model::ServerInfo {
        self.service.peer_info().expect("no peer info available")
    }

    pub async fn list_all_tools(&self) -> Vec<rmcp::model::Tool> {
        self.service.list_all_tools().await.expect("list_all_tools failed")
    }

    pub async fn call_tool_raw(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> CallToolResult {
        let tool_name: String = name.to_string();
        self.peer()
            .call_tool(CallToolRequestParams {
                meta: None,
                name: tool_name.into(),
                arguments: args.as_object().cloned(),
                task: None,
            })
            .await
            .expect("tool call failed")
    }

    pub async fn call_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> serde_json::Value {
        let result = self.call_tool_raw(name, args).await;
        let text = result
            .content
            .first()
            .and_then(|c| c.as_text())
            .map(|t| &t.text)
            .expect("no text content in response");
        serde_json::from_str(text).unwrap_or(serde_json::Value::String(text.clone()))
    }

    pub fn write_plan(&self, content: &str) -> PathBuf {
        let plan_path = self.tempdir_path.join("plan.md");
        std::fs::write(&plan_path, content).unwrap();
        plan_path
    }

    pub fn tempdir(&self) -> &Path {
        &self.tempdir_path
    }

    pub async fn shutdown(self) {
        let _ = self.service.cancel().await;
    }
}
