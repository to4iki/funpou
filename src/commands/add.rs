use std::path::Path;

use anyhow::Result;

use crate::config::Config;
use crate::memo::Memo;
use crate::obsidian;
use crate::storage;

pub fn execute(text: Vec<String>, data_path: &Path, config: &Config) -> Result<()> {
    let body = text.join(" ");
    let memo = Memo::new(body);

    // Always persist to JSONL first (source of truth)
    storage::append_memo(data_path, &memo)?;

    // Print confirmation to stderr
    let display = memo.format_display(&config.timestamp_format);
    eprintln!("{display}");

    // Optionally append to Obsidian vault
    if config.obsidian.is_enabled()
        && let Err(e) = obsidian::append_memo(&memo, config)
    {
        eprintln!("Warning: Obsidian sync failed: {e}");
    }

    Ok(())
}
