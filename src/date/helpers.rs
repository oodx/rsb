use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};

// --- Errors ---
#[derive(Debug, Clone)]
pub enum DateError {
    Parse(String),
}

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateError::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl std::error::Error for DateError {}

// Internal: parse a date string trying common formats, normalized to UTC.
fn parse_date_string(date_str: &str) -> Option<DateTime<Utc>> {
    // Try common formats with timezone first
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) { return Some(dt.with_timezone(&Utc)); }
    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) { return Some(dt.with_timezone(&Utc)); }
    if let Ok(dt) = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z") { return Some(dt.with_timezone(&Utc)); }
    if let Ok(dt) = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.3f %z") { return Some(dt.with_timezone(&Utc)); }

    // Fallback to naive local times (assume local tz) with/without fractional seconds
    if let Ok(ndt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") { return Local.from_local_datetime(&ndt).single().map(|d| d.with_timezone(&Utc)); }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.3f") { return Local.from_local_datetime(&ndt).single().map(|d| d.with_timezone(&Utc)); }

    None
}

// Internal: human-friendly duration (Xd Xh Xm Xs)
fn format_duration(duration: Duration) -> String {
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if minutes > 0 { parts.push(format!("{}m", minutes)); }
    if seconds > 0 || parts.is_empty() { parts.push(format!("{}s", seconds)); }
    parts.join(" ")
}

// --- Public helpers ---

/// Calculates the difference between two date strings and returns a human-readable string.
pub fn time_diff(start_str: &str, end_str: &str) -> String {
    if let (Some(start), Some(end)) = (parse_date_string(start_str), parse_date_string(end_str)) {
        let duration = end.signed_duration_since(start);
        format_duration(duration)
    } else { "Invalid date format".to_string() }
}

/// Converts a date string into a human-readable "time ago" string.
pub fn human_date(date_str: &str) -> String {
    if let Some(date) = parse_date_string(date_str) {
        let now = Utc::now();
        let duration = now.signed_duration_since(date);
        let formatted_duration = format_duration(duration);
        if duration > Duration::zero() { format!("{} ago", formatted_duration) }
        else { format!("in {}", format_duration(duration.abs())) }
    } else { "Invalid date format".to_string() }
}

/// Returns a human-friendly time remaining until the given date. If in the past, returns "0s".
pub fn time_until(date_str: &str) -> String {
    if let Some(target) = parse_date_string(date_str) {
        let now = Utc::now();
        if target > now { format!("in {}", format_duration(target.signed_duration_since(now))) }
        else { "0s".to_string() }
    } else { "Invalid date format".to_string() }
}

/// Returns current Unix timestamp (seconds) as i64.
pub fn current_timestamp() -> i64 { Utc::now().timestamp() }

/// Returns current Unix timestamp in milliseconds as i64.
pub fn current_timestamp_millis() -> i64 { Utc::now().timestamp_millis() }

/// Formats a Unix timestamp with a strftime-style format string in local time.
pub fn format_time(timestamp: i64, format: &str) -> String {
    let dt = Local
        .timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or_else(|| Local.timestamp_opt(0, 0).single().unwrap());
    dt.format(format).to_string()
}

/// Formats a Unix timestamp with a strftime-style format string in UTC.
pub fn format_time_utc(timestamp: i64, format: &str) -> String {
    let dt = Utc.timestamp_opt(timestamp, 0).single().unwrap_or_else(|| Utc.timestamp_opt(0, 0).single().unwrap());
    dt.format(format).to_string()
}

/// Parses a time string with a provided strftime-style format into a Unix timestamp (seconds).
pub fn parse_time(time_str: &str, format: &str) -> Result<i64, DateError> {
    match NaiveDateTime::parse_from_str(time_str, format) {
        Ok(ndt) => Ok(Local.from_local_datetime(&ndt)
            .single()
            .unwrap_or_else(|| Local.timestamp_opt(0, 0).single().unwrap())
            .with_timezone(&Utc)
            .timestamp()),
        Err(e) => Err(DateError::Parse(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_diff() {
        let start = "2025-01-01T12:00:00Z";
        let end = "2025-01-02T13:01:02Z";
        let diff = time_diff(start, end);
        assert!(diff.contains("1d") && diff.contains("1h") && diff.contains("1m") && diff.contains("2s"));
    }

    #[test]
    fn test_current_timestamp_and_format_parse() {
        let ts = current_timestamp();
        let date_str = format_time(ts, "%Y-%m-%d %H:%M:%S");
        let parsed = parse_time(&date_str, "%Y-%m-%d %H:%M:%S").unwrap();
        // parsed may differ by seconds boundary; ensure within a small range
        assert!((parsed - ts).abs() <= 1);
    }

    #[test]
    fn test_millis_and_utc_format() {
        let ms = current_timestamp_millis();
        assert!(ms > 0);
        let ts = ms / 1000;
        let s_local = format_time(ts, "%Y-%m-%d %H:%M:%S");
        let s_utc = format_time_utc(ts, "%Y-%m-%d %H:%M:%S");
        assert!(!s_local.is_empty() && !s_utc.is_empty());
    }
}
