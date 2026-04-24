use chrono::{DateTime, Local};

/// Convert Moment.js-style date tokens to chrono strftime specifiers.
///
/// Scans left-to-right consuming one token at a time. A sequential
/// `replace` chain is unsafe here: `replace("MM","%m")` injects a literal
/// `m` that a later `replace("mm","%M")` can re-glue with an adjacent `m`,
/// so `"MMmm"` would collapse to `"%%Mm"` instead of `"%m%M"`.
fn tokens_to_strftime(format: &str) -> String {
    const TOKENS: &[(&str, &str)] = &[
        ("YYYY", "%Y"),
        ("MM", "%m"),
        ("DD", "%d"),
        ("HH", "%H"),
        ("mm", "%M"),
        ("ss", "%S"),
    ];

    let mut result = String::with_capacity(format.len());
    let mut i = 0;
    while i < format.len() {
        let rest = &format[i..];
        if let Some((tok, sub)) = TOKENS.iter().find(|(t, _)| rest.starts_with(t)) {
            result.push_str(sub);
            i += tok.len();
        } else {
            let ch = rest.chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    result
}

/// Render a template by expanding `{...}` expressions.
///
/// - `{body}` is replaced with `body` (or empty string if `None`).
/// - Any other `{...}` content is treated as a date-format expression:
///   tokens are converted via [`tokens_to_strftime`] and formatted against `now`.
/// - Text outside braces is copied verbatim.
/// - A `{` with no matching `}` is kept as a literal.
pub fn render(template: &str, now: &DateTime<Local>, body: Option<&str>) -> String {
    let mut result = String::with_capacity(template.len());
    let mut remaining = template;

    while let Some(start) = remaining.find('{') {
        result.push_str(&remaining[..start]);
        let after_open = &remaining[start + 1..];

        match after_open.find('}') {
            Some(end) => {
                let expr = &after_open[..end];
                if expr == "body" {
                    result.push_str(body.unwrap_or(""));
                } else {
                    let strftime_format = tokens_to_strftime(expr);
                    result.push_str(&now.format(&strftime_format).to_string());
                }
                remaining = &after_open[end + 1..];
            }
            None => {
                result.push('{');
                remaining = after_open;
            }
        }
    }

    result.push_str(remaining);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_time() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 3, 20, 14, 5, 32).unwrap()
    }

    #[test]
    fn render_empty_template() {
        assert_eq!(render("", &fixed_time(), None), "");
    }

    #[test]
    fn render_literal_text_only() {
        assert_eq!(
            render("plain/path.md", &fixed_time(), None),
            "plain/path.md"
        );
    }

    #[test]
    fn render_date_tokens() {
        assert_eq!(
            render("daily/{YYYY-MM-DD}.md", &fixed_time(), None),
            "daily/2026-03-20.md"
        );
    }

    #[test]
    fn render_body_placeholder() {
        assert_eq!(
            render("msg: {body}", &fixed_time(), Some("hello")),
            "msg: hello"
        );
    }

    #[test]
    fn render_mixed_date_and_body() {
        assert_eq!(
            render("- {YYYY-MM-DD-HH:mm}: {body}", &fixed_time(), Some("hello")),
            "- 2026-03-20-14:05: hello"
        );
    }

    #[test]
    fn render_unclosed_brace_is_literal() {
        assert_eq!(
            render("daily/{YYYY}/{broken", &fixed_time(), None),
            "daily/2026/{broken"
        );
    }

    #[test]
    fn render_body_without_body_arg_yields_empty() {
        assert_eq!(render("[{body}]", &fixed_time(), None), "[]");
    }

    #[test]
    fn mm_vs_mm_disambiguation() {
        // Lowercase `mm` is minutes (%M), uppercase `MM` is month (%m).
        // Must not cross-contaminate.
        assert_eq!(render("{mm-MM}", &fixed_time(), None), "05-03");
    }

    #[test]
    fn adjacent_mm_and_mm_do_not_cross_contaminate() {
        // `{MMmm}` → month then minute, with no separator.
        // A naive `replace("MM","%m").replace("mm","%M")` would corrupt this
        // because the `m` from `%m` glues onto the next `m` before minute
        // substitution runs.
        assert_eq!(render("{MMmm}", &fixed_time(), None), "0305");
    }
}
