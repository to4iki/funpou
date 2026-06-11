use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Memo {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<Local>,
}

impl Memo {
    pub fn new(body: String) -> Self {
        Self::at(body, Local::now())
    }

    /// Build a memo with an explicit creation time.
    ///
    /// The id is always derived from `created_at`, keeping the on-disk
    /// identifier and sort key in lockstep. Tests and benchmarks use this to
    /// produce deterministic memos without duplicating the id-formatting rule.
    pub fn at(body: String, created_at: DateTime<Local>) -> Self {
        Self {
            id: created_at.format("%Y%m%d%H%M%S").to_string(),
            body,
            created_at,
        }
    }

    /// Format the memo for display using the given strftime format string.
    pub fn format_display(&self, timestamp_format: &str) -> String {
        format!(
            "{}: {}",
            self.created_at.format(timestamp_format),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_memo_id_derives_from_created_at() {
        let memo = Memo::new("hello world".into());
        // The id is the on-disk identifier and sort key; binding it to
        // created_at locks in the contract that they cannot drift apart.
        assert_eq!(memo.id, memo.created_at.format("%Y%m%d%H%M%S").to_string());
        assert_eq!(memo.body, "hello world");
    }
}
