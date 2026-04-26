mod cli;
mod commands;
mod config;
mod memo;
mod obsidian;
mod storage;

use std::io::{IsTerminal, Read};

use anyhow::Result;
use chrono::Local;
use clap::Parser;

use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = config::default_config_path()?;
    let config = config::load_config(&config_path)?;
    let data_path = storage::default_data_path()?;

    match cli.command {
        Command::Add { text } => {
            // When no args are given and stdin is piped, read the memo body from stdin
            let stdin_body = if text.is_empty() && !std::io::stdin().is_terminal() {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                Some(buf)
            } else {
                None
            };
            commands::add::execute(text, stdin_body, &data_path, &config)?;
        }
        Command::List {
            today,
            reverse,
            json,
        } => {
            let date = today.then(|| Local::now().date_naive());
            commands::list::execute(&data_path, &config, date, reverse, json)?;
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
