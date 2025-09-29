//! Sanity tests for CLI args to global functionality (v0.7.0+)

use rsb::prelude::*;

#[test]
fn test_cli_to_global_basic() {
    // Simulate command line args
    let args = vec![
        "myprogram".to_string(),
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3".to_string(),
    ];

    // Call cli_to_global directly
    rsb::cli::cli_to_global(&args);

    // Check stored values
    assert_eq!(get_var("cli_prog"), "myprogram");
    assert_eq!(get_var("cli_argv_0"), "myprogram");
    assert_eq!(get_var("cli_argc"), "3");
    assert_eq!(get_var("cli_arg_1"), "arg1");
    assert_eq!(get_var("cli_arg_2"), "arg2");
    assert_eq!(get_var("cli_arg_3"), "arg3");
    assert_eq!(get_var("cli_args"), "arg1;arg2;arg3");
    assert_eq!(get_var("cli_argv"), "myprogram;arg1;arg2;arg3");
}

#[test]
fn test_cli_to_global_no_args() {
    // Only program name
    let args = vec!["myprogram".to_string()];

    rsb::cli::cli_to_global(&args);

    assert_eq!(get_var("cli_prog"), "myprogram");
    assert_eq!(get_var("cli_argc"), "0");
    assert_eq!(get_var("cli_args"), "");
    assert_eq!(get_var("cli_argv"), "myprogram");
    assert!(!has_var("cli_arg_1"));
}

#[test]
fn test_cli_arg_macros() {
    // Set up test data
    let args = vec![
        "test".to_string(),
        "first".to_string(),
        "second".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    // Test cli_arg! macro
    assert_eq!(cli_arg!(1), "first");
    assert_eq!(cli_arg!(2), "second");
    assert_eq!(cli_arg!(3), ""); // Non-existent arg returns empty

    // Test cli_argc! macro
    assert_eq!(cli_argc!(), 2);

    // Test cli_prog! macro
    assert_eq!(cli_prog!(), "test");

    // Test cli_has_arg! macro
    assert!(cli_has_arg!(1));
    assert!(cli_has_arg!(2));
    assert!(!cli_has_arg!(3));

    // Test cli_args! macro (semicolon-separated)
    assert_eq!(cli_args!(), "first;second");

    // Test cli_argv! macro (returns Vec<String>)
    let argv = cli_argv!();
    assert_eq!(argv.len(), 2);
    assert_eq!(argv[0], "first");
    assert_eq!(argv[1], "second");
}

#[test]
fn test_cli_args_with_spaces() {
    let args = vec![
        "myapp".to_string(),
        "hello world".to_string(),
        "foo bar".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    assert_eq!(get_var("cli_arg_1"), "hello world");
    assert_eq!(get_var("cli_arg_2"), "foo bar");
    assert_eq!(get_var("cli_args"), "hello world;foo bar");
}

#[test]
fn test_cli_args_with_special_chars() {
    let args = vec![
        "/usr/bin/app".to_string(),
        "--flag=value".to_string(),
        "path/to/file.txt".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    assert_eq!(get_var("cli_prog"), "/usr/bin/app");
    assert_eq!(get_var("cli_arg_1"), "--flag=value");
    assert_eq!(get_var("cli_arg_2"), "path/to/file.txt");
}

#[test]
fn test_cli_bootstrap_integration() {
    // Simulate args
    let args = vec![
        "bootstrap_test".to_string(),
        "test_arg".to_string(),
    ];

    // Call bootstrap which should call cli_to_global internally
    rsb::cli::cli_bootstrap(&args);

    // Verify args were stored
    assert_eq!(get_var("cli_prog"), "bootstrap_test");
    assert_eq!(get_var("cli_arg_1"), "test_arg");
    assert_eq!(get_var("cli_argc"), "1");
}

#[test]
fn test_cli_argv_macro_empty() {
    // Set up with no args
    let args = vec!["program_only".to_string()];
    rsb::cli::cli_to_global(&args);

    let argv = cli_argv!();
    assert!(argv.is_empty());
}

#[test]
fn test_cli_args_1_based_indexing() {
    // Test that indexing follows bash convention (1-based)
    let args = vec![
        "bash_like".to_string(),
        "zero".to_string(),
        "one".to_string(),
        "two".to_string(),
    ];

    rsb::cli::cli_to_global(&args);

    // In bash: $0 is program, $1 is first arg
    assert_eq!(get_var("cli_argv_0"), "bash_like"); // Program name
    assert_eq!(cli_arg!(1), "zero");  // First positional arg
    assert_eq!(cli_arg!(2), "one");   // Second positional arg
    assert_eq!(cli_arg!(3), "two");   // Third positional arg
    assert_eq!(cli_arg!(4), "");      // Non-existent returns empty
}

#[test]
fn test_cli_args_overwrite() {
    // First call
    let args1 = vec!["prog1".to_string(), "arg1".to_string()];
    rsb::cli::cli_to_global(&args1);
    assert_eq!(get_var("cli_prog"), "prog1");
    assert_eq!(get_var("cli_arg_1"), "arg1");

    // Second call should overwrite
    let args2 = vec!["prog2".to_string(), "newarg".to_string()];
    rsb::cli::cli_to_global(&args2);
    assert_eq!(get_var("cli_prog"), "prog2");
    assert_eq!(get_var("cli_arg_1"), "newarg");
}