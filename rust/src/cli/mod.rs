//! CLI interface via clap.

use clap::Parser;

#[derive(Parser)]
#[command(name = "cruxdev", about = "Autonomous convergence engine — single binary")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Start the MCP server (stdio transport)
    Mcp {
        #[command(subcommand)]
        action: McpAction,
    },
    /// Show convergence engine health status
    Status,
    /// Install CruxDev into a project
    Install {
        /// Project directory (default: current)
        #[arg(default_value = ".")]
        project_dir: String,
    },
}

#[derive(clap::Subcommand)]
enum McpAction {
    /// Start the MCP server
    Start,
}

impl Cli {
    pub async fn run(self) {
        match self.command {
            Some(Commands::Mcp { action }) => match action {
                McpAction::Start => {
                    crate::server::run_server().await;
                }
            },
            Some(Commands::Status) => {
                let dir = std::env::current_dir().unwrap_or_default();
                let report = crate::status::get_status(dir.to_str().unwrap_or("."));
                println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
            }
            Some(Commands::Install { project_dir }) => {
                let result = crate::install::install(&project_dir);
                println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
            }
            None => {
                println!("CruxDev convergence engine. Use --help for commands.");
            }
        }
    }
}
