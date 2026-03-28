pub mod adoption;
pub mod bus;
pub mod cli;
pub mod competitors;
pub mod dispatch;
pub mod domain;
pub mod engine;
pub mod evolution;
pub mod git;
pub mod growth;
pub mod graph;
pub mod gtv;
pub mod improvement;
pub mod install;
pub mod normalize;
pub mod research;
pub mod server;
pub mod stream;
pub mod status;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.run().await;
}
