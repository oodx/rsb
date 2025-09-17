// RSB UAT Tests - Date Module Visual Demonstrations
// Tests demonstrate the date functionality with visual output for user acceptance

use rsb::prelude::*;

#[test]
fn uat_date_macro_formats_demo() {
    println!("\n=== UAT: Date Macro Formats Demo ===\n");

    // Demonstrate all basic date macro patterns
    println!("🗓️  Date Macro Formats:");
    println!("   Default:     {}", date!());
    println!("   ISO:         {}", date!(iso));
    println!("   Epoch:       {}", date!(epoch));
    println!("   Epoch (ms):  {}", date!(epoch_ms));
    println!("   UTC ISO:     {}", date!(utc_iso));
    println!("   Human:       {}", date!(human));

    println!("\n📅 Custom Formats:");
    println!("   Year only:   {}", date!("%Y"));
    println!("   Month/Day:   {}", date!("%m/%d"));
    println!("   Time only:   {}", date!("%H:%M:%S"));
    println!("   Custom:      {}", date!("%A, %B %d, %Y at %I:%M %p"));
    println!("   Compact:     {}", date!("%Y%m%d_%H%M%S"));
}

#[test]
fn uat_timestamp_utilities_demo() {
    println!("\n=== UAT: Timestamp Utilities Demo ===\n");

    let ts = rsb::date::current_timestamp();
    let ts_ms = rsb::date::current_timestamp_millis();

    println!("⏰ Current Timestamps:");
    println!("   Unix timestamp (s):  {}", ts);
    println!("   Unix timestamp (ms): {}", ts_ms);
    println!(
        "   Difference ratio:    {:.1}",
        ts_ms as f64 / (ts * 1000) as f64
    );

    println!("\n🔄 Timestamp Formatting:");
    println!(
        "   Local time:  {}",
        rsb::date::format_time(ts, "%Y-%m-%d %H:%M:%S %Z")
    );
    println!(
        "   UTC time:    {}",
        rsb::date::format_time_utc(ts, "%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "   Custom:      {}",
        rsb::date::format_time(ts, "%A, %B %d at %I:%M %p")
    );

    // Demonstrate historical timestamp
    let historical = 1640995200; // 2022-01-01 00:00:00 UTC
    println!("\n📜 Historical Timestamp Demo ({}):", historical);
    println!(
        "   Local:       {}",
        rsb::date::format_time(historical, "%Y-%m-%d %H:%M:%S")
    );
    println!(
        "   UTC:         {}",
        rsb::date::format_time_utc(historical, "%Y-%m-%d %H:%M:%S")
    );
}

#[test]
fn uat_time_differences_demo() {
    println!("\n=== UAT: Time Differences Demo ===\n");

    // Demo with various time differences
    let samples = vec![
        ("1 hour ago", "2024-01-01T10:00:00Z", "2024-01-01T11:00:00Z"),
        ("1 day", "2024-01-01T12:00:00Z", "2024-01-02T12:00:00Z"),
        ("Complex", "2024-01-01T09:15:30Z", "2024-01-03T14:45:45Z"),
        (
            "Just seconds",
            "2024-01-01T12:00:00Z",
            "2024-01-01T12:00:42Z",
        ),
    ];

    println!("⏳ Time Difference Calculations:");
    for (desc, start, end) in samples {
        let diff = rsb::date::time_diff(start, end);
        println!(
            "   {:<12} {} → {}",
            desc,
            diff,
            end.split('T').next().unwrap()
        );
    }

    // Demo with invalid inputs
    println!("\n❌ Error Handling:");
    println!(
        "   Invalid input: {}",
        rsb::date::time_diff("invalid", "2024-01-01T12:00:00Z")
    );
}

#[test]
fn uat_human_readable_dates_demo() {
    println!("\n=== UAT: Human Readable Dates Demo ===\n");

    // Create sample dates for demonstration
    let samples = vec![
        ("Recent past", "2024-01-01T12:00:00Z"),
        ("Far past", "2020-06-15T09:30:00Z"),
        ("Near future", "2030-12-25T18:00:00Z"),
        ("Far future", "2050-01-01T00:00:00Z"),
    ];

    println!("👥 Human Date Descriptions:");
    for (desc, date_str) in samples {
        let human = rsb::date::human_date(date_str);
        println!(
            "   {:<12} {} ({})",
            desc,
            human,
            date_str.split('T').next().unwrap()
        );
    }

    println!("\n⏰ Time Until Dates:");
    let future_samples = vec![
        ("New Year 2030", "2030-01-01T00:00:00Z"),
        ("Mid Century", "2050-07-04T12:00:00Z"),
        ("Already past", "2020-01-01T00:00:00Z"),
    ];

    for (desc, date_str) in future_samples {
        let until = rsb::date::time_until(date_str);
        println!("   {:<14} {}", desc, until);
    }
}

#[test]
fn uat_date_parsing_demo() {
    println!("\n=== UAT: Date Parsing Demo ===\n");

    // Demo parsing various date formats
    let parse_samples = vec![
        ("ISO format", "2024-06-15T14:30:00", "%Y-%m-%dT%H:%M:%S"),
        ("US format", "06/15/2024 2:30 PM", "%m/%d/%Y %I:%M %p"),
        ("Simple date", "2024-12-25", "%Y-%m-%d"),
        ("Time only", "14:30:45", "%H:%M:%S"),
    ];

    println!("📝 Date Parsing Examples:");
    for (desc, date_str, format) in parse_samples {
        match rsb::date::parse_time(date_str, format) {
            Ok(timestamp) => {
                let formatted = rsb::date::format_time_utc(timestamp, "%Y-%m-%d %H:%M:%S UTC");
                println!(
                    "   {:<12} '{}' → {} (ts: {})",
                    desc, date_str, formatted, timestamp
                );
            }
            Err(e) => {
                println!("   {:<12} '{}' → ERROR: {}", desc, date_str, e);
            }
        }
    }

    // Demo error cases
    println!("\n❌ Parse Error Demonstrations:");
    let error_cases = vec![
        ("Bad format", "2024-13-45", "%Y-%m-%d"),
        ("Wrong pattern", "2024/06/15", "%Y-%m-%d"),
        ("Invalid time", "25:99:99", "%H:%M:%S"),
    ];

    for (desc, bad_input, format) in error_cases {
        match rsb::date::parse_time(bad_input, format) {
            Ok(_) => println!("   {:<12} Unexpected success!", desc),
            Err(e) => println!("   {:<12} '{}' → {}", desc, bad_input, e),
        }
    }
}

#[test]
fn uat_date_macro_integration_demo() {
    println!("\n=== UAT: Date Macro Integration Demo ===\n");

    println!("🔗 Integration with RSB Global Context:");

    // Demonstrate using date macro with global context
    set_var("CURRENT_DATE", &date!(iso));
    set_var("TIMESTAMP", &date!(epoch));
    set_var("READABLE_DATE", &date!(human));

    println!("   Stored in context:");
    println!("     CURRENT_DATE    = {}", get_var("CURRENT_DATE"));
    println!("     TIMESTAMP       = {}", get_var("TIMESTAMP"));
    println!("     READABLE_DATE   = {}", get_var("READABLE_DATE"));

    // Demonstrate using with param! macro
    println!("\n📋 Integration with param! macro:");
    println!("   param!(CURRENT_DATE) = {}", param!("CURRENT_DATE"));
    println!("   param!(TIMESTAMP)    = {}", param!("TIMESTAMP"));

    // Demonstrate date operations
    let current_ts = get_var("TIMESTAMP").parse::<i64>().unwrap_or(0);
    if current_ts > 0 {
        let one_hour_later = current_ts + 3600;
        println!(
            "   One hour later      = {}",
            rsb::date::format_time_utc(one_hour_later, "%Y-%m-%d %H:%M:%S UTC")
        );
    }

    println!("\n✅ Date module successfully integrated with RSB context system!");
}
