// RSB Sanity Tests - Core Functionality Verification
// These tests verify the baseline features used in ProntoDB are working correctly.
// After discovering critical defects in RSB's param! macro, these tests ensure 
// the core functionality actually works as advertised.

use rsb::prelude::*;

#[test]
fn test_bootstrap_and_args() {
    // Test that bootstrap provides Args struct
    let test_args = vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()];
    let args = rsb::cli::Args::new(&test_args);
    
    // Args behavior: skips program name (arg[0]), starts indexing from 1
    
    assert_eq!(args.len(), 3);
    // RSB Args skips the program name (arg[0]) and starts from arg[1]
    // So args.get_or(1, "") gets the second item in the Vec, which is "arg1"
    assert_eq!(args.get_or(1, ""), "arg1");  // This is actually args[1] from the Vec
    assert_eq!(args.get_or(2, ""), "arg2");  // This is actually args[2] from the Vec
    assert_eq!(args.get_or(10, "default"), "default");
}

#[test] 
fn test_global_context_basic() {
    // Test set_var/get_var/has_var - core of RSB global context
    set_var("TEST_KEY", "test_value");
    
    assert!(has_var("TEST_KEY"));
    assert_eq!(get_var("TEST_KEY"), "test_value");
    assert!(!has_var("NONEXISTENT_KEY"));
    assert_eq!(get_var("NONEXISTENT_KEY"), "");
}

#[test]
fn test_param_basic_access() {
    // Test basic param! macro functionality
    set_var("PARAM_TEST", "hello_world");
    
    assert_eq!(param!("PARAM_TEST"), "hello_world");
    assert_eq!(param!("MISSING_VAR", default: "fallback"), "fallback");
    assert_eq!(param!("PARAM_TEST", default: "fallback"), "hello_world");
}

#[test]
fn test_param_prefix_removal_literal() {
    // Test the FIXED prefix removal functionality
    set_var("HOME_PATH", "/home/user");
    set_var("FILE_PATH", "/path/to/file.txt");
    
    assert_eq!(param!("HOME_PATH", prefix: "/home"), "/user");
    assert_eq!(param!("FILE_PATH", prefix: "/path/to"), "/file.txt");
    assert_eq!(param!("HOME_PATH", prefix: "/notfound"), "/home/user"); // No match
}

#[test]
fn test_param_suffix_removal_literal() {
    // Test the FIXED suffix removal functionality  
    set_var("FILENAME", "document.txt");
    set_var("ARCHIVE", "backup.tar.gz");
    
    assert_eq!(param!("FILENAME", suffix: ".txt"), "document");
    assert_eq!(param!("ARCHIVE", suffix: ".gz"), "backup.tar");
    assert_eq!(param!("FILENAME", suffix: ".pdf"), "document.txt"); // No match
}

#[test]
fn test_param_case_transformations() {
    // Test case transformation functionality
    set_var("WORD", "hello");
    set_var("CAPS", "WORLD");
    
    assert_eq!(param!("WORD", upper), "HELLO");
    assert_eq!(param!("CAPS", lower), "world");
    assert_eq!(param!("WORD", upper: first), "Hello");
    assert_eq!(param!("CAPS", lower: first), "wORLD");
}

#[test]
fn test_param_substring_extraction() {
    // Test substring operations
    set_var("TEXT", "abcdefghij");
    
    assert_eq!(param!("TEXT", sub: 0, 3), "abc");
    assert_eq!(param!("TEXT", sub: 3, 4), "defg");
    assert_eq!(param!("TEXT", sub: 5), "fghij");
    assert_eq!(param!("TEXT", len), 10);
}

#[test]
fn test_param_pattern_replacement() {
    // Test string replacement functionality
    set_var("PATH_VAR", "/path/to/file");
    set_var("MULTI", "a,b,c,a");
    
    assert_eq!(param!("PATH_VAR", replace: "/" => "_"), "_path/to/file");
    assert_eq!(param!("PATH_VAR", replace: "/" => "_", all), "_path_to_file");
    assert_eq!(param!("MULTI", replace: "a" => "X"), "X,b,c,a");
    assert_eq!(param!("MULTI", replace: "a" => "X", all), "X,b,c,X");
}

#[test]
fn test_param_alt_patterns() {
    // Test alternative value patterns
    set_var("HAS_VALUE", "content");
    set_var("EMPTY_VALUE", "");
    
    assert_eq!(param!("HAS_VALUE", alt: "alternative"), "alternative");
    assert_eq!(param!("EMPTY_VALUE", alt: "alternative"), "");
    assert_eq!(param!("MISSING_VAR", alt: "alternative"), "");
}

