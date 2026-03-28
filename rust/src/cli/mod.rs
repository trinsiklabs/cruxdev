//! CLI interface via clap.

use clap::Parser;
use std::io::Write;

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
    /// Show prioritized work queue — what to work on next
    Prioritize {
        /// Project directory (default: current)
        #[arg(default_value = ".")]
        project_dir: String,
        /// GitHub repo (e.g., "trinsiklabs/cruxdev")
        #[arg(long, default_value = "")]
        repo: String,
        /// Max items to show (default: 10)
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Stream terminal output via SSE for live viewer
    Stream {
        /// Log file to tail (default: .cruxdev/evolution/cron.log)
        #[arg(long, default_value = ".cruxdev/evolution/cron.log")]
        log_file: String,
        /// Port to listen on (default: 8765)
        #[arg(long, default_value_t = 8765)]
        port: u16,
    },
    /// Run an autonomous evolution cycle (gather → evaluate → integrate → post → engage)
    Evolve {
        /// Project directory (default: current)
        #[arg(default_value = ".")]
        project_dir: String,
        /// GitHub repo (e.g., "trinsiklabs/cruxdev")
        #[arg(long, default_value = "")]
        repo: String,
        /// Dry run — log actions without executing (default: true)
        #[arg(long, default_value_t = true)]
        dry_run: bool,
        /// Run continuously until no new signals found
        #[arg(long)]
        continuous: bool,
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
            Some(Commands::Stream { log_file, port }) => {
                crate::stream::run_sse_server(log_file, port).await;
            }
            Some(Commands::Prioritize { project_dir, repo, limit }) => {
                let items = crate::engine::priority::scan_work_sources(&project_dir, &repo);
                let total = items.len();
                let top: Vec<_> = items.into_iter().take(limit).collect();
                if top.is_empty() {
                    println!("No work items found. System is idle.");
                } else {
                    println!("Priority Queue ({} items, showing top {}):\n", total, top.len());
                    for (i, item) in top.iter().enumerate() {
                        println!("  {}. [{}] {} (score: {})", i + 1, item.source, item.title, item.score);
                        println!("     Action: {} — {}", item.action, item.description);
                        println!();
                    }
                }
            }
            Some(Commands::Evolve { project_dir, repo, dry_run, continuous }) => {
                run_evolve(&project_dir, &repo, dry_run, continuous);
            }
            None => {
                println!("CruxDev convergence engine. Use --help for commands.");
            }
        }
    }
}

fn run_evolve(project_dir: &str, repo: &str, dry_run: bool, continuous: bool) {
    use crate::evolution;

    let context_path = format!("{project_dir}/.cruxdev/evolution/context.json");
    let archive_path = format!("{project_dir}/.cruxdev/evolution/archive.jsonl");
    let log_path = format!("{project_dir}/.cruxdev/evolution/run.log");

    // Ensure dirs exist
    let _ = std::fs::create_dir_all(format!("{project_dir}/.cruxdev/evolution/posts"));

    let mut state = evolution::load_context(&context_path)
        .unwrap_or_else(|_| evolution::EvolutionState::new("cruxdev"));

    let mut log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .ok();

    let log = |msg: &str, file: &mut Option<std::fs::File>| {
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{ts}] {msg}");
        eprintln!("{line}");
        if let Some(f) = file.as_mut() {
            let _ = writeln!(f, "{line}");
        }
    };

    log(&format!("Evolution starting. dry_run={dry_run} continuous={continuous}"), &mut log_file);

    loop {
        // Check STOP file
        if evolution::is_stopped(project_dir) {
            log("STOP file detected. Halting.", &mut log_file);
            break;
        }

        let cycle = evolution::run_cycle(&mut state, project_dir, repo, dry_run);

        let gathered_count = cycle.gathered.len();
        let evaluated_count = cycle.evaluated.len();
        let posted_count = cycle.posted.len();
        let engaged_count = cycle.engaged.len();

        log(&format!(
            "Cycle #{}: gathered={gathered_count} evaluated={evaluated_count} posted={posted_count} engaged={engaged_count}",
            cycle.cycle_id
        ), &mut log_file);

        if let Some(err) = &cycle.error {
            log(&format!("Error: {err}"), &mut log_file);
        }

        // Persist
        let _ = evolution::append_to_archive(&archive_path, &cycle);
        let _ = evolution::save_context(&context_path, &state);

        // Print summary
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "cycle": cycle.cycle_id,
            "gathered": gathered_count,
            "evaluated": evaluated_count,
            "posted": posted_count,
            "engaged": engaged_count,
            "error": cycle.error,
        })).unwrap_or_default());

        if !continuous || gathered_count == 0 {
            log("No more signals or not in continuous mode. Done.", &mut log_file);
            break;
        }
    }

    log("Evolution complete.", &mut log_file);
}
