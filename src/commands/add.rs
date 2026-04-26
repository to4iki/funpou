use std::path::Path;

use anyhow::{Result, bail};

use crate::config::Config;
use crate::memo::Memo;
use crate::obsidian;
use crate::storage;

/// Resolve the memo body from CLI args and optional stdin input.
///
/// CLI args take precedence; stdin is used only when no args are given.
/// Returns an error when neither source yields non-empty text.
pub fn resolve_body(text: &[String], stdin: Option<String>) -> Result<String> {
    if !text.is_empty() {
        return Ok(text.join(" "));
    }

    match stdin {
        Some(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                bail!("No memo text provided (stdin was empty). See `fnp add --help`.");
            }
            Ok(trimmed.to_string())
        }
        None => {
            bail!("No memo text provided. Pass TEXT args or pipe via stdin. See `fnp add --help`.")
        }
    }
}

pub fn execute(
    text: Vec<String>,
    stdin: Option<String>,
    data_path: &Path,
    config: &Config,
) -> Result<()> {
    let body = resolve_body(&text, stdin)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_body_joins_cli_args_with_spaces() {
        let body = resolve_body(&["hello".into(), "world".into()], None).unwrap();
        assert_eq!(body, "hello world");
    }

    #[test]
    fn resolve_body_uses_stdin_when_no_args() {
        let body = resolve_body(&[], Some("from pipe".into())).unwrap();
        assert_eq!(body, "from pipe");
    }

    #[test]
    fn resolve_body_prefers_cli_args_over_stdin() {
        // When the user passes args AND pipes data, args win — stdin is ignored.
        // This keeps `fnp add foo` predictable even in unusual shell setups.
        let body = resolve_body(&["arg".into()], Some("pipe text".into())).unwrap();
        assert_eq!(body, "arg");
    }

    #[test]
    fn resolve_body_errors_without_args_or_stdin() {
        assert!(resolve_body(&[], None).is_err());
        assert!(resolve_body(&[], Some("   \n\t\n".into())).is_err());
    }

    #[test]
    fn resolve_body_only_trims_surrounding_whitespace_from_stdin() {
        // Surrounding whitespace is trimmed; internal newlines are kept verbatim.
        let body = resolve_body(&[], Some("  line1\nline2  \n".into())).unwrap();
        assert_eq!(body, "line1\nline2");
    }
}
