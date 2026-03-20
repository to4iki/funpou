mod cli;
mod commands;
mod config;
mod memo;
mod obsidian;
mod storage;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = config::default_config_path()?;
    let config = config::load_config(&config_path)?;
    let data_path = storage::default_data_path()?;

    match cli.command {
        Command::Add { text } => {
            commands::add::execute(text, &data_path, &config)?;
        }
        Command::List {
            limit,
            reverse,
            json,
        } => {
            commands::list::execute(&data_path, &config, limit, reverse, json)?;
        }
        Command::Config { path } => {
            commands::config::execute(&config, &config_path, path)?;
        }
    }

    Ok(())
}
