use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::config::Config;
use crate::memo::Memo;
use crate::template;

/// Format a memo entry using the configured entry format template.
fn format_entry(memo: &Memo, config: &Config) -> String {
    template::render(
        &config.obsidian.entry_format,
        &memo.created_at,
        Some(&memo.body),
    )
}

/// Insert an entry under the target heading in the given content.
/// Returns the modified content.
fn insert_under_heading(content: &str, heading: &str, entry: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let heading_trimmed = heading.trim();
    let heading_level = heading_trimmed.chars().take_while(|&c| c == '#').count();

    // Find the target heading
    let heading_idx = lines.iter().position(|line| line.trim() == heading_trimmed);

    match heading_idx {
        Some(idx) => {
            // Find the next heading of equal or higher level
            let insert_before = lines[idx + 1..]
                .iter()
                .position(|line| {
                    let trimmed = line.trim();
                    let level = trimmed.chars().take_while(|&c| c == '#').count();
                    level > 0
                        && level <= heading_level
                        && trimmed.len() > level
                        && trimmed.as_bytes()[level] == b' '
                })
                .map(|pos| idx + 1 + pos);

            let insert_at = insert_before.unwrap_or(lines.len());

            let mut output = lines[..insert_at].join("\n");
            output.push('\n');
            output.push_str(entry);
            output.push('\n');
            if insert_at < lines.len() {
                output.push_str(&lines[insert_at..].join("\n"));
                // Preserve trailing newline if original had one
                if content.ends_with('\n') {
                    output.push('\n');
                }
            } else if content.ends_with('\n') {
                // File ended with newline, keep it
            }

            output
        }
        None => {
            // Heading not found — append heading + entry at end
            let mut output = content.to_string();
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            output.push('\n');
            output.push_str(heading);
            output.push('\n');
            output.push_str(entry);
            output.push('\n');
            output
        }
    }
}

/// Append a memo to the Obsidian vault file.
pub fn append_memo(memo: &Memo, config: &Config) -> Result<()> {
    let relative_path = template::render(&config.obsidian.template_path, &memo.created_at, None);
    let file_path: PathBuf = [&config.obsidian.vault_path, &relative_path]
        .iter()
        .collect();

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let entry = format_entry(memo, config);

    let content = if file_path.exists() {
        fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read: {}", file_path.display()))?
    } else {
        String::new()
    };

    let new_content = insert_under_heading(&content, &config.obsidian.target_heading, &entry);

    fs::write(&file_path, &new_content)
        .with_context(|| format!("Failed to write: {}", file_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Local, TimeZone};

    fn fixed_time() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 3, 20, 14, 5, 32).unwrap()
    }

    #[test]
    fn insert_under_existing_heading_places_entry_just_before_next_sibling() {
        let content = "# Title\n\n## Memos\n- old entry\n\n## Other\nstuff\n";
        let result = insert_under_heading(content, "## Memos", "- new entry");
        assert_eq!(
            result,
            "# Title\n\n## Memos\n- old entry\n\n- new entry\n## Other\nstuff\n"
        );
    }

    #[test]
    fn insert_under_heading_skips_deeper_subheadings() {
        // `### Sub` is deeper than `## Memos`, so it belongs to the Memos
        // section and the entry must be placed AFTER it, not before.
        // The next stop is the sibling `## Other`.
        let content = "## Memos\n- old\n\n### Sub\nstuff\n\n## Other\nfinal\n";
        let result = insert_under_heading(content, "## Memos", "- new");
        assert_eq!(
            result,
            "## Memos\n- old\n\n### Sub\nstuff\n\n- new\n## Other\nfinal\n"
        );
    }

    #[test]
    fn insert_creates_heading_when_missing() {
        let content = "# Title\n\nSome content\n";
        let result = insert_under_heading(content, "## Memos", "- first entry");
        assert_eq!(
            result,
            "# Title\n\nSome content\n\n## Memos\n- first entry\n"
        );
    }

    #[test]
    fn insert_into_empty_file() {
        let result = insert_under_heading("", "## Memos", "- first entry");
        assert_eq!(result, "\n## Memos\n- first entry\n");
    }

    #[test]
    fn insert_at_end_when_no_next_heading() {
        let content = "# Title\n\n## Memos\n- old entry\n";
        let result = insert_under_heading(content, "## Memos", "- new entry");
        assert_eq!(result, "# Title\n\n## Memos\n- old entry\n- new entry\n");
    }

    #[test]
    fn insert_under_heading_appends_sequentially() {
        // Two consecutive memos should accumulate in chronological order
        // under the heading, mirroring how `append_memo` is called repeatedly.
        let content = "# 2026-03-20\n\n## Memos\n";
        let after_first = insert_under_heading(content, "## Memos", "- first");
        let after_second = insert_under_heading(&after_first, "## Memos", "- second");
        assert_eq!(
            after_second,
            "# 2026-03-20\n\n## Memos\n- first\n- second\n"
        );
    }

    #[test]
    fn format_entry_substitutes_timestamp_and_body() {
        let config = Config::default();
        let memo = Memo {
            id: "20260320140532".into(),
            body: "test memo".into(),
            created_at: fixed_time(),
        };
        assert_eq!(
            format_entry(&memo, &config),
            "- 2026-03-20-14:05: test memo"
        );
    }

    #[test]
    fn append_memo_writes_resolved_path_with_heading_and_entry() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = Config::default();
        config.obsidian.vault_path = dir.path().to_string_lossy().into();
        config.obsidian.template_path = "daily/{YYYY}/{YYYY-MM}.md".into();

        let memo = Memo {
            id: "20260320140532".into(),
            body: "obsidian test".into(),
            created_at: fixed_time(),
        };

        append_memo(&memo, &config).unwrap();

        let file_path = dir.path().join("daily/2026/2026-03.md");
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "\n## Memos\n- 2026-03-20-14:05: obsidian test\n");
    }
}
