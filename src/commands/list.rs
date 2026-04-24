use std::path::Path;

use anyhow::Result;
use chrono::NaiveDate;

use crate::config::Config;
use crate::memo::Memo;
use crate::storage;

/// Apply date filter and reverse to a list of memos.
///
/// Order of operations matters: date filter narrows the set first, then default
/// display order is newest first and `reverse` flips that to oldest first.
fn prepare_memos(mut memos: Vec<Memo>, date: Option<NaiveDate>, reverse: bool) -> Vec<Memo> {
    if let Some(target) = date {
        memos.retain(|m| m.created_at.date_naive() == target);
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
    reverse: bool,
    json: bool,
) -> Result<()> {
    let memos = storage::read_all(data_path)?;

    if memos.is_empty() {
        eprintln!("No memos found.");
        return Ok(());
    }

    let memos = prepare_memos(memos, date, reverse);

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
    fn reverse_returns_chronological_order() {
        let memos = vec![
            memo_at("first", 2025, 1, 1),
            memo_at("second", 2025, 6, 1),
            memo_at("third", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, true);
        assert_eq!(bodies(&result), vec!["first", "second", "third"]);
    }

    #[test]
    fn default_returns_newest_first() {
        let memos = vec![
            memo_at("a", 2025, 1, 1),
            memo_at("b", 2025, 6, 1),
            memo_at("c", 2025, 12, 1),
        ];

        let result = prepare_memos(memos, None, false);
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
        let result = prepare_memos(memos, Some(target), false);

        // Newest first within the same day
        assert_eq!(bodies(&result), vec!["target-evening", "target-morning"]);
    }

    #[test]
    fn date_filter_with_no_matches_returns_empty() {
        let memos = vec![memo_at("some memo", 2025, 6, 1)];
        let target = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let result = prepare_memos(memos, Some(target), false);
        assert!(result.is_empty());
    }
}
