// Options macro behavior tests (non-stdopts specific)

use rsb::prelude::*;

#[test]
fn long_boolean_flag_sets_true() {
    let args = vec!["rsb-test".to_string(), "--quiet".to_string()];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    assert_eq!(get_var("opt_quiet"), "true");
}

#[test]
fn long_with_value_sets_value_and_validates_paths() {
    use assert_fs::prelude::*;
    let tmp = assert_fs::TempDir::new().unwrap();
    let file = tmp.child("config.yml");
    file.touch().unwrap();
    let p = file.path().to_string_lossy().to_string();

    // Name contains "file" to trigger path existence validation
    let args = vec![
        "rsb-test".to_string(),
        format!("--input-file={}", p),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    assert_eq!(get_var("opt_input_file"), p);
}

#[test]
fn long_space_value_is_not_consumed_boolean_only() {
    // By design: --config value does not bind value; only sets opt_config=true
    let args = vec![
        "rsb-test".to_string(),
        "--config".to_string(),
        "app.conf".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    assert_eq!(get_var("opt_config"), "true");
}

#[test]
fn comma_list_values_remain_raw_for_caller_to_split() {
    let args = vec![
        "rsb-test".to_string(),
        "--features=a,b,c".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    let features = get_var("opt_features");
    assert_eq!(features, "a,b,c");
    let parts: Vec<&str> = features.split(',').collect();
    assert_eq!(parts, vec!["a", "b", "c"]);
}

#[test]
fn not_prefix_unsets_positive_and_sets_negative_marker() {
    // Start with --quiet, then explicitly negate it
    let args = vec![
        "rsb-test".to_string(),
        "--quiet".to_string(),
        "--not-quiet".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    // Value is explicitly false on the same key
    assert_eq!(get_var("opt_quiet"), "false");
}

#[test]
fn not_prefix_alone_sets_negative_marker_only() {
    let args = vec![
        "rsb-test".to_string(),
        "--not-trace".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    assert_eq!(get_var("opt_trace"), "false");
}

#[test]
fn multi_generic_sets_letters_and_supports_inline_and_commas() {
    // Comma-delimited form
    let args = vec![
        "rsb-test".to_string(),
        "--multi=d,q".to_string(),
    ];
    let args = rsb::cli::Args::new(&args);
    options!(&args);
    assert_eq!(get_var("opt_d"), "true");
    assert_eq!(get_var("opt_q"), "true");

    // Inline with negation toggle
    let args2 = vec![
        "rsb-test".to_string(),
        "--multi=dq!ts".to_string(),
    ];
    let args2 = rsb::cli::Args::new(&args2);
    options!(&args2);
    assert_eq!(get_var("opt_d"), "true");
    assert_eq!(get_var("opt_q"), "true");
    assert_eq!(get_var("opt_t"), "false");
    assert_eq!(get_var("opt_s"), "false");
}
