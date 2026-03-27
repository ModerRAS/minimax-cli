mod cli;
mod commands;
mod config;
mod config_file;
mod core;
mod keyring;

use clap::Parser;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting MiniMax CLI");
    cli::run().await
}
