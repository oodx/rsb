// UAT: Date macros and helpers demo (prints sample outputs)
use rsb::prelude::*;

#[test]
fn uat_date_showcase() {
    println!("UAT: date macros");
    println!("  now        : {}", date!());
    println!("  iso        : {}", date!(iso));
    println!("  epoch      : {}", date!(epoch));
    println!("  human      : {}", date!(human));
    println!("  year (%Y)  : {}", date!("%Y"));

    println!("\nUAT: date helpers");
    let now_ts = rsb::date::current_timestamp();
    let fmt = rsb::date::format_time(now_ts, "%Y-%m-%d %H:%M:%S");
    println!("  format_time: {} -> {}", now_ts, fmt);

    // Build a time 2 minutes in the past and 2 minutes in the future
    let past_ts = now_ts - 120;
    let future_ts = now_ts + 120;
    let past_str = rsb::date::format_time(past_ts, "%Y-%m-%d %H:%M:%S %z");
    let future_str = rsb::date::format_time(future_ts, "%Y-%m-%d %H:%M:%S %z");

    println!("  human_date : {} -> {}", past_str, rsb::date::human_date(&past_str));
    println!("  time_until : {} -> {}", future_str, rsb::date::time_until(&future_str));

    // Sanity asserts
    assert!(date!(iso).contains('T'));
    assert!(date!(human).contains(':'));
    assert!(date!(epoch).parse::<i64>().is_ok());
}

