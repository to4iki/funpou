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
        let now = Local::now();
        Self {
            id: now.format("%Y%m%d%H%M%S").to_string(),
            body,
            created_at: now,
        }
    }

    /// Format the memo for display using the given timestamp format.
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
    fn new_memo_has_timestamp_id() {
        let memo = Memo::new("test memo".into());
        assert_eq!(memo.id.len(), 14); // YYYYMMDDhhmmss
        assert!(memo.id.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn new_memo_stores_body() {
        let memo = Memo::new("hello world".into());
        assert_eq!(memo.body, "hello world");
    }

    #[test]
    fn serde_roundtrip() {
        let memo = Memo::new("roundtrip test".into());
        let json = serde_json::to_string(&memo).unwrap();
        let deserialized: Memo = serde_json::from_str(&json).unwrap();
        assert_eq!(memo, deserialized);
    }

    #[test]
    fn format_display_default() {
        let memo = Memo::new("display test".into());
        let output = memo.format_display("%Y-%m-%d %H:%M");
        assert!(output.ends_with(": display test"));
        // Timestamp portion should be 16 chars: "YYYY-MM-DD HH:MM"
        let timestamp_part = output.split(": ").next().unwrap();
        assert_eq!(timestamp_part.len(), 16);
    }
}
