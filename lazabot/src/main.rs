use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};

mod api;
mod captcha;
mod cli;
mod config;
mod core;
mod proxy;
mod tasks;

use cli::{execute_command, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    info!("Starting Lazabot CLI...");

    let cli = Cli::parse();
    execute_command(cli.command).await?;

    info!("Lazabot CLI completed successfully!");
    Ok(())
}
