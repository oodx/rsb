// RSB Sanity Tests - Date Module Core Functionality Verification
// Tests verify the date macro and utility functions work as advertised

use rsb::prelude::*;

#[test]
fn test_date_macro_basic_formats() {
    // Test all basic date macro patterns
    let default_date = date!();
    let iso_date = date!(iso);
    let epoch_str = date!(epoch);
    let epoch_ms_str = date!(epoch_ms);
    let utc_iso_date = date!(utc_iso);
    let human_date = date!(human);

    // Basic format validation
    assert!(!default_date.is_empty());
    assert!(iso_date.contains("T")); // ISO format has T separator
    assert!(iso_date.contains("T") && (iso_date.contains("+") || iso_date.contains("-") || iso_date.contains("Z"))); // ISO format has timezone
    assert!(epoch_str.parse::<i64>().is_ok()); // Epoch should be parseable as number
    assert!(epoch_ms_str.parse::<i64>().is_ok()); // Epoch ms should be parseable
    assert!(utc_iso_date.contains("T")); // UTC ISO format has T separator
    assert!(human_date.contains("-")); // Human format has date separators
    assert!(human_date.contains(":")); // Human format has time separators

    // Ensure epoch ms is larger than epoch seconds
    let epoch_val = epoch_str.parse::<i64>().unwrap();
    let epoch_ms_val = epoch_ms_str.parse::<i64>().unwrap();
    assert!(epoch_ms_val > epoch_val * 1000 - 1000); // Allow for test execution time
}

#[test]
fn test_date_macro_custom_format() {
    // Test custom format string
    let year_only = date!("%Y");
    let time_only = date!("%H:%M:%S");
    let custom_format = date!("%Y-%m-%d_%H-%M-%S");

    // Basic validation
    assert_eq!(year_only.len(), 4); // Year should be 4 digits
    assert!(year_only.parse::<i32>().is_ok()); // Year should be numeric
    assert!(time_only.contains(":")); // Time format has colons
    assert!(custom_format.contains("_")); // Custom format has underscore
    assert!(custom_format.contains("-")); // Custom format has hyphens
}

#[test]
fn test_current_timestamp_functions() {
    // Test standalone timestamp functions
    let ts = rsb::date::current_timestamp();
    let ts_ms = rsb::date::current_timestamp_millis();

    // Basic validation
    assert!(ts > 0);
    assert!(ts_ms > 0);
    assert!(ts_ms > ts * 1000 - 1000); // ms should be roughly 1000x seconds
}

#[test]
fn test_format_time_functions() {
    // Test time formatting functions
    let ts = 1640995200; // 2022-01-01 00:00:00 UTC

    let local_formatted = rsb::date::format_time(ts, "%Y-%m-%d %H:%M:%S");
    let utc_formatted = rsb::date::format_time_utc(ts, "%Y-%m-%d %H:%M:%S");

    // Basic validation - account for timezone differences
    assert!(local_formatted.contains("2022") || local_formatted.contains("2021"));
    assert!(utc_formatted.starts_with("2022"));
    assert!(local_formatted.contains("01-01") || local_formatted.contains("12-31")); // Might be different due to timezone
    assert!(utc_formatted.contains("2022-01-01 00:00:00"));
}

#[test]
fn test_parse_time_function() {
    // Test time parsing functionality
    let time_str = "2022-01-01 12:30:45";
    let format = "%Y-%m-%d %H:%M:%S";

    match rsb::date::parse_time(time_str, format) {
        Ok(timestamp) => {
            assert!(timestamp > 0);
            // Should be roughly around Jan 1, 2022
            assert!(timestamp > 1640000000); // Rough lower bound
            assert!(timestamp < 1650000000); // Rough upper bound
        }
        Err(_) => panic!("Valid time string should parse successfully"),
    }

    // Test invalid format
    let result = rsb::date::parse_time("invalid", "%Y-%m-%d");
    assert!(result.is_err());
}

#[test]
fn test_time_diff_function() {
    // Test time difference calculation
    let start = "2022-01-01T12:00:00Z";
    let end = "2022-01-02T13:01:02Z";

    let diff = rsb::date::time_diff(start, end);

    // Should contain duration components
    assert!(diff.contains("1d")); // 1 day
    assert!(diff.contains("1h")); // 1 hour
    assert!(diff.contains("1m")); // 1 minute
    assert!(diff.contains("2s")); // 2 seconds

    // Test invalid input
    let invalid_diff = rsb::date::time_diff("invalid", "also invalid");
    assert_eq!(invalid_diff, "Invalid date format");
}

#[test]
fn test_human_date_function() {
    // Test human-readable date formatting
    let past_date = "2020-01-01T12:00:00Z";
    let future_date = "2030-01-01T12:00:00Z";

    let past_human = rsb::date::human_date(past_date);
    let future_human = rsb::date::human_date(future_date);

    // Past date should say "ago"
    assert!(past_human.contains("ago"));

    // Future date should say "in"
    assert!(future_human.contains("in"));

    // Test invalid input
    let invalid_human = rsb::date::human_date("invalid");
    assert_eq!(invalid_human, "Invalid date format");
}

#[test]
fn test_time_until_function() {
    // Test time until functionality
    let future_date = "2030-01-01T12:00:00Z";
    let past_date = "2020-01-01T12:00:00Z";

    let time_until_future = rsb::date::time_until(future_date);
    let time_until_past = rsb::date::time_until(past_date);

    // Future date should show time remaining
    assert!(time_until_future.contains("in"));

    // Past date should return "0s"
    assert_eq!(time_until_past, "0s");

    // Test invalid input
    let invalid_until = rsb::date::time_until("invalid");
    assert_eq!(invalid_until, "Invalid date format");
}

#[test]
fn test_date_consistency() {
    // Test that date functions return consistent results
    let iso1 = date!(iso);
    let iso2 = date!(iso);
    let epoch1 = date!(epoch).parse::<i64>().unwrap();
    let epoch2 = date!(epoch).parse::<i64>().unwrap();

    // Should be very close in time (within a few seconds at most)
    assert!((epoch2 - epoch1).abs() <= 2);

    // ISO dates should start with same year-month-day
    assert_eq!(iso1[..10], iso2[..10]); // YYYY-MM-DD portion should match
}