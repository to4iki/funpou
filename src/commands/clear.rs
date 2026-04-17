use std::io::{self, BufRead, Write};
use std::path::Path;

use anyhow::Result;

use crate::storage;

/// Treat only an explicit "y" / "yes" (case-insensitive) as confirmation.
/// Any other input — including empty — is a refusal, matching the `[y/N]` default.
fn is_confirmed(input: &str) -> bool {
    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

pub fn execute(data_path: &Path, yes: bool) -> Result<()> {
    let memos = storage::read_all(data_path)?;

    if memos.is_empty() {
        eprintln!("No memos to clear.");
        return Ok(());
    }

    let count = memos.len();

    if !yes {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        write!(stdout, "Clear {count} memo(s)? [y/N]: ")?;
        stdout.flush()?;

        let mut answer = String::new();
        io::stdin().lock().read_line(&mut answer)?;

        if !is_confirmed(&answer) {
            eprintln!("Cancelled.");
            return Ok(());
        }
    }

    storage::clear_all(data_path)?;
    eprintln!("Cleared {count} memo(s).");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirms_on_y_and_yes_case_insensitive() {
        for input in ["y", "Y", "yes", "YES", "Yes", " y \n", "yes\n"] {
            assert!(is_confirmed(input), "expected confirmed for {input:?}");
        }
    }

    #[test]
    fn refuses_on_anything_else() {
        for input in ["", "\n", "n", "N", "no", "NO", "maybe", "yep", "yy"] {
            assert!(!is_confirmed(input), "expected refused for {input:?}");
        }
    }
}
