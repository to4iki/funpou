use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::memo::Memo;

/// Resolve the data file path from home and XDG_DATA_HOME, preferring XDG.
fn data_path_from(home: Option<&Path>, xdg_data_home: Option<&Path>) -> Option<PathBuf> {
    let base = xdg_data_home
        .map(PathBuf::from)
        .or_else(|| home.map(|h| h.join(".local").join("share")))?;
    Some(base.join("funpou").join("memos.jsonl"))
}

/// Returns the default JSONL storage path.
/// Prefers `$XDG_DATA_HOME/funpou/memos.jsonl`, falling back to `~/.local/share/funpou/memos.jsonl`.
pub fn default_data_path() -> Result<PathBuf> {
    let home = dirs::home_dir();
    let xdg = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from);
    data_path_from(home.as_deref(), xdg.as_deref()).context("Could not determine data directory")
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

/// Remove the JSONL file. No-op if the file does not exist.
pub fn clear_all(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    fs::remove_file(path).with_context(|| format!("Failed to remove file: {}", path.display()))
}

/// Read all memos from the JSONL file.
pub fn read_all(path: &Path) -> Result<Vec<Memo>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file =
        fs::File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let mut memos = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line.with_context(|| {
            format!(
                "Failed to read memo file line {line_number}: {}",
                path.display()
            )
        })?;

        if line.trim().is_empty() {
            continue;
        }

        let memo = serde_json::from_str(&line).with_context(|| {
            format!(
                "Failed to parse memo file line {line_number}: {}",
                path.display()
            )
        })?;
        memos.push(memo);
    }

    Ok(memos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_read_preserves_memos_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.jsonl");

        let memo1 = Memo::new("first memo".into());
        let memo2 = Memo::new("second memo".into());

        append_memo(&path, &memo1).unwrap();
        append_memo(&path, &memo2).unwrap();

        // Full equality also covers the JSONL serde roundtrip
        // (id, body, created_at all survive a write/read cycle).
        assert_eq!(read_all(&path).unwrap(), vec![memo1, memo2]);
    }

    #[test]
    fn clear_all_removes_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.jsonl");

        append_memo(&path, &Memo::new("to be cleared".into())).unwrap();
        assert!(path.exists());

        clear_all(&path).unwrap();
        assert!(!path.exists());
        assert!(read_all(&path).unwrap().is_empty());
    }

    #[test]
    fn clear_all_is_noop_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.jsonl");
        clear_all(&path).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn read_all_returns_empty_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.jsonl");
        let memos = read_all(&path).unwrap();
        assert!(memos.is_empty());
    }

    #[test]
    fn read_all_errors_on_malformed_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.jsonl");

        let memo = Memo::new("valid memo".into());
        append_memo(&path, &memo).unwrap();

        // Append a malformed line
        let mut file = OpenOptions::new().append(true).open(&path).unwrap();
        writeln!(file, "{{not valid json}}").unwrap();

        let err = read_all(&path).unwrap_err();
        assert!(err.to_string().contains("line 2"));
    }

    #[test]
    fn creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dir").join("memos.jsonl");

        let memo = Memo::new("nested test".into());
        append_memo(&path, &memo).unwrap();

        assert_eq!(read_all(&path).unwrap(), vec![memo]);
    }

    #[test]
    fn data_path_prefers_xdg_data_home() {
        let home = Path::new("/Users/foo");
        let xdg = Path::new("/custom/data");
        let path = data_path_from(Some(home), Some(xdg)).unwrap();
        assert_eq!(path, PathBuf::from("/custom/data/funpou/memos.jsonl"));
    }

    #[test]
    fn data_path_falls_back_to_local_share() {
        let home = Path::new("/Users/foo");
        let path = data_path_from(Some(home), None).unwrap();
        assert_eq!(
            path,
            PathBuf::from("/Users/foo/.local/share/funpou/memos.jsonl")
        );
    }

    #[test]
    fn data_path_none_without_home_or_xdg() {
        assert!(data_path_from(None, None).is_none());
    }
}