#[test]
fn test_to_number_conversions() {
    // Test number conversion functionality
    assert_eq!(to_number!("42"), 42);
    assert_eq!(to_number!("0"), 0);
    assert_eq!(to_number!("-123"), -123);
    assert_eq!(to_number!("invalid"), 0);
    assert_eq!(to_number!("invalid", default: 99), 99);
    assert_eq!(to_number!("  123  "), 123); // Trimming
}

#[test]
fn test_date_functions() {
    // Test date macro functionality (basic smoke test)
    let iso_date = date!(iso);
    let epoch_str = date!(epoch);
    let human_date = date!(human);
    
    // Basic format validation
    assert!(iso_date.contains("T")); // ISO format has T separator
    assert!(epoch_str.parse::<i64>().is_ok()); // Epoch should be parseable as number
    assert!(human_date.contains("-")); // Human format has date separators
    assert!(human_date.contains(":")); // Human format has time separators
}

#[test]
fn test_token_stream_validation() {
    // Test token stream validation (used in options processing)
    set_var("COMMA_TOKENS", "k1=v1,k2=v2,k3=v3");
    set_var("SEMICOLON_TOKENS", "k1=v1;k2=v2;k3=v3");
    set_var("NOT_TOKENS", "just some text");
    
    assert!(is_token_stream(&get_var("COMMA_TOKENS")));
    assert!(is_token_stream(&get_var("SEMICOLON_TOKENS")));
    assert!(!is_token_stream(&get_var("NOT_TOKENS")));
}

#[test]
fn test_options_macro_integration() {
    // Test that options! macro works with Args and populates global context
    let test_args = vec![
        "program".to_string(),
        "command".to_string(), 
        "--verbose".to_string(),
        "--config=test.conf".to_string(),
        "-d".to_string(),
        "--layout=k1=v1,k2=v2".to_string()
    ];
    
    let args = rsb::cli::Args::new(&test_args);
    options!(&args);
    
    // Verify options were parsed and stored
    assert_eq!(get_var("opt_verbose"), "true");
    assert_eq!(get_var("opt_config"), "test.conf");
    assert_eq!(get_var("opt_d"), "true");
    assert_eq!(get_var("opt_layout"), "k1=v1,k2=v2");
    
    // Verify token stream recognition
    assert!(is_token_stream(&get_var("opt_layout")));
}

#[test]
fn test_prefix_suffix_wildcard_patterns() {
    // Test wildcard pattern support in prefix/suffix removal
    set_var("PATTERN_FILE", "document.backup.txt");
    set_var("PATTERN_PATH", "src/main.rs");
    
    // These should work with the regex conversion we added
    assert_eq!(param!("PATTERN_FILE", suffix: "*.txt"), "document.backup");
    assert_eq!(param!("PATTERN_FILE", suffix: "*.backup.txt"), "document");
    
    // Note: Prefix wildcard patterns are more complex, testing basic case
    // The regex conversion handles simple wildcards like */filename patterns
}

#[test] 
fn test_environment_to_global_context() {
    // Test that environment variables are accessible through param!
    // (This simulates what bootstrap! does with get_env!)
    
    // Simulate environment variables being loaded
    set_var("HOME", "/test/home");
    set_var("USER", "testuser");
    set_var("PATH", "/usr/bin:/bin");
    
    // Should be accessible via param!
    assert_eq!(param!("HOME"), "/test/home");
    assert_eq!(param!("USER"), "testuser");  
    assert_eq!(param!("PATH", sub: 0, 8), "/usr/bin");
    
    // With defaults
    assert_eq!(param!("MISSING_ENV", default: "default_value"), "default_value");
}

#[test]
fn test_comprehensive_parameter_expansion() {
    // Integration test combining multiple param! features
    set_var("COMPLEX_VAR", "/home/user/project/file.backup.txt");
    
    // Chain of operations
    let step1 = param!("COMPLEX_VAR", prefix: "/home/user"); // "/project/file.backup.txt"
    set_var("STEP1", &step1);
    
    let step2 = param!("STEP1", suffix: ".txt"); // "/project/file.backup"
    set_var("STEP2", &step2);
    
    let step3 = param!("STEP2", replace: "/" => "_"); // "_project/file.backup"
    set_var("STEP3", &step3);
    
    assert_eq!(step1, "/project/file.backup.txt");
    assert_eq!(step2, "/project/file.backup");
    assert_eq!(step3, "_project/file.backup");
    
    // Length check
    assert!(param!("COMPLEX_VAR", len) > 20);
}
