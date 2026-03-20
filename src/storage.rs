use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::memo::Memo;

/// Returns the default JSONL storage path.
pub fn default_data_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .context("Could not determine data directory")?
        .join("funpou");
    Ok(data_dir.join("memos.jsonl"))
}

/// Append a single memo to the JSONL file.
pub fn append_memo(path: &Path, memo: &Memo) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    let line = serde_json::to_string(memo).context("Failed to serialize memo")?;
    writeln!(file, "{line}").context("Failed to write memo")?;

    Ok(())
}

/// Read all memos from the JSONL file.
/// Silently skips malformed lines.
pub fn read_all(path: &Path) -> Result<Vec<Memo>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file =
        fs::File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let memos: Vec<Memo> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect();

    Ok(memos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_read_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.jsonl");

        let memo1 = Memo::new("first memo".into());
        let memo2 = Memo::new("second memo".into());

        append_memo(&path, &memo1).unwrap();
        append_memo(&path, &memo2).unwrap();

        let memos = read_all(&path).unwrap();
        assert_eq!(memos.len(), 2);
        assert_eq!(memos[0].body, "first memo");
        assert_eq!(memos[1].body, "second memo");
    }

    #[test]
    fn read_all_returns_empty_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.jsonl");
        let memos = read_all(&path).unwrap();
        assert!(memos.is_empty());
    }

    #[test]
    fn read_all_skips_malformed_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.jsonl");

        let memo = Memo::new("valid memo".into());
        append_memo(&path, &memo).unwrap();

        // Append a malformed line
        let mut file = OpenOptions::new().append(true).open(&path).unwrap();
        writeln!(file, "{{not valid json}}").unwrap();

        let memos = read_all(&path).unwrap();
        assert_eq!(memos.len(), 1);
        assert_eq!(memos[0].body, "valid memo");
    }

    #[test]
    fn creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dir").join("memos.jsonl");

        let memo = Memo::new("nested test".into());
        append_memo(&path, &memo).unwrap();

        let memos = read_all(&path).unwrap();
        assert_eq!(memos.len(), 1);
    }
}
