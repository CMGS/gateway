//! Shared utilities.
//!
//! Layer L0. Grows as concrete helpers are needed. For now it carries
//! timestamp helpers using a fixed layout across the gateway.

use chrono::{DateTime, Local, Utc};

/// Canonical timestamp layout `"2006-01-02 15:04:05"` as a chrono format string.
pub const TS_LAYOUT: &str = "%Y-%m-%d %H:%M:%S";

/// Current unix seconds plus the formatted local time.
pub fn now_unix_and_formatted() -> (i64, String) {
    let now = Local::now();
    (now.timestamp(), now.format(TS_LAYOUT).to_string())
}

/// Format a UTC instant with the canonical layout.
pub fn format_ts(dt: DateTime<Utc>) -> String {
    dt.format(TS_LAYOUT).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_shape() {
        let (unix, s) = now_unix_and_formatted();
        assert!(unix > 0);
        // "YYYY-MM-DD HH:MM:SS" == 19 chars
        assert_eq!(s.len(), 19);
        assert_eq!(s.as_bytes()[4], b'-');
        assert_eq!(s.as_bytes()[13], b':');
    }
}
