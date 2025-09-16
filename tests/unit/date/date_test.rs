use rsb::prelude::*;

#[test]
fn test_date_macro_variants() {
    let any = date!();
    assert!(!any.is_empty());
    let iso = date!(iso);
    assert!(iso.contains('T'));
    let epoch = date!(epoch);
    assert!(epoch.parse::<i64>().is_ok());
    let epoch_ms = date!(epoch_ms);
    assert!(epoch_ms.parse::<i64>().is_ok());
    let human = date!(human);
    assert!(human.contains('-') && human.contains(':'));
    let utc_iso = date!(utc_iso);
    assert!(utc_iso.ends_with('Z') || utc_iso.contains('+') || utc_iso.contains('-'));
    let y = date!("%Y");
    assert!(y.len() == 4);
}

#[test]
fn test_date_helpers_basic() {
    // round-trip format/parse
    let now_ts = rsb::date::current_timestamp();
    let s = rsb::date::format_time(now_ts, "%Y-%m-%d %H:%M:%S");
    let parsed = rsb::date::parse_time(&s, "%Y-%m-%d %H:%M:%S").unwrap();
    assert!((parsed - now_ts).abs() <= 1);
}

#[test]
fn test_parse_formats_and_errors() {
    // Fractional seconds with tz
    let s = "2025-01-02 03:04:05.123 +0000";
    assert!(rsb::date::parse_time(s, "%Y-%m-%d %H:%M:%S%.3f %z").is_ok());

    // Naive local
    let s2 = "2025-01-02 03:04:05";
    assert!(rsb::date::parse_time(s2, "%Y-%m-%d %H:%M:%S").is_ok());

    // Error
    let bad = rsb::date::parse_time("bad", "%Y-%m-%d %H:%M:%S");
    assert!(bad.is_err());
}
