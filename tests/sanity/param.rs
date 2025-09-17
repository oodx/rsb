// RSB Sanity Tests - Param Module Core Functionality Verification
// Tests verify the param module functions work as documented in FEATURES_PARAMS

use rsb::prelude::*;

#[test]
fn test_basic_param_expansion() {
    // Test basic param! macro with context variables
    set_var("TEST_VAR", "test_value");
    set_var("EMPTY_VAR", "");

    // Basic variable retrieval
    assert_eq!(param!("TEST_VAR"), "test_value");
    assert_eq!(param!("EMPTY_VAR"), "");
    assert_eq!(param!("NONEXISTENT"), ""); // Empty string for missing vars

    unset_var("TEST_VAR");
    unset_var("EMPTY_VAR");
}

#[test]
fn test_param_defaults() {
    // Test default value functionality
    set_var("SET_VAR", "existing");

    // With set variable, should return variable value
    assert_eq!(param!("SET_VAR", default: "fallback"), "existing");

    // With unset variable, should return default
    assert_eq!(param!("UNSET_VAR", default: "fallback"), "fallback");

    // Empty variable should still use default
    set_var("EMPTY_VAR", "");
    assert_eq!(param!("EMPTY_VAR", default: "fallback"), "fallback");

    unset_var("SET_VAR");
    unset_var("EMPTY_VAR");
}

#[test]
fn test_param_prefix_suffix() {
    // Test prefix and suffix removal
    set_var("FILENAME", "document.txt.bak");
    set_var("PATH", "/home/user/docs/file.txt");

    // Test suffix removal
    assert_eq!(param!("FILENAME", suffix: ".bak"), "document.txt");
    assert_eq!(param!("FILENAME", suffix: ".txt.bak"), "document");

    // Test prefix removal
    assert_eq!(param!("PATH", prefix: "/home/user/"), "docs/file.txt");
    assert_eq!(param!("PATH", prefix: "/home/"), "user/docs/file.txt");

    // Test with non-matching patterns (should return original)
    assert_eq!(param!("FILENAME", suffix: ".xyz"), "document.txt.bak");
    assert_eq!(param!("PATH", prefix: "/tmp/"), "/home/user/docs/file.txt");

    unset_var("FILENAME");
    unset_var("PATH");
}

#[test]
fn test_param_wildcard_patterns() {
    // Test wildcard-aware prefix/suffix patterns
    set_var("ARCHIVE", "logs/2025/app.error.log");
    set_var("CONFIG", "config.dev.local.json");

    // Test wildcard suffix removal
    assert_eq!(param!("ARCHIVE", suffix: "*.log"), "logs/2025/app.error");
    assert_eq!(param!("CONFIG", suffix: "*.json"), "config.dev.local");

    // Test wildcard prefix removal
    assert_eq!(param!("ARCHIVE", prefix: "*/"), "2025/app.error.log");
    assert_eq!(param!("ARCHIVE", prefix: "*/*/"), "app.error.log");

    unset_var("ARCHIVE");
    unset_var("CONFIG");
}

#[test]
fn test_param_case_transforms() {
    // Test case transformation operations
    set_var("NAME", "foo_bar_baz");
    set_var("PHRASE", "alpha BETA gamma");

    // Test upper case transforms
    assert_eq!(param!("NAME", upper: "ba*"), "foo_Bar_baz");
    assert_eq!(param!("PHRASE", upper: "BE*"), "alpha BETA gamma"); // Should match BETA

    // Test lower case transforms
    assert_eq!(param!("PHRASE", lower: "BE*"), "alpha bETA gamma");

    unset_var("NAME");
    unset_var("PHRASE");
}

#[test]
fn test_param_length_operations() {
    // Test length and substring operations
    set_var("LONG_VAR", "this_is_a_long_string");

    // Test length
    let length = param!("LONG_VAR", len);
    assert_eq!(length, 21); // Should return usize as string representation

    // Test substring operations
    assert_eq!(param!("LONG_VAR", sub: 0, 4), "this");
    assert_eq!(param!("LONG_VAR", sub: 5, 2), "is");
    assert_eq!(param!("LONG_VAR", sub: 8), "a_long_string"); // From offset to end

    unset_var("LONG_VAR");
}

#[test]
fn test_param_replace_operations() {
    // Test string replacement functionality
    set_var("TEXT", "hello world hello");

    // Test single replacement
    assert_eq!(param!("TEXT", replace: "hello" => "hi"), "hi world hello");

    // Test replace all
    assert_eq!(param!("TEXT", replace: "hello" => "hi", all), "hi world hi");

    // Test no match
    assert_eq!(param!("TEXT", replace: "xyz" => "abc"), "hello world hello");

    unset_var("TEXT");
}

