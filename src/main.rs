mod cli;
mod config;
mod commands;
mod core;

use clap::Parser;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting MiniMax CLI");
    cli::run().await
}
