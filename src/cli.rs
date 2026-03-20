use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "funpou",
    version,
    about = "A minimal CLI tool for quick one-line memos"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Save a one-line memo with an automatic timestamp
    Add {
        /// Memo text (multiple words are joined with spaces)
        #[arg(required = true, num_args = 1..)]
        text: Vec<String>,
    },

    /// List saved memos
    List {
        /// Show only the last N memos
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Reverse order (oldest last)
        #[arg(short, long)]
        reverse: bool,

        /// Output raw JSONL for scripting
        #[arg(long)]
        json: bool,
    },

    /// Show resolved configuration
    Config {
        /// Print config file path only
        #[arg(long)]
        path: bool,
    },
}