#[test]
fn test_param_require_functionality() {
    // Test required variable validation
    set_var("REQ_VAR", "value");

    // Should work with set variable
    assert_eq!(param!("REQ_VAR", require: "must be set"), "value");

    // Note: Testing the failure case would require capturing stderr and exit code
    // which is complex in unit tests. This tests the success path.

    unset_var("REQ_VAR");
}

#[test]
fn test_context_operations() {
    // Test global context operations directly

    // Test variable setting and getting
    set_var("CTX_TEST", "context_value");
    assert_eq!(get_var("CTX_TEST"), "context_value");
    assert!(has_var("CTX_TEST"));

    // Test variable expansion in strings
    let expanded = expand_vars("Value is $CTX_TEST and more");
    assert_eq!(expanded, "Value is context_value and more");

    // Test complex expansion
    set_var("HOME", "/home/user");
    set_var("FILE", "config.txt");
    let complex = expand_vars("Path: $HOME/docs/$FILE");
    assert_eq!(complex, "Path: /home/user/docs/config.txt");

    // Test unsetting
    unset_var("CTX_TEST");
    assert!(!has_var("CTX_TEST"));
    assert_eq!(get_var("CTX_TEST"), "");

    unset_var("HOME");
    unset_var("FILE");
}

#[test]
fn test_bootstrap_context_loading() {
    // Test that bootstrap loads environment and XDG paths

    // After bootstrap, we should have basic environment variables
    // Note: This assumes bootstrap has been called during test setup

    // These should be available after bootstrap
    let script_name = get_var("SCRIPT_NAME");
    assert!(!script_name.is_empty());

    // XDG paths should be set up
    let xdg_config = get_var("XDG_CONFIG_HOME");
    // XDG_CONFIG_HOME might be empty if not set, but the variable should exist
    // after bootstrap sets it up
}

#[test]
fn test_param_alt_functionality() {
    // Test alternative value (returns alt only when variable is set and non-empty)
    set_var("SET_VAR", "existing");
    set_var("EMPTY_VAR", "");

    // With set variable, should return alt value
    assert_eq!(param!("SET_VAR", alt: "alternative"), "alternative");

    // With empty variable, should return empty (no alt)
    assert_eq!(param!("EMPTY_VAR", alt: "alternative"), "");

    // With unset variable, should return empty (no alt)
    assert_eq!(param!("UNSET_VAR", alt: "alternative"), "");

    unset_var("SET_VAR");
    unset_var("EMPTY_VAR");
}

#[test]
fn test_complex_param_scenarios() {
    // Test realistic parameter expansion scenarios

    // Scenario: Configuration file path handling
    set_var("USER_HOME", "/home/user");
    set_var("APP_NAME", "myapp");
    set_var("CONFIG_FILE", "config.production.json");

    // Build config path with fallback
    let config_dir = param!("XDG_CONFIG_HOME", default: "$USER_HOME/.config");
    let config_path = expand_vars("$config_dir/$APP_NAME/$CONFIG_FILE");
    assert!(config_path.contains("/myapp/config.production.json"));

    // Extract environment from config filename
    let env = param!("CONFIG_FILE", prefix: "config.");
    let env = rsb::string::str_suffix(&env, ".json", false); // Chain operations
    assert_eq!(env, "production");

    // Scenario: Log file processing
    set_var("LOG_FILE", "app.2025-01-15.error.log");

    let date_part = param!("LOG_FILE", prefix: "app.");
    let date_part = rsb::string::str_suffix(&date_part, ".error.log", false); // Chain operations
    assert_eq!(date_part, "2025-01-15");

    let extension = param!("LOG_FILE", prefix: "*.");
    assert_eq!(extension, "log");

    unset_var("USER_HOME");
    unset_var("APP_NAME");
    unset_var("CONFIG_FILE");
    unset_var("LOG_FILE");
}

#[test]
fn test_edge_cases() {
    // Test edge cases and boundary conditions

    // Test with empty variable names (should handle gracefully)
    assert_eq!(param!(""), "");

    // Test with very long variable values
    let long_value = "a".repeat(1000);
    set_var("LONG_VAR", &long_value);
    assert_eq!(param!("LONG_VAR"), long_value);
    assert_eq!(param!("LONG_VAR", sub: 0, 10), "aaaaaaaaaa");

    // Test substring beyond bounds
    assert_eq!(param!("LONG_VAR", sub: 999, 10), "a"); // Should not panic
    assert_eq!(param!("LONG_VAR", sub: 1001), ""); // Beyond length

    // Test with special characters
    set_var("SPECIAL", "Hello$World{Test}[123]");
    assert_eq!(param!("SPECIAL"), "Hello$World{Test}[123]");

    unset_var("LONG_VAR");
    unset_var("SPECIAL");
}
