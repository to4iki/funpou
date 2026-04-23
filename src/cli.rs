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
    ///
    /// Reads from stdin when no TEXT is given and stdin is piped,
    /// e.g. `echo "note" | fnp add` or `pbpaste | fnp add`.
    Add {
        /// Memo text (multiple words are joined with spaces)
        #[arg(num_args = 1..)]
        text: Vec<String>,
    },

    /// List saved memos
    List {
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
