use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "fnp",
    version,
    about = "Quick one-line memos with automatic timestamps"
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
        /// Filter to memos created today
        #[arg(long)]
        today: bool,

        /// Show only the last N memos
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Reverse order (oldest first)
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

    /// Delete all saved memos after confirmation
    Clear {
        /// Skip the confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}
