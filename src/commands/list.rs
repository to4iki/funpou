use std::path::Path;

use anyhow::Result;

use crate::config::Config;
use crate::storage;

pub fn execute(
    data_path: &Path,
    config: &Config,
    limit: Option<usize>,
    reverse: bool,
    json: bool,
) -> Result<()> {
    let mut memos = storage::read_all(data_path)?;

    if memos.is_empty() {
        eprintln!("No memos found.");
        return Ok(());
    }

    // Default: newest last (chronological order as stored)
    // --reverse: oldest last (reverse chronological)
    if reverse {
        memos.reverse();
    }

    // Apply limit (take last N before any reversal was applied)
    if let Some(n) = limit {
        let len = memos.len();
        if n < len {
            memos = memos.split_off(len - n);
        }
    }

    for memo in &memos {
        if json {
            let line = serde_json::to_string(memo)?;
            println!("{line}");
        } else {
            println!("{}", memo.format_display(&config.timestamp_format));
        }
    }

    Ok(())
}
