use std::path::Path;

use anyhow::Result;

use crate::config::Config;
use crate::memo::Memo;
use crate::storage;

/// Apply limit and reverse to a list of memos.
///
/// Limit selects the N most recent (last) memos, then default display order
/// is newest first and `reverse` flips that to oldest first.
fn prepare_memos(mut memos: Vec<Memo>, limit: Option<usize>, reverse: bool) -> Vec<Memo> {
    // Apply limit: keep only the last N (most recent) memos
    if let Some(n) = limit {
        let len = memos.len();
        if n < len {
            memos = memos.split_off(len - n);
        }
    }

    // Default: newest first (reverse chronological)
    // --reverse: oldest first (chronological)
    memos.reverse();

    if reverse {
        memos.reverse();
    }

    memos
}

pub fn execute(
    data_path: &Path,
    config: &Config,
    limit: Option<usize>,
    reverse: bool,
    json: bool,
) -> Result<()> {
    let memos = storage::read_all(data_path)?;

    if memos.is_empty() {
        eprintln!("No memos found.");
        return Ok(());
    }

    let memos = prepare_memos(memos, limit, reverse);

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

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Local, TimeZone};

    /// Create a Memo with a specific timestamp for deterministic testing.
    fn memo_at(body: &str, year: i32, month: u32, day: u32) -> Memo {
        let dt = Local.with_ymd_and_hms(year, month, day, 12, 0, 0).unwrap();
        Memo {
            id: dt.format("%Y%m%d%H%M%S").to_string(),
            body: body.to_string(),
            created_at: dt,
        }
    }

    fn bodies(memos: &[Memo]) -> Vec<&str> {
        memos.iter().map(|m| m.body.as_str()).collect()
    }

    #[test]
    fn limit_returns_most_recent_memos() {
        let memos = vec![
            memo_at("oldest", 2025, 1, 1),
            memo_at("middle", 2025, 6, 1),
            memo_at("newest", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, Some(2), false);
        assert_eq!(bodies(&result), vec!["newest", "middle"]);
    }

    #[test]
    fn limit_with_reverse_returns_most_recent_in_oldest_first_order() {
        let memos = vec![
            memo_at("memo-1", 2025, 1, 1),
            memo_at("memo-2", 2025, 3, 1),
            memo_at("memo-3", 2025, 5, 1),
            memo_at("memo-4", 2025, 7, 1),
            memo_at("memo-5", 2025, 9, 1),
        ];

        // `--reverse -n 3` should return the 3 most recent memos oldest first
        let result = prepare_memos(memos, Some(3), true);
        assert_eq!(bodies(&result), vec!["memo-3", "memo-4", "memo-5"]);
    }

    #[test]
    fn reverse_without_limit_returns_chronological_order() {
        let memos = vec![
            memo_at("first", 2025, 1, 1),
            memo_at("second", 2025, 6, 1),
            memo_at("third", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, true);
        assert_eq!(bodies(&result), vec!["first", "second", "third"]);
    }

    #[test]
    fn limit_exceeding_total_returns_all() {
        let memos = vec![
            memo_at("only-one", 2025, 1, 1),
            memo_at("only-two", 2025, 6, 1),
        ];

        let result = prepare_memos(memos, Some(10), false);
        assert_eq!(bodies(&result), vec!["only-two", "only-one"]);
    }

    #[test]
    fn no_limit_no_reverse_returns_newest_first() {
        let memos = vec![
            memo_at("a", 2025, 1, 1),
            memo_at("b", 2025, 6, 1),
            memo_at("c", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, false);
        assert_eq!(bodies(&result), vec!["c", "b", "a"]);
    }
}
