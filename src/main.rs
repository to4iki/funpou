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
            date,
            limit,
            reverse,
            json,
        } => {
            let date = date
                .as_deref()
                .map(commands::list::parse_date_filter)
                .transpose()?;
            commands::list::execute(&data_path, &config, date, limit, reverse, json)?;
        }
        Command::Config { path } => {
            commands::config::execute(&config, &config_path, path)?;
        }
        Command::Clear { yes } => {
            commands::clear::execute(&data_path, yes)?;
        }
    }

    Ok(())
}
