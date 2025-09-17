// RSB Sanity Tests - CLI Module Core Functionality Verification
// Tests verify the CLI module functions work as documented in FEATURES_CLI

use rsb::prelude::*;

#[test]
fn test_args_basic_operations() {
    // Test Args construction and basic operations
    let test_args = vec!["program".to_string(), "arg1".to_string(), "arg2".to_string(), "--flag".to_string()];
    let args = Args::new(&test_args);

    // Test positional access (1-indexed, skips program name)
    assert_eq!(args.get(1), "arg1".to_string());
    assert_eq!(args.get(2), "arg2".to_string());
    assert_eq!(args.get(3), "--flag".to_string());
    assert_eq!(args.get(4), ""); // Empty string for missing args

    // Test get_or with defaults
    assert_eq!(args.get_or(1, "default"), "arg1");
    assert_eq!(args.get_or(10, "default"), "default");

    // Test basic properties
    assert_eq!(args.len(), 4); // Includes program name
    assert!(!args.remaining().is_empty());
}

#[test]
fn test_args_flag_operations() {
    // Test flag presence and consumption
    let test_args = vec!["program".to_string(), "--verbose".to_string(), "arg1".to_string(), "--debug".to_string()];
    let mut args = Args::new(&test_args);

    // Test flag presence
    assert!(args.has("--verbose"));
    assert!(args.has("--debug"));
    assert!(!args.has("--nonexistent"));

    // Test flag consumption
    assert!(args.has_pop("--verbose"));
    assert!(!args.has("--verbose")); // Should be consumed
    assert!(args.has("--debug")); // Should still be there
}

#[test]
fn test_args_key_value_operations() {
    // Test key=value and key:array parsing
    let test_args = vec![
        "program".to_string(),
        "config=value".to_string(),
        "paths:file1,file2,file3".to_string(),
        "--flag".to_string()
    ];
    let mut args = Args::new(&test_args);

    // Test key=value parsing
    assert_eq!(args.get_kv("config"), Some("value".to_string()));
    assert_eq!(args.get_kv("nonexistent"), None);

    // Test key:array parsing
    let paths = args.get_array("paths").unwrap();
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0], "file1");
    assert_eq!(paths[1], "file2");
    assert_eq!(paths[2], "file3");

    let empty_array = args.get_array("nonexistent");
    assert!(empty_array.is_none());
}

#[test]
fn test_args_flag_with_values() {
    // Test --flag=value and --flag value patterns
    let test_args1 = vec!["program".to_string(), "--config=test.conf".to_string()];
    let mut args1 = Args::new(&test_args1);

    assert_eq!(args1.has_val("--config"), Some("test.conf".to_string()));
    assert_eq!(args1.has_val("--nonexistent"), None);

    let test_args2 = vec!["program".to_string(), "--config".to_string(), "test.conf".to_string()];
    let mut args2 = Args::new(&test_args2);

    assert_eq!(args2.has_val("--config"), Some("test.conf".to_string()));
}

#[test]
fn test_args_expansion() {
    // Test template expansion with $1..$N, $@, $#
    set_var("TEST_VAR", "expanded");

    let test_args = vec!["program".to_string(), "first".to_string(), "second".to_string()];
    let args = Args::new(&test_args);

    // Test positional expansion
    let expanded1 = args.expand("arg is $1");
    assert_eq!(expanded1, "arg is first");

    let expanded2 = args.expand("args are $1 and $2");
    assert_eq!(expanded2, "args are first and second");

    // Test all args expansion
    let expanded_all = args.expand("all: $@");
    assert!(expanded_all.contains("first"));
    assert!(expanded_all.contains("second"));

    // Test count expansion
    let expanded_count = args.expand("count: $#");
    assert_eq!(expanded_count, "count: 2"); // Excludes program name

    // Test variable expansion
    let expanded_var = args.expand("var: $TEST_VAR");
    assert_eq!(expanded_var, "var: expanded");

    unset_var("TEST_VAR");
}

