use anyhow::Result;
use tracing::{info, Level};
use clap::Parser;

mod api;
mod config;
mod cli;
mod proxy;
mod core;
mod tasks;
mod captcha;

use cli::{Cli, execute_command};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("Starting Lazabot CLI...");

    let cli = Cli::parse();
    execute_command(cli.command).await?;

    info!("Lazabot CLI completed successfully!");
    Ok(())
}
