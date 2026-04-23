use std::path::Path;

use anyhow::Result;
use chrono::NaiveDate;

use crate::config::Config;
use crate::memo::Memo;
use crate::storage;

/// Apply date filter, limit, and reverse to a list of memos.
///
/// Order of operations matters: date filter narrows the set first, then limit
/// selects the N most recent (last) memos from the filtered set, then default
/// display order is newest first and `reverse` flips that to oldest first.
fn prepare_memos(
    mut memos: Vec<Memo>,
    date: Option<NaiveDate>,
    limit: Option<usize>,
    reverse: bool,
) -> Vec<Memo> {
    // Filter by date first so limit/reverse operate on the narrowed set
    if let Some(target) = date {
        memos.retain(|m| m.created_at.date_naive() == target);
    }

    // Apply limit: keep only the last N (most recent) memos
    if let Some(n) = limit {
        let len = memos.len();
        if n < len {
            memos = memos.split_off(len - n);
        }
    }

    // Default: newest first (reverse chronological)
    // --reverse: oldest first (chronological)
    if !reverse {
        memos.reverse();
    }

    memos
}

pub fn execute(
    data_path: &Path,
    config: &Config,
    date: Option<NaiveDate>,
    limit: Option<usize>,
    reverse: bool,
    json: bool,
) -> Result<()> {
    let memos = storage::read_all(data_path)?;

    if memos.is_empty() {
        eprintln!("No memos found.");
        return Ok(());
    }

    let memos = prepare_memos(memos, date, limit, reverse);

    if memos.is_empty() {
        eprintln!("No memos found.");
        return Ok(());
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

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Local, TimeZone};

    /// Create a Memo with a specific timestamp for deterministic testing.
    fn memo_at(body: &str, year: i32, month: u32, day: u32) -> Memo {
        memo_at_hour(body, year, month, day, 12)
    }

    fn memo_at_hour(body: &str, year: i32, month: u32, day: u32, hour: u32) -> Memo {
        let dt = Local
            .with_ymd_and_hms(year, month, day, hour, 0, 0)
            .unwrap();
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

        let result = prepare_memos(memos, None, Some(2), false);
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
        let result = prepare_memos(memos, None, Some(3), true);
        assert_eq!(bodies(&result), vec!["memo-3", "memo-4", "memo-5"]);
    }

    #[test]
    fn reverse_without_limit_returns_chronological_order() {
        let memos = vec![
            memo_at("first", 2025, 1, 1),
            memo_at("second", 2025, 6, 1),
            memo_at("third", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, None, true);
        assert_eq!(bodies(&result), vec!["first", "second", "third"]);
    }

    #[test]
    fn limit_exceeding_total_returns_all() {
        let memos = vec![
            memo_at("only-one", 2025, 1, 1),
            memo_at("only-two", 2025, 6, 1),
        ];

        let result = prepare_memos(memos, None, Some(10), false);
        assert_eq!(bodies(&result), vec!["only-two", "only-one"]);
    }

    #[test]
    fn no_limit_no_reverse_returns_newest_first() {
        let memos = vec![
            memo_at("a", 2025, 1, 1),
            memo_at("b", 2025, 6, 1),
            memo_at("c", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, None, false);
        assert_eq!(bodies(&result), vec!["c", "b", "a"]);
    }

    #[test]
    fn date_filter_keeps_only_memos_on_given_date() {
        let memos = vec![
            memo_at_hour("earlier-day", 2025, 6, 1, 9),
            memo_at_hour("target-morning", 2025, 6, 2, 9),
            memo_at_hour("target-evening", 2025, 6, 2, 20),
            memo_at_hour("later-day", 2025, 6, 3, 9),
        ];

        let target = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let result = prepare_memos(memos, Some(target), None, false);

        // Newest first within the same day
        assert_eq!(bodies(&result), vec!["target-evening", "target-morning"]);
    }

    #[test]
    fn date_filter_with_no_matches_returns_empty() {
        let memos = vec![memo_at("some memo", 2025, 6, 1)];
        let target = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let result = prepare_memos(memos, Some(target), None, false);
        assert!(result.is_empty());
    }

    #[test]
    fn date_filter_applies_before_limit() {
        // Limit should count memos from the filtered set, not the original list
        let memos = vec![
            memo_at_hour("other-day-1", 2025, 5, 1, 10),
            memo_at_hour("other-day-2", 2025, 5, 2, 10),
            memo_at_hour("target-1", 2025, 6, 1, 9),
            memo_at_hour("target-2", 2025, 6, 1, 12),
            memo_at_hour("target-3", 2025, 6, 1, 18),
        ];
        let target = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();

        let result = prepare_memos(memos, Some(target), Some(2), false);
        assert_eq!(bodies(&result), vec!["target-3", "target-2"]);
    }
}