#[test]
fn test_args_utility_methods() {
    // Test remaining, all, join methods
    let test_args = vec!["program".to_string(), "arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
    let args = Args::new(&test_args);

    // Test remaining (should exclude program name)
    let remaining = args.remaining();
    assert_eq!(remaining.len(), 3);
    assert!(remaining.contains(&"arg1".to_string()));
    assert!(remaining.contains(&"arg2".to_string()));
    assert!(remaining.contains(&"arg3".to_string()));

    // Test all (should include program name)
    let all = args.all();
    assert_eq!(all.len(), 4);
    assert!(all.contains(&"program".to_string()));

    // Test join
    let joined = args.join(" ");
    assert!(joined.contains("arg1"));
    assert!(joined.contains("arg2"));
    assert!(joined.contains("arg3"));
}

#[test]
fn test_cli_bootstrap() {
    // Test CLI bootstrap functionality
    let test_args = vec!["test_program".to_string(), "--verbose".to_string()];

    // Test bootstrap from args
    rsb::cli::cli_bootstrap(&test_args);
    // Bootstrap is side-effect only, sets up global context

    // Should have set up script context
    assert!(!get_var("SCRIPT_NAME").is_empty());
}

#[test]
fn test_cli_bootstrap_from_env() {
    // Test convenience bootstrap from environment
    // This should work without panicking
    rsb::cli::cli_bootstrap_from_env();
    // Bootstrap is side-effect only

    // Should have set up script context
    assert!(!get_var("SCRIPT_NAME").is_empty());
}

#[test]
fn test_edge_cases() {
    // Test edge cases and boundary conditions

    // Test empty args
    let empty_args: Vec<String> = vec![];
    let args = Args::new(&empty_args);
    assert_eq!(args.len(), 0);
    assert_eq!(args.get(1), "");
    assert!(args.remaining().is_empty());

    // Test single program name
    let single_args = vec!["program".to_string()];
    let args = Args::new(&single_args);
    assert_eq!(args.len(), 1);
    assert_eq!(args.get(1), ""); // No args after program name
    assert!(args.remaining().is_empty());

    // Test args with spaces and special characters
    let special_args = vec!["program".to_string(), "arg with spaces".to_string(), "special=chars!@#".to_string()];
    let mut args = Args::new(&special_args);
    assert_eq!(args.get(1), "arg with spaces".to_string());
    assert_eq!(args.get_kv("special"), Some("chars!@#".to_string()));
}

#[test]
fn test_complex_scenarios() {
    // Test realistic CLI scenarios

    // Scenario: Build command with options and targets
    let build_args = vec![
        "cargo-like".to_string(),
        "build".to_string(),
        "--target".to_string(),
        "x86_64-unknown-linux-gnu".to_string(),
        "--features=visual,dev-pty".to_string(),
        "--verbose".to_string(),
        "package1".to_string(),
        "package2".to_string()
    ];
    let mut args = Args::new(&build_args);

    assert_eq!(args.get(1), "build".to_string());
    assert_eq!(args.has_val("--target"), Some("x86_64-unknown-linux-gnu".to_string()));
    assert_eq!(args.has_val("--features"), Some("visual,dev-pty".to_string()));
    assert!(args.has("--verbose"));
    assert_eq!(args.get(3), "package1".to_string());
    assert_eq!(args.get(4), "package2".to_string());

    // Scenario: Config loading with multiple formats
    let config_args = vec![
        "app".to_string(),
        "database=postgresql://localhost/app".to_string(),
        "features:logging,metrics,auth".to_string(),
        "--env=production".to_string(),
        "--dry-run".to_string()
    ];
    let mut args = Args::new(&config_args);

    assert_eq!(args.get_kv("database"), Some("postgresql://localhost/app".to_string()));
    let features = args.get_array("features").unwrap();
    assert_eq!(features, vec!["logging".to_string(), "metrics".to_string(), "auth".to_string()]);
    assert_eq!(args.has_val("--env"), Some("production".to_string()));
    assert!(args.has("--dry-run"));
}